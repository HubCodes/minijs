use parser::parse;
use sema::from_ast::SymbolTableBuilder;

fn main() {
    let code =
        "(function(a){
            a = 1;
        })();
        ";
    let ast = parse(code);
    let mut symbol_table_builder = SymbolTableBuilder::new();
    let symbol_table = match ast {
        Ok(ast) => symbol_table_builder.from_ast(&ast),
        Err(_) => panic!(":("),
    };
    print!("{:?}", symbol_table);
}
