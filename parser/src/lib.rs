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
    fn positive_number() {
        let parse_result = TERM_PARSER.parse("42").unwrap();
        assert_eq!(Term::Int(42), parse_result);
    }

    #[test]
    fn negative_number() {
        let is_err = TERM_PARSER.parse("-42").is_err();
        assert_eq!(true, is_err);
    }

    #[test]
    fn too_big_number() {
        let is_err = TERM_PARSER.parse("12345678912345678912345").is_err();
        assert_eq!(true, is_err);
    }
}
