use helium_asm::helium::errors::Error;
use helium_asm::helium::lexer::Lexer;
use helium_asm::helium::parsing;
use helium_asm::Config;
use std::fs::read_to_string;
use std::process::exit;

use owo_colors::OwoColorize;
/*
TODO: Features to finish:
    - Instruction Validity check
    - refactor errors to a global error system.
    - assembler.
    -
 */

fn main() {
    let conf = Config::from_args().unwrap_or_else(|| panic!("{}", "No file provided.".red()));
    let name = conf.get_file();
    let file_contents =
        read_to_string(name.clone()).unwrap_or_else(|_| panic!("{}", "Failed to read file.".red()));

    let tokens = Lexer::new(&name, &file_contents)
        .lex()
        .unwrap_or_else(|e| display_errors_and_exit(e));
    let tree = parsing::Parser::new(&tokens, name.clone())
        .parse(None)
        .unwrap_or_else(|e| {
            for err in e {
                println!("{}", err.red().bold());
            }
            exit(1)
        });

    print!("{}", tree);
}

fn display_errors_and_exit(errors: Vec<Error>) -> ! {
    println!("Errors : {:?}", errors);
    exit(1)
}
