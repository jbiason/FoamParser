use pest::Parser;

use crate::FoamParser;
use crate::Rule;

#[test]
fn comment() {
    let result =
        FoamParser::parse(Rule::multi_comment, "/* this is comment */");
    assert!(result.is_ok());
}

#[test]
fn broken() {
    let result = FoamParser::parse(Rule::multi_comment, "/* this is comment");
    assert!(result.is_err());
}

#[test]
fn embedded() {
    let result =
        FoamParser::parse(Rule::multi_comment, "/* this /* is */ comment */");
    assert!(result.is_ok());
}
