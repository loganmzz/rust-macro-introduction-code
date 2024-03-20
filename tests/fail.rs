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
