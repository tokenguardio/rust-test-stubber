#[macro_use]
extern crate clap;
use std::fs;
use clap::App;

mod ast;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let input = 
        matches
            .value_of("INPUT")
            .expect("Argument is mandatory");
    let output = 
        matches
            .value_of("OUTPUT")
            .expect("Argument is mandatory");

    process_file(input, output);
}

fn process_file(in_path: &str, out_path: &str) {
    let input 
        = fs::read_to_string(in_path)
            .expect("Couldn't open the input file");
    let processed
        = ast::process_string(input)
            .expect("Couldn't parse input file");
    fs::write(out_path, processed)
        .expect("Couldn't write to the output file");
}
    