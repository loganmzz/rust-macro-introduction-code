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
