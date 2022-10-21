<!-- cargo-sync-readme start -->

# `antlion`

A magical _meta_ function that evaluate (at compile-time if used inside a
macro which is the point of taking a `TokenStream` input) any Rust expr!

```rust
use antlion::Sandbox;
use quote::quote;

let test = Sandbox::new().unwrap();
let x: u32 = test.eval(quote! { 2 + 2 }).unwrap();
assert!(x == 4);
```

This library indeed is not what would benefit the most your crate build
time, but it was still design in mind with the will of caching sandbox
compilation.

<!-- cargo-sync-readme end -->
