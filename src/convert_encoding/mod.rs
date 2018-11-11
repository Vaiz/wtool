
use common;

pub struct Converter;

impl Converter {
    fn new() -> Converter {
        Converter {}
    }
}

impl common::Command for Converter {
    fn create() -> Box<Converter> {
        Box::<Converter>::new(Converter::new())
    }
    fn name() -> &'static str {
        "convert_encoding"
    }
    fn fill_subcommand<'a,'b>(&self, app : clap::App<'a,'b>) -> clap::App<'a,'b> {
        let sub_cmd =
            clap::App::new(Self::name())
                .arg(clap::Arg::with_name("path").required(true))
                .arg(
                    clap::Arg::with_name("folder")
                        .short("-f")
                        .long("--folder"))
                .arg(
                    clap::Arg::with_name("recursive")
                        .short("-r")
                        .long("--recursive"))
                .arg(
                    clap::Arg::with_name("codepage")
                        .short("-c")
                        .long("--codepage")
                        .required(true)
                        .takes_value(true));
        app.subcommand(sub_cmd)
    }
    fn run(&self, _: Option<&clap::ArgMatches>) {
        println!("{}: TODO", Self::name());
    }
}