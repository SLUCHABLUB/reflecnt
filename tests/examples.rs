use itertools::Itertools as _;
use logos::Logos as _;
use pretty_assertions::assert_eq;
use reflecnt::ast::Module;
use reflecnt::lexer::Token;

fn tokens(source: &str) -> String {
    Token::lexer(source)
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
        .iter()
        .join("\n")
        + "\n"
}

fn ast(source: &str) -> String {
    let module = Module::parse(Token::lexer(source).peekable()).unwrap();

    format!("{module:#?}\n")
}

#[test]
fn hello_world() {
    let source = include_str!("../resources/hello_world.txt");

    assert_eq!(
        tokens(source),
        include_str!("../resources/hello_world.tokens"),
        "unexpected token output",
    );
    assert_eq!(
        ast(source),
        include_str!("../resources/hello_world.ast"),
        "unexpected AST output",
    );
}
