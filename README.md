# Développer des macros en Rust

Bienvenue sur le projet de démonstration de la présentation ["Développer des macros en Rust"](https://github.com/loganmzz/rust-macro-introduction-presentation).

Le but est de développer une macro dérivative "Data" :

* Implémenter des getters
* Implémenter [`Default`](https://doc.rust-lang.org/std/default/trait.Default.html)
* Implémenter [`Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
* ...


Retrouvez ici les étapes pas-à-pas :

* [00 - Initialisation](https://github.com/loganmzz/rust-macro-introduction-code/tree/00-init)
* [01 - Blueprint](https://github.com/loganmzz/rust-macro-introduction-code/tree/01-blueprint) :arrow_down_small: (vous êtes ici)
* [02 - impl Default](https://github.com/loganmzz/rust-macro-introduction-code/tree/02-impl-default)
* [03 - Modules](https://github.com/loganmzz/rust-macro-introduction-code/tree/03-modules)
* [04 - Debug](https://github.com/loganmzz/rust-macro-introduction-code/tree/04-debug)
* [05 - Attribut](https://github.com/loganmzz/rust-macro-introduction-code/tree/05-attribute)
* [06 - Gestion des erreurs](https://github.com/loganmzz/rust-macro-introduction-code/tree/06-errors)
* [Fin](https://github.com/loganmzz/rust-macro-introduction-code/tree/99-final)

## 01 - Blueprint

### A. Structuration des blueprints

```rust
// Input: Ce que doit consommer la macro

// Output: Ce que doit produire la macro

// Test: Ce qui valide la sortie de la macro
mod tests {
    use super::*;
}
```

### B. Création du blueprint `Unit`

```rust
// tests/blueprint_unit.rs
// Input
struct Unit;

// Output
impl ::std::default::Default for Unit {
    fn default() -> Self {
        Self
    }
}

// Test
mod tests {
    use super::*;
    use std::default::Default;

    #[test]
    fn unit_impl_default() {
        <Unit as Default>::default();
    }
}
```

### C. Création du blueprint `Named`

```rust
// tests/blueprint_named.rs
// Input
struct Named {
    string: String,
    number: usize,
    boolean: bool,
}

// Output
impl ::std::default::Default for Named {
    fn default() -> Self {
        Self {
            string: ::std::default::Default::default(),
            number: ::std::default::Default::default(),
            boolean: ::std::default::Default::default(),
        }
    }
}

// Test
mod tests {
    use super::*;
    use std::default::Default;

    #[test]
    fn named_impl_default() {
        let default: Named = Default::default();

        assert_eq!(String::default(), default.string, "string");
        assert_eq!(usize::default(), default.number, "number");
        assert_eq!(bool::default(), default.boolean, "boolean");
    }
}
```

### D. Création du blueprint `Tuple`

```rust
// tests/blueprint_tuple.rs
// Input
struct Tuple(
    String,
    usize,
    bool,
);

// Output
impl ::std::default::Default for Tuple {
    fn default() -> Self {
        Self(
            ::std::default::Default::default(),
            ::std::default::Default::default(),
            ::std::default::Default::default(),
        )
    }
}

// Test
mod tests {
    use super::*;
    use std::default::Default;

    #[test]
    fn tuple_impl_default() {
        let default: Tuple = Default::default();

        assert_eq!(String::default(), default.0, "0");
        assert_eq!(usize::default(), default.1, "1");
        assert_eq!(bool::default(), default.2, "2");
    }
}
```
