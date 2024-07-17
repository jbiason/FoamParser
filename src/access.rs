//! Functions to access elements in the tree.

use crate::Foam;
use crate::FoamError;

impl<'a> Foam<'a> {
    /// Retrieve the values from the dictionary. A few things to note:
    ///
    /// 1. The element *must* be a [`Foam::Dictionary`], or it will return
    ///    [`FoamError::NotADictionary`]
    /// 2. The key *must* exist, or it will return [`FoamError::NoSuchKey`]
    /// 3. Because Foam supports multiple values in the same key, the result will be an array with
    ///    all [`Foam`] elements inside it.
    pub fn get(&self, key: &str) -> Result<&[Foam<'a>], FoamError> {
        match self {
            Foam::Dictionary(inner) => inner
                .get(key)
                .map(|content| content.as_slice())
                .ok_or(FoamError::NoSuchKey),
            _ => Err(FoamError::NotADictionary),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Foam, FoamError};

    #[test]
    fn get_dict() {
        let root = Foam::parse("var value;").unwrap();
        let value = root.get("var");
        let expected = vec![Foam::Value("value")];
        assert_eq!(value, Ok(expected.as_slice()));
    }

    #[test]
    fn get_no_dict() {
        let root = Foam::parse("var ( 1 2 3 );").unwrap();
        let level1 = root.get("var").unwrap();
        let error = level1[0].get("1");
        assert_eq!(error, Err(FoamError::NotADictionary));
    }

    #[test]
    fn get_no_key() {
        let root = Foam::parse("var value;").unwrap();
        let error = root.get("value");
        assert_eq!(error, Err(FoamError::NoSuchKey));
    }
}
