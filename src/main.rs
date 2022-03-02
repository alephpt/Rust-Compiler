extern crate idiom_core;
extern crate clap;

use idiom_core::lexer::*;

use clap::{App, SubCommand};

fn main() -> std::io::Result<()> {
    println!("Hello, Lexer!\n");

    let application = App::new("Idiom")
        .version("0.1a")
        .author("Richard Christopher <alephpt1@gmail.com>\n")
        .about("Idiom - An Expression that cannot be understood from the meanings of its seperate words, but must be learned as a whole of the expression. (Not to be taken Literally.) .. except this is a Compiler")
        .arg_from_usage("-v --verbose   'Run with more information'")
        .subcommand(SubCommand::with_name("debug").args_from_usage(
            "
            --show=[TOKENS]...   'Show specific steps in the compiling process (tokens, ast, ..)'
            <INPUT>     'File to load'

            "
        ))
        .get_matches();

    match application.subcommand() {
        ("debug", Some(matching)) => { 
            let filename = matching.value_of("INPUT").unwrap();
            let text = std::fs::read_to_string(filename)?;
            let lexer = Lexer::new(&text);
            let shows = matching.values_of("show").unwrap_or_default().collect::<Vec<&str>>();
            if shows.contains(&"tokens") {
                let mut lexer = lexer.clone();
 
                loop {
                    match lexer.next_token() {
                        Ok(TokenType::EOF) => { println!("Breaking.. EOF.. "); break; },
                        Ok(tok) => println!("{0:?}", tok),
                        Err(err) => println!("{0:?}", err),
                    }
                }
            }
        }
        _ => {}
    }

    Ok(())

}
