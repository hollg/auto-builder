use auto_builder::Builder;

#[test]
fn returns_err_if_not_all_fields_set() {
    #[derive(Builder)]
    struct Foo {
        _a: i32,
        _b: Option<i32>,
    }

    let _foo = FooBuilder::new()._a(1).build();
    assert!(_foo.is_err());
}

#[test]
fn returns_ok_if_all_fields_set() {
    #[derive(Builder)]
    struct Foo {
        _a: i32,
        _b: Option<i32>,
    }

    let _foo = FooBuilder::new()._a(1)._b(Some(2)).build();
    assert!(_foo.is_ok());

    let instance = _foo.unwrap();
    assert!(instance._a == 1);
    assert!(instance._b == Some(2));

}
