
mod lexer;
mod syntaxanalyzer;
mod semantic;
mod htmlgenerator;

use lexer::Lexer;
use syntaxanalyzer::{Parser, SyntaxAnalyzer};
use semantic::SemanticAnalyzer;
use htmlgenerator::HtmlGenerator;

use std::env;
use std::fs;
use std::process;

// Runs the compiler steps in order. If any step fails, it prints the error and stops.
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: lolcompiler <file.lol>");
        process::exit(1);
    }

    let filename = &args[1];

    if !filename.to_lowercase().ends_with(".lol") {
        eprintln!("Error: input file must have a .lol extension");
        process::exit(1);
    }

    let source = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            process::exit(1);
        }
    };

    let mut lexer = Lexer::new(&source);

    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);

    if let Err(e) = parser.parse_lolcode() {
        eprintln!("{}", e);
        process::exit(1);
    }

    println!("Syntax analysis successful.");

    let tree = match &mut parser.tree {
        Some(tree) => tree,
        None => {
            eprintln!("Error: parse tree was not created");
            process::exit(1);
        }
    };

    let mut semantic = SemanticAnalyzer::new();

    if let Err(e) = semantic.analyze(tree) {
        eprintln!("{}", e);
        process::exit(1);
    }

    println!("Semantic analysis successful.");

    let generator = HtmlGenerator::new();

    let output_path = match generator.generate(tree, filename) {
        Ok(path) => path,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    println!("HTML generated: {}", output_path.display());

    if let Err(e) = generator.open_browser(&output_path) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
