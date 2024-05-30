use std::fs::File;
use std::io::{self, BufReader, Read, Write};

pub fn run_file(path: &String) -> io::Result<()> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    run(contents);

    Ok(())
}

pub fn run_prompt() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        if !line.trim().is_empty() {
            run(line);
        }
    }
}

pub fn run(source: String) {
    print!("RUN: {}", source);
}
