use std::{collections::HashMap, fmt::Display};

use crate::{lexer::Type, parser::Node};

#[derive(Debug, Clone)]
pub enum Returnable {
    Number(i32),
    Lambda(String, Type, Node),
}

impl Display for Returnable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Returnable::Number(n) => format!("Number({})", n),
                Returnable::Lambda(_, ty, _) => format!("Lambda({})", ty),
            }
        )
    }
}

impl Node {
    fn interp_env(&self, env: &mut HashMap<String, Returnable>) -> Returnable {
        match self {
            Node::Number(n) => Returnable::Number(*n),
            Node::Lambda(sym, ty, body) => {
                Returnable::Lambda(sym.clone(), ty.clone(), *body.clone())
            }
            Node::Var(sym) => env[sym].clone(),
            Node::Application(rator, rand) => match rator.interp_env(env) {
                Returnable::Lambda(sym, _, body) => {
                    let operand = rand.interp_env(env);
                    env.insert(sym, operand);
                    body.interp_env(env)
                }
                Returnable::Number(_) => unreachable!(),
            },
            Node::Let(sym, _, rhs, body) => {
                let rhs_v = rhs.interp_env(env);
                env.insert(sym.to_string(), rhs_v);
                body.interp_env(env)
            }
        }
    }
    pub fn interp(&self) -> Returnable {
        self.interp_env(&mut HashMap::new())
    }
}
