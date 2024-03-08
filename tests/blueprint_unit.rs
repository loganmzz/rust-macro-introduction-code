// Input
struct Unit;

// Output
impl ::std::default::Default for Unit {
    fn default() -> Self {
        Self
    }
}

impl ::std::fmt::Debug for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f
            .debug_struct("Unit")
            .finish()
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

    #[test]
    fn unit_impl_debug() {
        assert_eq!(
            "Unit",
            format!("{:?}", Unit),
        );
    }
}
