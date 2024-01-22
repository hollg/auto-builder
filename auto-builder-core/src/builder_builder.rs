use proc_macro2::TokenStream;

/// A trait for building a `proc_macro2::TokenStream` representing a builder for some target struct.
pub trait BuilderBuilder {
    /// Builds a builder which requires all fields to be set explicitly and returns the generated `TokenStream`.
    fn build_builder(&self) -> TokenStream;

    /// Builds a builder that sets all uninitialized fields to their default values and returns the generated `TokenStream`.
    fn build_builder_default(&self) -> TokenStream;
}
