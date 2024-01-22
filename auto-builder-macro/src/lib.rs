use auto_builder_core::{BasicBuilderBuilder, BuilderBuilder};
use proc_macro::TokenStream;
use syn::DeriveInput;

/// Implement the builder pattern for the target struct
#[proc_macro_derive(Builder, attributes(builder))]
pub fn builder_derive_macro(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(item).unwrap();
    let should_default = ast
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("builder"))
        .any(|attr| {
            if let Ok(value) = attr.parse_args::<proc_macro2::TokenStream>() {
                value.to_string() == "default"
            } else {
                false
            }
        });

    match should_default {
        true => BasicBuilderBuilder::new(ast).build_builder_default().into(),
        false => BasicBuilderBuilder::new(ast).build_builder().into(),
    }
}
