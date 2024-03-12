use quote::quote;

use crate::model;

pub fn generate(data: model::Data) -> proc_macro2::TokenStream {
    let ident = data.ident;
    let ident_str = ident.to_string();
    let debug_method = match data.format {
        model::StructFormat::Named => quote::format_ident!("debug_struct"),
        model::StructFormat::Tuple => quote::format_ident!("debug_tuple"),
    };
    let default_fields: proc_macro2::TokenStream = data.fields.content
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
    let debug_fields: proc_macro2::TokenStream = data.fields.content
        .iter()
        .map(|field| {
            if let Some(false) = field.options.debug {
                return quote!();
            }
            if let Some(ref ident) = field.ident {
                let ident_str = ident.to_string();
                quote::quote! {
                    .field(#ident_str, &self.#ident)
                }
            } else {
                let ident = proc_macro2::Literal::usize_unsuffixed(field.ordinal);
                quote::quote! {
                    .field(&self.#ident)
                }
            }
        })
        .collect();
    let field_group = proc_macro2::Group::new(data.fields.delimiter, default_fields);
    quote::quote! {
        impl ::std::default::Default for #ident {
            fn default() -> Self {
                Self #field_group
            }
        }

        impl ::std::fmt::Debug for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f
                    .#debug_method(#ident_str)
                    #debug_fields
                    .finish()
            }
        }
    }
}
