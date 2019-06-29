use std::io::Read;

use crate::common;

use super::encoding::EncodingRef;

fn read_file(path: &str) -> std::io::Result<Vec<u8>> {
    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .open(path);

    if file.is_err() {
        return Err(file.err().unwrap());
    }

    let mut file = file.unwrap();
    let mut buf = Vec::<u8>::new();
    let result = file.read_to_end(&mut buf);
    if result.is_ok() { Ok(buf) } else { Err(result.err().unwrap()) }
}

pub enum EBomPolicy {
    EIgnore,
    EWithBom,
    EWithoutBom,
}

fn get_bom_mark(e: &EncodingRef) -> Vec<u8> {
    match e.name() {
        "utf-8" => { vec![0xEF, 0xBB, 0xBF] }
        "utf-16be" => { vec![0xFE, 0xFF] }
        "utf-16le" => { vec![0xFF, 0xFE] }
        _ => { vec![] }
    }
}

fn is_same_encoding(data: &[u8], e: &EncodingRef, bom_policy: &EBomPolicy) -> bool {
    match bom_policy {
        EBomPolicy::EIgnore => {}
        EBomPolicy::EWithBom => {
            let bom = get_bom_mark(e);
            if data.len() < bom.len() { return false; }
            for i in 0..bom.len() {
                if data[i] != bom[i] { return false; }
            }
        }
        EBomPolicy::EWithoutBom => {
            let bom = get_bom_mark(e);
            if data.len() < bom.len() { return false; }
            let mut has_bom = true;
            for i in 0..bom.len() {
                if data[i] != bom[i] {
                    has_bom = false;
                    break;
                }
            }
            if has_bom {
                return false;
            }
        }
    }

    let trap = encoding::types::DecoderTrap::Strict;
    let (result, _) =
        encoding::types::decode(&data, trap, *e);
    return result.is_ok();
}

pub fn is_file_has_same_encoding(filepath: &str, e: &EncodingRef, bom_policy: &EBomPolicy) -> std::io::Result<bool> {
    let file_content = read_file(filepath);
    if file_content.is_err() {
        return Err(file_content.err().unwrap());
    }
    let file_content = file_content.unwrap();
    Ok(is_same_encoding(&file_content[..], e, bom_policy))
}


pub struct DetectEncodingCmd;

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
        let result = read_file(filepath);
        if result.is_err() {
            eprintln!("Failed to read file {}. Error: {}", filepath, result.err().unwrap());
            return;
        }

        let file_data = result.unwrap();
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