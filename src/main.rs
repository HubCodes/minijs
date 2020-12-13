use parser::parse;
use sema::from_ast::SymbolTableBuilder;

fn main() {
    let ast = parse("let a = 1;(function(a){a=1;})();");
    let mut symbol_table_builder = SymbolTableBuilder::new();
    // ast에서 스코프 값을 계산할 수 있다면
    // 스코프 키로 사용 가능할듯
    // 문제는 그렇게 했을때 외부에서 테스트 하는게 조금 힘들수도있다는건데...
    let symbol_table = match ast {
        Ok(ast) => symbol_table_builder.from_ast(&ast),
        Err(_) => panic!(":("),
    };
    print!("{:?}", symbol_table);
}
