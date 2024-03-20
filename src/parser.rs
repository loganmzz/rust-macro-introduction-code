use crate::model;
use darling::FromMeta;

trait FoldSynResult {
    type Item;

    fn fold_syn_result<O,G,FG,FC,E,>(
        self,
        init: O,
        get: FG,
        combine: FC,
    ) -> syn::Result<O>
    where
        FG: FnMut(Self::Item)->Result<G,E>,
        FC: FnMut(O,G)->O,
        E: Into<syn::Error>,
    ;
}

impl<ITER: Iterator> FoldSynResult for ITER {
    type Item = ITER::Item;

    fn fold_syn_result<O,G,FG,FC,E,>(
        self,
        init: O,
        mut get: FG,
        mut combine: FC,
    ) -> syn::Result<O>
    where
        FG: FnMut(Self::Item)->Result<G,E>,
        FC: FnMut(O,G)->O,
        E: Into<syn::Error>, {
        self
            .fold(syn::Result::Ok(init), |acc, e| {
                match get(e) {
                    Ok(g) => match acc {
                        Ok(o) => Ok(combine(o, g)),
                        e => e,
                    },
                    Err(error) => match acc {
                        Ok(_) => Err(error.into()),
                        Err(mut existing) => {
                            existing.combine(error.into());
                            Err(existing)
                        }
                    },
                }
            })
    }
}

pub fn parse_field_attributes(attrs: &Vec<syn::Attribute>) -> syn::Result<model::FieldOptions> {
    attrs
        .into_iter()
        .fold_syn_result(
            model::FieldOptions::default(),
            |attr| model::FieldOptions::from_meta(&attr.meta),
            |mut options, parsed| {
                if parsed.debug.is_some() {
                    options.debug = parsed.debug;
                }
                options
            },
        )
}

pub fn parse(input: syn::DeriveInput) -> syn::Result<model::Data> {
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
    let fields_len = fields.len();
    let fields = model::Fields {
        delimiter,
        content: fields
        .into_iter()
        .enumerate()
        .fold_syn_result(
            Vec::with_capacity(fields_len),
            |(ordinal, field)| parse_field_attributes(&field.attrs).map(|options| (field.ident.clone(), ordinal, options)),
            |mut field_list, (ident, ordinal, options)| {
                field_list.push(model::Field {
                    ident,
                    ordinal,
                    options,
                });
                field_list
            },
        )?,
    };
    Ok(model::Data {
        ident,
        format,
        fields,
    })
}
