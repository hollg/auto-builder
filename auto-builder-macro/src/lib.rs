use auto_builder_core::{BasicBuilderBuilder, BuilderBuilder, GenericBuilderBuilder};
use proc_macro::TokenStream;
use syn::DeriveInput;

/// Implement the builder pattern for the target struct
#[proc_macro_derive(Builder, attributes(builder))]
pub fn builder_derive_macro(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(item).unwrap();
    let should_default = ast.attrs.iter().any(|attr| {
        attr.path().is_ident("builder")
            && if let Ok(value) = attr.parse_args::<proc_macro2::TokenStream>() {
                value.to_string() == "default"
            } else {
                false
            }
    });

    let is_generic = ast.generics.params.iter().any(|param| {
        matches!(param, syn::GenericParam::Type(_))
    });

    match (should_default, is_generic) {
        (true, true) => {
            let builder_builder = GenericBuilderBuilder::<i32>::new(ast);
            builder_builder.build_builder_default().into()
        }
        (false, true) => {
            let builder_builder = GenericBuilderBuilder::<i32>::new(ast);
            builder_builder.build_builder().into()
        }
        (true, false) => {
            let builder_builder = BasicBuilderBuilder::new(ast);
            builder_builder.build_builder_default().into()
        }
        (false, false) => {
            let builder_builder = BasicBuilderBuilder::new(ast);
            builder_builder.build_builder().into()
        }
    }
}
