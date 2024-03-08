#[proc_macro_derive(Data,)]
pub fn data_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    // Permet de débugger l'instance de syn::DeriveInput
    #[cfg(feature = "debug")]
    eprintln!("{:#?}", input);

    let output = data_macro_derive_impl(input);

    // Permet de débugger le code généré par la macro
    #[cfg(feature = "debug")]
    eprintln!("{}", output);

    output.into()
}

/// Génère le flux de token de manière testable
/// car indépendant de proc_macro !
fn data_macro_derive_impl(input: syn::DeriveInput) -> proc_macro2::TokenStream {
    let ident = &input.ident;
    let (fields, delimiter) = match input.data {
        syn::Data::Struct(ref data) => (
            data.fields.iter().collect::<Vec<_>>(),
            match data.fields {
                syn::Fields::Named(_) => proc_macro2::Delimiter::Brace,
                syn::Fields::Unnamed(_) => proc_macro2::Delimiter::Parenthesis,
                syn::Fields::Unit => proc_macro2::Delimiter::None,
            },
        ),
        syn::Data::Enum(_) => panic!("enum are not supported!"),
        syn::Data::Union(_) => panic!("union are not supported!"),
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
