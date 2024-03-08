#[proc_macro_derive(Data,)]
pub fn data_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    #[cfg(feature = "debug_log")]
    eprintln!("{:#?}", input);

    let output = data_macro_derive_impl(input);
    #[cfg(feature = "debug_log")]
    eprintln!("{}", output);

    proc_macro::TokenStream::from(output)
}

fn data_macro_derive_impl(input: syn::DeriveInput) -> proc_macro2::TokenStream {
    let ident = &input.ident;
    let (fields, delimiter) = match input {
        syn::DeriveInput {
            data: syn::Data::Struct(
                syn::DataStruct {
                    fields: syn::Fields::Named(
                        syn::FieldsNamed {
                            named: ref fields,
                             ..
                        }
                    ),
                    ..
                }
            ),
            ..
        } => (fields.iter().collect(), proc_macro2::Delimiter::Brace,),
        syn::DeriveInput {
            data: syn::Data::Struct(
                syn::DataStruct {
                    fields: syn::Fields::Unnamed(
                        syn::FieldsUnnamed {
                            unnamed: ref fields,
                            ..
                        }
                    ),
                    ..
                }
            ),
            ..
        } => (fields.iter().collect(), proc_macro2::Delimiter::Parenthesis,),
        _ => (vec![], proc_macro2::Delimiter::None,),
    };
    let default_fields: proc_macro2::TokenStream = fields
        .iter()
        .map(|field| {
            let prefix = if let Some(ref ident) = field.ident {
                quote::quote!(#ident:)
            } else {
                quote::quote!()
            };
            quote::quote! {
                #prefix ::std::default::Default::default(),
            }
        })
        .collect();
    let field_group = proc_macro2::Group::new(delimiter, default_fields);
    quote::quote! {
        impl ::std::default::Default for #ident {
            fn default() -> Self {
                Self #field_group
            }
        }
    }
}
