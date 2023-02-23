use pest::Parser;

use crate::FoamParser;
use crate::Rule;

#[test]
fn dict() {
    let result = FoamParser::parse(Rule::dict_attribution, "dict { a 1; }");
    assert!(result.is_ok(), "{:?}", result);
}

#[test]
fn two_attribs() {
    let result =
        FoamParser::parse(Rule::dict_attribution, "dict { a 1; b 2; }");
    assert!(result.is_ok(), "{:?}", result);
}

#[test]
fn dict_with_list() {
    let result =
        FoamParser::parse(Rule::dict_attribution, "dict { l ( 1 2 ); }");
    assert!(result.is_ok(), "{:?}", result);
}

#[test]
fn dict_with_dicts() {
    let result =
        FoamParser::parse(Rule::dict_attribution, "dict { d { a 1; } }");
    assert!(result.is_ok(), "{:?}", result);
}
