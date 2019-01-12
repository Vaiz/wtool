extern crate encoding;

use convert_encoding::encoding::types::{/*RawDecoder,DecoderTrap,StringWriter,*/EncodingRef};
use std::io::Read;
use std::io::Write;
use common;

pub struct Converter;

impl Converter {
    fn new() -> Converter {
        Converter {}
    }
}

impl Converter {
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
    fn write_file(path: &str, data: &Vec<u8>) {
        let mut file = std::fs::OpenOptions::new()
            .read(false)
            .write(true)
            .create(true)
            .truncate(true)
            .open(path).unwrap();
        file.write_all(&data[..]);
        file.flush();
    }
    /*fn decode(data: &Vec<u8>) -> (Option<String>, encoding::types::EncodingRef) {
        let encodings = encoding::all::encodings();
        let trap = encoding::types::DecoderTrap::Strict;
        for en in encodings {
            let fail_back_encoder = *en;
            let (result, encoder_ref) = encoding::types::decode(&data[..], trap, fail_back_encoder);
            if result.is_ok() {
                return (Some(result.unwrap()), encoder_ref);
            }
        }
        (None, encoding::all::ERROR)
    }*/
    fn convert_file(
        src_path: &str,
        tgt_path: &str,
        decoder_ref: convert_encoding::encoding::types::EncodingRef,
        encoder_ref: convert_encoding::encoding::types::EncodingRef,
    ) {
        let file_data = Self::read_file(src_path);
        if file_data.is_empty() { return; }

        //let (result, decoder_ref) = Self::decode(&file_data);
        let result = decoder_ref.decode(&file_data[..], encoding::types::DecoderTrap::Strict);

        if result.is_err() {
            println!("Failed to decode file {}", src_path);
            return;
        }

        let mut result = result.unwrap();
        let result = encoder_ref.encode(result.as_str(), encoding::types::EncoderTrap::Strict);
        Self::write_file(tgt_path, &result.unwrap());

        println!("Converting complete. Source file: {}, Target file {}, Decoder: {}, Encoder: {}",
                 src_path, tgt_path, decoder_ref.name(), encoder_ref.name());
    }
    fn convert_folder(
        src_path: &str,
        tgt_path: &str,
        decoder_ref: convert_encoding::encoding::types::EncodingRef,
        encoder_ref: convert_encoding::encoding::types::EncodingRef,
        extension: Option<&str>,
        recursive: bool,
    ) {
        let mut dirs = std::collections::VecDeque::<(String,String)>::new();
        dirs.push_back((String::from(src_path), String::from(tgt_path)));

        /*while !dirs.is_empty() {
            let (src_path, tgt_path) = dirs.pop_front().unwrap();
            let read_dir_res = std::fs::read_dir(src_path);
            for entry in read_dir_res.into_iter() {
                //let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    let file_name = entry.file_name().unwrap();
                    let tgt_path = tgt_path + file_name;
                    dirs.push_back(String::from(path), tgt_path);
                } else if path.is_file() {
                    if extension.is_some() {
                        if path.extension != extension {
                            continue;
                        }
                    }
                    Self::convert_file(path.to_str().unwrap(), tgt_path + entry.file_name().unwrap(), decoder_ref, encoder_ref);
                }
            }
        }*/
    }
}

impl common::Command for Converter {
    fn create() -> Box<Converter> {
        Box::<Converter>::new(Converter::new())
    }
    fn name() -> &'static str {
        "convert_encoding"
    }
    fn fill_subcommand<'a, 'b>(&self, app: clap::App<'a, 'b>) -> clap::App<'a, 'b> {
        let sub_cmd =
            clap::App::new(Self::name())
                .arg(clap::Arg::with_name("path").required(true))
                .arg(
                    clap::Arg::with_name("folder")
                        .short("f")
                        .long("folder"))
                .arg(
                    clap::Arg::with_name("recursive")
                        .short("r")
                        .long("recursive"))
                .arg(
                    clap::Arg::with_name("source_codepage")
                        .long("src_codepage")
                        .required(true)
                        .takes_value(true)
                        .help("Supported encodings: utf8, cp1251, ..."))
                .arg(
                    clap::Arg::with_name("target_codepage")
                        .long("tgt_codepage")
                        .required(true)
                        .takes_value(true)
                        .help("Supported encodings: utf8, cp1251, ..."))
                .arg(
                    clap::Arg::with_name("target_path")
                        .long("tgt_path")
                        .takes_value(true)
                        .help("result files path"))
                .arg(
                    clap::Arg::with_name("extension")
                        .short("e")
                        .long("extension")
                        .takes_value(true));

        app.subcommand(sub_cmd)
    }
    fn run(&self, args: Option<&clap::ArgMatches>) {
        let args = args.unwrap();
        let src_path = args.value_of("path").unwrap();
        let tgt_path =
            if args.is_present("target_path")
                { args.value_of("target_path").unwrap() } else { src_path };
        let decoder = args.value_of("source_codepage").unwrap();
        let encoder = args.value_of("target_codepage").unwrap();
        let decoder_ref = encoding::label::encoding_from_whatwg_label(decoder).unwrap();
        let encoder_ref = encoding::label::encoding_from_whatwg_label(encoder).unwrap();
        let is_folder = args.is_present("folder");

        if decoder == encoder {
            println!("Source and Target encoding are same");
            return;
        }

        println!("Source path: [{}], Target path: [{}], Decoder: [{}], Encoder: [{}]", src_path, tgt_path, decoder, encoder);

        if is_folder { panic!("Folders not implemented");
        } else { Self::convert_file(src_path, tgt_path, decoder_ref, encoder_ref); }
    }
}