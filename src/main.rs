#[macro_use]
extern crate clap;
use clap::App;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let input_str = matches.value_of("INPUT").unwrap();
    let input = fs::read(input_str)?;

    // TODO: implement compress/decompress as separate modules in different files
    if matches.is_present("decompress") {
        println!("Decompressing file {}...", input_str);
        println!("Done");
    } else {
        println!("Compressing file {}...", input_str);
        println!("Done");
    }

    Ok(())
}
