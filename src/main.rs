mod interpreter;
mod lexer;
mod parser;
mod type_check;

use std::io;

use anyhow::Result;

use crate::{lexer::lex_lc, parser::parse};

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
