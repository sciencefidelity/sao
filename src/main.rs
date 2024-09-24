use sao::Sao;
use std::{env, io, process};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => Sao::run_prompt(),
        2 => Sao::run_file(&args[1]),
        _ => {
            println!("Usage: sao [script]");
            process::exit(64);
        }
    }?;

    Ok(())
}
