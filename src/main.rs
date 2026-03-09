mod lexer;
mod parser;

use std::{collections::HashMap, io, iter::Peekable, str::Chars, vec::IntoIter};

use anyhow::Result;

use crate::{lexer::lex_lc, parser::parse, type_check::CheckedType};

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
    let p = parse(l)?;
    dbg!(&p);
    let type_check_result = p.clone().type_check();
    match type_check_result {
        CheckedType::Type(program_type) => {
            println!("Program Type via Type Checker: {}", program_type);
        }
        CheckedType::Error(err) => println!("The program failed to type check: {}", err),
    }
    println!("Program Result via Interpretation: {}", p.interp());
    Ok(())
}
