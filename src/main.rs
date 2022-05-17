use zoker2::{ASTTraverser, calc_operation, parse_zok};

fn main() {
    let res = parse_zok("contract A { function check(uint age) returns uint { return age > 19; } }").unwrap();
    let contracts = ASTTraverser::traverse(res);
    let calc_op = calc_operation();
    let context = contracts.get(0).unwrap().apply(calc_op);
    println!("{:#?}", context.finalize());
}

// contract A { function check(uint age) returns uint { return age > 19; } }
