extern crate idiom_core;

use idiom_core::lexer::*;

fn main() {
    println!("Hello, Lexer!");

    let mut _lexer_delim = Lexer::new(",,{[([( ^ > $ < , < ) ) . . [ > ] > > ");

    let mut _lexer_num = Lexer::new("
        .e
        16b10Fe23
        16b16.4
        16b10FA
        16bG
        203
        0.5
        2.2
        .798
        1e49
        1e+49
        48b73
        .1e
        .2.
        e.5
        2b1010
        2b3
        16bAF0
        16bG
        64bf02/(523&2393f0jaf
        8b123751
        8b99
    ");

    let mut _lexer_func = Lexer::new(" 
        go main, arg1 arg2 -
        |   when, arg1 > arg2 -
            |    value <- (arg1 + arg2).
            _
            ^ <- value.
        _.
    ");

    loop {
        match _lexer_num.next_token() {
            Ok(TokenType::EOF) => break,
            Ok(tok) => println!("{0:?}", tok),
            Err(err) => println!("{0:?}", err),
        }
    }
}
