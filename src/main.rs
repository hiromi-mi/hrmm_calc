use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::env;
use std::io::prelude::*;

pub enum Token {
    Plus,
    Minus,
    Multiply,
    Divide,
    OpenParensis,
    CloseParensis,
    Int(u32),
    Float(f32),
    String(String),
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
        Err(why) => panic!("Couldn't read files."),
        Ok(_) => print!("contains:\n {}", string),
    }
    string.split("\n").map(|x| x.to_string()).collect()
    //string.split("\n").map(|x| x.to_string()).collect()
}

fn parse(path_str : &String) {
    let result = loadfile(path_str);

    for row in &result {
        let lexer = Lexer::new(row);
        lexer.parse_l();
    }
}

#[derive(Default)]
pub struct Lexer {
    chars: Vec<char>,
    string: String,
    pos: usize,
}

impl Lexer {
    pub fn new(parse_str : &String) -> Lexer {
        Lexer {chars : parse_str.chars().collect(), string : parse_str.clone(), pos : 0}
    }

    pub fn parse_l(&self) -> Vec<Token> {
        let mut tokens = Vec::new();
        tokens.push(self.parse_next());
        tokens
    }

    fn parse_next(&self) -> Token {
        let a_char = self.chars[self.pos];
        let a_token = match a_char {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Multiply,
            '/' => Token::Divide,
            '(' => Token::OpenParensis,
            ')' => Token::CloseParensis,
            '"' => self.parse_string(),
            '0'...'9' => self.parse_int(),
            _ => unreachable!(),
        };
        a_token
    }

    fn parse_string(&self) -> Token {
        Token::String("".to_string())
    }

    fn parse_int(&self) -> Token {
        let lastindex;
        for (index,item) in self.chars[self.pos,self.chars.len()].iter().enumerate() {
            if !item.is_digit(10) {
                lastindex = index-1+self.pos;
            }
        }
        Token::Int(self.chars[self.pos,lastindex]::<str>::collect()::parse())
    }
}

fn parse_line(parse_str: &str) -> Vec<Token>{
    let chars : Vec<_> = parse_str.chars().collect();
    let mut token_vec : Vec<Token> = Vec::new();
    for (index, a_char) in chars.iter().enumerate() {
        let a_token = match a_char {
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Multiply),
            '/' => Some(Token::Divide),
            '(' => Some(Token::OpenParensis),
            ')' => Some(Token::CloseParensis),
            ' ' => None,
            a @ _ => None,
        };

        if let Some(a_token_) = a_token {
            token_vec.push(a_token_);
        }
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
