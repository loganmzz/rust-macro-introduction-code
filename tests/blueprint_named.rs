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
    use super::*;
    use std::default::Default;

    #[test]
    fn named_impl_default() {
        let default: Named = Default::default();

        assert_eq!(String::default(), default.string, "string");
        assert_eq!(usize::default(), default.number, "number");
        assert_eq!(bool::default(), default.boolean, "boolean");
    }

    #[test]
    fn named_impl_debug() {
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
