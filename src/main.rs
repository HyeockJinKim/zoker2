use zoker2::{ASTTraverser, parse_zok};

fn main() {
    let res = parse_zok("contract A { function check(uint age) returns uint { return age > 19; } }").unwrap();
    let contracts = ASTTraverser::traverse(res);
}

// contract A { function check(uint age) returns uint { return age > 19; } }
