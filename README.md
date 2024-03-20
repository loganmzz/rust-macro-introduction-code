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
* [05 - Attribut](https://github.com/loganmzz/rust-macro-introduction-code/tree/05-attribute)
* [06 - Gestion des erreurs](https://github.com/loganmzz/rust-macro-introduction-code/tree/06-errors) :arrow_down_small: (vous êtes ici)
* [Fin](https://github.com/loganmzz/rust-macro-introduction-code/tree/99-final)

## 06 - Gestion des erreurs

### A. Cas de test

```rust
// tests/fail.rs
use demo_data::Data;

#[derive(Data)]
struct FailNotCompile {
    string: String,
    #[data(debug=false,foobar,baroof,)]
    #[data(barfoo,)]
    number: usize,
    boolean: bool,
    #[data(foobaz)]
    foobaz: bool,
}
```

### B. Suppression des `panic!`

N/A !

### C. Suppression des `.unwrap()`

```rust
// src/parser.rs
pub fn parse_field_attributes(attrs: &Vec<syn::Attribute>) -> syn::Result<model::FieldOptions> {
    // ...
    for attr in attrs {
        if attr.path().is_ident("data") {
            let parsed = model::FieldOptions::from_meta(&attr.meta)?;
            // ...
        }
    }
    Ok(options)
}

pub fn parse(input: syn::DeriveInput) -> syn::Result<model::Data> {
    // ...
    let fields = model::Fields {
        // ...
        content: fields
            // ...
            .map(|(ordinal, field)| {
                // ...
                let options = match parse_field_attributes(&field.attrs)?;
                Ok(model::Field {
                    ident,
                    ordinal,
                    options,
                })
            })
            .collect::<syn::Result<_>>()?,
    };
    Ok(model::Data {
        ident,
        format,
        fields,
    })
}

// src/lib.rs
fn data_macro_derive_impl(input: syn::DeriveInput) -> proc_macro2::TokenStream {
    match parser::parse(input) {
        Ok(data) => generator::generate(data),
        Err(error) => error.into_compile_error(),
    }
}
```

### D. Collecte des erreurs

```rust
// src/parser.rs
pub fn parse_field_attributes(attrs: &Vec<syn::Attribute>) -> syn::Result<model::FieldOptions> {
    let mut result = syn::Result::Ok(model::FieldOptions::default());
    for attr in attrs {
        if attr.path().is_ident("data") {
            result = match model::FieldOptions::from_meta(&attr.meta) {
                Ok(parsed) => {
                    result.map(|mut options| {
                        if parsed.debug.is_some() {
                            options.debug = parsed.debug;
                        }
                        options
                    })
                },
                Err(error) => {
                    match result {
                        Ok(_) => Err(error.into()),
                        Err(mut existing) => {
                            existing.combine(error.into());
                            Err(existing)
                        }
                    }
                }
            }
        }
    }
    result
}

pub fn parse(input: syn::DeriveInput) -> syn::Result<model::Data> {
    // ...
    let fields_len = fields.len();
    let fields = model::Fields {
        // ...
        content: fields
            // ...
            .fold(syn::Result::Ok(Vec::with_capacity(fields_len)), |result, (ordinal,field)| {
                let ident = field.ident.clone();
                match parse_field_attributes(&field.attrs) {
                    Ok(options) => {
                        match result {
                            Ok(mut field_list) => {
                                field_list.push(model::Field {
                                    ident,
                                    ordinal,
                                    options,
                                });
                                Ok(field_list)
                            },
                            e => e,
                        }
                    },
                    Err(error) => {
                        match result {
                            Ok(_) => Err(error),
                            Err(mut existing) => {
                                existing.combine(error);
                                Err(existing)
                            },
                        }
                    },
                }
            })?,
    };
    // ...
```

### E. Un peu de refactoring ...

```rust
// src/parser.rs
trait FoldSynResult {
    type Item;

    fn fold_syn_result<O,G,FG,FC,E,>(
        self,
        init: O,
        get: FG,
        combine: FC,
    ) -> syn::Result<O>
    where
        FG: FnMut(Self::Item)->Result<G,E>,
        FC: FnMut(O,G)->O,
        E: Into<syn::Error>,
    ;
}

impl<ITER: Iterator> FoldSynResult for ITER {
    type Item = ITER::Item;

    fn fold_syn_result<O,G,FG,FC,E,>(
        self,
        init: O,
        mut get: FG,
        mut combine: FC,
    ) -> syn::Result<O>
    where
        FG: FnMut(Self::Item)->Result<G,E>,
        FC: FnMut(O,G)->O,
        E: Into<syn::Error>, {
        self
            .fold(syn::Result::Ok(init), |acc, e| {
                match get(e) {
                    Ok(g) => match acc {
                        Ok(o) => Ok(combine(o, g)),
                        e => e,
                    },
                    Err(error) => match acc {
                        Ok(_) => Err(error.into()),
                        Err(mut existing) => {
                            existing.combine(error.into());
                            Err(existing)
                        }
                    },
                }
            })
    }
}

pub fn parse_field_attributes(attrs: &Vec<syn::Attribute>) -> syn::Result<model::FieldOptions> {
    attrs
        .into_iter()
        .fold_syn_result(
            model::FieldOptions::default(),
            |attr| model::FieldOptions::from_meta(&attr.meta),
            |mut options, parsed| {
                if parsed.debug.is_some() {
                    options.debug = parsed.debug;
                }
                options
            },
        )
}


pub fn parse(input: syn::DeriveInput) -> syn::Result<model::Data> {
    // ...
    let fields = model::Fields {
        // ...
        content: fields
            // ...
            .fold_syn_result(
                Vec::with_capacity(fields_len),
                |(ordinal, field)| parse_field_attributes(&field.attrs).map(|options| (field.ident.clone(), ordinal, options)),
                |mut field_list, (ident, ordinal, options)| {
                    field_list.push(model::Field {
                        ident,
                        ordinal,
                        options,
                    });
                    field_list
                },
            )?,
    };
    // ...
}
```
