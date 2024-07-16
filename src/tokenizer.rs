use logos::Logos;

#[allow(dead_code)]
#[derive(Logos, Debug)]
#[logos(skip r"[ \t\n\r]")]
pub(crate) enum Token<'a> {
    #[regex(r#"\/\*[^\/\*]*\*\/"#, |lex| lex.slice())]
    MultilineComment(&'a str),

    #[regex(r#""[^"]+""#, |lex| lex.slice().trim_start_matches('"').trim_end_matches('"'))]
    #[regex("[a-zA-Z0-9_]+", |lex| lex.slice())]
    Keyword(&'a str),

    #[regex(r#"//[^\n]*"#, |lex| lex.slice())]
    Comment(&'a str),

    #[token(";")]
    End,

    #[token("{")]
    DictStart,

    #[token("}")]
    DictEnd,

    #[token("(")]
    ListStart,

    #[token(")")]
    ListEnd,
}
