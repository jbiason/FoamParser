//! Generates the Foam structure from a source.

use std::collections::HashMap;

use logos::Logos;

use crate::tokenizer::Token;
use crate::Foam;
use crate::FoamError;

impl<'a> Foam<'a> {
    pub fn parse(content: &'a str) -> Result<Foam<'a>, FoamError<'a>> {
        let mut lexer = Token::lexer(content);
        get_dict(&mut lexer)
    }
}

fn get_dict<'a>(
    lexer: &mut logos::Lexer<'a, Token<'a>>,
) -> Result<Foam<'a>, FoamError<'a>> {
    let mut result = HashMap::new();
    let mut key = None;
    let mut key_values = Vec::new();
    loop {
        let token = lexer.next();
        tracing::debug!(?token);
        match token {
            None => break,
            Some(Err(_)) => return Err(FoamError::EndOfContent),

            Some(Ok(Token::ListEnd)) => {
                return Err(FoamError::UnexpectedToken {
                    token: ")",
                    structure: "dictionary",
                })
            }
            // Some elements are only possible if we have a defined key. For example, doing `{(1)}`
            // is not valid, 'cause we don't have the dictionary key yet.
            Some(Ok(Token::ListStart)) if key.is_none() => {
                return Err(FoamError::UnexpectedToken {
                    token: "(",
                    structure: "dictionary",
                })
            }
            // Same as above
            Some(Ok(Token::DictStart)) if key.is_none() => {
                return Err(FoamError::UnexpectedToken {
                    token: "{",
                    structure: "dictionary",
                })
            }
            // `;` is acceptable only if we are alredy processing a list of values, like
            // `dict { list 1 2 3; }`. If we see the `;` and we are not processing a list, then
            // something is wrong.
            Some(Ok(Token::End)) if key.is_none() => {
                return Err(FoamError::UnexpectedToken {
                    token: ";",
                    structure: "dictionary",
                })
            }

            // As per the rule above, if we have the key, then we have a list of elements, and we
            // can just push them into the current key. This resets the key and its values.
            Some(Ok(Token::End)) => {
                result.insert(key.unwrap(), key_values);
                key = None;
                key_values = Vec::new();
            }

            Some(Ok(Token::MultilineComment(_))) => continue,
            Some(Ok(Token::Comment(_))) => continue,

            Some(Ok(Token::Keyword(token))) if key.is_none() => {
                key = Some(token);
            }
            Some(Ok(Token::Keyword(token))) => {
                key_values.push(Foam::Value(token))
            }
            Some(Ok(Token::ListStart)) => key_values.push(get_list(lexer)?),
            Some(Ok(Token::DictStart)) => key_values.push(get_dict(lexer)?),

            Some(Ok(Token::DictEnd)) => {
                break;
            }
        }
    }
    if let Some(key) = key {
        result.insert(key, key_values);
    }

    Ok(Foam::Dictionary(result))
}

