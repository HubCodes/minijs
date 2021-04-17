use parser::parse;

fn main() {
    let code =
        "(function(a){
            let x = { b: 1 };
        })();
        ";
    let ast = parse(code);
    match ast {
        Ok(ast) => println!("{:#?}", ast),
        Err(_) => panic!(":("),
    }
}
