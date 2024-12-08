use lox_treewalk::{interpreter::Interpreter, parser::Parser, scanner::Scanner};
use std::{
    env,
    error::Error,
    fs,
    io::{self, Write},
};

fn main() -> Result<(), Box<dyn Error>> {
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

pub fn run_file(file_path: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    let mut interpreter = Interpreter::new();
    let exit_code = run(&contents, &mut interpreter);
    if exit_code != 0 {
        std::process::exit(exit_code);
    }
    Ok(())
}

pub fn run_prompt() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    let mut interpreter = Interpreter::new();
    loop {
        buffer.clear();
        print!("> ");
        io::stdout().flush()?;
        match io::stdin().read_line(&mut buffer) {
            Ok(n) => {
                if n == 1 {
                    break;
                }
                run(&buffer.trim(), &mut interpreter);
            }
            Err(error) => {
                eprintln!("Error: {error}");
                break;
            }
        }
    }
    Ok(())
}

fn run(source: &str, interpreter: &mut Interpreter) -> i32 {
    let mut scanner = Scanner::new(source);
    scanner.scan_tokens();

    let mut parser = Parser::new(scanner.tokens);
    let parse_result = parser.parse();

    if parse_result.is_err() || scanner.had_error {
        return 65;
    }

    match interpreter.interpret(parse_result.unwrap()) {
        Ok(()) => (),
        Err(error) => {
            println!("{error}");
            return 70;
        }
    }

    0
}
