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
* [04 - Debug](https://github.com/loganmzz/rust-macro-introduction-code/tree/04-debug)
* [05 - Attribut](https://github.com/loganmzz/rust-macro-introduction-code/tree/05-attribute)
* [06 - Gestion des erreurs](https://github.com/loganmzz/rust-macro-introduction-code/tree/06-errors)
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
proc-macro = true

[dependencies]
proc-macro2 = "1.0.73"
quote = "1.0.34"
syn = { version = "2.0.44", features = ["extra-traits"] }
```

### C. Initialisation de la macro

```rust
// src/lib.rs
#[proc_macro_derive(Data,)]
pub fn derive_data(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let output = quote::quote!();
    proc_macro::TokenStream::from(output)
}
```
