use crate::error::SyntaxError;
use derive_more::Display;
use logos::Lexer;
use logos::Logos;
use std::iter::Peekable;
use subenum::subenum;

#[derive(Copy, Clone, PartialEq, Debug, Display, Logos)]
#[logos(skip r"\s+")]
#[logos(error(SyntaxError<'static>, callback = SyntaxError::invalid_token))]
#[subenum(NonWhitespaceToken)]
pub enum Token<'source> {
    #[display("`{_0}`")]
    #[regex(r"[\p{XID_Start}_]\p{XID_Continue}*")]
    #[subenum(NonWhitespaceToken)]
    Identifier(&'source str),

    #[display("`{_0}`")]
    #[regex(r#"("(.|\")+")"#)]
    #[subenum(NonWhitespaceToken)]
    // TODO: Name the field.
    TextLiteral(&'source str),
    #[display("`{_0}`")]
    #[regex(r"[0-9]([0-9_]*[0-9])?i?")]
    #[subenum(NonWhitespaceToken)]
    // TODO: Name the field.
    NumberLiteral(&'source str),

    #[display("a line break")]
    #[regex(r"\n", priority = 42)]
    LineBreak,

    #[display("`,`")]
    #[token(",")]
    #[subenum(NonWhitespaceToken)]
    Comma,
    #[display("`=`")]
    #[token("=")]
    #[subenum(NonWhitespaceToken)]
    Assignment,

    #[display("`(`")]
    #[token("(")]
    #[subenum(NonWhitespaceToken)]
    OpeningParenthesis,
    #[display("`)`")]
    #[token(")")]
    #[subenum(NonWhitespaceToken)]
    ClosingParenthesis,

    #[display("`{{`")]
    #[token("{")]
    #[subenum(NonWhitespaceToken)]
    OpeningBrace,
    #[display("`}}`")]
    #[token("}")]
    #[subenum(NonWhitespaceToken)]
    ClosingBrace,
}

impl<'source> Token<'source> {
    pub(crate) fn is_whitespace(&self) -> bool {
        self.as_non_whitespace().is_none()
    }

    pub(crate) fn as_non_whitespace(&self) -> Option<NonWhitespaceToken<'source>> {
        NonWhitespaceToken::try_from(*self).ok()
    }
}

// TODO: Remove this type and make the parser function generic over any iterable.
pub type TokenSet = Vec<Token<'static>>;

pub(crate) type Tokens<'source> = Peekable<Lexer<'source, Token<'source>>>;

#[expect(unused, reason = "TODO")]
pub(crate) fn skip_whitespace(tokens: &mut Tokens<'_>) {
    while tokens
        .next_if(|token| token.as_ref().is_ok_and(Token::is_whitespace))
        .is_some()
    {}
}
