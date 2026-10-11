use crate::lexer::Token;
use crate::lexer::TokenSet;
use literator::Literator;
use logos::Lexer;
use non_zero::non_zero;
use std::num::NonZeroUsize;
use thiserror::Error;

#[derive(Clone, PartialEq, Debug, Default, Error)]
pub(crate) enum SyntaxError<'source> {
    #[error("invalid token `{token}` at {line}:{column}")]
    InvalidToken {
        line: NonZeroUsize,
        column: NonZeroUsize,
        token: String,
    },
    #[error("could not find a matching open bracket for the closing bracket")]
    UnmatchedClosingBracket,
    #[error("expected {}", one_of(expected_one_of))]
    UnexpectedEndOfFile { expected_one_of: TokenSet },
    #[error("expected an expression, found {found}")]
    ExpectedExpression { found: Token<'source> },
    #[error(
        "expected {}, found {}",
        one_of(expected_one_of),
        or_end_of_file(found)
    )]
    Expected {
        expected_one_of: TokenSet,
        found: Option<Token<'source>>,
    },
    #[default]
    #[error("unknown syntax error")]
    Unknown,
}

impl SyntaxError<'_> {
    pub(crate) fn invalid_token<'source>(
        lexer: &mut Lexer<'source, Token<'source>>,
    ) -> SyntaxError<'static> {
        let [line, column] = line_column_pair(lexer);

        SyntaxError::InvalidToken {
            line,
            column,
            token: lexer.slice().to_owned(),
        }
    }
}

fn line_column_pair<'source>(lexer: &Lexer<'source, Token<'source>>) -> [NonZeroUsize; 2] {
    let span = lexer.span();

    let line = non_zero!(1_usize).saturating_add(
        lexer.source()[..span.start]
            .chars()
            .filter(|char| *char == '\n')
            .count(),
    );
    let column = lexer.source()[..span.start]
        .rfind('\n')
        .map(|index| span.start - index)
        .and_then(NonZeroUsize::new)
        .unwrap_or(non_zero!(1));

    [line, column]
}

fn one_of(tokens: &TokenSet) -> String {
    debug_assert_ne!(tokens, &[]);

    match tokens.as_slice() {
        [] => "nothing".to_owned(),
        [token] => token.to_string(),
        _ => format!("one of {}", tokens.iter().oxford_join_and()),
    }
}

fn or_end_of_file(token: &Option<Token>) -> String {
    match token {
        Some(token) => token.to_string(),
        None => "end of file".to_owned(),
    }
}
