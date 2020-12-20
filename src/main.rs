use espritc::run;

use std::{env, fs};

fn main() {
    let args: Vec<_> = env::args().collect();

    let filename = args.get(1).unwrap_or(&String::from("main.es")).clone();

    let input = fs::read_to_string(filename).unwrap_or(String::new());

    run(&input);
}
