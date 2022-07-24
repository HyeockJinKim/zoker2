use std::collections::HashMap;
use crate::ast;
use crate::ast::{ContractStatement, Parameter};
use crate::error::{ZokError, ZokErrorType};
use std::fs;

type ZokResult<T> = Result<T, ZokError>;

pub struct VCTraverser {
    input: HashMap<String, usize>,
    in_pub: HashMap<String, usize>,
    input_index: usize,
    in_pub_index: usize,
}

impl VCTraverser {
    fn new() -> Self {
        Self {
            input: Default::default(),
            in_pub: Default::default(),
            input_index: 0,
            in_pub_index: 0,
        }
    }

    pub fn traverse(ast: ast::Program) -> ZokResult<String> {
        match ast {
            ast::Program::GlobalStatements(v) => {
                for gs in v {
                    let mut zelf = Self::new();
                    let stmt = zelf.traverse_global_statement(&gs);
                    return match stmt {
                        Ok(res) => {
                            Ok(res)
                        }
                        Err(err) => {
                            Err(err)
                        }
                    };
                }
                Err(ZokError {
                    error: ZokErrorType::EOF,
                    location: Default::default(),
                })
            }
        }
    }

    fn traverse_global_statement(&mut self, stmt: &ast::GlobalStatement) -> ZokResult<String> {
        match &stmt.node {
            ast::GlobalStatementType::ContractStatement { contract_name, members } => {
                let contents = match fs::read_to_string("src/traverser/zkboo.sol") {
                    Ok(content) => Ok(content),
                    Err(err) => Err(ZokError {
                        error: ZokErrorType::UnsupportedError,
                        location: Default::default(),
                    }),
                };
                let mut contents = contents?;
                let mut s = String::new();
                let mut funcs = vec![];
                for member in members {
                    let (mem, func) = self.traverse_member(member)?;
                    s.push_str(mem.as_str());
                    funcs.push(func);
                }
                for func in funcs {
                    contents = contents.replace("{circuit}", func.as_str());
                }
                Ok(contents.replace("{contract_name}", contract_name)
                    .replace("{functions}", s.as_str()))
            }
        }
    }

    fn traverse_member(&mut self, stmt: &ContractStatement) -> ZokResult<(String, String)> {
        match &stmt.node {
            ast::ContractStatementType::FunctionStatement { function_name, parameters, return_type: _return_type, statements } => {
                let mut s = String::new();
                s.push_str("    function _");
                s.push_str(function_name.as_str());
                s.push_str("(IKosVariable4V[] memory input, uint32[] memory in_pub) internal pure returns (IKosVariable4V[] memory) {\n");
                for parameter in parameters {
                    self.traverse_parameter(parameter);
                }
                for statement in statements {
                    let stmt = self.traverse_statement(statement);
                    s.push_str(stmt.as_str());
                }
                s.push_str("    }\n\n");

                s.push_str("    function ");
                s.push_str(function_name.as_str());
                s.push_str("(VerifyingProof memory proof) public pure returns (uint256) {\n");
                s.push_str("        if (ZKBoo_verify(proof)) { return proof.output[0]; } else { return 0; }\n    ");
                s.push_str("}\n");
                Ok((s, function_name.clone()))
            }
        }
    }

    // TODO
    fn traverse_parameter(&mut self, param: &Parameter) {
        match &param.node {
            ast::ParameterType::Private { variable_type: _variable_type, variable }
            => {
                self.input.insert(variable.to_string(), self.input_index);
                self.input_index += 1;
            } // TODO: 이후 타입도 같이 넘겨야 함
            ast::ParameterType::Public { variable_type: _variable_type, variable }
            => {
                self.in_pub.insert(variable.to_string(), self.in_pub_index);
                self.in_pub_index += 1;
            }
        }
    }

