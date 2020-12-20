use espritc::{run, Tokenizer};

use std::{env, fs};

fn main() {
    let args: Vec<_> = env::args().collect();

    let filename = args.get(1).unwrap_or(&String::from("main.es")).clone();

    let input = fs::read_to_string(&filename).unwrap_or(String::new());

    let mut tokenizer = Tokenizer::new(&input, &filename);
    let tokens = run(&mut tokenizer);

    match tokens {
        Ok(tokens) => {
            for token in tokens {
                println!("{}", token);
            }
        }
        Err(err) => {
            eprintln!("{}", err);

            panic!()
        }
    }
}
