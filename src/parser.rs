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

fn parse_literal(tokens: &mut Peekable<IntoIter<Token>>) -> Result<Node> {
    match tokens.next_if(|tok| matches!(tok, Token::Number(_) | Token::Lambda)) {
        Some(Token::Number(n)) => Ok(Node::Number(n)),
        Some(Token::Lambda) => {
            let id = consume_identifier(tokens);
            let t = consume_type(tokens);
            Ok(Node::Lambda(id?, t?, Box::new(parse_top(tokens)?)))
        }
        Some(Token::Identifier(ident)) => Ok(Node::Var(ident)),
        Some(Token::LeftParen | Token::RightParen | Token::TypeAnnotation(_)) | None => {
            bail!("expected literal")
        }
    }
}

fn parse_top(tokens: &mut Peekable<IntoIter<Token>>) -> Result<Node> {
    Ok(parse_literal(tokens)?)
}

pub fn parse(tokens: Vec<Token>) -> Result<Node> {
    Ok(parse_top(&mut tokens.into_iter().peekable())?)
}
