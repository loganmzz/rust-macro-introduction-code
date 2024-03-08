#[proc_macro_derive(Data,)]
pub fn derive_data(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let output = quote::quote!();
    proc_macro::TokenStream::from(output)
}
