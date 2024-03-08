use crate::model;

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
                model::Field {
                    ident,
                    ordinal,
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
