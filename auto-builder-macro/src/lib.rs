use auto_builder_core::BuilderBuilder;
use proc_macro::TokenStream;
use syn::DeriveInput;

/// Implement the builder pattern for the target struct
#[proc_macro_derive(Builder)]
pub fn builder_derive_macro(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(item).unwrap();
    BuilderBuilder::new(ast).build_builder().into()
}
