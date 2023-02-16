//! Parse a Foam file into a major structure.

use std::collections::HashMap;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "foam.pest"]
struct FoamParser;

#[derive(Debug, PartialEq)]
pub enum FoamElement {
    Values(Vec<String>),
    // Attribution {
    //     variable: String,
    //     values: Vec<String>,
    // },
}

/// The main Foam parser.
#[derive(Debug, PartialEq)]
pub struct Foam {
    root: HashMap<String, FoamElement>,
}
// Note: I could done this inside the enum, but I don't want to expose the
//       HashMap to the user.

impl AsRef<HashMap<String, FoamElement>> for Foam {
    fn as_ref(&self) -> &HashMap<String, FoamElement> {
        &self.root
    }
}

impl TryFrom<&str> for Foam {
    type Error = (); // FIX

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let file = FoamParser::parse(Rule::file, value)
            .expect("Failed to parse file")
            .next()
            .unwrap();
        let mut root = HashMap::new();

        for line in file.into_inner() {
            match line.as_rule() {
                Rule::attribution => {
                    let mut inner_rules = line.into_inner();
                    let name: &str = inner_rules.next().unwrap().as_str();
                    let values = inner_rules
                        .into_iter()
                        .map(|x| x.as_str().to_string())
                        .collect::<Vec<String>>();
                    root.insert(name.into(), FoamElement::Values(values));
                }
                Rule::EOI => (),
                r => panic!("I don't know what {r:?} is!"),
            }
        }
        Ok(Foam { root })
    }
}

#[cfg(test)]
mod parser {
    use super::*;

    #[test]
    fn multi_comment() {
        let parse =
            FoamParser::parse(Rule::multi_comment, "/* this is comment */");
        assert!(parse.is_ok());
    }

    #[test]
    fn definition() {
        let parse = FoamParser::parse(Rule::definition, "foamFile");
        assert!(parse.is_ok(), "{:#?}", parse);
        let parse = FoamParser::parse(Rule::definition, "div(phi,U)");
        assert!(parse.is_ok(), "{:#?}", parse);
        let parse = FoamParser::parse(
            Rule::definition,
            "div((nuEff*dev2(T(grad(U)))))",
        );
        assert!(parse.is_ok(), "{:#?}", parse);
        let parse = FoamParser::parse(Rule::definition, "FoamFile");
        assert!(parse.is_ok(), "{:#?}", parse);
        let parse = FoamParser::parse(Rule::definition, "foam_file");
        assert!(parse.is_ok());
    }

    #[test]
    fn attribution() {
        let parse = FoamParser::parse(Rule::attribution, "version 2.0;");
        assert!(parse.is_ok(), "{:#?}", parse);
        let parse = FoamParser::parse(
            Rule::attribution,
            "div(U)          Gauss linear;",
        );
        assert!(parse.is_ok(), "{:#?}", parse);
    }

    #[test]
    fn broken_multi() {
        let parse = FoamParser::parse(Rule::multi_comment, "/* bad comment");
        assert!(parse.is_err());
    }

    #[test]
    fn single_comment() {
        let parse =
            FoamParser::parse(Rule::single_comment, "// this is comment");
        assert!(parse.is_ok());
    }

    #[test]
    fn broken_value() {
        let parse = FoamParser::parse(Rule::value, "123");
        assert!(parse.is_ok(), "{:?}", parse);

        let parse = FoamParser::parse(Rule::value, "asd ");
        assert!(parse.is_ok(), "{:?}", parse);
    }

    // #[test]
    // fn attribution() {
    //     let parse = FoamParser::parse(Rule::attribution, "version 2.0;");
    //     assert!(parse.is_ok(), "{:?}", parse);
    // }

    // #[test]
    // fn broken_attribution() {
    //     let parse = FoamParser::parse(Rule::attribution, "version 2.0");
    //     assert!(parse.is_err(), "{:?}", parse);
    // }

