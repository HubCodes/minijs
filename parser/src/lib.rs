#[macro_use] extern crate lalrpop_util;
#[macro_use] extern crate lazy_static;

use lang::ast::Term;
use lalrpop_util::{ParseError, lexer};

lalrpop_mod!(grammar);

pub fn parse(code: &str) -> Result<Term, ParseError<usize, lexer::Token, &str>> {
    let mut symbol_id = 0;
    grammar::TermParser::new().parse(&mut symbol_id, code)
}

#[cfg(test)]
mod tests {
    use super::grammar;
    use lang::ast::*;

    lazy_static! {
        static ref TERM_PARSER: grammar::TermParser = grammar::TermParser::new();
        static ref EXPR_PARSER: grammar::ExprParser = grammar::ExprParser::new();
    }

    #[test]
    fn positive_int() {
        let mut symbol_id = 0;
        let parse_result = TERM_PARSER.parse(&mut symbol_id, "42").unwrap();
        assert_eq!(Term::Num(Num::Int(42)), parse_result);
    }

    #[test]
    #[should_panic]
    fn negative_int() {
        let mut symbol_id = 0;
        TERM_PARSER.parse(&mut symbol_id, "-42").unwrap();
    }

    #[test]
    fn too_big_int_into_double() {
        let mut symbol_id = 0;
        let parse_result = TERM_PARSER.parse(&mut symbol_id, "12345678912345678912345").unwrap();
        assert_eq!(Term::Num(Num::Double(12345678912345678912345.)), parse_result);
    }

    #[test]
    fn positive_double() {
        let mut symbol_id = 0;
        let parse_result = TERM_PARSER.parse(&mut symbol_id, "42.24").unwrap();
        assert_eq!(Term::Num(Num::Double(42.24)), parse_result);
    }

    #[test]
    fn double_with_exp() {
        let mut symbol_id = 0;
        let parse_result = TERM_PARSER.parse(&mut symbol_id, "42.2e-3").unwrap();
        assert_eq!(Term::Num(Num::Double(42.2e-3)), parse_result);
    }

    #[test]
    fn string_literal_singlequote() {
        let mut symbol_id = 0;
        let parse_result = TERM_PARSER.parse(&mut symbol_id, "'Hello, world!'").unwrap();
        assert_eq!(Term::Str("Hello, world!".to_string()), parse_result);
    }

    #[test]
    fn string_literal_doublequote() {
        let mut symbol_id = 0;
        let parse_result = TERM_PARSER.parse(&mut symbol_id, "\"Hello, world!\"").unwrap();
        assert_eq!(Term::Str("Hello, world!".to_string()), parse_result);
    }

    #[test]
    fn string_literal_empty() {
        let mut symbol_id = 0;
        let parse_result = TERM_PARSER.parse(&mut symbol_id, "''").unwrap();
        assert_eq!(Term::Str("".to_string()), parse_result);
    }

    #[test]
    fn string_literal_one_char() {
        let mut symbol_id = 0;
        let parse_result = TERM_PARSER.parse(&mut symbol_id, "'1'").unwrap();
        assert_eq!(Term::Str("1".to_string()), parse_result);
    }

    #[test]
    fn symbol() {
        let mut symbol_id = 0;
        let parse_result = TERM_PARSER.parse(&mut symbol_id, "name").unwrap();
        assert_eq!(Term::Symbol(Symbol { id: 0, name: "name".to_string() }), parse_result);
    }

    #[test]
    fn two_symbols() {
        let mut symbol_id = 0;
        let first_parse_result = TERM_PARSER.parse(&mut symbol_id, "name1").unwrap();
        let second_parse_result = TERM_PARSER.parse(&mut symbol_id, "name2").unwrap();
        let first_expected = Term::Symbol(Symbol { id: 0, name: "name1".to_string() });
        let second_expected = Term::Symbol(Symbol { id: 1, name: "name2".to_string() });
        assert_eq!(first_expected, first_parse_result);
        assert_eq!(second_expected, second_parse_result);
    }

    #[test]
    fn postfix_expr_indexing() {
        let mut symbol_id = 0;
        let parse_result = EXPR_PARSER.parse(&mut symbol_id, "a[1]").unwrap();
        let target_symbol = Expr::Term(Term::Symbol(Symbol { id: 0, name: "a".to_string() }));
        let target_index = Expr::Term(Term::Num(Num::Int(1)));
        let expected = Expr::Unop(Unop::Index, Box::new(target_symbol), Box::new(target_index));
        assert_eq!(expected, parse_result);
    }
}
