use rlox::Lox;
use std::{env, io, process};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => Lox::run_prompt(),
        2 => Lox::run_file(&args[1]),
        _ => {
            println!("Usage: jlox [script]");
            process::exit(64);
        }
    }?;

    Ok(())
}
