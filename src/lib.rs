use lalrpop_util::lalrpop_mod;
use crate::error::ZokError;
use crate::parser::ast;
use crate::parser::lexer::make_tokenizer;

pub mod error;
mod parser;
// mod rewriter;
// mod zkboo;
mod location;

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
