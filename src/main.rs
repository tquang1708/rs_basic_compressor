#[macro_use]
extern crate clap;
use clap::App;
use std::fs;

mod lzw;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let input_str = matches.value_of("INPUT").unwrap();
    let input = fs::read(input_str)?;

    if !matches.is_present("decompress") {
        println!("Compressing file {}...", input_str);

        let lzw_output = lzw::lzw_encode(input)?;
        fs::write(input_str.to_owned() + ".compressed", lzw_output)?;

        println!("Done");
    } else {
        println!("Decompressing file {}...", input_str);

        let lzw_output = lzw::lzw_decode(input)?;
        fs::write(input_str.to_owned() + ".decompressed", lzw_output)?;

        println!("Done");
    }

    Ok(())
}
