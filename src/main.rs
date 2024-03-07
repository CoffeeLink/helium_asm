use helium_asm::helium::errors::HeliumError;
use helium_asm::helium::lexer::Lexer;
use helium_asm::helium::parsing;
use helium_asm::Config;
use std::fs::read_to_string;
use std::process::exit;

use owo_colors::OwoColorize;
use helium_asm::helium::validator::validate_tree;
/*
TODO: Features to finish:
    - Instruction Validity check
    - refactor errors to a global error system.
    - assembler.
 */

fn main() {
    let conf = Config::from_args().unwrap_or_else(|| panic!("{}", "No file provided.".red()));
    let name = conf.get_file();
    let file_contents =
        read_to_string(name.clone()).unwrap_or_else(|_| panic!("{}", "Failed to read file.".red()));

    let tokens = Lexer::new(&name, &file_contents)
        .tokenize()
        .unwrap_or_else(|e| display_errors_and_exit(e, &file_contents));

    let tree = parsing::Parser::new(&tokens, name.clone())
        .parse(None)
        .unwrap_or_else(|e| {
            for err in e {
                println!("{}", err.red().bold());
            }
            exit(1)
        });
    print!("{}", tree);

    let errors = validate_tree(&tree);

    if !errors.is_empty() {
        display_errors_and_exit(errors, &file_contents)
    }
}

fn display_errors_and_exit(errors: Vec<HeliumError>, source: &str) -> ! {
    for err in errors {

        let line = source.lines().nth(err.pos.line).unwrap();
        let error_line = format!("{}{}{}",
                                 err.pos.find_before(line).unwrap(),
                                 err.pos.find(line).unwrap().to_string().underline().yellow(),
                                 err.pos.find_after(line).unwrap());
        let error_line = error_line.trim();

        println!("{} {}:{}", "An error occurred on: ".bright_blue(), err.pos.line.underline(), (err.pos.chr_start + err.pos.length).underline());
        println!("   {}", "| ".blue());
        println!("{} {}{}",
                 err.pos.line.bright_red(),
                 "|".blue(),
                 error_line);
        println!("   {}", "|\n".blue());
        println!("{}", err.message.bright_red());
    }
    exit(1)
}
