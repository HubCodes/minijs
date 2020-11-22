#[macro_use] extern crate lalrpop_util;

use std::convert::Infallible;
use std::string::ParseError;

lalrpop_mod!(grammar);

pub fn parse(code: &str) -> i32 {
    grammar::TermParser::new().parse(code).unwrap()
}

#[cfg(test)]
mod tests {
    fn calculator1() {
        assert!(grammar::TermParser::new().parse("22").is_ok());
        assert!(grammar::TermParser::new().parse("(22)").is_ok());
        assert!(grammar::TermParser::new().parse("((((22))))").is_ok());
        assert!(grammar::TermParser::new().parse("((22)").is_err());
    }
}
