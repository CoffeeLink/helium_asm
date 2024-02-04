use std::fs::read_to_string;
use std::process::exit;
use helium_asm::Config;
use helium_asm::helium::errors::Error;
use helium_asm::helium::lexer::Lexer;

fn main() {
    let conf = Config::from_args()
        .expect("Epic Fail");
    let file_contents = read_to_string(conf.get_file())
        .expect("Failed to read file.");

    let tokens = match Lexer::new(&file_contents)
        .lex()
    {
        Ok(tokens) => tokens,
        Err(errors) => {
            display_errors(errors);
            exit(1);
        }
    };

    for token in tokens {
        println!("{}", token);
    }
}

fn display_errors(errors : Vec<Error>) {
    println!("Errors : {:?}", errors);
}
