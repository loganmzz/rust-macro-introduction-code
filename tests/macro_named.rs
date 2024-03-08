use demo_data::Data;

// Input
#[derive(Data)]
struct Named {
    string: String,
    number: usize,
    boolean: bool,
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
