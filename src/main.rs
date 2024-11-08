use lox_treewalk::{run_file, run_prompt};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Some(_) = env::args().nth(2) {
        println!("Usage: lox_treewalk [script]");
        std::process::exit(64);
    }
    let res = match env::args().nth(1) {
        Some(file_path) => run_file(&file_path),
        None => run_prompt(),
    };
    if let Err(error) = res {
        eprintln!("Error: {error}");
    }
    Ok(())
}
