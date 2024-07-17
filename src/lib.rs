//! Parse a Foam file into a major structure.

use std::collections::HashMap;

mod access;
mod output;
mod parser;
mod tokenizer;

/// The structures inside a Foamfile
#[derive(Debug, PartialEq)]
pub enum Foam<'a> {
    /// A dictionary (key/value pairs).
    /// The root of a foam documentation is always a dictionary, and the entries at the top level
    /// are the keys. E.g., a foamfile consisting of
    ///
    /// ```cpp
    /// variable value;
    /// ```
    ///
    /// ... is actually a dictionary with "variable" as the key and "value" as its value; in the
    /// same way,
    ///
    /// ```cpp
    /// variable
    /// {
    ///     variable value;
    /// }
    /// ```
    ///
    /// ... is a dictionary in which the value is another dictionary.
    Dictionary(HashMap<&'a str, Vec<Foam<'a>>>),

    /// A single value.
    Value(&'a str),

    /// A list.
    List(Vec<Foam<'a>>),

    /// A dimensional list.
    /// This works kinda like Lists, but are used for dimensional content (for whatever that means).
    Dimension(Vec<&'a str>),
}

/// Errors.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum FoamError<'a> {
    #[error("Unexpected end of content")]
    EndOfContent,

    #[error("While processing dictionary {name:?}, found not values")]
    NoDictValues { name: &'a str },

    #[error("Invalid end of a dictionary: {token:?}")]
    InvalidDictEnd { token: &'a str },

    #[error("Expected a keyword, found {token:?} (at {start} to {end})")]
    MissingKeyword {
        token: &'a str,
        start: usize,
        end: usize,
    },

    #[error("Unexpected keyword {token:?} when processing {structure}")]
    UnexpectedToken { token: &'a str, structure: &'a str },

    #[error(
        "Requested key from dictionary, but current object is not a dictionary"
    )]
    NotADictionary,

    #[error("The requested key does not exist")]
    NoSuchKey,
}
