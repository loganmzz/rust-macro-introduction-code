pub struct Data {
    pub ident: proc_macro2::Ident,
    pub fields: Fields,
}

pub struct Fields {
    pub delimiter: proc_macro2::Delimiter,
    pub content: Vec<Field>,
}

pub struct Field {
    pub ident: Option<proc_macro2::Ident>,
}
