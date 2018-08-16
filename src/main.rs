use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::env;
use std::io::prelude::*;

enum Token {
    Plus,
    Minus,
}

fn loadfile(path_str: &String) -> Vec<String> {
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
    string.split("\n").map(|x| x.to_string()).collect()
}

fn parse(path_str : &String) {
    let result = loadfile(path_str);

    for row in &result {
        parse_line(row);
    }
}

fn parse_line(parse_str: &String) -> Vec<Token>{
    let chars : Vec<_> = parse_str.chars().collect();
    let mut token_vec : Vec<Token> = Vec::new();
    for a_char in &chars {
        let a_token = match a_char {
            '+' => Token::Plus,
            _ => Token::Minus,
        };

        token_vec.push(a_token);
    }
    token_vec
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
