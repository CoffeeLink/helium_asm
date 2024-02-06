use std::fs::read_to_string;
use std::process::exit;
use helium_asm::Config;
use helium_asm::helium::errors::Error;
use helium_asm::helium::lexer::Lexer;
use helium_asm::helium::parser::{Parser};
/*
TODO: Refactor everything to as readable as possible. (priority: Less Tabs)

Features to finish:
    - Include system
    - const pre_load constants with a const storage
 */

fn main() {
    let conf = Config::from_args()
        .expect("Epic Fail");
    let name = conf.get_file();
    let file_contents = read_to_string(name.clone())
        .expect("Failed to read file.");

    let tokens = Lexer::new(&file_contents)
        .lex()
        .unwrap_or_else(|e|{
            display_errors_and_exit(e)
    });

    let parsed = Parser::new(&tokens, name)
        .parse(None)
        .unwrap_or_else(|e|{
            display_errors_and_exit(e)
        });

    println!("{:?}", parsed)
}

fn display_errors_and_exit(errors : Vec<Error>) -> ! {
    println!("Errors : {:?}", errors);
    exit(1)
}
