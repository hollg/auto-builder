use builder_derive::Builder;

#[derive(Builder)]
struct Foo {
    a: i32,
}

fn main() {
    let foo_builder: FooBuilder = FooBuilder::new();
    dbg!(foo_builder);
    
}

// test main
#[test]
fn test_main() {
    main();
}