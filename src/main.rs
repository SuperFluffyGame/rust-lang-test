mod lang;

// mod math;

// mod math2;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // let string = std::env::args().skip(1).next().unwrap();

    let path = r"C:\Users\super\OneDrive\Documents\coding\RUST\math-parser\test.txt";

    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let tokens = lang::tokenize(contents);

    if let Ok(tokens) = tokens {
        println!("{:#?}", tokens);
    } else if let Err(e) = tokens {
        println!("{}", e);
    }

    Ok(())
}