    // TODO
    fn traverse_statement(&mut self, stmt: &ast::Statement) -> String {
        let mut s = String::new();
        match &stmt.node {
            ast::StatementType::IfStatement { condition, if_statements, else_statements } => {
                // Self::traverse_expression(&mut ctx.sub_context(), condition);
                // TODO: condition을 적용할 연산을 정의해야함 (assign에서 11111 을 & 하고 더할 값)
            }
            ast::StatementType::ForEachStatement { iterator, iterable, statements } => {
                // TODO: foreach는 vector를 type 추가한 후에 구현 가능
                // let var = Self::traverse_expression(iterable);
                // TODO: iterator를 받아서 처리
                // let ops = statements.iter().fold(ctx.sub_context(), Self::traverse_statement).ops();
                // ctx.with_op(repeat()) // TODO: iterable을 받아서 처리
                //     .with_op(push(Functor::new(ops)))
            }
            ast::StatementType::ReturnStatement { return_value } => {
                s.push_str("        IKosVariable4V[] memory out = new IKosVariable4V[](1);\n");
                s.push_str("        out[0] = ");
                s.push_str(self.traverse_expression(return_value).as_str());
                s.push_str(";\n");
                s.push_str("        return out;\n");
            }
            ast::StatementType::InitializerStatement { variable_type: _variable_type, variable, default } => {
                s.push_str("        ");
                s.push_str("IKosVariable4V ");
                s.push_str(variable.as_str());
                match default {
                    Some(expr) => {
                        s.push_str(" = ");
                        s.push_str(self.traverse_expression(expr).as_str());
                    }
                    None => {}
                };
                s.push_str(";\n");
            }
            ast::StatementType::AssignStatement { left, operator: _, right } => {
                s.push_str("        ");
                s.push_str(left.as_str());
                s.push_str(" = ");
                s.push_str(self.traverse_expression(right).as_str());
                s.push_str(";\n");
            }
            ast::StatementType::Expression { expression } => {}
        };
        s
    }

    // TODO
    fn traverse_expression(&mut self, expr: &ast::Expression) -> String {
        let mut s = String::new();
        match &expr.node {
            ast::ExpressionType::BinaryExpression {
                left, operator, right
            } => {
                let v1 = self.traverse_expression(left);
                let v2 = self.traverse_expression(right);
                let op = Self::traverse_operator(operator);
                s.push_str(op);
                s.push_str("(");
                s.push_str(v1.as_str());
                s.push_str(",");
                s.push_str(v2.as_str());
                s.push_str(")");
            }
            ast::ExpressionType::FunctionCallExpression { function_name, arguments: _arguments } => {
                // TODO:
                s.push_str("_");
                s.push_str(function_name.as_str());
                s.push_str("(");
                s.push_str("proof");
                s.push_str(")");
            }
            ast::ExpressionType::Number { value } => {
                s.push_str("IKosVariable_new_value(");
                s.push_str(format!("{}", value).as_str());
                s.push_str(")");
            }
            ast::ExpressionType::Identifier { value } => {
                if self.input.contains_key(value) {
                    s.push_str("input[");
                    s.push_str(format!("{}", self.input.get(value).unwrap()).as_str());
                    s.push_str("]");
                } else if self.in_pub.contains_key(value) {
                    s.push_str("in_pub[");
                    s.push_str(format!("{}", self.in_pub.get(value).unwrap()).as_str());
                    s.push_str("]");
                }
            }
        }
        s
    }

    // TODO
    fn traverse_operator(op: &ast::BinaryOperator) -> &str {
        match op {
            ast::BinaryOperator::Add => "add_op",
            // ast::BinaryOperator::Sub => "",
            // ast::BinaryOperator::Mul => ctx.bin_op(lazy_mul),
            // ast::BinaryOperator::Div => ctx.bin_op(lazy_div),
            // TODO: 나중에 구현
            // BinaryOperator::Mod => ctx.with_op(bin_op(add)),
            // BinaryOperator::And => ctx.with_op(bin_op(add)),
            // ast::BinaryOperator::Or => "bit_or",
            // ast::BinaryOperator::Lt => ctx.with_op(bin_op(add)),
            // ast::BinaryOperator::Le => ctx.with_op(bin_op(add)),
            ast::BinaryOperator::Gt => "gt_op",
            // BinaryOperator::Ge => ctx.with_op(bin_op(add)),
            // BinaryOperator::Eq => ctx.with_op(bin_op(add)),
            // BinaryOperator::NotEq => ctx.with_op(bin_op(add)),
            ast::BinaryOperator::BitAnd => "bit_and",
            ast::BinaryOperator::BitOr => "bit_or",
            ast::BinaryOperator::BitXor => "bit_xor",
            ast::BinaryOperator::LShift => "rshift",
            ast::BinaryOperator::RShift => "lshift",
            _ => "",
        }
    }
}
