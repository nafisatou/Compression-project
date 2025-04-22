use std::fs;
use std::fs::File;
use std::io::Write;
use clap::{Arg, Command};

mod rle;
mod lz;


fn main() {
    let matches = Command::new("RustCompressor")
        .about("Compress and decompress files")
        .arg(Arg::new("mode").value_name("compress|decompress").required(true))
        .arg(Arg::new("input").value_name("INPUT").required(true))
        .arg(Arg::new("output").value_name("OUTPUT").required(true))
        .arg(Arg::new("rle").long("rle").action(clap::ArgAction::SetTrue).conflicts_with("lz"))
        .arg(Arg::new("lz").long("lz").action(clap::ArgAction::SetTrue))
        .get_matches();
    

    let mode = matches.get_one::<String>("mode").unwrap();
    let input_path = matches.get_one::<String>("input").unwrap();
    let output_path = matches.get_one::<String>("output").unwrap();

    let input_data = fs::read(input_path).expect("Failed to read input file");
    let use_rle = matches.get_flag("rle");
    let use_lz = matches.get_flag("lz");

    let result = match (mode.as_str(), use_rle, use_lz) {
        ("compress", true, _) => rle::compress(&input_data),
        ("decompress", true, _) => rle::decompress(&input_data),
        ("compress", _, true) => lz::compress(&input_data),
        ("decompress", _, true) => lz::decompress(&input_data),
        _ => {
            eprintln!("Please specify a valid mode (compress/decompress) with --rle or --lz");
            std::process::exit(1);
        }
        
    };

    let mut output_file = File::create(output_path).expect("Failed to create output file");
    output_file.write_all(&result).expect("Failed to write output");
}

