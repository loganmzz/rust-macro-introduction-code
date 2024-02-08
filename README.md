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
* [02 - impl Default](https://github.com/loganmzz/rust-macro-introduction-code/tree/02-impl-default) :arrow_down_small: (vous êtes ici)
* [03 - Modules](https://github.com/loganmzz/rust-macro-introduction-code/tree/03-modules)
* [04 - Debug](https://github.com/loganmzz/rust-macro-introduction-code/tree/04-debug)
* [05 - Attribut](https://github.com/loganmzz/rust-macro-introduction-code/tree/05-attribute)
* [06 - Gestion des erreurs](https://github.com/loganmzz/rust-macro-introduction-code/tree/06-errors)
* [Fin](https://github.com/loganmzz/rust-macro-introduction-code/tree/99-final)

## 02 - impl `Default`

### A. Préparation du fichier de test

Initialiser le fichier de tests `tests/macro_unit.rs` en recopiant les sections `Input` et `Test` :

```rust
// tests/macro_unit.rs
// Input
// copier depuis tests/blueprint_unit.rs
struct Unit;

// Test
// copier depuis tests/blueprint_units.rs
mod tests { /* ... */ }
```

Puis ajouter l'import de la macro ainsi que l'attribut `derive` sur la structure :

```rust
// tests/macro_unit.rs
use demo_data::Data;

// Input
#[derive(Data)]
struct Unit;
```

### B. Implémentation simplissime

```rust
// src/lib.rs
pub fn derive_data(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // ...
    let output = quote::quote! {
        impl ::std::default::Default for Unit {
            fn default() -> Self {
                Self
            }
        }
    };
    // ...
}
```

Récupérer le nom de la structure :

```rust
// src/lib.rs
pub fn derive_data(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // ...
    let ident = &input.ident;
    let output = quote::quote! {
        impl ::std::default::Default for #ident { /* ... */ }
    };
    // ...
}
```

### C. Implémentation `Named`

Initialiser le fichier de tests `tests/macro_named.rs`

```rust
// tests/macro_named.rs
use demo_data::Data;

// Input

#[derive(Data)]
// copier depuis tests/blueprint_named.rs
struct Named { /* ... */ }

// Test
// copier depuis tests/blueprint_named.rs
mod tests { /* ... */ }
```

Debug des informations :

```rust
// src/lib.rs
pub fn derive_data(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = /* ... */;
    eprintln!("{:#?}", input);
    // ...
}
```

```shell
cargo test --test macro_named
```

```rust
DeriveInput {
    attrs: [],
    vis: Visibility::Inherited,
    ident: Ident {
        ident: "Named",
        span: #0 bytes(54..59),
    },
    generics: Generics {
        lt_token: None,
        params: [],
        gt_token: None,
        where_clause: None,
    },
    data: Data::Struct {
        struct_token: Struct,
        fields: Fields::Named {
            brace_token: Brace,
            named: [
                Field {
                    attrs: [],
                    vis: Visibility::Inherited,
                    mutability: FieldMutability::None,
                    ident: Some(
                        Ident {
                            ident: "string",
                            span: #0 bytes(66..72),
                        },
                    ),
                    colon_token: Some(
                        Colon,
                    ),
                    ty: Type::Path {
                        qself: None,
                        path: Path {
                            leading_colon: None,
                            segments: [
                                PathSegment {
                                    ident: Ident {
                                        ident: "String",
                                        span: #0 bytes(74..80),
                                    },
                                    arguments: PathArguments::None,
                                },
                            ],
                        },
                    },
                },
                Comma,
                Field {
                    attrs: [],
                    vis: Visibility::Inherited,
                    mutability: FieldMutability::None,
                    ident: Some(
                        Ident {
                            ident: "number",
                            span: #0 bytes(86..92),
                        },
                    ),
                    colon_token: Some(
                        Colon,
                    ),
                    ty: Type::Path {
                        qself: None,
                        path: Path {
                            leading_colon: None,
                            segments: [
                                PathSegment {
                                    ident: Ident {
                                        ident: "usize",
                                        span: #0 bytes(94..99),
                                    },
                                    arguments: PathArguments::None,
                                },
                            ],
                        },
                    },
                },
                Comma,
                Field {
                    attrs: [],
                    vis: Visibility::Inherited,
                    mutability: FieldMutability::None,
                    ident: Some(
                        Ident {
                            ident: "boolean",
                            span: #0 bytes(105..112),
                        },
                    ),
                    colon_token: Some(
                        Colon,
                    ),
                    ty: Type::Path {
                        qself: None,
                        path: Path {
                            leading_colon: None,
                            segments: [
                                PathSegment {
                                    ident: Ident {
                                        ident: "bool",
                                        span: #0 bytes(114..118),
                                    },
                                    arguments: PathArguments::None,
                                },
                            ],
                        },
                    },
                },
                Comma,
            ],
        },
        semi_token: None,
    },
}
```

Récupérer la liste des champs :

```rust
// src/lib.rs
pub fn derive_data(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // ...
    let fields = match input {
        syn::DeriveInput {
            data: syn::Data::Struct(
                syn::DataStruct {
                    fields: syn::Fields::Named(
                        syn::FieldsNamed {
                            named: ref fields,
                             ..
                        }
                    ),
                    ..
                }
            ),
            ..
        } => fields.iter().collect(),
        _ => vec![],
    };
    // ...
}
```

Construction de l'initialisation des champs :

```rust
// src/lib.rs
pub fn derive_data(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // ...
    let fields = match input { /* ... */ };
    let default_fields: proc_macro2::TokenStream = fields
        .iter()
        .map(|field| {
            let ident = field.ident.as_ref().unwrap();
            quote! {
                #ident: ::std::default::Default::default(),
            }
        })
        .collect();
    let output = quote::quote! {
        impl ::std::default::Default for #ident {
            fn default() -> Self {
                Self {
                    #default_fields
                }
            }
        }
    };
```

### C. Implémentation `Tuple`

Initialiser le fichier de tests `tests/macro_tuple.rs`

```rust
// tests/macro_tuple.rs
use demo_data::Data;

// Input
#[derive(Data)]
// copier depuis tests/blueprint_tuple.rs
struct Tuple( /* ... */ )

// Test
// copier depuis tests/blueprint_tuple.rs
mod tests { /* ... */ }
```

Récupérer la liste des champs :

```rust
// src/lib.rs
pub fn derive_data(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // ...
    let fields = match input {
        // ...
        syn::DeriveInput {
            data: syn::Data::Struct(
                syn::DataStruct {
                    fields: syn::Fields::Unnamed(
                        syn::FieldsUnnamed {
                            unnamed: ref fields,
                            ..
                        }
                    ),
                    ..
                }
            ),
            ..
        } => fields.iter().collect(),
        // ...
    };
    // ...
}
```

Construction de l'initialisation des champs :

```rust
// src/lib.rs
pub fn derive_data(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // ...
    let fields = match input { /* ... */ };
    let default_fields: proc_macro2::TokenStream = fields
        .iter()
        .map(|field| {
            let prefix = if let Some(ref ident) = field.ident {
                quote::quote!(#ident:)
            } else {
                quote::quote!()
            };
            quote::quote! {
                #prefix ::std::default::Default::default(),
            }
        })
        .collect();
    // ...
}
```

Corrigeons la méthode `default` :

```rust
// src/lib.rs
pub fn derive_data(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let (fields, delimiter) = match input {
        // ...
                    fields: syn::Fields::Named(/* .. */),
        // ...
        => (fields.iter().collect(), proc_macro2::Delimiter::Brace,),
        // ...
        // ...
                    fields: syn::Fields::Unnamed(/* .. */),
        // ...
        => (fields.iter().collect(), proc_macro2::Delimiter::Parenthesis,),
        _ => (vec![], proc_macro2::Delimiter::None,),
    };
    // ...
    let field_group = proc_macro2::Group::new(delimiter, default_fields);
    let output = quote::quote! {
        impl ::std::default::Default for #ident {
            fn default() -> Self {
                Self #field_group
            }
        }
    };
}
```
