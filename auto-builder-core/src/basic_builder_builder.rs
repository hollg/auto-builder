use crate::builder_builder::BuilderBuilder;
use proc_macro2::TokenStream;
use syn::{punctuated::Punctuated, token::Comma, DeriveInput, Field};

pub struct BasicBuilderBuilder {
    /// The AST of the struct we're deriving the builder for
    ast: DeriveInput,
}

// A basic builder builder that handles non-generic target structs
impl BasicBuilderBuilder {
    pub fn new(ast: DeriveInput) -> Self {
        Self { ast }
    }

    fn ident(&self) -> &syn::Ident {
        &self.ast.ident
    }

    fn builder_name(&self) -> syn::Ident {
        quote::format_ident!("{}Builder", self.ident())
    }

    fn builder_fields(&self) -> Vec<TokenStream> {
        self.target_fields()
            .iter()
            .map(|field| {
                let field_name = field.ident.as_ref().expect("Expected field name");
                let field_type = &field.ty;
                quote::quote!(
                    #field_name: Option::<#field_type>,
                )
            })
            .collect::<Vec<_>>()
    }

    fn builder_field_values(&self) -> Vec<TokenStream> {
        self.target_fields()
            .iter()
            .map(|field| {
                let field_name = field.ident.as_ref().expect("Expected field name");
                quote::quote!(
                    #field_name: None,
                )
            })
            .collect()
    }

    fn builder_field_values_default(&self) -> Vec<TokenStream> {
        self.target_fields()
            .iter()
            .map(|field| {
                let field_name = field.ident.as_ref().expect("Expected field name");
                quote::quote!(
                    #field_name: Some(Default::default()),
                )
            })
            .collect()
    }

    fn builder_methods(&self) -> Vec<TokenStream> {
        self.target_fields()
            .iter()
            .map(|field| {
                let field_name = field.ident.as_ref().expect("Expected field name");
                let field_type = &field.ty;
                quote::quote!(
                    fn #field_name(&mut self, value: #field_type) -> &mut Self {
                        self.#field_name = Some(value);
                        self
                    }
                )
            })
            .collect()
    }

    fn builder_build_checks(&self) -> Vec<TokenStream> {
        self.target_fields().iter().map(|field| {
            let field_name = field.ident.as_ref().expect("Expected field name");
            quote::quote!(
                if self.#field_name.is_none() {
                    return Err(format!("Expected field to be set: {}", stringify!(#field_name).to_string()));
                }
            )
        }).collect::<Vec<_>>()
    }

    fn target_fields(&self) -> &Punctuated<Field, Comma> {
        match self.ast.data {
            syn::Data::Struct(syn::DataStruct {
                fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
                ..
            }) => named,
            _ => panic!("Builder macro only supports structs with named fields"),
        }
    }

    fn target_field_values(&self) -> Vec<TokenStream> {
        self.target_fields().iter().map(|field| {
            let field_name = field.ident.as_ref().expect("Expected field name");
            quote::quote!(
                #field_name: self.#field_name.clone().expect(format!("Expected field to be set: {}", stringify!(#field_name).to_string()).as_str()),
            )
        }).collect()
    }
}

impl BuilderBuilder for BasicBuilderBuilder {
    fn build_builder(&self) -> TokenStream {
        let ident = &self.ast.ident;

        // Generate builder struct
        let builder_struct_name = self.builder_name();
        let builder_fields = self.builder_fields();
        let builder_field_values = self.builder_field_values();
        let builder_methods = self.builder_methods();
        let instance_field_values = self.target_field_values();
        let checks_for_builder = self.builder_build_checks();

        quote::quote!(
            #[derive(Clone, Debug)]
            struct #builder_struct_name {
                #(#builder_fields)*
            }

            #[allow(dead_code)]
            impl #builder_struct_name {
                fn new() -> #builder_struct_name {
                    #builder_struct_name {
                        #(#builder_field_values)*
                    }
                }

                #(#builder_methods)*

                fn build(&self) -> Result<#ident, String> {
                    #(#checks_for_builder)*
                    Ok(#ident {
                        #(#instance_field_values)*
                    })
                }
            }

        )
    }

    fn build_builder_default(&self) -> TokenStream {
        let ident = &self.ast.ident;

        // Generate builder struct
        let builder_struct_name = self.builder_name();
        let builder_fields = self.builder_fields();
        let builder_field_values = self.builder_field_values_default();
        let builder_methods = self.builder_methods();
        let instance_field_values = self.target_field_values();
        let checks_for_builder = self.builder_build_checks();

        quote::quote!(
            #[derive(Clone, Debug)]
            struct #builder_struct_name {
                #(#builder_fields)*
            }

            #[allow(dead_code)]
            impl #builder_struct_name {
                fn new() -> #builder_struct_name {
                    #builder_struct_name {
                        #(#builder_field_values)*
                    }
                }

                #(#builder_methods)*

                fn build(&self) -> Result<#ident, String> {
                    #(#checks_for_builder)*
                    Ok(#ident {
                        #(#instance_field_values)*
                    })
                }
            }

        )
    }
}
