#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    // example of argument parsing from the clap documentation
    // https://docs.rs/clap/2.33.3/clap/
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // input is a required arg so we can unwrap safely
    let input_str = matches.value_of("INPUT").unwrap();
}
