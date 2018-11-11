extern crate clap;

mod common;
mod convert_encoding;

fn main() {
    let mut disp = common::Dispatcher::new();
    disp.add_cmd::<convert_encoding::Converter>();

    let mut app =
        clap::App::new("wtool")
            .version("0.1")
            .about("windows os help utils")
            .author("vaiz");
    app = disp.fill_subcommands(app);

    let matches = app.get_matches();

    let (cmd_name, args) = matches.subcommand();

    //let cmd_name = matches.value_of("command_name").unwrap();
    println!("{}", cmd_name);
    disp.run(cmd_name, args);
}
