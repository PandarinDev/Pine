mod ast_parser;
mod ast_executor;
mod tokens;

use ast_parser::AstParser;
use ast_executor::AstExecutor;

const PINE_FILE_EXTENSION: &str = "pn";

fn main() {
    if std::env::args().len() < 2 {
        eprintln!("Usage: pine <source_file>");
        std::process::exit(1);
    }
    let file_path = std::path::PathBuf::from(std::env::args().nth(1).unwrap());
    if !file_path.is_file() {
        eprintln!("File at '{}' does not exist.", file_path.to_string_lossy());
        std::process::exit(1);
    }
    let extension = file_path.extension();
    if extension.is_none() || extension.unwrap() != PINE_FILE_EXTENSION {
        eprintln!("Pine can only parse '.{}' files.", PINE_FILE_EXTENSION);
        std::process::exit(1);
    }

    let source = match std::fs::read_to_string(&file_path) {
        Ok(source) => source,
        Err(_) => {
            eprintln!("Failed to read file '{}'.", file_path.to_string_lossy());
            std::process::exit(1);
        }
    };

    let ast_parser = AstParser;
    let ast = match ast_parser.parse(&source) {
        Ok(ast) => ast,
        Err(_) => {
            eprintln!("Failed to parse source code.");
            std::process::exit(1);
        }
    };

    AstExecutor::execute(&ast);
}
