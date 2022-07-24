use crate::{ast, ZokError};
use std::collections::HashMap;
use std::os::macos::raw::stat;
use crate::ast::Statement;
use crate::error::ZokErrorType;
use crate::zkboo::ikos::IKosVariable4P;
use crate::zkboo::zkboo::{Circuit4P, Circuit4PTrait, Proof, ProvingProof, ZkBoo};

type ZokResult<T> = Result<T, ZokError>;

#[derive(Clone, Debug, PartialEq)]
enum VarType {
    Private,
    Public,
    Instance,
}

#[derive(Clone, Debug, PartialEq)]
struct Var {
    id: usize,
    typ: VarType
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProofTraverser {
    prover: ZkBoo,
    params: Vec<u32>,
    cond: Option<IKosVariable4P>,
    vars: HashMap<String, Var>,
    input: Vec<u32>,
    in_pub: Vec<u32>,
    statements: Vec<Statement>,
    input_var: Vec<IKosVariable4P>,
    instance_var: Vec<IKosVariable4P>,
    out: Vec<IKosVariable4P>,
}

impl Circuit4PTrait for ProofTraverser {
    fn circuit(&mut self, input: Vec<IKosVariable4P>, input_pub: &[u32]) -> Vec<IKosVariable4P> {
        self.input_var = input;
        self.in_pub = input_pub.to_vec();

        let statements = self.statements.clone();
        for statement in statements {
            self.traverse_statement(statement);
        }
        // TODO: Input, Input_Pub을 사용안해서 값이 제대로 안 들어가고 있었음
        self.out.clone()
    }
}

impl ProofTraverser {
    fn new(params: Vec<u32>) -> Self {
        Self {
            prover: ZkBoo::new(2, 3, 2, 32),
            params,
            vars: Default::default(),
            cond: None,
            input: vec![],
            in_pub: vec![],
            statements: vec![],
            input_var: vec![],
            instance_var: vec![],
            out: vec![],
        }
    }

    pub fn traverse(ast: ast::Program, contract: String, function: String, params: Vec<u32>) -> ZokResult<()> {
        let zk_boo = ZkBoo::new(2, 3, 2, 32);
        match ast {
            // 단일 컨트랙트만 지원
            ast::Program::GlobalStatements(v) => {
                for gs in v {
                    let mut zelf = Self::new(params.clone());
                    zelf.traverse_global_statement(gs, contract, function)?;
                    println!("{:?}, {:?}, {:?}", zelf.input, zelf.in_pub, zelf.out);
                    let mut res = zk_boo.prove(ProvingProof::new(zelf.input.clone(), zelf.in_pub.clone(), 1, Box::new(zelf))).unwrap();
                    let challenge = ZkBoo::query_random_oracle(
                        res.input_len,
                        1,
                        &res.out_data,
                        &res.three_views,
                    );
                    let response = zk_boo.build_response(&res.views, &challenge);
                    let two_views = zk_boo.rebuild_proof(&mut res, &challenge);
                    print!("[");
                    print!("{:?},", res.input_len);
                    print!("{:?},", res.input_pub);
                    print!("{:?},\"0x", res.output);
                    for &ch in &challenge.clone() {
                        print!("{:02x?}", ch);
                    }
                    print!("\",\"0x");
                    for &ch in &two_views.clone() {
                        print!("{:02x?}", ch);
                    }
                    print!("\",[");
                    for i in 0..3 {
                        print!("[\"0x");
                        for &ch in &response[i].rand_tape_seed.clone() {
                            print!("{:02x?}", ch);
                        }
                        print!("\",");
                        print!("{:?},{:?}],", response[i].in_data, response[i].out_data);
                    }
                    print!("[\"0x");
                    for &ch in &response[3].rand_tape_seed.clone() {
                        print!("{:02x?}", ch);
                    }
                    print!("\",");
                    print!("{:?},{:?}]]", response[3].in_data, response[3].out_data);
                    println!("]");
                    return Ok(());
                }
            }
        };
        Err(ZokError {
            error: ZokErrorType::EOF,
            location: Default::default(),
        })
    }

    fn traverse_global_statement(&mut self, stmt: ast::GlobalStatement, contract: String, function: String) -> ZokResult<()> {
        match stmt.node {
            ast::GlobalStatementType::ContractStatement { contract_name, members } => {
                if contract_name.eq(contract.as_str()) {
                    for member in members {
                        match self.traverse_member(member, function.clone()) {
                            Ok(_) => {
                                return Ok(());
                            }
                            Err(_) => { continue; }
                        }
                    }
                };
                Err(ZokError {
                    error: ZokErrorType::EOF,
                    location: Default::default(),
                })
            }
        }
    }

    fn traverse_member(&mut self, stmt: ast::ContractStatement, function: String) -> ZokResult<()> {
        match stmt.node {
            ast::ContractStatementType::FunctionStatement { function_name, parameters, return_type: _return_type, statements } => {
                if !function_name.eq(function.as_str()) {
                    return Err(ZokError {
                        error: ZokErrorType::EOF,
                        location: Default::default(),
                    });
                }
                let parameters = parameters.clone();
                let statements = statements.clone();
                for param in parameters {
                    self.traverse_parameter(param.clone());
                }
                // for statement in statements.clone() {
                //     self.traverse_statement(statement);
                // }
                self.statements = statements;
                Ok(())
            }
        }
    }

