use parser::parse;

fn main() {
    let code =
        "(function(a){
            x.y.z.w;
        })();
        ";
    let ast = parse(code);
    match ast {
        Ok(ast) => println!("{:#?}", ast),
        Err(_) => panic!(":("),
    }
}
