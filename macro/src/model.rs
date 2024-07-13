pub enum StructFormat {
    Named,
    Tuple,
}
pub struct Data {
    pub ident: proc_macro2::Ident,
    pub format: StructFormat,
    pub fields: Fields,
}

pub struct Fields {
    pub delimiter: proc_macro2::Delimiter,
    pub content: Vec<Field>,
}

#[derive(Default,FromMeta,)]
pub struct FieldOptions {
    pub debug: Option<bool>,
}

pub struct Field {
    pub ident: Option<proc_macro2::Ident>,
    pub ordinal: usize,
    pub options: FieldOptions,
}
