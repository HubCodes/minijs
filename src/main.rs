use parser::parse;

fn main() {
    let code =
        "x.y.z;";
    let ast = parse(code);
    match ast {
        Ok(ast) => println!("{:#?}", ast),
        Err(_) => panic!(":("),
    }
}
