use auto_builder::Builder;

#[test]
fn supports_struct_with_single_generic_param() {
    #[derive(Builder)]
    struct Foo<T> {
        _a: T,
    }
    let mut builder: FooBuilder<i32> = FooBuilder::new();
    let _foo = builder._a(1).build::<i32>();
    assert!(_foo.is_ok());

    let instance = _foo.unwrap();
    assert!(instance._a == 1);
}

struct Foo<T> {
    a: T,
}

impl<T> Foo<T> {
    fn new(a: T) -> Self {
        Self { a }
    }
}

struct FooBuilder<T> {
    a: Option<T>,
}

impl<T> FooBuilder<T> {
    fn new() -> Self {
        Self { a: None }
    }

    fn _a(mut self, a: T) -> Self {
        self.a = Some(a);
        self
    }

    fn build(self) -> Result<Foo<T>, String> {
        let a = self.a.ok_or_else(|| "a not set".to_string())?;
        Ok(Foo::new(a))
    }
}
