use parser::parse;
use translator::trans::Translator;

fn main() {
    let code =
        "x.y.z;";
    let ast = parse(code);
    let mut translator = Translator::new();
    match ast {
        Ok(ast) => println!("{:#?}", translator.trans(ast).serialize()),
        Err(_) => panic!(":("),
    }
}
