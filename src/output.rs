//! Outputs the contents of a processed foamfile.

use std::fmt::Display;

use crate::Foam;

/// List of characters that will force values to be quoted.
/// (We could easily add quotes everywhere, but better like this).
const NEED_QUOTE: &'static str = " \t()[]{}*\"";

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
                    write!(f, "{}\n{}{{", safe_keyword(key), in_level)?;

                    for element in content {
                        element.display(level + 1, f)?;
                    }
                }
                write!(f, "")
            }
            Foam::Value(value) => write!(f, "{} ", safe_keyword(value)),
            Foam::List(values) => {
                write!(f, "(\n")?;
                for element in values {
                    element.display(level + 1, f)?;
                }
                write!(f, "{})\n", in_level)
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
