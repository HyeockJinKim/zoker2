mod ast_traverser;

use crate::ast;
use crate::caller::Caller;
pub use ast_traverser::ASTTraverser;
// pub trait Traverser {
//     fn traverse(&self, ast: ast::Program) -> Vec<dyn Caller>;
// }
