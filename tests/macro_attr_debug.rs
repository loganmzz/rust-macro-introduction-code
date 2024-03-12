use demo_data::Data;

// Input
#[derive(Data)]
struct AttrDebugNamedIgnoreNumber {
    string: String,
    #[data(debug=false)]
    number: usize,
    boolean: bool,
}

#[derive(Data)]
struct AttrDebugNamedForceNumber {
    string: String,
    #[data(debug=true)]
    number: usize,
    boolean: bool,
}

#[derive(Data)]
struct AttrDebugNamedNoValueNumber {
    string: String,
    #[data(debug)]
    number: usize,
    boolean: bool,
}

#[derive(Data)]
struct AttrDebugNamedUnspecifiedNumber {
    string: String,
    #[data()]
    number: usize,
    boolean: bool,
}

#[derive(Data)]
struct AttrDebugTupleIgnore1(
    String,
    #[data(debug=false)]
    usize,
    bool,
);

// Test
mod tests {
    use super::*;

    #[test]
    fn named_impl_debug_false() {
        let debug = AttrDebugNamedIgnoreNumber {
            string: "world".to_string(),
            number: 42,
            boolean: false,
        };

        assert_eq!(
            "AttrDebugNamedIgnoreNumber { string: \"world\", boolean: false }",
            format!("{:?}", debug),
        );
    }

    #[test]
    fn named_impl_debug_true() {
        let debug = AttrDebugNamedForceNumber {
            string: "world".to_string(),
            number: 42,
            boolean: false,
        };

        assert_eq!(
            "AttrDebugNamedForceNumber { string: \"world\", number: 42, boolean: false }",
            format!("{:?}", debug),
        );
    }

    #[test]
    fn named_impl_debug_novalue() {
        let debug = AttrDebugNamedNoValueNumber {
            string: "world".to_string(),
            number: 42,
            boolean: false,
        };

        assert_eq!(
            "AttrDebugNamedNoValueNumber { string: \"world\", number: 42, boolean: false }",
            format!("{:?}", debug),
        );
    }

    #[test]
    fn named_impl_debug_unspecified() {
        let debug = AttrDebugNamedUnspecifiedNumber {
            string: "world".to_string(),
            number: 42,
            boolean: false,
        };

        assert_eq!(
            "AttrDebugNamedUnspecifiedNumber { string: \"world\", number: 42, boolean: false }",
            format!("{:?}", debug),
        );
    }

    #[test]
    fn tuple_impl_debug() {
        let debug = AttrDebugTupleIgnore1(
            "world".to_string(),
            42,
            false,
        );

        assert_eq!(
            "AttrDebugTupleIgnore1(\"world\", false)",
            format!("{:?}", debug),
        );
    }
}
