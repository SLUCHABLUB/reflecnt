use itertools::Itertools as _;
use logos::Logos as _;
use pretty_assertions::assert_eq;
use reflecnt::ast::Module;
use reflecnt::error::SyntaxError;
use reflecnt::lexer::Token;

fn syntax_to_anyhow(syntax: SyntaxError<'_>) -> anyhow::Error {
    anyhow::Error::msg(syntax.to_string())
}

fn tokens(source: &str) -> anyhow::Result<String> {
    Ok(Token::lexer(source)
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .join("\n")
        + "\n")
}

fn ast(source: &str) -> anyhow::Result<String> {
    let module = Module::parse(Token::lexer(source).peekable()).map_err(syntax_to_anyhow)?;

    Ok(format!("{module:#?}\n"))
}

macro_rules! test_file {
    ($name:ident) => {
        #[test]
        fn $name() -> anyhow::Result<()> {
            let source = include_str!(concat!("../resources/", stringify!($name), ".source"));

            assert_eq!(
                tokens(source)?,
                include_str!(concat!("../resources/", stringify!($name), ".tokens")),
                "unexpected token output",
            );
            assert_eq!(
                ast(source)?,
                include_str!(concat!("../resources/", stringify!($name), ".ast")),
                "unexpected AST output",
            );

            Ok(())
        }
    };
}

test_file!(hello_world);
test_file!(fizz_buzz);
