use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::env;
use std::io::prelude::*;

fn parse(path_str : &String) {
    let path = Path::new(path_str);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open"),
        Ok(file) => file,
    };

    let mut string = String::new();
    match file.read_to_string(&mut string) {
        Err(why) => panic!("Couldn't open"),
        Ok(_) => print!("contains:\n {}", string),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            parse(&args[1]);
        },
        _ => {
            println!("Usage: {} FILENAME", &args[0]);
        },
    };
}
