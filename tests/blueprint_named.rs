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

    #[test]
    fn named_impl_default() {
        let default = Named::default();

        assert_eq!(String::default(), default.string, "string");
        assert_eq!(usize::default(), default.number, "number");
        assert_eq!(bool::default(), default.boolean, "boolean");
    }
}
