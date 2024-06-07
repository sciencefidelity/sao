use rlox::Lox;

use std::env;
use std::io;
use std::process;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: jlox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        let _ = Lox::run_file(&args[1])?;
    } else {
        Lox::run_prompt()?;
    }

    Ok(())
}
