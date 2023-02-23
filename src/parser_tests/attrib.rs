use pest::Parser;

use crate::FoamParser;
use crate::Rule;

#[test]
fn single_value() {
    let result = FoamParser::parse(Rule::attribution, "a_var value;");
    assert!(result.is_ok());
}

#[test]
fn multiple_values() {
    let result = FoamParser::parse(Rule::attribution, "a_var value1 valu2;");
    assert!(result.is_ok());
}

#[test]
fn invalid_defintion() {
    let result = FoamParser::parse(Rule::attribution, "1_var value;");
    assert!(result.is_err());
}
