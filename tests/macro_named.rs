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
}