fn get_list<'a>(
    lexer: &mut logos::Lexer<'a, Token<'a>>,
) -> Result<Foam<'a>, FoamError<'a>> {
    let mut result = Vec::new();
    loop {
        match lexer.next() {
            None => return Err(FoamError::EndOfContent),
            Some(Err(_)) => return Err(FoamError::EndOfContent),

            Some(Ok(Token::DictEnd)) => {
                return Err(FoamError::UnexpectedToken {
                    token: "}",
                    structure: "list",
                })
            }
            Some(Ok(Token::End)) => {
                return Err(FoamError::UnexpectedToken {
                    token: ";",
                    structure: "list",
                })
            }

            Some(Ok(Token::MultilineComment(_))) => continue,
            Some(Ok(Token::Comment(_))) => continue,

            Some(Ok(Token::Keyword(token))) => result.push(Foam::Value(token)),
            Some(Ok(Token::DictStart)) => result.push(get_dict(lexer)?),
            Some(Ok(Token::ListStart)) => result.push(get_list(lexer)?),

            Some(Ok(Token::ListEnd)) => break,
        }
    }
    Ok(Foam::List(result))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn single_attribution() {
        let result = Foam::parse("variable value;");
        let map = HashMap::from([("variable", vec![Foam::Value("value")])]);
        assert_eq!(result, Ok(Foam::Dictionary(map)));
    }

    #[test]
    fn mutiple_attributions() {
        let result = Foam::parse("variable value1 value2 value3;");
        let map = HashMap::from([(
            "variable",
            vec![
                Foam::Value("value1"),
                Foam::Value("value2"),
                Foam::Value("value3"),
            ],
        )]);
        assert_eq!(result, Ok(Foam::Dictionary(map)));
    }

    #[test]
    fn multiple_variables() {
        let result = Foam::parse("var1 value1;\nvar2 value2 value3;");
        let map = HashMap::from([
            ("var1", vec![Foam::Value("value1")]),
            ("var2", vec![Foam::Value("value2"), Foam::Value("value3")]),
        ]);
        assert_eq!(result, Ok(Foam::Dictionary(map)));
    }

    #[test]
    fn simple_list() {
        let result = Foam::parse("var (value1 value2);");
        let map = HashMap::from([(
            "var",
            vec![Foam::List(vec![
                Foam::Value("value1"),
                Foam::Value("value2"),
            ])],
        )]);
        assert_eq!(result, Ok(Foam::Dictionary(map)));
    }

    #[test]
    fn lists_with_lists() {
        let result = Foam::parse("var ( value1 ( inner2 ) );");
        let map = HashMap::from([(
            "var",
            vec![Foam::List(vec![
                Foam::Value("value1"),
                Foam::List(vec![Foam::Value("inner2")]),
            ])],
        )]);
        assert_eq!(result, Ok(Foam::Dictionary(map)));
    }

    #[test]
    fn simple_dict() {
        let result = Foam::parse("entry { var value; }");
        let inner = HashMap::from([("var", vec![Foam::Value("value")])]);
        let map = HashMap::from([("entry", vec![Foam::Dictionary(inner)])]);
        assert_eq!(result, Ok(Foam::Dictionary(map)));
    }

    #[test]
    fn dict_with_multiple_values() {
        let result = Foam::parse("entry { var1 value1; var2 value2; }");
        let inner = HashMap::from([
            ("var1", vec![Foam::Value("value1")]),
            ("var2", vec![Foam::Value("value2")]),
        ]);
        let outer = HashMap::from([("entry", vec![Foam::Dictionary(inner)])]);
        assert_eq!(result, Ok(Foam::Dictionary(outer)));
    }

    #[test]
    fn dict_with_lists() {
        let result = Foam::parse("outer { a_list ( 1 2 3 ); }");
        let inner = HashMap::from([(
            "a_list",
            vec![Foam::List(vec![
                Foam::Value("1"),
                Foam::Value("2"),
                Foam::Value("3"),
            ])],
        )]);
        let outer = HashMap::from([("outer", vec![Foam::Dictionary(inner)])]);
        assert_eq!(result, Ok(Foam::Dictionary(outer)));
    }

    #[test]
    fn all_types() {
        let result = Foam::parse("attribution 1;list (1 2);dict {inner 1;}");
        let attribution = vec![Foam::Value("1")];
        let list = Foam::List(vec![Foam::Value("1"), Foam::Value("2")]);
        let dict = Foam::Dictionary(HashMap::from([(
            "inner",
            vec![Foam::Value("1")],
        )]));
        let main = Foam::Dictionary(HashMap::from([
            ("attribution", attribution),
            ("list", vec![list]),
            ("dict", vec![dict]),
        ]));
        assert_eq!(result, Ok(main));
    }

    #[test]
    fn named_dicts() {
        let result =
            Foam::parse("list ( name1 { inner 1; } name2 { inner 2; } )")
                .unwrap();
        println!("{result:?}");
    }
}
