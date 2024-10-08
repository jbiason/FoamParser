//! Outputs the contents of a processed foamfile.

use std::fmt::Display;

use crate::Foam;

/// List of characters that will force values to be quoted.
/// (We could easily add quotes everywhere, but better like this).
const NEED_QUOTE: &str = " \t()[]{}*\"";

impl<'a> Foam<'a> {
    fn display(
        &self,
        level: usize,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let in_level = " ".repeat(level * 3);
        write!(f, "{}", in_level)?;

        match self {
            Foam::Dictionary(map) => {
                for (key, content) in map {
                    write!(f, "{}", safe_keyword(key))?;

                    if level != 0 {
                        // Root is a FoamDictionary, but we don't wrap things in a dictionary
                        write!(f, "\n{}{{", in_level)?;
                    }

                    let mut need_quote = true;
                    for element in content {
                        if let Foam::List(_) = element {
                            need_quote = false;
                        }
                        element.display(level + 1, f)?;
                    }
                    if need_quote {
                        writeln!(f, ";")?;
                    }
                }
                write!(f, "")
            }
            Foam::Value(value) => write!(f, "{} ", safe_keyword(value)),
            Foam::List(values) => {
                writeln!(f, "(")?;
                for element in values {
                    element.display(level + 1, f)?;
                }
                writeln!(f)?;
                writeln!(f, "{});", in_level)
            }
            Foam::Dimension(values) => {
                write!(f, "[ ")?;
                for element in values {
                    write!(f, "{} ", element)?;
                }
                write!(f, "];")
            }
        }
    }
}

/// Checks a keyword for any characters that may force it to be quoted. Returns the quoted string
/// if needed, or the same string back if it doesn't.
fn safe_keyword(keyword: &str) -> String {
    if keyword.chars().any(|char| NEED_QUOTE.contains(char)) {
        format!("\"{}\"", keyword)
    } else {
        keyword.to_string()
    }
}

impl<'a> Display for Foam<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display(0, f)
    }
}

#[cfg(test)]
mod test {
    use crate::Foam;

    #[test]
    fn simple() {
        let data = Foam::parse("var value;").unwrap();
        let formatted = data.to_string();
        assert_eq!(formatted, "var   value ;\n");
    }

    // Issue: HashMaps in Rust have absolutely no guarantee of order, even when run more than once.
    //        (or maybe I just need better key names)
    // #[test]
    // fn two_values() {
    //     let data = Foam::parse("var1 value1; var2 value2;").unwrap();
    //     let formatted = data.to_string();
    //     // Our dictionaries do not guarantee order (this is not in the spec, anyway)
    //     assert_eq!(formatted, "var2   value2 ;\nvar1   value1 ;\n");
    // }

    #[test]
    fn a_list() {
        let data = Foam::parse("var ( 1 2 3 );").unwrap();
        let formatted = data.to_string();
        assert_eq!(formatted, "var   (\n      1       2       3 \n   );\n");
    }
}
