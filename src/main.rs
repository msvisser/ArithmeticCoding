use std::io::Write;
use std::io::Read;
use std::io::BufWriter;
use std::fs::File;
use std::time::Instant;

mod bitpacking;
use self::bitpacking::{BitPacker, BitUnpacker};
mod coding;
use self::coding::{ACEncoder, ACDecoder, Probability, ProbabilityZeroOrder, ProbabilityFirstOrder};

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
              --first-order        'Enable first order encode/decode'
              <INPUT>...           'Sets the input file to use'")
        .get_matches();

    let input_files = matches.values_of("INPUT").unwrap();
    let output_file = matches.value_of("output");

    if let Some(_) = output_file {
        if matches.occurrences_of("INPUT") > 1 {
            println!("Cannot specify output file when multiple input files are used");
            return Ok(())
        }
    }

    for input_file in input_files {
        let start = Instant::now();

        // Start off with equal probabilities for each symbol
        let probs = vec![1; 257];
        let probability: Box<Probability> = if matches.is_present("first-order") {
            Box::new(ProbabilityFirstOrder::new(&probs))
        } else {
            Box::new(ProbabilityZeroOrder::new(&probs))
        };

        if matches.is_present("decode") {
            let output_file_default = &(input_file.to_owned() + ".dec");
            let output_file = output_file.unwrap_or(output_file_default);

            let infile = File::open(input_file)?;
            let outfile = File::create(output_file)?;
            let mut writer = BufWriter::new(outfile);

            let bytes = infile.bytes().map(|x| x.unwrap());
            let decoder = ACDecoder::new(BitUnpacker::new(bytes), probability);
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
            let encoder = BitPacker::new(ACEncoder::new(bytes, probability));
            for byte in encoder {
                writer.write(&[byte])?;
            }

            println!("Encoded {} as {}", input_file, output_file);
        }

        let end = Instant::now();
        let duration = end - start;
        println!("{:?}", duration);
    }

    Ok(())
}
