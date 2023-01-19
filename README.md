<!-- cargo-sync-readme start -->

# `antlion`

A magical _meta_ function that evaluate (at compile-time if used inside a
macro which is the point of taking a `TokenStream` input) any Rust expr!

## Example

```rust
use antlion::Sandbox;
use quote::quote;

let test = Sandbox::new("calc").unwrap();
let x: u32 = test.eval(quote! { 2 + 2 }).unwrap();
assert!(x == 4);
```

This library indeed is not what would benefit the most your crate build
time, but it was still design in mind with the will of caching sandbox
compilation.

## Acknowledgments

⚠️ This is still a working experiment, not yet production ready.

This project was part of a work assignment as an
[IOG](https://github.com/input-output-hk) contractor.

## License

Licensed under either of [Apache License](LICENSE-APACHE), Version 2.0 or
[MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.

<!-- cargo-sync-readme end -->
