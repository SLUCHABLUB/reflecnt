use crate::error::SyntaxError;
use derive_more::Display;
use logos::Lexer;
use logos::Logos;
use std::iter::Peekable;

#[derive(Copy, Clone, PartialEq, Debug, Display, Logos)]
#[logos(skip r"\s+")]
#[logos(error(SyntaxError<'static>, callback = SyntaxError::invalid_token))]
pub enum Token<'source> {
    #[display("`{_0}`")]
    #[regex(r"[\p{XID_Start}_]\p{XID_Continue}*")]
    Identifier(&'source str),

    #[display("`{_0}`")]
    #[regex(r#"("(.|\")+")"#)]
    // TODO: Name the field.
    TextLiteral(&'source str),

    #[display("a line break")]
    #[regex(r"\n", priority = 42)]
    LineBreak,

    #[display("`,`")]
    #[token(",")]
    Comma,

    #[display("`(`")]
    #[token("(")]
    OpeningParenthesis,
    #[display("`)`")]
    #[token(")")]
    ClosingParenthesis,
}

impl<'source> Token<'source> {
    pub(crate) fn is_whitespace(&self) -> bool {
        self.as_non_whitespace().is_none()
    }

    pub(crate) fn as_non_whitespace(&self) -> Option<NonWhitespaceToken<'source>> {
        Some(match self {
            Token::Identifier(identifier) => NonWhitespaceToken::Identifier(identifier),
            Token::TextLiteral(escaped) => NonWhitespaceToken::TextLiteral(escaped),
            Token::LineBreak => return None,
            Token::Comma => NonWhitespaceToken::Comma,
            Token::OpeningParenthesis => NonWhitespaceToken::OpeningParenthesis,
            Token::ClosingParenthesis => NonWhitespaceToken::ClosingParenthesis,
        })
    }
}

// TODO: Remove this type and make the parser function generic over any iterable.
pub type TokenSet = Vec<Token<'static>>;

pub(crate) enum NonWhitespaceToken<'source> {
    Identifier(&'source str),
    TextLiteral(&'source str),

    Comma,

    OpeningParenthesis,
    ClosingParenthesis,
}

pub(crate) type Tokens<'source> = Peekable<Lexer<'source, Token<'source>>>;

#[expect(unused, reason = "TODO")]
pub(crate) fn skip_whitespace(tokens: &mut Tokens<'_>) {
    while tokens
        .next_if(|token| token.as_ref().is_ok_and(Token::is_whitespace))
        .is_some()
    {}
}
