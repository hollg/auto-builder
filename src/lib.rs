use builder_derive::Builder;

#[derive(Builder, Debug)]
struct Foo {
    a: i32,
}

fn main() {
    let mut foo_builder = FooBuilder::new();
    foo_builder.a(1);

    let foo = foo_builder.build();
    dbg!(foo);
    
}

// test main
#[test]
fn test_main() {
    main();
}