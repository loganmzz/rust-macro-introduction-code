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
* [04 - Debug](https://github.com/loganmzz/rust-macro-introduction-code/tree/04-debug) :arrow_down_small: (vous êtes ici)
* [05 - Attribut](https://github.com/loganmzz/rust-macro-introduction-code/tree/05-attribute)
* [06 - Gestion des erreurs](https://github.com/loganmzz/rust-macro-introduction-code/tree/06-errors)
* [Fin](https://github.com/loganmzz/rust-macro-introduction-code/tree/99-final)

## 04 - Debug

#### A. Création des blueprints

```rust
// tests/blueprint_unit.rs
// Output
impl ::std::fmt::Debug for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f
            .debug_struct("Unit")
            .finish()
    }
}

// Test
mod tests {
    #[test]
    fn unit_impl_debug() {
        assert_eq!(
            "Unit",
            format!("{:?}", Unit),
        );
    }
}
```

```rust
// tests/blueprint_named.rs
// Output
impl ::std::fmt::Debug for Named {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f
            .debug_struct("Named")
            .field("string", &self.string)
            .field("number", &self.number)
            .field("boolean", &self.boolean)
            .finish()
    }
}

// Test
mod tests {
    #[test]
    fn named_immpl_debug() {
        let debug =  Named {
            string: "world".to_string(),
            number: 42,
            boolean: false,
        };

        assert_eq!(
            "Named { string: \"world\", number: 42, boolean: false }",
            format!("{:?}", debug),
        );
    }
}
```
```rust
// tests/blueprint_tuple.rs
// Output
impl ::std::fmt::Debug for Tuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f
            .debug_tuple("Tuple")
            .field(&self.0)
            .field(&self.1)
            .field(&self.2)
            .finish()
    }
}

// Test
mod tests {
    #[test]
    fn tuple_impl_debug() {
        let debug = Tuple(
            "world".to_string(),
            42,
            false,
        );

        assert_eq!(
            "Tuple(\"world\", 42, false)",
            format!("{:?}", debug),
        );
    }
}
```

#### B. Mise à jour des tests

```rust
// tests/macro_unit.rs
// tests/macro_named.rs
// tests/macro_tuple.rs
```

#### C. Implémentation `Unit`

```rust
// src/generator.rs
pub fn generate(data: model::Data) -> proc_macro2::TokenStream {
    // ...
    let ident_str = ident.to_string();
    quote::quote! {
        // ...
        impl ::std::fmt::Debug for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f
                    .debug_struct(#ident_str)
                    .finish()
            }
        }
    }
}
```

#### D. Implémentation `Named`

```rust
// src/generator.rs
pub fn generate(data: model::Data) -> proc_macro2::TokenStream {
    // ...
    let debug_fields: proc_macro2::TokenStream = data.fields.content
        .iter()
        .map(|field| {
            if let Some(ref ident) = field.ident {
                let ident_str = ident.to_string();
                quote::quote! {
                    .field(#ident_str, &self.#ident)
                }
            } else {
                quote::quote!()
            }
        })
        .collect();
    quote::quote! {
        // ...
        impl ::std::fmt::Debug for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f
                    .debug_struct(#ident_str)
                    #debug_fields
                    .finish()
            }
        // ...
    }
}
```

#### E. Implémentation `Tuple`

Pour les champs :

```rust
// src/model.rs
pub struct Field {
    // ...
    pub ordinal: usize,
}

// src/parser.rs
pub fn parse(input: syn::DeriveInput) -> model::Data {
    // ...
    let fields = model::Fields {
        // ...
        content: fields
            .into_iter()
            .enumerate()
            .map(|(ordinal, field)| {
                // ...
                model::Field {
                    // ...
                    ordinal,
                }
            })
        /..
    }
}
// src/generator.rs
pub fn generate(data: model::Data) -> proc_macro2::TokenStream {
    // ...
    let debug_fields: proc_macro2::TokenStream = data.fields.content
        .iter()
        .map(|field| {
            if let Some(ref ident) = field.ident {
                // ...
            } else {
                let ident = proc_macro2::Literal::usize_unsuffixed(field.ordinal);
                quote::quote! {
                    .field(&self.#ident)
                }
            }
        })
        .collect();
}
```

Pour la méthode :

```rust
// src/model.rs
pub enum StructFormat {
    Named,
    Tuple,
}

pub struct Data {
    // ...
    pub format: StructFormat,
    // ...
}

// src/parser.rs
pub fn parse(input: syn::DeriveInput) -> model::Data {
    // ...
    let (format, fields, delimiter) = match input {
        // ...
        => (model::StructFormat::Named, fields.iter().collect(), proc_macro2::Delimiter::Brace,),
        // ...
        => (model::StructFormat::Tuple, fields.iter().collect(), proc_macro2::Delimiter::Parenthesis,),
        _ => (model::StructFormat::Named, vec![], proc_macro2::Delimiter::None,),
    }
    // ...
    model::Data {
        ident,
        format,
        fields,
    }
}

// src/generator.rs
pub fn generate(data: model::Data) -> proc_macro2::TokenStream {
    // ...
    let debug_method = match data.format {
        model::StructFormat::Named => quote::format_ident!("debug_struct"),
        model::StructFormat::Tuple => quote::format_ident!("debug_tuple"),
    };
    let debug_fields: proc_macro2::TokenStream = /* ... */;
    // ...
    quote::quote! {
        impl ::std::fmt::Debug for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f
                    .#debug_method(#ident_str)
                // ...
            }
        }
    }
}
```
