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
* [04 - impl Debug](https://github.com/loganmzz/rust-macro-introduction-code/tree/04-impl-debug)
* [05 - Attribut](https://github.com/loganmzz/rust-macro-introduction-code/tree/05-attribute)
* [06 - Gestion des erreurs](https://github.com/loganmzz/rust-macro-introduction-code/tree/06-errors)
* [07 - API publique](https://github.com/loganmzz/rust-macro-introduction-code/tree/07-public-api)
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
fn data_macro_derive_impl(input: syn::DeriveInput) -> proc_macro2::TokenStream {
    quote::quote! {
        impl ::std::default::Default for Unit {
            fn default() -> Self {
                Self
            }
        }
    }
}
```

Récupérer le nom de la structure :

```rust
// src/lib.rs
fn data_macro_derive_impl(input: syn::DeriveInput) -> proc_macro2::TokenStream {
    let ident = &input.ident;
    quote::quote! {
        impl ::std::default::Default for #ident { /* ... */ }
    }
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

```shell
cargo test --test macro_named --features debug
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
fn data_macro_derive_impl(input: syn::DeriveInput) -> proc_macro2::TokenStream {
    // ...
    let fields = match input.data {
        syn::Data::Struct(ref data) => data.fields.iter().collect::<Vec<_>>(),
        syn::Data::Enum(_) => panic!("enum are not supported!"),
        syn::Data::Union(_) => panic!("union are not supported!"),
    };
    // ...
}
```

Construction de l'initialisation des champs :

```rust
// src/lib.rs
fn data_macro_derive_impl(input: syn::DeriveInput) -> proc_macro2::TokenStream {
    // ...
    let fields = /* ... */;
    let default_fields: proc_macro2::TokenStream = fields
        .iter()
        .map(|field| {
            let ident = field.ident.as_ref().unwrap();
            quote::quote! {
                #ident: ::std::default::Default::default(),
            }
        })
        .collect();
    quote::quote! {
        impl ::std::default::Default for #ident {
            fn default() -> Self {
                Self {
                    #default_fields
                }
            }
        }
    }
}
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

Construction de l'initialisation des champs :

```rust
// src/lib.rs
fn data_macro_derive_impl(input: syn::DeriveInput) -> proc_macro2::TokenStream {
    // ...
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
fn data_macro_derive_impl(input: syn::DeriveInput) -> proc_macro2::TokenStream {
    let (fields, delimiter) = match input.data {
        syn::Data::Struct(ref data) => (
            data.fields.iter().collect::<Vec<_>>(),
            match data.fields {
                syn::Fields::Named(_) => proc_macro2::Delimiter::Brace,
                syn::Fields::Unnamed(_) => proc_macro2::Delimiter::Parenthesis,
                syn::Fields::Unit => proc_macro2::Delimiter::None,
            },
        ),
        // ...
    };
    // ...
    let field_group = proc_macro2::Group::new(delimiter, default_fields);
    quote::quote! {
        impl ::std::default::Default for #ident {
            fn default() -> Self {
                Self #field_group
            }
        }
    }
}
```
