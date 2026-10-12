#![expect(dead_code, reason = "we currently only look at the debug-print output")]

use crate::error::SyntaxError;
use crate::lexer::NonWhitespaceToken;
use crate::lexer::Token;
use crate::lexer::TokenSet;
use crate::lexer::Tokens;
use itertools::Itertools as _;
use std::iter;

#[derive(Debug)]
pub struct Module<'source> {
    pub statements: Vec<Statement<'source>>,
}

impl Module<'_> {
    pub fn parse<'source>(
        mut tokens: Tokens<'source>,
    ) -> Result<Module<'source>, SyntaxError<'source>> {
        Ok(Module {
            statements: iter::from_fn(|| Statement::parse(&mut tokens).transpose())
                .try_collect()?,
        })
    }
}

#[derive(Debug)]
pub enum Statement<'source> {
    Expression(Expression<'source>),
}

impl Statement<'_> {
    fn parse<'source>(
        tokens: &mut Tokens<'source>,
    ) -> Result<Option<Statement<'source>>, SyntaxError<'source>> {
        if tokens.peek().is_none() {
            return Ok(None);
        }

        Expression::parse_line(tokens)
            .map(Statement::Expression)
            .map(Some)
    }
}

#[derive(Debug)]
pub enum Expression<'source> {
    Identifier(&'source str),
    Text {
        // TODO: Store the value.
        escaped: &'source str,
    },
    Number {
        // TODO: Store the value.
        string: &'source str,
    },
    Call {
        callee: Box<Expression<'source>>,
        arguments: Table<'source>,
    },
}

impl Expression<'_> {
    fn parse_line<'source>(
        tokens: &mut Tokens<'source>,
    ) -> Result<Expression<'source>, SyntaxError<'source>> {
        Expression::parse_terminated(tokens, Token::LineBreak)
    }

    fn parse_parenthesised<'source>(
        tokens: &mut Tokens<'source>,
    ) -> Result<Expression<'source>, SyntaxError<'source>> {
        Expression::parse_terminated(tokens, Token::ClosingParenthesis)
    }

    fn parse_terminated<'source>(
        tokens: &mut Tokens<'source>,
        terminator: Token<'static>,
    ) -> Result<Expression<'source>, SyntaxError<'source>> {
        Expression::parse_terminated_by(tokens, vec![terminator])
    }

    fn parse_terminated_by<'source>(
        tokens: &mut Tokens<'source>,
        terminators: TokenSet,
    ) -> Result<Expression<'source>, SyntaxError<'source>> {
        let expression = Expression::parse_until(tokens, terminators.clone())?;

        let terminator = tokens.next().transpose()?;

        if !terminator.is_some_and(|terminator| terminators.contains(&terminator)) {
            return Err(SyntaxError::Expected {
                expected_one_of: terminators,
                found: terminator,
            });
        }

        Ok(expression)
    }

    fn parse_until<'source>(
        tokens: &mut Tokens<'source>,
        terminators: TokenSet,
    ) -> Result<Expression<'source>, SyntaxError<'source>> {
        let Some(token) = tokens.next().transpose()? else {
            return Err(SyntaxError::UnexpectedEndOfFile {
                expected_one_of: terminators,
            });
        };

        if terminators.contains(&token) {
            return Err(SyntaxError::ExpectedExpression { found: token });
        }

        let Some(token) = token.as_non_whitespace() else {
            return Expression::parse_terminated_by(tokens, terminators);
        };

        let callee = match token {
            NonWhitespaceToken::Identifier(identifier) => Expression::Identifier(identifier),
            NonWhitespaceToken::TextLiteral(escaped) => Expression::Text { escaped },
            NonWhitespaceToken::NumberLiteral(string) => Expression::Number { string },
            NonWhitespaceToken::Comma
            | NonWhitespaceToken::Assignment
            | NonWhitespaceToken::OpeningBrace => {
                return Err(SyntaxError::ExpectedExpression {
                    found: Token::from(token),
                });
            }
            NonWhitespaceToken::OpeningParenthesis => {
                Expression::parse_terminated(tokens, Token::ClosingParenthesis)?
            }
            NonWhitespaceToken::ClosingParenthesis | NonWhitespaceToken::ClosingBrace => {
                return Err(SyntaxError::UnmatchedClosingBracket);
            }
        };

        let operand = Expression::parse_calls(callee, tokens)?;

        // TODO: Validate the terminator?

        Ok(operand)
    }

    fn parse_calls<'source>(
        callee: Expression<'source>,
        tokens: &mut Tokens<'source>,
    ) -> Result<Expression<'source>, SyntaxError<'source>> {
        Expression::parse_call(callee, tokens)?
            .map(|call| Expression::parse_calls(call, tokens))
            .unwrap_or_else(Ok)
    }

    fn parse_call<'source>(
        callee: Expression<'source>,
        tokens: &mut Tokens<'source>,
    ) -> Result<Result<Expression<'source>, Expression<'source>>, SyntaxError<'source>> {
        Ok(match tokens.next_if_eq(&Ok(Token::OpeningParenthesis)) {
            Some(_opening_parenthesis) => Ok(Expression::Call {
                callee: Box::new(callee),
                arguments: Table::parse(tokens, Token::ClosingParenthesis)?,
            }),
            None => Err(callee),
        })
    }
}

#[derive(Debug)]
pub struct Table<'source> {
    pub entries: Vec<TableEntry<'source>>,
}

impl Table<'_> {
    fn parse<'source>(
        tokens: &mut Tokens<'source>,
        terminator: Token<'static>,
    ) -> Result<Table<'source>, SyntaxError<'source>> {
        Ok(Table {
            entries: iter::from_fn(|| TableEntry::parse_terminated(tokens, terminator).transpose())
                .try_collect()?,
        })
    }
}

#[derive(Debug)]
pub struct TableEntry<'source> {
    pub key: Option<Expression<'source>>,
    pub value: Expression<'source>,
}

impl TableEntry<'_> {
    fn parse_terminated<'source>(
        tokens: &mut Tokens<'source>,
        terminator: Token<'static>,
    ) -> Result<Option<TableEntry<'source>>, SyntaxError<'source>> {
        if let Some(_terminator) = tokens.next_if_eq(&Ok(terminator)) {
            return Ok(None);
        }

        let value = Expression::parse_until(tokens, vec![terminator, Token::Comma])?;

        tokens.next_if_eq(&Ok(Token::Comma));

        Ok(Some(TableEntry { key: None, value }))
    }
}
