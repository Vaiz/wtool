extern crate clap;

mod common;
mod convert_encoding;

fn main() {
    let matches = clap::App::new("wtool")
        .version("0.1")
        .about("windows os help utils")
        .author("vaiz")
        .arg(
            clap::Arg::with_name("command_name")
                .required(true))
        .get_matches();

    let cmd_name = matches.value_of("command_name").unwrap();
    println!("{}", cmd_name);

    let mut disp = common::Dispatcher::new();
    disp.add_cmd::<convert_encoding::Converter>();
    disp.run(cmd_name);

}
