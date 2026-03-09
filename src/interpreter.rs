use crate::parser::Node;

#[derive(Debug, Clone)]
pub enum Returnable {
    Number(i32),
    Lambda(Node),
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
