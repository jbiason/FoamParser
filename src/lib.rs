//! Parse a Foam file into a major structure.

use std::collections::HashMap;

mod tokenizer;
mod parser;

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
    Dictionary(HashMap<&'a str, Foam<'a>>),

    /// A list of values.
    /// In Foam, it is possible to have multiple assigned values to a single variable, like
    ///
    /// ```cpp
    /// variable 1 2 3;
    /// ```
    Values(Vec<&'a str>),

    /// A list.
    /// Weirdly enough, although it is possible to have a single attribution to have multiple
    /// values, there is a List property that also allows multiple values.
    /// Here, we are splitting those in two different camps: Attributions without list should only
    /// contain single values, while Lists can have other Foam structures inside.
    List(Vec<Foam<'a>>),

    /// A dimensional list.
    /// This works kinda like Lists, but are used for dimensional content.
    Dimension(Vec<&'a str>),
}

