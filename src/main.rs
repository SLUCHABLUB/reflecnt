mod arguments;

use crate::arguments::Arguments;
use clap::Parser as _;
use itertools::Itertools as _;
use logos::Logos as _;
use reflecnt::ast::Module;
use reflecnt::error::SyntaxError;
use reflecnt::lexer::Token;
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

    let module = Module::parse(lexer.peekable()).map_err(syntax_to_anyhow)?;

    println!("--- AST ---\n{module:#?}");

    Ok(())
}

fn syntax_to_anyhow(syntax: SyntaxError<'_>) -> anyhow::Error {
    anyhow::Error::msg(syntax.to_string())
}
