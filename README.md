![Crates.io Version](https://img.shields.io/crates/v/auto-builder) 
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/hollg/auto-builder/test.yml?branch=main&event=push&style=flat&label=tests)



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
