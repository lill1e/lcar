use std::{fmt::Display, iter::Peekable, str::Chars};

use anyhow::{Result, bail};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Number,
    Function(Box<Type>, Box<Type>),
}

#[derive(Debug)]
pub enum Token {
    Identifier(String),
    TypeAnnotation(Type),
    Lambda,
    Number(i32),
    LeftParen,
    RightParen,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Type::Number => String::from("Number"),
                Type::Function(in_ty, out_ty) => format!("{} -> {}", in_ty, out_ty),
            }
        )
    }
}

fn lex_word(iter: &mut Peekable<Chars<'_>>) -> Token {
    let mut acc = String::new();
    while let Some(c) = iter.next_if(|&c| c.is_alphanumeric()) {
        acc.push(c);
    }
    match acc.as_str() {
        "lambda" => Token::Lambda,
        "number" => Token::TypeAnnotation(Type::Number),
        _ => Token::Identifier(acc),
    }
}

fn lex_number(iter: &mut Peekable<Chars<'_>>) -> Token {
    let mut n: u32 = 0;
    while iter.peek().is_some() {
        let curr = iter.peek();
        match curr {
            Some(c) => match c {
                c if c.is_numeric() => match c.to_digit(10) {
                    Some(digit) => {
                        n *= 10;
                        n += digit;
                        iter.next();
                    }
                    None => break,
                },
                _ => break,
            },
            None => break,
        }
    }
    return Token::Number(n as i32);
}

pub fn lex_lc(program: String) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut iter = program.chars().into_iter().peekable();
    while iter.peek().is_some() {
        match iter.peek() {
            Some(c) => match c {
                _ if c.is_numeric() => tokens.push(lex_number(&mut iter)),
                'A'..='Z' | 'a'..='z' => tokens.push(lex_word(&mut iter)),
                '(' => {
                    tokens.push(Token::LeftParen);
                    iter.next();
                }
                ')' => {
                    tokens.push(Token::RightParen);
                    iter.next();
                }
                ':' => {
                    iter.next();
                    while iter.next_if(|c| c.is_whitespace()).is_some() {}
                    match lex_word(&mut iter) {
                        Token::TypeAnnotation(t) => tokens.push(Token::TypeAnnotation(t)),
                        _ => bail!("expected word"),
                    }
                }
                _ => {
                    iter.next();
                }
            },
            None => {}
        }
    }
    Ok(tokens)
}
