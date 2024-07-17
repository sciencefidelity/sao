use std::{
    env,
    fs::File,
    io::{self, Write},
    process,
};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: generate_ast <output directory>");
        process::exit(64);
    }
    let output_dir = &args[1];
    define_ast(
        &output_dir,
        "Expr",
        vec![
            "Binary : Expr left, Token operator, Expr right",
            "Grouping : Expr expression",
            "Literal : Object value",
            "Unary : Token operator, Expr right",
        ],
    )?;

    Ok(())
}

fn define_ast(output_dir: &String, base_name: &str, types: Vec<&str>) -> io::Result<()> {
    let path = format!("{}/{}.rs", output_dir, base_name.to_lowercase());

    let mut buffer = File::create(path)?;
    writeln!(buffer, "struct {} {{", base_name)?;
    for t in types {
        let mut split = t.split(":");
        let class_name = split.next().unwrap().trim();
        let fields = split.next().unwrap().trim();
        define_type(&mut buffer, &base_name, &class_name, &fields)?;
    }
    writeln!(buffer)?;
    writeln!(buffer, "impl {} {{", base_name)?;
    writeln!(buffer)?;
    writeln!(buffer, "}}")?;

    Ok(())
}

fn define_type(
    buffer: &mut File,
    base_name: &str,
    class_name: &str,
    field_list: &str,
) -> io::Result<()> {
    writeln!(buffer)?;

    Ok(())
}
