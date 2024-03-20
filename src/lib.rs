#[macro_use]
extern crate darling;

mod generator;
mod model;
mod parser;

#[proc_macro_derive(Data,attributes(data,),)]
pub fn derive_data(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    #[cfg(feature = "debug_log")]
    eprintln!("{:#?}", input);

    let output = data_macro_derive_impl(input);
    #[cfg(feature = "debug_log")]
    eprintln!("{}", output);

    proc_macro::TokenStream::from(output)
}

fn data_macro_derive_impl(input: syn::DeriveInput) -> proc_macro2::TokenStream {
    match parser::parse(input) {
        Ok(data) => generator::generate(data),
        Err(error) => error.into_compile_error(),
    }
}
