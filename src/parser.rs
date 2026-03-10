use std::{iter::Peekable, vec::IntoIter};

use anyhow::{Result, bail};

use crate::lexer::{Token, Type};

#[derive(Debug, Clone)]
pub enum Node {
    Number(i32),
    Var(String),
    Lambda(String, Type, Box<Node>),
    Application(Box<Node>, Box<Node>),
}

fn consume_identifier(tokens: &mut Peekable<IntoIter<Token>>) -> Result<String> {
    if let Some(Token::Identifier(id)) = tokens.next_if(|tok| matches!(tok, Token::Identifier(_))) {
        Ok(id)
    } else {
        bail!("consume type mismatch")
    }
}

fn consume_type(tokens: &mut Peekable<IntoIter<Token>>) -> Result<Type> {
    if let Some(Token::TypeAnnotation(t)) =
        tokens.next_if(|tok| matches!(tok, Token::TypeAnnotation(_)))
    {
        Ok(t)
    } else {
        bail!("consume type mismatch")
    }
}

fn consume_right_paren(tokens: &mut Peekable<IntoIter<Token>>) -> Result<()> {
    if let Some(Token::RightParen) = tokens.next_if(|tok| matches!(tok, Token::RightParen)) {
        Ok(())
    } else {
        bail!("consume type mismatch")
    }
}

fn parse_lambda(tokens: &mut Peekable<IntoIter<Token>>) -> Result<Node> {
    let id = consume_identifier(tokens)?;
    let t = consume_type(tokens)?;
    let body = parse_top(tokens)?;
    consume_right_paren(tokens)?;
    Ok(Node::Lambda(id, t, Box::new(body)))
}

fn parse_literal(tokens: &mut Peekable<IntoIter<Token>>) -> Result<Node> {
    if let Some(tok) = tokens.next_if(|tok| {
        matches!(
            tok,
            Token::Number(_) | Token::Identifier(_) | Token::LeftParen
        )
    }) {
        match tok {
            Token::Number(n) => Ok(Node::Number(n)),
            Token::Identifier(ident) => Ok(Node::Var(ident)),
            Token::LeftParen => {
                if let Some(Token::Lambda) = tokens.next_if(|tok| matches!(tok, Token::Lambda)) {
                    parse_lambda(tokens)
                } else if let Some(Token::LeftParen) = tokens.peek() {
                    Ok(Node::Application(
                        Box::new(parse_top(tokens)?),
                        Box::new(parse_top(tokens)?),
                    ))
                } else {
                    bail!("expected to apply a lambda")
                }
            }
            _ => unreachable!(),
        }
    } else {
        bail!("expected literal")
    }
}

fn parse_top(tokens: &mut Peekable<IntoIter<Token>>) -> Result<Node> {
    parse_literal(tokens)
}

pub fn parse(tokens: Vec<Token>) -> Result<Node> {
    parse_top(&mut tokens.into_iter().peekable())
}
