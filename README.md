# Développer des macros en Rust

Bienvenue sur le projet de démonstration de la présentation ["Développer des macros en Rust"](https://github.com/loganmzz/rust-macro-introduction-presentation).

Le but est de développer une macro dérivative "Data" :

* Implémenter des getters
* Implémenter [`Default`](https://doc.rust-lang.org/std/default/trait.Default.html)
* Implémenter [`Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
* ...


Retrouvez ici les étapes pas-à-pas :

* [00 - Initialisation](https://github.com/loganmzz/rust-macro-introduction-code/tree/00-init)
* [01 - Blueprint](https://github.com/loganmzz/rust-macro-introduction-code/tree/01-blueprint)
* [02 - impl Default](https://github.com/loganmzz/rust-macro-introduction-code/tree/02-impl-default)
* [03 - Modules](https://github.com/loganmzz/rust-macro-introduction-code/tree/03-modules) :arrow_down_small: (vous êtes ici)
* [04 - impl Debug](https://github.com/loganmzz/rust-macro-introduction-code/tree/04-impl-debug)
* [05 - Attribut](https://github.com/loganmzz/rust-macro-introduction-code/tree/05-attribute)
* [06 - Gestion des erreurs](https://github.com/loganmzz/rust-macro-introduction-code/tree/06-errors)
* [Fin](https://github.com/loganmzz/rust-macro-introduction-code/tree/99-final)

## 03 - Modules

#### A. Création des modules

```rust
// src/model.rs
struct Data;
```

```rust
// src/parser.rs
use crate::model;

pub fn parse(input: syn::DeriveInput) -> model::Data {
    model::Data
}
```

```rust
// src/generator.rs
use crate::model;

pub fn generate(data: model::Data) -> proc_macro2::TokenStream {
    quote::quote!()
}
```

```rust
// src/lib.rs
mod generator;
mod model;
mod parser;
```

#### B. `parser`

```rust
// src/parser.rs
pub fn parse(input: syn::DeriveInput) -> model::Data {
    let ident = input.ident.clone();
    let (fields, delimiter) = match input { /* ... */ };
    let fields = model::Fields {
        delimiter,
        content: fields
            .into_iter()
            .map(|field| {
                let ident = field.ident.clone();
                model::Field {
                    ident,
                }
            })
            .collect()
    };
    model::Data {
        ident,
        fields,
    }
}
```

#### C. `model`

```rust
// src/model.rs
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
```

#### D. `generator`

```rust
/// src/generator.rs
pub fn generate(data: model::Data) -> proc_macro2::TokenStream {
    let ident = data.ident;
    let default_fields: proc_macro2::TokenStream = data.fields.content /* ... */;
    let field_group = proc_macro2::Group::new(data.fields.delimiter, default_fields);
    quote::quote! { /* ... */ }
}
```

#### E. `lib`

```rust
// src/lib.rs
pub fn data_macro_derive_impl(input: syn::DeriveInput) -> proc_macro2::TokenStream {
    let data = parser::parse(input);
    generator::generate(data)
}
```
