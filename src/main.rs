mod math;
// mod math2;
use std::io::prelude::Write;

fn main() -> std::io::Result<()> {
    let string = std::env::args().skip(1).next();

    let ast;
    match string {
        Some(val) => {
            let tokens = math::tokenize(&mut val.chars().peekable()).unwrap();
            // println!("{:?}", tokens);
            ast = math::parse_ast(&tokens);
        }
        _ => {
            std::io::stderr().write(b"No Input Provided\n")?;
            std::process::exit(1);
        }
    }

    // println!("{:#?}", ast);

    if let Ok(result) = math::eval_ast(&ast) {
        println!("{:?}", result);
    }

    Ok(())
}
