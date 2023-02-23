//! Parse a Foam file into a major structure.

use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "foam.pest"]
struct FoamParser;

#[derive(Debug, PartialEq)]
pub enum FoamElement {
    Element(String),
    Attribution(String, Vec<FoamElement>),
    List(Vec<FoamElement>),
}

/// The main Foam parser.
#[derive(Debug, PartialEq)]
pub struct Foam {
    root: Vec<FoamElement>,
}
// Note: I could done this inside the enum, but I don't want to expose the
//       HashMap to the user.

impl AsRef<Vec<FoamElement>> for Foam {
    fn as_ref(&self) -> &Vec<FoamElement> {
        &self.root
    }
}

#[derive(Debug)]
pub enum FoamError {
    InvalidFile,
    EmptyFile,
}

impl From<pest::error::Error<Rule>> for FoamError {
    fn from(_: pest::error::Error<Rule>) -> Self {
        FoamError::InvalidFile
    }
}

impl TryFrom<&str> for Foam {
    type Error = FoamError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let file = FoamParser::parse(Rule::file, value)?
            .next()
            .ok_or(FoamError::EmptyFile)?;
        println!("Root: {:?}", file);

        let elements = file
            .into_inner()
            .map(|pair| Foam::parse_value(pair))
            .collect::<Vec<FoamElement>>();
        Ok(Foam { root: elements })
    }
}

impl Foam {
    fn parse_value(pair: Pair<Rule>) -> FoamElement {
        println!("{:?}", pair);
        match pair.as_rule() {
            Rule::attribution => {
                let mut inner_rules = pair.into_inner();
                let definition =
                    inner_rules.next().unwrap().as_str().to_string();
                let elements = inner_rules
                    .map(|pair| Foam::parse_value(pair))
                    .collect::<Vec<FoamElement>>();
                FoamElement::Attribution(definition, elements)
            }
            Rule::value => FoamElement::Element(pair.as_str().to_string()),
            Rule::list => {
                println!("List: {:?}", pair);
                let values = pair
                    .into_inner()
                    .map(|pair| {
                        println!("List inner: {:?}", pair);
                        Foam::parse_value(pair)
                    })
                    .collect::<Vec<FoamElement>>();
                FoamElement::List(values)
            }
            r => panic!("Can't handle {r:?}"),
        }
    }
}

#[cfg(test)]
mod parser {
    use super::*;

    mod attrib {
        use super::*;
        #[test]
        fn single_value() {
            let result = FoamParser::parse(Rule::attribution, "a_var value;");
            assert!(result.is_ok());
        }

        #[test]
        fn multiple_values() {
            let result =
                FoamParser::parse(Rule::attribution, "a_var value1 valu2;");
            assert!(result.is_ok());
        }

        #[test]
        fn invalid_defintion() {
            let result = FoamParser::parse(Rule::attribution, "1_var value;");
            assert!(result.is_err());
        }
    }

    mod multi_comment {
        use super::*;

        #[test]
        fn comment() {
            let result =
                FoamParser::parse(Rule::multi_comment, "/* this is comment */");
            assert!(result.is_ok());
        }

        #[test]
        fn broken() {
            let result =
                FoamParser::parse(Rule::multi_comment, "/* this is comment");
            assert!(result.is_err());
        }

        #[test]
        fn embedded() {
            let result = FoamParser::parse(
                Rule::multi_comment,
                "/* this /* is */ comment */",
            );
            assert!(result.is_ok());
        }
    }

    mod list {
        use super::*;

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
    }
}

#[cfg(test)]
mod foam_struct {
    use super::*;

    #[test]
    fn attribution() {
        let text = "a_var value;";
        let result = Foam::try_from(text).unwrap();
        let expected = vec![FoamElement::Attribution(
            String::from("a_var"),
            vec![FoamElement::Element(String::from("value"))],
        )];
        assert_eq!(result.as_ref(), &expected);
    }

    #[test]
    fn multiple_attribution() {
        let text = "a_var value1 value2 value3;";
        let result = Foam::try_from(text).unwrap();
        let expected = vec![FoamElement::Attribution(
            String::from("a_var"),
            vec![
                FoamElement::Element(String::from("value1")),
                FoamElement::Element(String::from("value2")),
                FoamElement::Element(String::from("value3")),
            ],
        )];
        assert_eq!(result.as_ref(), &expected);
    }

    #[test]
    fn two_attributions() {
        let text = "var1 value1; var2 value2;";
        let result = Foam::try_from(text).unwrap();
        let expected = vec![
            FoamElement::Attribution(
                String::from("var1"),
                vec![FoamElement::Element(String::from("value1"))],
            ),
            FoamElement::Attribution(
                String::from("var2"),
                vec![FoamElement::Element(String::from("value2"))],
            ),
        ];
        assert_eq!(result.as_ref(), &expected);
    }

    #[test]
    fn a_list() {
        let text = "var ( 1 2 );";
        let result = Foam::try_from(text).unwrap();
        let expected = vec![FoamElement::Attribution(
            String::from("var"),
            vec![FoamElement::List(vec![
                FoamElement::Element(String::from("1")),
                FoamElement::Element(String::from("2")),
            ])],
        )];
        assert_eq!(result.as_ref(), &expected);
    }
}
