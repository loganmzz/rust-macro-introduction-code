#[cfg(feature = "test_fail")]
mod test_fail {

  use demo_data::Data;

  #[derive(Data)]
  struct FailNotCompile {
      string: String,
      #[data(debug=false,foobar,baroof,)]
      #[data(barfoo,)]
      number: usize,
      boolean: bool,
      #[data(foobaz)]
      foobaz: bool,
  }

  #[derive(Data)]
  enum UnsupportedEnum {
    SomeVariant,
  }

  #[derive(Data)]
  union UnsupportedUnion {
    number: usize,
  }
}
