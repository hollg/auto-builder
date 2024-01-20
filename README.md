# auto-builder

This crate provides a derive macro that implements the builder pattern for any struct.

```rs
#[derive(Builder)]
struct Foo {           
    a: i32,
    b: Option<i32>,
 }

let foo = FooBuilder::new().a(1).b(Some(2)).build();
assert!(foo.is_ok());

```
