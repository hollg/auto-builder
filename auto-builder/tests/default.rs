use auto_builder::Builder;

#[test]
fn uses_default_values_for_uninitialised_fields_on_structs_that_implement_default() {
    #[derive(Builder, Default)]
    #[builder(default)]
    struct Foo {
        _a: i32,
        _b: Option<i32>,
    }

    let _foo = FooBuilder::new()._a(1).build();
    assert!(_foo.is_ok());

    let instance = _foo.unwrap();
    assert!(instance._a == 1);
    assert!(instance._b.is_none());
}
