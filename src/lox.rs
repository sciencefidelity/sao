use crate::{Scanner, Token};
use std::fs::{self, File};
use std::io::{self, BufReader, Cursor, Read, Write};
use std::path::Path;

pub struct Lox;

impl Lox {
    pub fn run_file(path: &String) -> io::Result<()> {
        let path = Path::new(&path);
        let file = fs::read(&path)?;
        let contents = Cursor::new(file);

        Self::run(contents)?;

        Ok(())
    }

    pub fn run_prompt() -> io::Result<()> {
        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let buffer = input.into_bytes();
            let line = Cursor::new(buffer);

            Self::run(line)?;
        }
    }

    fn run(src: Cursor<Vec<u8>>) -> io::Result<()> {
        let mut scanner = Scanner::new(src);
        let tokens = scanner.scan_tokens();

        println!("{tokens:#?}");

        Ok(())
    }

    pub fn error(line: usize, message: &str) {
        Self::report(line, "".to_owned(), message.to_owned());
    }

    fn report(line: usize, r#where: String, message: String) {
        eprintln!("[line {line}] Error{where}: {message}");
    }
}
