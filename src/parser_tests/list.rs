use pest::Parser;

use crate::FoamParser;
use crate::Rule;

#[test]
fn with_size() {
    let result = FoamParser::parse(Rule::list, "2 ( 1 2 )");
    assert!(result.is_ok());
}

#[test]
fn no_size() {
    let result = FoamParser::parse(Rule::list, "(1 2)");
    assert!(result.is_ok());
}

#[test]
fn list_in_list() {
    let result = FoamParser::parse(Rule::list, "( (1 2) (3 4))");
    assert!(result.is_ok());
}
