#[macro_use] extern crate lalrpop_util;
#[macro_use] extern crate lazy_static;

use lang::ast::Term;
use lalrpop_util::{ParseError, lexer};

lalrpop_mod!(grammar);

pub fn parse(code: &str) -> Result<Term, ParseError<usize, lexer::Token, &str>> {
    grammar::TermParser::new().parse(code)
}

#[cfg(test)]
mod tests {
    use super::grammar;
    use lang::ast::*;

    lazy_static! {
        static ref TERM_PARSER: grammar::TermParser = grammar::TermParser::new();
    }

    #[test]
    fn positive_int() {
        let parse_result = TERM_PARSER.parse("42").unwrap();
        assert_eq!(Term::Num(Num::Int(42)), parse_result);
    }

    #[test]
    fn negative_int() {
        let is_err = TERM_PARSER.parse("-42").is_err();
        assert_eq!(true, is_err);
    }

    #[test]
    fn too_big_int_into_double() {
        let parse_result = TERM_PARSER.parse("12345678912345678912345").unwrap();
        assert_eq!(Term::Num(Num::Double(12345678912345678912345.)), parse_result);
    }

    #[test]
    fn positive_double() {
        let parse_result = TERM_PARSER.parse("42.24").unwrap();
        assert_eq!(Term::Num(Num::Double(42.24)), parse_result);
    }

    #[test]
    fn double_with_exp() {
        let parse_result = TERM_PARSER.parse("42.2e-3").unwrap();
        assert_eq!(Term::Num(Num::Double(42.2e-3)), parse_result);
    }
}
