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
* [03 - Modules](https://github.com/loganmzz/rust-macro-introduction-code/tree/03-modules)
* [04 - Debug](https://github.com/loganmzz/rust-macro-introduction-code/tree/04-debug)
* [05 - Attribut](https://github.com/loganmzz/rust-macro-introduction-code/tree/05-attribute) :arrow_down_small: (vous êtes ici)
* [06 - Gestion des erreurs](https://github.com/loganmzz/rust-macro-introduction-code/tree/06-errors)
* [Fin](https://github.com/loganmzz/rust-macro-introduction-code/tree/99-final)

## 05 - Attribut

### A. Déclaration de l'attribut

```rust
// src/lib.rs
#[proc_macro_derive(Data,attributes(data,),)]
```

### B. Premier test

```rust
// tests/macro_attr_debug.rs
use demo_data::Data;

// Input
#[derive(Data)]
struct AttrDebugNamedIgnoreNumber {
    string: String,
    #[data(debug=false)]
    number: usize,
    boolean: bool,
}

#[derive(Data)]
struct AttrDebugNamedForceNumber {
    string: String,
    #[data(debug=true)]
    number: usize,
    boolean: bool,
}

#[derive(Data)]
struct AttrDebugNamedNoValueNumber {
    string: String,
    #[data(debug)]
    number: usize,
    boolean: bool,
}

#[derive(Data)]
struct AttrDebugNamedUnspecifiedNumber {
    string: String,
    #[data()]
    number: usize,
    boolean: bool,
}

#[derive(Data)]
struct AttrDebugTupleIgnore1(
    String,
    #[data(debug=false)]
    usize,
    bool,
);

// Test
mod tests {
    use super::*;

    #[test]
    fn named_impl_debug_false() {
        let debug = AttrDebugNamedIgnoreNumber {
            string: "world".to_string(),
            number: 42,
            boolean: false,
        };

        assert_eq!(
            "AttrDebugNamedIgnoreNumber { string: \"world\", boolean: false }",
            format!("{:?}", debug),
        );
    }

    #[test]
    fn named_impl_debug_true() {
        let debug = AttrDebugNamedForceNumber {
            string: "world".to_string(),
            number: 42,
            boolean: false,
        };

        assert_eq!(
            "AttrDebugNamedForceNumber { string: \"world\", number: 42, boolean: false }",
            format!("{:?}", debug),
        );
    }

    #[test]
    fn named_impl_debug_novalue() {
        let debug = AttrDebugNamedNoValueNumber {
            string: "world".to_string(),
            number: 42,
            boolean: false,
        };

        assert_eq!(
            "AttrDebugNamedNoValueNumber { string: \"world\", number: 42, boolean: false }",
            format!("{:?}", debug),
        );
    }

    #[test]
    fn named_impl_debug_unspecified() {
        let debug = AttrDebugNamedUnspecifiedNumber {
            string: "world".to_string(),
            number: 42,
            boolean: false,
        };

        assert_eq!(
            "AttrDebugNamedUnspecifiedNumber { string: \"world\", number: 42, boolean: false }",
            format!("{:?}", debug),
        );
    }

    #[test]
    fn tuple_impl_debug() {
        let debug = AttrDebugTupleIgnore1(
            "world".to_string(),
            42,
            false,
        );

        assert_eq!(
            "AttrDebugTupleIgnore1(\"world\", false)",
            format!("{:?}", debug),
        );
    }
}
```

### C. Implémentation

```rust
// src/model.rs
#[derive(Default,)]
pub struct FieldOptions {
    pub debug: Option<bool>,
}

pub struct Field {
    // ...
    pub options: FieldOptions,
}

// src/parser.rs
pub fn parse_field_attributes(attrs: &Vec<syn::Attribute>) -> model::FieldOptions {
    let mut options = model::FieldOptions::default();
    for attr in attrs {
        if attr.path().is_ident("data") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("debug") {
                    let value: syn::LitBool = meta.value()?.parse()?;
                    options.debug = Some(value.value());
                    Ok(())
                } else {
                    Err(meta.error(format!("Unsupported field data attribute option: {:?}", meta.path)))
                }
            })
            .unwrap();
        }
    }
    options
}

pub fn parse(input: syn::DeriveInput) -> model::Data {
    // ...
    let fields = model::Fields {
        // ...
        content: fields
            // ...
            .map(|(ordinal, field)| {
                // ...
                let options = parse_field_attributes(&field.attrs);
                model::Field {
                    // ...
                    options,
                }
            })
            // ...
    };
    // ...
}

// src/generator.rs
pub fn generate(data: model::Data) -> proc_macro2::TokenStream {
    // ...
    let debug_fields: proc_macro2::TokenStream = data.fields.content
        .iter()
        .map(|field| {
            if let Some(false) = field.options.debug {
                return quote!();
            }
            // ...
        })
        .collect();
}
```

### D. Crate `darling`

```toml
# Cargo.toml
[dependencies]
darling = "0.20.8"
```

```rust
// src/lib.rs
#[macro_use]
extern crate darling;

// src/model.rs
#[derive(Default,FromMeta,)]
pub struct FieldOptions {
    // ...
}

// src/parser.rs
use darling::FromMeta;

pub fn parse_field_attributes(attrs: &Vec<syn::Attribute>) -> model::FieldOptions {
    let mut options = model::FieldOptions::default();
    for attr in attrs {
        if attr.path().is_ident("data") {
            let parsed = model::FieldOptions::from_meta(&attr.meta).unwrap();
            if parsed.debug.is_some() {
                options.debug = parsed.debug;
            }
        }
    }
    options
}
```
