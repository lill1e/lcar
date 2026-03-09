use crate::parser::Node;
use std::{collections::HashMap, fmt::Display};

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
