# variadic_generics

[![](http://meritbadge.herokuapp.com/variadic_generics)](https://crates.io/crates/variadic_generics)

A first attempt in using traits & tuples to work around Rusts lack of variadic generics.

## [link to API documentation](https://docs.rs/variadic_generics)

## example usage

Add to your Cargo.toml:

```toml
[dependencies]
variadic_generics = "0.1"
```

Add to your crate root:

```rust
#[macro_use]
extern crate variadic_generics;
```

* [option_tuple](examples/option_tuple.rs) showcases how to implement
  `(Option<T>...) -> Option<(T...)>`

## contribution guidelines

Make sure to `cargo install rustfmt` and `cargo fmt` the codebase before creating any commits!

