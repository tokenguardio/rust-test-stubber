#[macro_use]
extern crate clap;
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

    ast::process_file(input, output)
        .expect("Cannot process the file");
}
    