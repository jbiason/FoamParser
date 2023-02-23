//! Parse a Foam file into a major structure.

#[cfg(test)]
mod parser_tests;
#[cfg(test)]
mod struct_tests;

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
            r => panic!("Can't handle \"{r:?}\""),
        }
    }
}
