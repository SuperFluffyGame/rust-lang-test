mod lang;
use math_parser::*;
// mod math;

// mod math2;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // let program_start = std::time::Instant::now();
    time!(program);

    let arg_path = std::env::args().skip(1).next().unwrap_or("".to_string());

    let base_path = std::env::current_dir()?;

    let path;

    if arg_path.starts_with("/") || arg_path.starts_with("./") {
        path = base_path.join(arg_path);
    } else {
        path = std::path::PathBuf::from(arg_path);
    }

    if !path.exists() {
        eprintln!("Path does not exist: '{}'", path.display());
        std::process::exit(1);
    }

    time!(file);

    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    time_end!(file);

    time!(tokenize);
    let tokens = lang::tokenize(contents);
    time_end!(tokenize);

    if let Ok(tokens) = tokens {
        println!("{:?}", tokens);
    } else if let Err(e) = tokens {
        eprintln!("{}", e);
    }

    time_end!(program);

    println!("\nTIMES:");
    time_print!(file);
    time_print!(tokenize);
    time_print!(program);

    Ok(())
}
