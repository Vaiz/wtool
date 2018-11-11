use common;
//use common::Command;

//#[derive(Command)]
pub struct Converter;

impl Converter {
    fn new() -> Converter {
        Converter {}
    }
}
//impl common::Command for Converter {
impl common::Command for Converter {
    fn create() -> Box<Converter> {
        Box::<Converter>::new(Converter::new())
    }
    fn name() -> &'static str {
        "convert-encoding"
    }
    fn run(&self) {
        println!("{}: TODO", Self::name());
    }
}