    // TODO
    fn traverse_parameter(&mut self, param: ast::Parameter) {
        match &param.node {
            ast::ParameterType::Private { variable_type: _variable_type, variable }
            => {
                let val = self.params.pop().unwrap();
                self.vars.insert(variable.clone(), Var{
                    id: self.input.len(),
                    typ: VarType::Private,
                });
                self.input.push(val);
            }
            ast::ParameterType::Public { variable_type: _variable_type, variable }
            => {
                let val = self.params.pop().unwrap();
                self.vars.insert(variable.clone(), Var{
                    id: self.in_pub.len(),
                    typ: VarType::Public,
                });
                self.in_pub.push(val);
            }
        };
    }

    // TODO
    fn traverse_statement(&mut self, stmt: ast::Statement) {
        match &stmt.node {
            ast::StatementType::IfStatement { condition, if_statements, else_statements } => {
                let new_cond = self.traverse_expression(condition);
                if let Some(_) = self.cond.take() {
                    // TODO: not working
                } else {
                    self.cond = Some(new_cond.clone());
                }
                for statement in if_statements {
                    self.traverse_statement(statement.clone());
                }

                if let Some(_) = self.cond.take() {
                    // TODO: not working
                } else {
                    self.cond = Some(new_cond.negate());
                }
                for statement in else_statements {
                    self.traverse_statement(statement.clone());
                }
                self.cond = None;
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
                self.out = vec![self.traverse_expression(return_value)]
            }
            ast::StatementType::InitializerStatement { variable_type: _variable_type, variable, default } => {
                //TODO: default값은 나중에
                // self.vars.insert(variable.clone(), 0);
            }
            ast::StatementType::AssignStatement { left, operator: _, right } => {
                // TODO:
                let r = self.traverse_expression(right);
                self.vars.insert(left.to_string(), Var{
                    id: self.instance_var.len(),
                    typ: VarType::Instance,
                });
                self.instance_var.push(r);
            }
            ast::StatementType::Expression { expression } => {}
        }
    }

    // TODO
    fn traverse_expression(&mut self, expr: &ast::Expression) -> IKosVariable4P {
        let res = match &expr.node {
            ast::ExpressionType::BinaryExpression {
                left, operator, right
            } => {
                let v1 = self.traverse_expression(left);
                let v2 = self.traverse_expression(right);
                println!("{:?} bin {:?} {:?}", v1, v2, operator);
                match operator {
                    ast::BinaryOperator::Add => v1.add_op(&v2),
                    // ast::BinaryOperator::Sub => "",
                    // ast::BinaryOperator::Mul => ctx.bin_op(lazy_mul),
                    // ast::BinaryOperator::Div => ctx.bin_op(lazy_div),
                    // TODO: 나중에 구현
                    // BinaryOperator::Mod => ctx.with_op(bin_op(add)),
                    // BinaryOperator::And => ctx.with_op(bin_op(add)),
                    // ast::BinaryOperator::Or => "bit_or",
                    // ast::BinaryOperator::Lt => ctx.with_op(bin_op(add)),
                    // ast::BinaryOperator::Le => ctx.with_op(bin_op(add)),
                    ast::BinaryOperator::Gt => v1.gt(&v2),
                    // BinaryOperator::Ge => ctx.with_op(bin_op(add)),
                    // BinaryOperator::Eq => ctx.with_op(bin_op(add)),
                    // BinaryOperator::NotEq => ctx.with_op(bin_op(add)),
                    ast::BinaryOperator::BitAnd => v1.bit_and(&v2),
                    ast::BinaryOperator::BitOr => v1.bit_or(&v2),
                    ast::BinaryOperator::BitXor => v1.xor(&v2),
                    // ast::BinaryOperator::LShift => v1.lshift(&v2),
                    // ast::BinaryOperator::RShift => v1.rshift(&v2),
                    _ => IKosVariable4P::new_value(0),
                }
            }
            ast::ExpressionType::FunctionCallExpression { function_name, arguments: _arguments } => {
                // TODO: 이후 구현
                IKosVariable4P::new_value(0)
            }
            ast::ExpressionType::Number { value } => {
                IKosVariable4P::new_value(value.clone() as u32)
            }
            ast::ExpressionType::Identifier { value } => {
                let id = self.vars.get(value.as_str()).unwrap().clone();
                match id.typ {
                    VarType::Private => self.input_var.get(id.id).unwrap().clone(),
                    VarType::Public => IKosVariable4P::new_value(self.in_pub.get(id.id).unwrap().clone() as u32),
                    VarType::Instance => self.instance_var.get(id.id).unwrap().clone(),
                }
            }
        };
        if let Some(cond) = self.cond.take() {
            res.bit_and(&cond)
        } else {
            res
        }
    }
}
