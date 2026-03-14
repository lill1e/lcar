use std::{collections::HashMap, fmt::Display};

use crate::{lexer::Type, parser::Node};

#[derive(Debug, Clone)]
pub enum TypeError {
    UnboundVariable,
    Cascade(Box<TypeError>),
    CascadeDouble(Box<TypeError>, Box<TypeError>),
    NonFunctionApplication(Type),
    FunctionInputMismatch,
    BindingTypeMismatch,
}

#[derive(Debug, Clone)]
pub enum CheckedType {
    Type(Type),
    Error(TypeError),
}

impl Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TypeError::UnboundVariable => String::from("Unbound Variable"),
                TypeError::Cascade(err) => format!(">{}", err),
                TypeError::CascadeDouble(err1, err2) => format!("{}<=>{}", err1, err2),
                TypeError::NonFunctionApplication(ty) =>
                    format!("Function Application expected a Function, but got {}", ty),
                TypeError::FunctionInputMismatch => String::from("Operand Type mismatch"),
                TypeError::BindingTypeMismatch => String::from(
                    "Right hand size of a Variable Binding does not match the specified type"
                ),
            }
        )
    }
}

impl Display for CheckedType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CheckedType::Type(ty) => format!("{}", ty),
                CheckedType::Error(err) => format!("{}", err),
            }
        )
    }
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
            Node::Let(sym, t, rhs, body) => match rhs.type_check_env(env) {
                CheckedType::Type(rhs_type) => {
                    if *t == rhs_type {
                        env.insert(sym.clone(), t.clone());
                        match body.type_check_env(env) {
                            CheckedType::Type(body_type) => CheckedType::Type(body_type),
                            CheckedType::Error(err) => {
                                CheckedType::Error(TypeError::Cascade(Box::new(err)))
                            }
                        }
                    } else {
                        CheckedType::Error(TypeError::BindingTypeMismatch)
                    }
                }
                CheckedType::Error(err) => CheckedType::Error(err),
            },
        }
    }
    pub fn type_check(&self) -> CheckedType {
        self.type_check_env(&mut HashMap::new())
    }
}
