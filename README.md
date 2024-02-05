# Développer des macros en Rust

Bienvenue sur le projet de démonstration de la présentation ["Développer des macros en Rust"](https://github.com/loganmzz/rust-macro-introduction-presentation).

Le but est de développer une macro dérivative "Data" :

* Implémenter des getters
* Implémenter [`Default`](https://doc.rust-lang.org/std/default/trait.Default.html)
* Implémenter [`Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
* ...


Retrouvez ici les étapes pas-à-pas :

* [00 - Initialisation](https://github.com/loganmzz/rust-macro-introduction-code/tree/00-init) :arrow_down_small: (vous êtes ici)
* [01 - Blueprint](https://github.com/loganmzz/rust-macro-introduction-code/tree/01-blueprint)
* [02 - impl Default](https://github.com/loganmzz/rust-macro-introduction-code/tree/02-impl-default)
* [03 - Modules](https://github.com/loganmzz/rust-macro-introduction-code/tree/03-modules)
* [04 - impl Debug](https://github.com/loganmzz/rust-macro-introduction-code/tree/04-impl-debug)
* [05 - Attribut](https://github.com/loganmzz/rust-macro-introduction-code/tree/05-attribute)
* [06 - Gestion des erreurs](https://github.com/loganmzz/rust-macro-introduction-code/tree/06-errors)
* [07 - API publique](https://github.com/loganmzz/rust-macro-introduction-code/tree/07-public-api)
* [Fin](https://github.com/loganmzz/rust-macro-introduction-code/tree/99-final)

## 00 - Initialisation

### A. Création du projet Rust

```bash
cargo init --lib --name demo-data
```

### B. Configuration du projet Rust

```toml
# Cargo.toml
[lib]
proc-macro = true         # Ajoute la crate `proc-macro` lors de la compilation.
                          # Mais pas accessible pour les tests...
                          # La crate ne peut plus exporter que des macros !

[dependencies]
proc-macro2 = "1.0.73"    # Alternative à `proc-macro2` accessible pour écrire des tests ou des libs.
quote = "1.0.34"          # Génération de flux de tokens.
syn = "2.0.44"            # Analyseur syntaxique
```


### C. Initialisation de la macro

```rust
// src/lib.rs
#[proc_macro_derive(Data,)]
pub fn data_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let output = quote::quote!();
    proc_macro::TokenStream::from(output)
}
```

### D. Un peu de refactoring

```rust
// src/lib.rs
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
    quote::quote!()
}
```

```toml
# Cargo.toml

[features]
debug = [
  "syn/extra-traits", # Permet de débugger les types de syn
]
```
