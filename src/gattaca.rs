use std::env;
use std::io::{self, Read, Write};

fn main() {
    let usage = "usage: gattaca <-e/-d>";
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("{}", usage);
        std::process::exit(1);
    }

    match args[1].as_str() {
        "-e" => {
            let mut input = String::new();
            io::stdin().read_to_string(&mut input).unwrap();
            let input = input.into_bytes();
            let output = gattaca::to_gattaca(input);
            io::stdout().write_all(&output).unwrap();
        },
        "-d" => {
            let mut input = String::new();
            io::stdin().read_to_string(&mut input).unwrap();
            let input = input.into_bytes();
            let output = gattaca::from_gattaca(input);
            io::stdout().write_all(&output).unwrap();
        },
        _ => {
            println!("{}", usage);
            std::process::exit(1);
        }
    }
}