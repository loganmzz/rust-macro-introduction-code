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
    let (format, fields, delimiter) = match input {
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
        } => (model::StructFormat::Named, fields.iter().collect(), proc_macro2::Delimiter::Brace,),
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
        } => (model::StructFormat::Tuple, fields.iter().collect(), proc_macro2::Delimiter::Parenthesis,),
        _ => (model::StructFormat::Named, vec![], proc_macro2::Delimiter::None,),
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
    model::Data {
        ident,
        format,
        fields,
    }
}
