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
    List(Vec<String>),
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
                println!("Attribution: {:?}", pair);
                let mut inner_rules = pair.into_inner();
                let definition =
                    inner_rules.next().unwrap().as_str().to_string();
                println!("Defition: {}", definition);
                let elements = inner_rules
                    .map(|pair| {
                        println!("Att inner: {:?}", pair);
                        Foam::parse_value(pair)
                    })
                    .collect::<Vec<FoamElement>>();
                FoamElement::Attribution(definition, elements)
            }
            Rule::value => {
                println!("Value: {:?}", pair);
                FoamElement::Element(pair.as_str().to_string())
            }
            Rule::list => todo!(),
            Rule::element => todo!(),
            r => panic!("Can't handle {r:?}"),
        }
    }
}

#[cfg(test)]
mod parser {
    use super::*;

    #[test]
    fn attribution() {
        let result = FoamParser::parse(Rule::attribution, "a_var value;");
        assert!(result.is_ok())
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
}
