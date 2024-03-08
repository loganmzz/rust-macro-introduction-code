use demo_data::Data;

// Input
#[derive(Data)]
struct Tuple(
    String,
    usize,
    bool,
);

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
