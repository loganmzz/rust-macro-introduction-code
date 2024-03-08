use crate::model;

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
                model::Field {
                    ident,
                    ordinal,
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
