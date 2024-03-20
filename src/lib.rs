#[macro_use]
extern crate darling;

mod generator;
mod model;
mod parser;

#[proc_macro_derive(Data,attributes(data,),)]
pub fn derive_data(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    // Permet de débugger l'instance de syn::DeriveInput
    #[cfg(feature = "debug")]
    eprintln!("{:#?}", input);

    let output = data_macro_derive_impl(input);

    // Permet de débugger le code généré par la macro
    #[cfg(feature = "debug")]
    eprintln!("{}", output);

    output.into()
}

/// Génère le flux de token de manière testable
/// car indépendant de proc_macro !
fn data_macro_derive_impl(input: syn::DeriveInput) -> proc_macro2::TokenStream {
    match parser::parse(input) {
        Ok(data) => generator::generate(data),
        Err(error) => error.into_compile_error(),
    }
}
