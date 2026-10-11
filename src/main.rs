mod arguments;
mod ast;
mod error;
mod lexer;

use crate::arguments::Arguments;
use crate::ast::Module;
use crate::error::SyntaxError;
use crate::lexer::Token;
use clap::Parser as _;
use itertools::Itertools as _;
use logos::Logos as _;
use std::fs::read_to_string;

fn main() -> anyhow::Result<()> {
    let arguments = Arguments::parse();

    let source = read_to_string(&arguments.file)?;

    println!(
        "--- TOKENS ---\n{}\n",
        Token::lexer(&source)
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .join("\n")
    );

    let lexer = Token::lexer(&source);

    let module = Module::parse(lexer.peekable()).map_err(SyntaxError::anyhow)?;

    println!("--- AST ---\n{module:#?}");

    Ok(())
}
