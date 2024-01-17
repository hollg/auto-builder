use proc_macro::TokenStream;
use syn::DeriveInput;

/// Implement the builder pattern for the target struct
#[proc_macro_derive(Builder)]
pub fn builder_derive_macro(item: TokenStream) -> TokenStream {
    // parse
    let ast: DeriveInput = syn::parse(item).unwrap();

    // generate
    impl_builder_trait(ast)
}

fn impl_builder_trait(ast: DeriveInput) -> TokenStream {
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

    // Clone the iterator to avoid using it after it's been moved
    // TODO: figure out how to avoid this clone
    // let builder_fields_clone = builder_fields.clone();

    quote::quote!(
        #[derive(Debug)]
        struct #builder_struct_name {
            // #(#builder_fields)*
        }

        impl #builder_struct_name {
            fn build(&self) -> #ident {
                #ident {
                    #(#builder_fields)*
                }
            }
        }

    )
    .into()

    // Generate builder methods
}
