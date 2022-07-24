#![allow(unused_parens)]

use lalrpop_util::lalrpop_mod;
use crate::error::ZokError;
use crate::traverser::{ProofTraverser, VCTraverser};
use crate::parser::ast;
use crate::parser::lexer::make_tokenizer;
use crate::zkboo::zkboo::ZkBoo;

mod parser;
mod zkboo;
mod location;
mod error;
mod traverser;

lalrpop_mod!(
    #[allow(clippy::all)]
    pub zok
);

macro_rules! do_lalr_parsing {
    ($input: expr, $parser: ident) => {{
        let lxr = make_tokenizer($input);
        match zok::$parser::new().parse(lxr) {
            Err(err) => Err(ZokError::from(err)),
            Ok(top) => Ok(top),
        }
    }};
}

pub fn parse_program(source: &str) -> Result<ast::Program, ZokError> {
    do_lalr_parsing!(source, ProgramParser)
}

pub fn parse_zok(code: &str) -> Result<ast::Program, ZokError> {
    let ast = parse_program(code)?;
    Ok(ast)
}

fn main() {
    // let res = parse_zok("contract A { function sum(private uint a, private uint b, private uint c) returns uint { return a + b + c; } }").unwrap();
    let res = parse_zok("contract A { function check(private uint age) returns uint { return age > 19; } }").unwrap();
    let contracts = VCTraverser::traverse(res.clone()).unwrap();
    ProofTraverser::traverse(res, "A".to_string(), "check".to_string(), vec![25]).unwrap();

    println!("{}", contracts);
}
