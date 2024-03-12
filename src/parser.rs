use crate::model;
use darling::FromMeta;

pub fn parse_field_attributes(attrs: &Vec<syn::Attribute>) -> model::FieldOptions {
    let mut options = model::FieldOptions::default();
    for attr in attrs {
        if attr.path().is_ident("data") {
            let parsed = model::FieldOptions::from_meta(&attr.meta).unwrap();
            if parsed.debug.is_some() {
                options.debug = parsed.debug;
            }
        }
    }
    options
}

pub fn parse(input: syn::DeriveInput) -> model::Data {
    let ident = input.ident.clone();
    let (fields, (format, delimiter)) = match input.data {
        syn::Data::Struct(ref data) => (
            data.fields.iter().collect::<Vec<_>>(),
            match data.fields {
                syn::Fields::Named(_) => (
                    model::StructFormat::Named,
                    proc_macro2::Delimiter::Brace,
                ),
                syn::Fields::Unnamed(_) => (
                    model::StructFormat::Tuple,
                    proc_macro2::Delimiter::Parenthesis,
                ),
                syn::Fields::Unit => (
                    model::StructFormat::Named,
                    proc_macro2::Delimiter::None,
                ),
            },
        ),
        syn::Data::Enum(_) => panic!("enum are not supported!"),
        syn::Data::Union(_) => panic!("union are not supported!"),
    };
    let fields = model::Fields {
        delimiter,
        content: fields
            .into_iter()
            .enumerate()
            .map(|(ordinal,field)| {
                let ident = field.ident.clone();
                let options = parse_field_attributes(&field.attrs);
                model::Field {
                    ident,
                    ordinal,
                    options,
                }
            })
            .collect()
    };
    model::Data{
        ident,
        format,
        fields,
    }
}
