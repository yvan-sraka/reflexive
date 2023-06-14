//! # `reflexive`
//!
//! A magical _meta_ function that evaluate (at compile-time if used inside a
//! macro which is the point of taking a `TokenStream` input) any Rust expr!
//!
//! ## Example
//!
//! ```rust
//! use reflexive::Sandbox;
//! use quote::quote;
//!
//! let test = Sandbox::new("calc").unwrap();
//! let x: u32 = test.eval(quote! { 2 + 2 }).unwrap();
//! assert!(x == 4);
//! ```
//!
//! This library indeed is not what would benefit the most your crate build
//! time, but it was still design in mind with the will of caching sandbox
//! compilation.
//!
//! ## Acknowledgments
//!
//! ⚠️ This is still a working experiment, not yet production ready.
//!
//! This project was part of a work assignment as an
//! [IOG](https://github.com/input-output-hk) contractor.
//!
//! ## License
//!
//! Licensed under either of [Apache License](LICENSE-APACHE), Version 2.0 or
//! [MIT license](LICENSE-MIT) at your option.
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted
//! for inclusion in this project by you, as defined in the Apache-2.0 license,
//! shall be dual licensed as above, without any additional terms or conditions.

#![forbid(unsafe_code)]

use proc_macro2::TokenStream;
use quote::quote;
use std::io::{Result, Write};
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
use std::sync::Mutex;
use std::{env, fs, io};

/// Internal representation of a `Sandbox`
///
/// A `Sandbox` is a throwable Cargo project made to evaluate arbitrary Rust
/// expression.
#[non_exhaustive]
pub struct Sandbox {
    lock: Mutex<()>,
    root_dir: PathBuf,
}

impl Sandbox {
    /// Create a `Sandbox` in `$OUT_DIR` folder
    ///
    /// `$OUT_DIR` is set by Cargo when `build.rs` is present :)
    pub fn new(uuid: &str) -> Result<Self> {
        let out_dir = env!("OUT_DIR");
        let mut root_dir = PathBuf::from(out_dir);
        root_dir.push(uuid);
        Command::new("mkdir")
            .args(["-p", root_dir.to_str().unwrap()])
            .output()?;
        Command::new("cargo")
            .current_dir(&root_dir)
            .args(["new", "sandbox"])
            .args(["--vcs", "none"])
            .output()?;

        // add [workspace] in Cargo.toml if not present
        let mut cargo_toml_path = root_dir.clone();
        cargo_toml_path.push("sandbox/Cargo.toml");
        if !fs::read_to_string(&cargo_toml_path)?.contains("[workspace]") {
            fs::OpenOptions::new()
                .append(true)
                .open(cargo_toml_path)?
                .write_all(b"\n[workspace]\n")?;
        }

        root_dir.push("sandbox");
        Ok(Sandbox {
            root_dir,
            lock: Mutex::new(()),
        })
    }

    /// Rely on `cargo add` to install dependencies in your sandbox
    ///
    /// https://doc.rust-lang.org/cargo/commands/cargo-add.html
    pub fn deps(self, deps: &[&str]) -> Result<Self> {
        let Self { lock, root_dir } = &self;
        let lock = lock.lock().unwrap();
        for dep in deps {
            Command::new("cargo")
                .args(["add", dep])
                .current_dir(root_dir)
                .output()?;
        }
        drop(lock);
        Ok(self)
    }

    /// Evaluate in the Sandbox a given Rust expression
    ///
    /// `quote! { }` would help you to generate a `proc_macro2::TokenStream`
    pub fn eval<T: FromStr + ToString>(&self, expr: TokenStream) -> Result<T> {
        let Self { lock, root_dir } = self;
        let _lock = lock.lock().unwrap();
        let wrapper = quote! {
            use std::io::prelude::*;
            fn main() -> std::io::Result<()> {
                let mut file = std::fs::File::create("output")?;
                let output = { #expr }.to_string();
                file.write_all(output.as_bytes())?;
                Ok(())
            }
        };
        fs::write(root_dir.join("src/main.rs"), wrapper.to_string())?;
        Command::new("cargo")
            .arg("run")
            .current_dir(root_dir)
            .output()?;
        let output = fs::read_to_string(root_dir.join("output"))?
            .parse()
            .or(Err(io::ErrorKind::Other))?;
        fs::remove_file(root_dir.join("output"))?;
        Ok(output)
    }
}
