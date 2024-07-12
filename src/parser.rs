//! Generates the Foam structure from a source.

use std::collections::HashMap;

use crate::Foam;

pub fn parse<'a>(content: &'a str) -> Foam<'a> {
    let root = Foam::Dictionary(HashMap::new());
    return root;
}
