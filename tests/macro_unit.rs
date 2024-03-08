use demo_data::Data;

// Input
#[derive(Data)]
struct Unit;

// Test
mod tests {
    use super::*;
    use std::default::Default;

    #[test]
    fn unit_impl_default() {
        Unit::default();
    }

    #[test]
    fn unit_impl_debug() {
        assert_eq!(
            "Unit",
            format!("{:?}", Unit),
        );
    }
}
