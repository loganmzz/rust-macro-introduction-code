mod generator;
mod model;
mod parser;

#[proc_macro_derive(Data,)]
pub fn data_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
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
    let data = parser::parse(input);
    generator::generate(data)
}