    // #[test]
    // fn dictionary() {
    //     let text = "FoamFile\n{\nversion 2.0;\nformat ascii;\nclass dictionary;\nlocation system;\nobject caseSetupDict;\n}";
    //     let parse = FoamParser::parse(Rule::dictionary, text);
    //     assert!(parse.is_ok(), "{:#?}", parse);
    // }

    // #[test]
    // fn dict_in_dict() {
    //     let text = "dict1 { dict2 { class bad; } }";
    //     let parse = FoamParser::parse(Rule::dictionary, text);
    //     assert!(parse.is_ok(), "{:#?}", parse);
    // }

    // #[test]
    // fn list() {
    //     let text = "list_name ( 1 2 3 );";
    //     let parse = FoamParser::parse(Rule::list, text);
    //     assert!(parse.is_ok(), "{:#?}", parse);
    // }

    // #[test]
    // fn sized_list() {
    //     let text = "list_name 3 ( 1 2 3 );";
    //     let parse = FoamParser::parse(Rule::list, text);
    //     assert!(parse.is_ok(), "{:#?}", parse);
    // }
}

// #[cfg(test)]
// mod file {
// use super::*;
// use pest::Parser;

// #[test]
// fn chained_comments() {
//     let text = "/* this is one comment */\n// And this is another";
//     let parse = FoamParser::parse(Rule::file, text);
//     assert!(parse.is_ok(), "{:?}", parse);
// }

// #[test]
// fn attribution_with_comment() {
//     let text = "version 2.0; // this is good";
//     let parse = FoamParser::parse(Rule::file, text);
//     assert!(parse.is_ok(), "{:?}", parse);
// }

// #[test]
// fn mid_comment() {
//     let text = "version /* this is comment */ 2.0;";
//     let parse = FoamParser::parse(Rule::file, text);
//     assert!(parse.is_ok(), "{:?}", parse);
// }

// #[test]
// fn complex() {
//     let text = "/* this is file */\nList ( 1\n2 // this is ok\n 3);";
//     let parse = FoamParser::parse(Rule::file, text);
//     assert!(parse.is_ok(), "{:?}", parse);
// }

// // Note: the following files were retrieved from OpenFoam examples:
// // https://develop.openfoam.com/Development/openfoam/-/tree/develop/tutorials/incompressible/pimpleFoam/RAS/rotatingFanInRoom/system
// #[test]
// fn control_dict() {
//     let text = include_bytes!("../resources/controlDict");
//     let parse = FoamParser::parse(Rule::file, &std::str::from_utf8(text).unwrap());
//     assert!(parse.is_ok(), "{:?}", parse);
// }

// #[test]
// fn block_mesh_dict() {
//     let text = include_bytes!("../resources/blockMeshDict");
//     let parse = FoamParser::parse(Rule::file, &std::str::from_utf8(text).unwrap());
//     assert!(parse.is_ok(), "{:?}", parse);
// }

// // #[test]
// // fn create_patch_dict() {
// //     let text = include_bytes!("../resources/createPatchDict");
// //     let parse = FoamParser::parse(Rule::file, &std::str::from_utf8(text).unwrap());
// //     assert!(parse.is_ok(), "{:?}", parse);
// // }

// #[test]
// fn fv_scheme() {
//     let text = include_bytes!("../resources/fvSchemes");
//     let parse = FoamParser::parse(Rule::file, &std::str::from_utf8(text).unwrap());
//     assert!(parse.is_ok(), "{:?}", parse);
// }
// }

#[cfg(test)]
mod parsing {
    use super::*;

    #[test]
    fn just_attribs() {
        let source = "version 2.0 1.0 0.0;";
        let result = Foam::try_from(source).unwrap();
        let expected = HashMap::from([(
            String::from("version"),
            FoamElement::Values(vec![
                String::from("2.0"),
                String::from("1.0"),
                String::from("0.0"),
            ]),
        )]);
        assert_eq!(result.as_ref(), &expected);
    }
}
