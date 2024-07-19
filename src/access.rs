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

    /// Retrieve the first element from a dictionary.
    ///
    /// 1. The element *must* be a [`Foam::Dictionary`], or it will return
    ///    [`FoamError::NotADictionary`]
    /// 2. The key *must* exist, or it will return [`FoamError::NoSuchKey`]
    /// 3. Foam supports multiple values from the same key, but this function will always return
    ///    just the first one.
    pub fn get_first(&self, key: &str) -> Result<&Foam<'a>, FoamError> {
        self.get(key).map(|x| &x[0])
    }

    /// Retrieve the first [`Foam::Value`] from a dictionary.
    ///
    /// 1. The element *must* be a [`Foam::Dictionary`], or it will return
    ///    [`FoamError::NotADictionary`]
    /// 2. The key *must* exist, or it will return [`FoamError::NoSuchKey`]
    /// 3. Foam supports multiple values from the same key, and this function will retrieve the
    ///    first one that is a [`Foam::Value`].
    /// 4. If none of the elements of the key is a [`Foam::Value`], this function will return
    ///    [`FoamError::NoSuchValue`]
    /// 5. The function will return the inner value of the [`Foam::Value`], so there is no need to
    ///    "destructure" the result.
    pub fn get_first_value(&self, key: &str) -> Result<&str, FoamError> {
        match self.get(key) {
            Ok(entries) => {
                let first =
                    entries.iter().find(|x| matches!(x, Foam::Value(_)));
                if let Some(Foam::Value(entry)) = first {
                    Ok(entry)
                } else {
                    Err(FoamError::NoSuchValue)
                }
            }
            Err(e) => Err(e),
        }
    }

    /// Retrieve the first [`Foam::List`] from a dictionary.
    ///
    /// 1. The element *must* be a [`Foam::Dictionary`], or it will return
    ///    [`FoamError::NotADictionary`]
    /// 2. The key *must* exist, or it will return [`FoamError::NoSuchKey`]
    /// 3. Foam supports multiple values from the same key, and this function will retrieve the
    ///    first one that is a [`Foam::List`].
    /// 4. If none of the elements of the key is a [`Foam::List`], this function will return
    ///    [`FoamError::NoSuchValue`]
    /// 5. The function will return the inner array of the [`Foam::LIst`], so there is no need to
    ///    "destructure" the result.
    /// 6. Note that *only* the List will be returned as an array; it will still hold any [`Foam`]
    ///    elements as they are.
    pub fn get_first_list(&self, key: &str) -> Result<&[Foam<'a>], FoamError> {
        match self.get(key) {
            Ok(entries) => {
                let first = entries.iter().find(|x| matches!(x, Foam::List(_)));
                if let Some(Foam::List(entries)) = first {
                    Ok(entries)
                } else {
                    Err(FoamError::NoSuchValue)
                }
            }
            Err(e) => Err(e),
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

    #[test]
    fn get_first() {
        let root = Foam::parse("var (1) 2;").unwrap();
        let first = root.get_first("var");
        let list = vec![Foam::Value("1")];
        assert_eq!(first, Ok(&Foam::List(list)));
    }

    #[test]
    fn get_first_value() {
        let root = Foam::parse("var (1) 2;").unwrap();
        let first = root.get_first_value("var");
        assert_eq!(first, Ok("2"));
    }

    #[test]
    fn get_first_list() {
        let root = Foam::parse("var 1 2 ( 3 4 );").unwrap();
        let first = root.get_first_list("var");
        let list = vec![Foam::Value("3"), Foam::Value("4")];
        assert_eq!(first, Ok(list.as_slice()));
    }
}
