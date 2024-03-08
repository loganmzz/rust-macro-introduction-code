use crate::model;

pub fn generate(data: model::Data) -> proc_macro2::TokenStream {
    let ident = data.ident;
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
    let field_group = proc_macro2::Group::new(data.fields.delimiter, default_fields);
    quote::quote! {
        impl ::std::default::Default for #ident {
            fn default() -> Self {
                Self #field_group
            }
        }
    }
}
