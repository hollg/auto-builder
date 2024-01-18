use proc_macro::TokenStream;
use syn::DeriveInput;

/// Implement the builder pattern for the target struct
#[proc_macro_derive(Builder)]
pub fn builder_derive_macro(item: TokenStream) -> TokenStream {
    // parse
    let ast: DeriveInput = syn::parse(item).unwrap();

    // generate
    impl_builder_struct(ast)
}

fn impl_builder_struct(ast: DeriveInput) -> TokenStream {
    // Get identifier of target struct
    let ident = ast.ident;

    // Get fields of target struct
    let fields = match ast.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => panic!("Builder macro only supports structs with named fields"),
    };

    // Generate builder struct
    let builder_struct_name = quote::format_ident!("{}Builder", ident);

    let builder_fields = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().expect("Expected field name");
        let field_type = &field.ty;
        quote::quote!(
            #field_name: Option::<#field_type>,
        )
    });

    let builder_field_values = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().expect("Expected field name");
        quote::quote!(
            #field_name: None,
        )
    });

    let builder_methods = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().expect("Expected field name");
        let field_type = &field.ty;
        quote::quote!(
            fn #field_name(&mut self, value: #field_type) -> &mut Self {
                self.#field_name = Some(value);
                self
            }
        )
    });

    let instance_field_values = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().expect("Expected field name");
        quote::quote!(
            #field_name: self.#field_name.clone().expect(format!("Expected field to be set: {}", stringify!(#field_name).to_string()).as_str()),
        )
    });

    quote::quote!(
        #[derive(Debug)]
        struct #builder_struct_name {
            #(#builder_fields)*
        }

        impl #builder_struct_name {

            fn new() -> #builder_struct_name {
                #builder_struct_name {
                    #(#builder_field_values)*
                }
            }

            #(#builder_methods)*

            fn build(&self) -> #ident {
                #ident {
                    #(#instance_field_values)*
                }
            }
        }

    )
    .into()

    // Generate builder methods
}
