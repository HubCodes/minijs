#[macro_use] extern crate lalrpop_util;
#[macro_use] extern crate lazy_static;

lalrpop_mod!(grammar);

pub fn parse(code: &str) -> i32 {
    grammar::TermParser::new().parse(code).unwrap()
}

#[cfg(test)]
mod tests {
    use super::grammar;
    lazy_static! {
        static ref TERM_PARSER: grammar::TermParser = grammar::TermParser::new();
    }

    #[test]
    fn positive_number() {
        let parse_result = TERM_PARSER.parse("42").unwrap();
        assert_eq!(42, parse_result);
    }

    #[test]
    fn negative_number() {
        let is_err = TERM_PARSER.parse("-42").is_err();
        assert_eq!(true, is_err);
    }

    /*
    #[test]
    fn too_big_number() {
        let is_err = TERM_PARSER.parse("12345678912345678912345").is_err();
        assert_eq!(true, is_err);
    }
     */
}
