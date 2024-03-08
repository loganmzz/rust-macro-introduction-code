mod generator;
mod model;
mod parser;

#[proc_macro_derive(Data,)]
pub fn data_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    #[cfg(feature = "debug_log")]
    eprintln!("{:#?}", input);

    let output = data_macro_derive_impl(input);
    #[cfg(feature = "debug_log")]
    eprintln!("{}", output);

    proc_macro::TokenStream::from(output)
}

fn data_macro_derive_impl(input: syn::DeriveInput) -> proc_macro2::TokenStream {
    let data = parser::parse(input);
    generator::generate(data)
}
