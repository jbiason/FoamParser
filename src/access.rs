//! Functions to access elements in the tree.

use std::collections::HashMap;

use crate::Foam;
use crate::FoamError;

impl<'a> Foam<'a> {
    /// Retrieve the values from the dictionary. A few things to note:
    ///
    /// Retrieving an element will always return an array, as Foam support multiple values in the
    /// same key.
    ///
    /// ```
    /// # use foamparser::Foam;
    /// let root = Foam::parse("var value;").unwrap();
    /// let var = root.get("var");
    /// let expected = vec![Foam::Value("value")];
    /// assert_eq!(var, Ok(expected.as_slice()));
    /// ```
    ///
    /// If the key does not exist, the function will fail with [`FoamError::NoSuchKey`].
    ///
    /// ```
    /// # use foamparser::Foam;
    /// # use foamparser::FoamError;
    /// let root = Foam::parse("var value;").unwrap();
    /// let var2 = root.get("var2");
    /// assert_eq!(var2, Err(FoamError::NoSuchKey));
    /// ```
    ///
    /// One curious side effect of the fact that Foam supports multiple values for a variable is
    /// and that we return an array, `.get()` also works 'cause the primitive type also have it.
    /// But doing it so in one specific value will result in [`FoamError::NotADictionary`].
    ///
    /// ```
    /// # use foamparser::Foam;
    /// # use foamparser::FoamError;
    /// let root = Foam::parse("var ( 1 2 3 );").unwrap();
    /// let level1 = root.get("var").unwrap();
    /// let error = level1[0].get("1");
    /// assert_eq!(error, Err(FoamError::NotADictionary));
    /// ```
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
    /// This is a helper function that does the same as above, but returns the first element in the
    /// Dictionary.
    ///
    /// ```
    /// # use foamparser::Foam;
    /// # use foamparser::FoamError;
    /// let root = Foam::parse("var (1) 2;").unwrap();
    /// let first = root.get_first("var");
    /// let list = vec![Foam::Value("1")];
    /// assert_eq!(first, Ok(&Foam::List(list)));
    /// ```
    ///
    /// All the same rules for retrieving elements from dictionaries in [`Foam::get`] still
    /// apply.
    pub fn get_first(&self, key: &str) -> Result<&Foam<'a>, FoamError> {
        self.get(key).map(|x| &x[0])
    }

    /// Retrieve the first [`Foam::Value`] from a dictionary.
    ///
    /// This is a helper function that does something similar to [`Foam::get_first`], but retrieve the
    /// first element that it is a value, and returns its internal content. Any other element that
    /// is not a [`Foam::Value`] will be skipped.
    ///
    /// ```
    /// # use foamparser::Foam;
    /// # use foamparser::FoamError;
    /// let root = Foam::parse("var (1) 2;").unwrap();
    /// let skips_the_list = root.get_first_value("var");
    /// assert_eq!(skips_the_list, Ok("2"));
    /// ```
    ///
    /// If none of the elements are values, return [`FoamError::NoSuchValue`].
    ///
    /// ```
    /// # use foamparser::Foam;
    /// # use foamparser::FoamError;
    /// let root = Foam::parse("var (1) (2)").unwrap();
    /// let no_values = root.get_first_value("var");
    /// assert_eq!(no_values, Err(FoamError::NoSuchValue))
    /// ```
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
    /// This works similar to [`Foam::get_first_value`], but skips any elements that are not lists.
    ///
    /// ```
    /// # use foamparser::Foam;
    /// # use foamparser::FoamError;
    /// let root = Foam::parse("var 1 2 ( 3 4 );").unwrap();
    /// let var = root.get_first_list("var");
    /// let expected_list = vec![Foam::Value("3"), Foam::Value("4")];
    /// assert_eq!(var, Ok(expected_list.as_slice()));
    /// ```
    ///
    /// Same as [`Foam::get_first_value`], if none of the elements are lists, returns
    /// [`FoamError::NoSuchValue`].
    ///
    /// ```
    /// # use foamparser::Foam;
    /// # use foamparser::FoamError;
    /// let root = Foam::parse("var 1 2;").unwrap();
    /// let var = root.get_first_list("var");
    /// assert_eq!(var, Err(FoamError::NoSuchValue));
    /// ```
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

    /// Retrieve the first [`Foam::Dictionary`] from a dictionary.
    ///
    /// This works similar to [`Foam::get_first_value`], but skips any elements that are not lists.
    ///
    /// ```
    /// # use foamparser::Foam;
    /// # use foamparser::FoamError;
    /// # use std::collections::HashMap;
    /// let root = Foam::parse("not_dict 1; dict { a 1; }").unwrap();
    /// let var = root.get_first_dict("dict");
    /// let inner = HashMap::from([
    ///     ("a", vec![Foam::Value("1")])
    /// ]);
    /// assert_eq!(var, Ok(&inner));
    /// ```
    ///
    /// If the element is not a dictionary, [`FoamError::NoSuchValue`] will be returned as error.
    ///
    /// ```
    /// # use foamparser::Foam;
    /// # use foamparser::FoamError;
    /// let root = Foam::parse("not_dict 1; maybe_not_dict ( a 1 );").unwrap();
    /// let var = root.get_first_dict("maybe_not_dict");
    /// assert_eq!(var, Err(FoamError::NoSuchValue));
    /// ```
    ///
    /// If the there is not even an element with that key, [`FoamError::NoSuchKey`] will be
    /// returned.
    ///
    /// ```
    /// # use foamparser::Foam;
    /// # use foamparser::FoamError;
    /// let root = Foam::parse("not_dict 1; dict { a 1; }").unwrap();
    /// let var = root.get_first_dict("noSuchThing");
    /// assert_eq!(var, Err(FoamError::NoSuchKey));
    pub fn get_first_dict(
        &self,
        key: &str,
    ) -> Result<&HashMap<&str, Vec<Foam<'a>>>, FoamError> {
        match self.get(key) {
            Ok(entries) => {
                let first =
                    entries.iter().find(|x| matches!(x, Foam::Dictionary(_)));
                if let Some(Foam::Dictionary(entry)) = first {
                    Ok(&entry)
                } else {
                    Err(FoamError::NoSuchValue)
                }
            }
            Err(e) => Err(e),
        }
    }

    /// Treat the current element as a dictionary and returns its underlying map.
    ///
    /// ```
    /// # use foamparser::Foam;
    /// # use foamparser::FoamError;
    /// # use std::collections::HashMap;
    /// let root = Foam::parse("outer { inner 2; var 2; }").unwrap();
    /// let outer = root.get_first("outer").unwrap();
    /// assert_eq!(
    ///     outer.as_dict(),
    ///     Ok(
    ///         &HashMap::from([
    ///             ("inner", vec![Foam::Value("2")]),
    ///             ("var", vec![Foam::Value("2")])
    ///         ])
    ///     )
    /// )
    /// ```
    ///
    /// Trying to treat any other element as a Dictionary will result in
    /// [`FoamError::NotADictionary`]
    ///
    /// ```
    /// # use foamparser::Foam;
    /// # use foamparser::FoamError;
    /// let root = Foam::parse("var 2;").unwrap();
    /// let var = root.get_first("var").unwrap();
    /// // var is a Foam::Value
    /// let dict_maybe = var.as_dict();
    /// assert_eq!(dict_maybe, Err(FoamError::NotADictionary))
    /// ```
    pub fn as_dict(&self) -> Result<&HashMap<&str, Vec<Foam<'a>>>, FoamError> {
        match self {
            Foam::Dictionary(inner) => Ok(inner),
            _ => Err(FoamError::NotADictionary),
        }
    }

    /// Treat the current element as a value and return its underlying value.
    ///
    /// ```
    /// # use foamparser::Foam;
    /// # use foamparser::FoamError;
    /// let root = Foam::parse("var 3;").unwrap();
    /// let var = root.get_first("var").unwrap();
    /// let value = var.as_value().unwrap();
    /// assert_eq!(value, "3");
    /// ```
    ///
    /// Trying to use an element that is not a value will result in [`FoamError::NotAValue`].
    ///
    /// ```
    /// # use foamparser::Foam;
    /// # use foamparser::FoamError;
    /// let root = Foam::parse("var (3);").unwrap();
    /// let var = root.get_first("var").unwrap();
    /// let value = var.as_value();
    /// assert_eq!(value, Err(FoamError::NotAValue))
    /// ```
    pub fn as_value(&self) -> Result<&'a str, FoamError> {
        match self {
            Foam::Value(inner) => Ok(inner),
            _ => Err(FoamError::NotAValue),
        }
    }
}
