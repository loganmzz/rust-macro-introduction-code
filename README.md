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
* [06 - Gestion des erreurs](https://github.com/loganmzz/rust-macro-introduction-code/tree/06-errors)
* [07 - API publique](https://github.com/loganmzz/rust-macro-introduction-code/tree/07-public-api) :arrow_down_small: (vous êtes ici)
* [Fin](https://github.com/loganmzz/rust-macro-introduction-code/tree/99-final)

## 07 - API publique

Une crate `proc-macro` ne peut exporter que des macros.
Mais il est possible de créer une crate "chapeau" / "publique".

### A. Création de la crate `api`

```shell
cargo new api --lib --name demo-data-api
```

```toml
# api/Cargo.toml
[package]
name = "demo-data-api"
version.workspace = true
edition.workspace = true
```

```rust
// api/src/lib.rs
use std::fmt::Debug;

pub trait Data : Default + Debug {}
```

### B. Création de la crate `macro`

```shell
cargo new macro --lib --name demo-data-macro &&
mv src/*.rs macro/src/
```

_Note : les tests d'intégrations (`/tests`) restent à la racine car le code généré peut dépendre de l'API._

```toml
# macro/Cargo.toml
[package]
name = "demo-data-macro"
version.workspace = true
edition.workspace = true

[lib]
proc-macro = true         # Ajoute la crate `proc-macro` lors de la compilation.
                          # Mais pas accessible pour les tests...
                          # La crate ne peut plus exporter que des macros !

[dependencies]
darling = { workspace = true }        # Permet de parser les attributs dans une structure
                                      # personnalisée à la manière de serde
proc-macro2 = { workspace = true }    # Alternative à `proc-macro2` accessible pour écrire des tests ou des libs.
quote = { workspace = true }          # Génération de flux de tokens.
syn = { workspace = true }            # Analyseur syntaxique

[features]
debug = [
  "syn/extra-traits",     # Permet de débugger les types de syn
]
```

### C. Gestion de la crate "chapeau"

```toml
# Cargo.toml
[package]
name = "demo-data"
version = "0.1.0"
edition = "2021"

[workspace]
members = [
  "api",
  "macro",
]
default-members = [
    ".",
    "api",
    "macro",
]

[workspace.package]
version = "0.1.0"
edition = "2021"

[workspace.dependencies]  # Permet de définir un référentiel des versions
darling = "0.20.8"
proc-macro2 = "1.0.73"
quote = "1.0.34"
syn = "2.0.44"


[dependencies]
demo-data-api = { version = "=0.1.0", path = "api" }      # On synchronise les version avec la crate "chapeau"
demo-data-macro = { version = "=0.1.0", path = "macro" }  # On synchronise les version avec la crate "chapeau"

[features]
debug = [
  "demo-data-macro/debug",     # Permet de débugger les types de syn
]
test_fail = []
```

```rust
// src/lib.rs
pub use ::demo_data_api::*;
pub use ::demo_data_macro::*;
```

### D. Publication

```shell
cargo publish --package demo-data-api &&   # D'abord l'API
cargo publish --package demo-data-macro && # Puis les macros
cargo publish --package demo-data          # Enfin la crate "chapeau"
```
