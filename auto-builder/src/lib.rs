//! This crate provides a derive macro to implement the [builder pattern](https://en.wikipedia.org/wiki/Builder_pattern) for any struct.
//!
//! Deriving the `Builder` macro for a struct named `Foo` will generate a builder struct named `FooBuilder` with methods to set each field of `Foo` and a `build` method to create a `Foo` from the builder. The `build` method will fail if any fields have not been set. It returns a `Result<Foo, String>` where the `String` is an error message indicating which fields have not been set.
//!
//! ## Example
//! ```
//! use auto_builder::Builder;
//!
//! #[derive(Builder)]
//! struct Foo {
//!   bar: i32,
//!   baz: String,
//! }
//!
//! let complete_foo = FooBuilder::new().bar(42).baz("hello".to_string()).build();
//! let incomplete_foo = FooBuilder::new().bar(42).build();
//!
//! assert!(complete_foo.is_ok());
//! assert!(incomplete_foo.is_err());
//! ```

pub use auto_builder_macro::Builder;
