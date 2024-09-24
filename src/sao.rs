use crate::{Scanner, Token};
use std::fs::{self, File};
use std::io::{self, BufReader, Cursor, Read, Write};
use std::path::Path;

pub struct Sao;

impl Sao {
    /// # Errors
    ///
    /// Will return `Err` if `path` does not exist.
    pub fn run_file(path: &String) -> io::Result<()> {
        let path = Path::new(&path);
        let file = fs::read(path)?;
        let contents = Cursor::new(file);

        Self::run(contents);

        Ok(())
    }

    /// # Errors
    ///
    /// Will return `Err` if stdin fails to read line.
    pub fn run_prompt() -> io::Result<()> {
        loop {
            print!("> ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let buffer = input.into_bytes();
            let line = Cursor::new(buffer);

            Self::run(line);
        }
    }

    fn run(src: Cursor<Vec<u8>>) {
        let mut scanner = Scanner::new(src);
        let tokens = scanner.scan_tokens();

        println!("{tokens:#?}");
    }

    pub fn error(line: usize, msg: &str) {
        Self::report(line, "", msg);
    }

    fn report(line: usize, r#where: &str, msg: &str) {
        eprintln!("[line {line}] Error{where}: {msg}");
    }
}
