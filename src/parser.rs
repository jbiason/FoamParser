//! Generates the Foam structure from a source.

use std::collections::HashMap;

use logos::Logos;

use crate::tokenizer::Token;
use crate::Foam;
use crate::FoamError;

pub fn parse<'a>(content: &'a str) -> Result<Foam<'a>, FoamError<'a>> {
    let mut lexer = Token::lexer(content);
    let mut root = HashMap::new();

    loop {
        if let Some(key) = get_keyword(&mut lexer)? {
            let value = get_value(&mut lexer, "root")?;
            root.insert(key, value);
        } else {
            break;
        }
    }

    Ok(Foam::Dictionary(root))
}

fn get_keyword<'a>(
    lexer: &mut logos::Lexer<'a, Token<'a>>,
) -> std::result::Result<Option<&'a str>, FoamError<'a>> {
    loop {
        match lexer.next() {
            None => return Ok(None),
            Some(Err(_)) => return Err(FoamError::EndOfContent),
            Some(Ok(Token::Keyword(token))) => return Ok(Some(token)),
            Some(Ok(Token::MultilineComment(_))) => continue,
            Some(Ok(Token::Comment(_))) => continue,
            Some(Ok(_)) => {
                let token = lexer.slice();
                let span = lexer.span();
                return Err(FoamError::MissingKeyword {
                    token,
                    start: span.start,
                    end: span.end,
                });
            }
        }
    }
}

fn get_value<'a>(
    lexer: &mut logos::Lexer<'a, Token<'a>>,
    structure: &'a str,
) -> std::result::Result<Vec<Foam<'a>>, FoamError<'a>> {
    let mut result = Vec::new();
    loop {
        match lexer.next() {
            // Input ended before we got all values
            None => {
                // There is a compromise here: `dict { dict value; }` would make the first dict
                // being recognized and added to the list, but the end token (;) isn't used at the
                // end of dictionaries, so although we process everything, the parser will bail out
                // with an error.
                //
                // On the other hand, `var value` is an error ('cause attributions do require the
                // end token) but if we end here there will be no harm done, anyway.
                break;
            }
            Some(Err(_)) => return Err(FoamError::EndOfContent),

            // Tokens that are closing some structure that we are not processing anyway.
            Some(Ok(Token::ListEnd)) => {
                return Err(FoamError::UnexpectedKeyword {
                    token: ")",
                    structure,
                })
            }
            Some(Ok(Token::DictEnd)) => {
                return Err(FoamError::UnexpectedKeyword {
                    token: "}",
                    structure,
                })
            }

            // Tokens we are ignoring 'cause we don't keep them in our structures.
            Some(Ok(Token::MultilineComment(_))) => continue,
            Some(Ok(Token::Comment(_))) => continue,

            // Acceptable tokens
            Some(Ok(Token::Keyword(token))) => result.push(Foam::Value(token)),
            Some(Ok(Token::DictStart)) => unimplemented!(), // result.push(get_dict(lexer)?),
            Some(Ok(Token::ListStart)) => result.push(get_list(lexer)?),

            // Proper ending
            Some(Ok(Token::End)) => break,
        }
    }
    Ok(result)
}

fn get_dict<'a>(
    lexer: &mut logos::Lexer<'a, Token<'a>>,
) -> Result<Foam<'a>, FoamError<'a>> {
    let mut result = HashMap::new();
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
                return Err(FoamError::UnexpectedKeyword {
                    token: "}",
                    structure: "list",
                })
            }
            Some(Ok(Token::End)) => {
                return Err(FoamError::UnexpectedKeyword {
                    token: ";",
                    structure: "list",
                })
            }

            Some(Ok(Token::MultilineComment(_))) => continue,
            Some(Ok(Token::Comment(_))) => continue,

            Some(Ok(Token::Keyword(token))) => result.push(Foam::Value(token)),
            Some(Ok(Token::DictStart)) => unimplemented!(), // result.push(get_dict(lexer)?),
            Some(Ok(Token::ListStart)) => result.push(get_list(lexer)?),

            Some(Ok(Token::ListEnd)) => break,
        }
    }

    // Just one small note: Lists are followed by ';', which we need to consume to not confused the
    // parent caller. Also note that ';' is the only acceptable follower of the end of a list.
    match lexer.next() {
        Some(Ok(Token::End)) => (), // this is ok
        None => return Err(FoamError::EndOfContent),
        Some(Err(_)) => return Err(FoamError::EndOfContent),
        Some(Ok(_token)) => {
            return Err(FoamError::UnexpectedKeyword {
                token: "?",
                structure: "list",
            })
        }
    }

    Ok(Foam::List(result))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn keyword_direct() {
        let mut lexer = Token::lexer("token");
        let result = get_keyword(&mut lexer).unwrap();
        assert_eq!(result, Some("token"));
    }

    #[test]
    fn keyword_with_tabs() {
        let mut lexer = Token::lexer("\t\t\ttoken");
        let result = get_keyword(&mut lexer).unwrap();
        assert_eq!(result, Some("token"));
    }

    #[test]
    fn keyword_with_a_comment() {
        let mut lexer = Token::lexer("// this is a comment\ntoken");
        let result = get_keyword(&mut lexer).unwrap();
        assert_eq!(result, Some("token"));
    }

    #[test]
    fn keyword_with_multiline_comment() {
        let mut lexer =
            Token::lexer("/* this is comment\nmultiline, actually*/\ntoken");
        let result = get_keyword(&mut lexer).unwrap();
        assert_eq!(result, Some("token"));
    }

    #[test]
    fn keyword_quoted() {
        let mut lexer = Token::lexer("\"this is token\"");
        let result = get_keyword(&mut lexer).unwrap();
        assert_eq!(result, Some("this is token"));
    }

    #[test]
    fn keyword_empty_input() {
        let mut lexer = Token::lexer("");
        let result = get_keyword(&mut lexer);
        assert_eq!(result, Ok(None));
    }

    #[test]
    fn keyword_wrong_token() {
        let mut lexer = Token::lexer("{ inDict value; }");
        let result = get_keyword(&mut lexer);
        assert_eq!(
            result,
            Err(FoamError::MissingKeyword {
                token: "{",
                start: 0,
                end: 1
            })
        )
    }

    #[test]
    fn single_attribution() {
        let result = parse("variable value;");
        let map = HashMap::from([("variable", vec![Foam::Value("value")])]);
        assert_eq!(result, Ok(Foam::Dictionary(map)));
    }

    #[test]
    fn mutiple_attributions() {
        let result = parse("variable value1 value2 value3;");
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
        let result = parse("var1 value1;\nvar2 value2 value3;");
        let map = HashMap::from([
            ("var1", vec![Foam::Value("value1")]),
            ("var2", vec![Foam::Value("value2"), Foam::Value("value3")]),
        ]);
        assert_eq!(result, Ok(Foam::Dictionary(map)));
    }

    #[test]
    fn simple_list() {
        let result = parse("var (value1 value2);");
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
        let result = parse("var ( value1 ( inner2 ); );");
        let map = HashMap::from([(
            "var",
            vec![Foam::List(vec![
                Foam::Value("value1"),
                Foam::List(vec![Foam::Value("inner2")]),
            ])],
        )]);
        assert_eq!(result, Ok(Foam::Dictionary(map)));
    }
}
