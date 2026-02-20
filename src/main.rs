use std::{collections::HashMap, io, iter::Peekable, str::Chars, vec::IntoIter};

use anyhow::{Result, bail};

#[derive(Debug, Clone, Copy)]
enum PrimOp {}

#[derive(Debug, Clone)]
enum Node {
    Number(i32),
    Var(String),
    Lambda(String, Type, Box<Node>),
    Application(Box<Node>, Box<Node>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Type {
    Number,
    Function(Box<Type>, Box<Type>),
}

#[derive(Debug, Clone)]
enum Returnable {
    Number(i32),
    Lambda(Node),
}

#[derive(Debug, Clone)]
enum TypeError {
    UnboundVariable,
    Cascade(Box<TypeError>),
    CascadeDouble(Box<TypeError>, Box<TypeError>),
    NonFunctionApplication(Type),
    FunctionInputMismatch,
}

#[derive(Debug, Clone)]
enum CheckedType {
    Type(Type),
    Error(TypeError),
}

impl Node {
    fn type_check_env(&self, env: &mut HashMap<String, Type>) -> CheckedType {
        match self {
            Node::Number(_) => CheckedType::Type(Type::Number),
            Node::Var(sym) => {
                if env.contains_key(sym) {
                    CheckedType::Type(env[sym].clone())
                } else {
                    CheckedType::Error(TypeError::UnboundVariable)
                }
            }
            // Node::Lambda(sym, body) => match body.type_check_env(env) {
            //     CheckedType::Type(t) => {
            //         env.insert(sym, v)
            //     }
            //     CheckedType::Error(err) => CheckedType::Error(err),
            // },
            Node::Application(rator, rand) => {
                match (rator.type_check_env(env), rand.type_check_env(env)) {
                    (CheckedType::Type(rator_type), CheckedType::Type(rand_type)) => {
                        match rator_type {
                            Type::Function(inp, out) => {
                                if *inp == rand_type {
                                    CheckedType::Type(*out)
                                } else {
                                    CheckedType::Error(TypeError::FunctionInputMismatch)
                                }
                            }
                            _ => CheckedType::Error(TypeError::NonFunctionApplication(rator_type)),
                        }
                    }
                    (CheckedType::Error(rator_error), CheckedType::Error(rand_error)) => {
                        CheckedType::Error(TypeError::CascadeDouble(
                            Box::new(rator_error),
                            Box::new(rand_error),
                        ))
                    }
                    (_, CheckedType::Error(rand_error)) => {
                        CheckedType::Error(TypeError::Cascade(Box::new(rand_error)))
                    }
                    (CheckedType::Error(rator_error), _) => {
                        CheckedType::Error(TypeError::Cascade(Box::new(rator_error)))
                    }
                }
            }
            Node::Lambda(sym, sym_type, body) => {
                env.insert(sym.clone(), sym_type.clone());
                match body.type_check_env(env) {
                    CheckedType::Type(t) => {
                        CheckedType::Type(Type::Function(Box::new(sym_type.clone()), Box::new(t)))
                    }
                    CheckedType::Error(err) => CheckedType::Error(err),
                }
            }
        }
    }
    fn type_check(&self) -> CheckedType {
        self.type_check_env(&mut HashMap::new())
    }
    // fn interp_env(&self, env: HashMap<String, Returnable>) -> Returnable {
    //     match self {
    //         Node::Number(n) => Returnable::Number(*n),
    //         Node::Lambda(sym, body) => Returnable::Lambda(self.clone()),
    //         Node::Var(sym) => match env.get(sym) {
    //             Some(v) => {}
    //             None => {}
    //         },
    //         Node::Lambda(sym, body) => {}
    //     }
    // }
    // fn interp(&self) -> Returnable {
    //     self.interp_env(HashMap::new())
    // }
}

#[derive(Debug)]
enum Token {
    Identifier(String),
    TypeAnnotation(Type),
    Lambda,
    Number(i32),
    LeftParen,
    RightParen,
}

pub fn lex_word(iter: &mut Peekable<Chars<'_>>) -> Token {
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

fn lex_lc(program: String) -> Result<Vec<Token>> {
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

fn parse(tokens: Vec<Token>) -> Result<Node> {
    Ok(parse_top(&mut tokens.into_iter().peekable())?)
}

fn main() -> Result<()> {
    let mut in_str = String::new();
    let mut temp_str = String::new();
    loop {
        io::stdin().read_line(&mut temp_str)?;
        if temp_str == "\n" || temp_str == "\r\n" {
            break;
        }
        in_str += &temp_str;
        temp_str.clear();
    }
    let l = lex_lc(in_str)?;
    dbg!(&l);
    let p = parse(l);
    dbg!(p?);
    Ok(())
}
