use std::io::Write;
use std::io::Read;
use std::io::BufWriter;
use std::fs::File;

mod bitpacking;
use self::bitpacking::{BitPacker, BitUnpacker};
mod coding;
use self::coding::{ACEncoder, ACDecoder};

extern crate clap;
use clap::App;

fn main() -> std::io::Result<()> {
    let matches =
        App::new("Arithmetic Encoder")
        .version("1.0")
        .about("Use arithmetic coding to encode or decode a file")
        .args_from_usage("
              -d, --decode         'Decode the input file'
              -o, --output=[file]  'Sets the output file location'
              <INPUT>              'Sets the input file to use'")
        .get_matches();

    let input_file = matches.value_of("INPUT").unwrap();
    let output_file = matches.value_of("output");

    if matches.is_present("decode") {
        let output_file_default = &(input_file.to_owned() + ".dec");
        let output_file = output_file.unwrap_or(output_file_default);

        let infile = File::open(input_file)?;
        let outfile = File::create(output_file)?;
        let mut writer = BufWriter::new(outfile);

        let bytes = infile.bytes().map(|x| x.unwrap());
        let decoder = ACDecoder::new(BitUnpacker::new(bytes));
        for byte in decoder {
            writer.write(&[byte])?;
        }

        println!("Decoded {} as {}", input_file, output_file);
    } else {
        let output_file_default = &(input_file.to_owned() + ".enc");
        let output_file = output_file.unwrap_or(output_file_default);

        let infile = File::open(input_file)?;
        let outfile = File::create(output_file)?;
        let mut writer = BufWriter::new(outfile);

        let bytes = infile.bytes().map(|x| x.unwrap());
        let encoder = BitPacker::new(ACEncoder::new(bytes));
        for byte in encoder {
            writer.write(&[byte])?;
        }

        println!("Encoded {} as {}", input_file, output_file);
    }

    Ok(())
}
