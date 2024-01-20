pub use auto_builder_macro::Builder;

#[cfg(test)]
mod test {
    use super::*;
    // test main
    #[test]
    fn returns_err_if_not_all_fields_set() {
        #[derive(Builder)]
        struct Foo {
            _a: i32,
            _b: Option<i32>,
        }

        let foo = FooBuilder::new()._a(1).build();
        assert!(foo.is_err());
    }

    #[test]
    fn returns_ok_if_all_fields_set() {
        #[derive(Builder)]
        struct Foo {
            _a: i32,
            _b: Option<i32>,
        }

        let foo = FooBuilder::new()._a(1)._b(Some(2)).build();
        assert!(foo.is_ok());
    }
}
