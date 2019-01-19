use std::io::Read;

use crate::common;

pub struct DetectEncodingCmd;

impl DetectEncodingCmd {
    fn read_file(path: &str) -> Vec<u8> {
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open(path).unwrap();
        let mut buf = Vec::<u8>::new();
        file.read_to_end(&mut buf);
        buf
    }
}

impl common::Command for DetectEncodingCmd {
    fn create() -> Box<DetectEncodingCmd> { Box::<_>::new(DetectEncodingCmd {}) }
    fn name() -> &'static str { "detect" }
    fn fill_subcommand<'a, 'b>(&self, app: clap::App<'a, 'b>) -> clap::App<'a, 'b>
    {
        let sub_cmd =
            clap::App::new(Self::name())
                .arg(clap::Arg::with_name("filepath").required(true));
        app.subcommand(sub_cmd)
    }
    fn run(&self, args: Option<&clap::ArgMatches>)
    {
        let args = args.unwrap();
        let filepath = args.value_of("filepath").unwrap();
        let file_data = DetectEncodingCmd::read_file(filepath);


        let encodings = encoding::all::encodings();
        let trap = encoding::types::DecoderTrap::Strict;
        for en in encodings {
            let fail_back_encoder = *en;

            let (result, encoder_ref) =
                encoding::types::decode(&file_data[..], trap, fail_back_encoder);

            if result.is_ok() {
                let file_content = result.unwrap();
                let first_line = file_content.lines().next();
                if first_line.is_some() {
                    let line = first_line.unwrap();
                    println!("{}: {}", en.name(), if line.len() < 60 { line } else { line[0..60].as_ref() });
                }
            }
        }
    }
}