use esprit::run;

use std::fs;

fn main() {
    let input = fs::read_to_string("examples/main.es").unwrap();

    run(&input);
}
