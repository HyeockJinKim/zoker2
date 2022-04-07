use zoker2::parse_zok;

fn main() {
    let res = parse_zok("contract A { function f() returns uint { return 0; } }").unwrap();
    let a = Some(0);
    a.unwrap_or(20);
}

// contract A { function f() returns uint { return 0; } }
