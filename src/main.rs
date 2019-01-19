extern crate clap;

mod common;
mod fs;
mod encoding;

fn main() {
    let mut disp = common::Dispatcher::new();
    disp
        .add_cmd::<fs::FileSystemDispatcher>()
        .add_cmd::<encoding::EncodingDispatcher>();

    let mut app =
        clap::App::new("wtool")
            .version("0.1")
            .about("windows os help utils")
            .author("vaiz");
    app = disp.fill_subcommands(app);

    let matches = app.get_matches();
    let (cmd_name, args) = matches.subcommand();

    if cmd_name.is_empty() {
        print!("{}", matches.usage());
        return;
    }

    disp.run(cmd_name, args);
}
