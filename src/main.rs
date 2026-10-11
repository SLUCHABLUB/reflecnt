mod arguments;
mod ast;
mod error;
mod lexer;

use crate::arguments::Arguments;
use crate::ast::Module;
use crate::lexer::Token;
use clap::Parser as _;
use itertools::Itertools as _;
use logos::Logos as _;
use std::fs::read_to_string;
use std::io;

fn main() -> io::Result<()> {
    let arguments = Arguments::parse();

    let source = read_to_string(&arguments.file)?;

    match Token::lexer(&source).collect::<Result<Vec<_>, _>>() {
        Ok(tokens) => println!("{}", tokens.iter().join("\n")),
        Err(error) => println!("{error:#?}"),
    }

    println!();

    let lexer = Token::lexer(&source);

    let module = Module::parse(lexer.peekable());

    println!("{module:#?}");

    Ok(())
}
