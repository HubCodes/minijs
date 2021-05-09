use parser::parse;
use translator::trans::Translator;

fn main() {
    let code =
        "let x = function(y){ y * 2; };";
    let ast = parse(code);
    let mut translator = Translator::new();
    match ast {
        Ok(ast) => println!("{:#?}", translator.trans(ast).serialize()),
        Err(_) => panic!(":("),
    }
}
