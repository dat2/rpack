use combine::error::ParseError;
use combine::parser::char::{char, crlf, digit, hex_digit, newline, spaces, string};
use combine::parser::choice::{choice, optional};
use combine::parser::combinator::{not_followed_by, try};
use combine::parser::error::unexpected;
use combine::parser::item::{none_of, one_of, satisfy, token, value};
use combine::parser::repeat::{count, many, many1, sep_by, skip_until};
use combine::parser::sequence::between;
use combine::stream::state::State;
use combine::{eof, Parser, Stream};
use failure::{self, Error};
use javascript::ast::*;
use std::collections::HashSet;
use unicode_xid::UnicodeXID;

// https://www.ecma-international.org/ecma-262/8.0/index.html#sec-ecmascript-language-lexical-grammar

// https://www.ecma-international.org/ecma-262/8.0/index.html#sec-white-space
fn whitespace<I>() -> impl Parser<Input = I, Output = ()>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    spaces().map(|_| ())
}

// https://www.ecma-international.org/ecma-262/8.0/index.html#sec-line-terminators
// <LF> | <CR> | <LS> | <PS> | <CRLF>
fn line_terminator<I>() -> impl Parser<Input = I, Output = ()>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    newline()
        .or(char('\u{000D}'))
        .or(char('\u{2028}'))
        .or(char('\u{2029}'))
        .or(crlf())
        .map(|_| ())
}

// https://www.ecma-international.org/ecma-262/8.0/index.html#sec-comments
fn comment<I>() -> impl Parser<Input = I, Output = ()>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    try(block_comment()).or(line_comment())
}

fn block_comment<I>() -> impl Parser<Input = I, Output = ()>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (string("/*"), skip_until(try(string("*/"))), string("*/")).map(|_| ())
}

fn line_comment<I>() -> impl Parser<Input = I, Output = ()>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        string("//"),
        skip_until(line_terminator()),
        line_terminator(),
    ).map(|_| ())
}

fn skip_tokens<I>() -> impl Parser<Input = I, Output = ()>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    whitespace().or(comment())
}

// https://www.ecma-international.org/ecma-262/8.0/index.html#sec-names-and-keywords
fn satisfy_id_start(c: char) -> bool {
    UnicodeXID::is_xid_start(c) || c == '$' || c == '_'
}

fn id_start<I>() -> impl Parser<Input = I, Output = char>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    satisfy(satisfy_id_start)
}

fn unicode_id_start<I>() -> impl Parser<Input = I, Output = char>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    try(unicode_escape_sequence().map(|x| x.0).then(|c| {
        if satisfy_id_start(c) {
            value(c).left()
        } else {
            unexpected(c).map(|_| ' ').right()
        }
    })).or(id_start())
}

fn satisfy_id_continue(c: char) -> bool {
    // 200c = ZWNJ, 200d = ZWJ
    UnicodeXID::is_xid_continue(c) || c == '\u{200C}' || c == '\u{200D}' || c == '$' || c == '_'
}

fn id_continue<I>() -> impl Parser<Input = I, Output = char>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    satisfy(satisfy_id_continue)
}

fn unicode_id_continue<I>() -> impl Parser<Input = I, Output = char>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    try(unicode_escape_sequence().map(|x| x.0).then(|c| {
        if satisfy_id_continue(c) {
            value(c).left()
        } else {
            unexpected(c).map(|_| ' ').right()
        }
    })).or(id_continue())
}

// TODO strict mode
fn identifier<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (unicode_id_start(), many(unicode_id_continue()))
        .map(|(s, c): (char, String)| s.to_string() + &c)
        .then(|id| {
            if KEYWORDS.contains::<str>(&id)
                || FUTURE_RESERVED_WORDS.contains::<str>(&id)
                || FUTURE_RESERVED_WORDS_STRICT.contains::<str>(&id)
                || id == "null"
                || id == "true"
                || id == "false"
            {
                unexpected("reserved word")
                    .map(|_| String::new())
                    .message("reserved word")
                    .right()
            } else {
                value(id).left()
            }
        })
}

// https://www.ecma-international.org/ecma-262/8.0/index.html#sec-reserved-words
lazy_static! {
    static ref KEYWORDS: HashSet<&'static str> = {
        [
            "await",
            "break",
            "case",
            "catch",
            "class",
            "const",
            "continue",
            "debugger",
            "default",
            "delete",
            "do",
            "else",
            "export",
            "extends",
            "finally",
            "for",
            "function",
            "if",
            "import",
            "in",
            "instanceof",
            "new",
            "return",
            "super",
            "switch",
            "this",
            "throw",
            "try",
            "typeof",
            "var",
            "void",
            "while",
            "with",
            "yield",
        ].iter()
            .cloned()
            .collect()
    };
    static ref FUTURE_RESERVED_WORDS: HashSet<&'static str> =
        { ["enum"].iter().cloned().collect() };
    static ref FUTURE_RESERVED_WORDS_STRICT: HashSet<&'static str> = {
        [
            "implements",
            "package",
            "protected",
            "interface",
            "public",
            "private",
        ].iter()
            .cloned()
            .collect()
    };
}

#[cfg(test)]
mod lexical_tests {
    use super::*;

    #[test]
    fn test_line_comment() {
        assert_eq!(comment().parse("//\n"), Ok(((), "")));
        assert_eq!(comment().parse("// hello\n"), Ok(((), "")));
    }

    #[test]
    fn test_block_comment() {
        assert_eq!(comment().parse("/**/"), Ok(((), "")));
        assert_eq!(comment().parse("/* * */"), Ok(((), "")));
        assert_eq!(comment().parse("/** * **/"), Ok(((), "")));
        assert_eq!(comment().parse("/* hello *\n\t */"), Ok(((), "")));
    }

    #[test]
    fn test_identifier() {
        // making sure that the unicode_escape_sequence satisifies things
        // eg. ZWNJ and ZWJ are not allowed as starts
        assert!(identifier().parse(r"\u000a").is_err());
        assert!(identifier().parse(r"\u200d").is_err());
        assert!(identifier().parse(r"\u200c").is_err());
        // testing $, _, unicode_escape_sequence as start
        assert_eq!(identifier().parse(r"\u24"), Ok(("$".to_string(), "")));
        assert_eq!(identifier().parse(r"_"), Ok(("_".to_string(), "")));
        // testing $, _, ZWNJ, ZWJ, unicode_escape_sequence as continue
        assert_eq!(identifier().parse(r"a_"), Ok(("a_".to_string(), "")));
        assert_eq!(identifier().parse(r"a$"), Ok(("a$".to_string(), "")));
        assert_eq!(
            identifier().parse(r"_\u200d"),
            Ok(("_\u{200d}".to_string(), ""))
        );
        assert_eq!(
            identifier().parse(r"_\u200c"),
            Ok(("_\u{200c}".to_string(), ""))
        );
    }

    #[test]
    fn test_identifier_reserved_word() {
        for &keyword in KEYWORDS.iter() {
            assert!(identifier().parse(keyword).is_err());
        }
        for &keyword in FUTURE_RESERVED_WORDS.iter() {
            assert!(identifier().parse(keyword).is_err());
        }
        for &keyword in FUTURE_RESERVED_WORDS_STRICT.iter() {
            assert!(identifier().parse(keyword).is_err());
        }
        // null literal
        assert!(identifier().parse("null").is_err());
        // boolean literal
        assert!(identifier().parse("true").is_err());
        assert!(identifier().parse("false").is_err());
    }
}

// https://www.ecma-international.org/ecma-262/8.0/index.html#sec-null-literals
fn null_literal<I>() -> impl Parser<Input = I, Output = NullLiteral>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    string("null").map(|_| NullLiteral)
}

// https://www.ecma-international.org/ecma-262/8.0/index.html#sec-boolean-literals
fn boolean_literal<I>() -> impl Parser<Input = I, Output = BooleanLiteral>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(string("true")).map(|_| true),
        string("false").map(|_| false),
    ))
}

// https://www.ecma-international.org/ecma-262/8.0/index.html#sec-literals-numeric-literals
fn numeric_literal<I>() -> impl Parser<Input = I, Output = NumberLiteral>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(binary_integer_literal()),
        try(octal_integer_literal()),
        try(hex_integer_literal()),
        decimal_literal(),
    ))
}

fn decimal_literal<I>() -> impl Parser<Input = I, Output = NumberLiteral>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        optional(decimal_integer_literal()),
        optional(
            (token('.'), many::<String, _>(digit()))
                .map(|(c, s): (char, String)| c.to_string() + &s),
        ),
        optional(exponent_part()),
    ).map(|(literal_opt, digits_opt, exponent_opt)| {
            literal_opt.unwrap_or_else(String::new)
                + &digits_opt.unwrap_or_else(String::new)
                + &exponent_opt.unwrap_or_else(String::new)
        })
        .map(|s| s.parse::<f64>().unwrap())
}

fn decimal_integer_literal<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        string("0").skip(not_followed_by(digit())).map(String::from),
        (one_of("123456789".chars()), many::<String, _>(digit()))
            .map(|(c, s): (char, String)| c.to_string() + &s),
    ))
}

fn exponent_part<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        token('e').or(token('E')),
        optional(token('-').or(token('+'))),
        many1::<String, _>(digit()),
    ).map(
        |(e, sign_opt, digits): (char, Option<char>, String)| match sign_opt {
            Some(sign) => e.to_string() + &sign.to_string() + &digits,
            None => e.to_string() + &digits,
        },
    )
}

fn binary_integer_literal<I>() -> impl Parser<Input = I, Output = NumberLiteral>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        token('0'),
        token('b').or(token('B')),
        many1::<String, _>(one_of("01".chars())),
    ).map(|(_, _, digits)| i64::from_str_radix(&digits, 2).unwrap() as f64)
}

fn octal_integer_literal<I>() -> impl Parser<Input = I, Output = NumberLiteral>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        token('0'),
        token('o').or(token('O')),
        many1::<String, _>(one_of("01234567".chars())),
    ).map(|(_, _, digits)| i64::from_str_radix(&digits, 8).unwrap() as f64)
}

fn hex_integer_literal<I>() -> impl Parser<Input = I, Output = NumberLiteral>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        token('0'),
        token('x').or(token('X')),
        many1::<String, _>(hex_digit()),
    ).map(|(_, _, digits)| i64::from_str_radix(&digits, 16).unwrap() as f64)
}

// https://www.ecma-international.org/ecma-262/8.0/index.html#sec-literals-string-literals
fn string_literal<I>() -> impl Parser<Input = I, Output = StringLiteral>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    try(double_quote_string()).or(single_quote_string())
}

fn double_quote_string<I>() -> impl Parser<Input = I, Output = StringLiteral>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    between(
        token('"'),
        token('"'),
        many(double_quote_string_character()),
    )
}

fn double_quote_string_character<I>() -> impl Parser<Input = I, Output = char>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    // U+005C (REVERSE SOLIDUS), U+000D (CARRIAGE RETURN), U+2028 (LINE SEPARATOR), U+2029 (PARAGRAPH SEPARATOR), and U+000A (LINE FEED)
    escape_sequence().map(|x| x.0).or(none_of(
        "\u{005c}\u{000D}\u{2028}\u{2029}\u{000A}\"".chars(),
    ))
}

fn single_quote_string<I>() -> impl Parser<Input = I, Output = StringLiteral>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    between(
        token('\''),
        token('\''),
        many(single_quote_string_character()),
    )
}

fn single_quote_string_character<I>() -> impl Parser<Input = I, Output = char>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    // U+005C (REVERSE SOLIDUS), U+000D (CARRIAGE RETURN), U+2028 (LINE SEPARATOR), U+2029 (PARAGRAPH SEPARATOR), and U+000A (LINE FEED)
    escape_sequence()
        .map(|x| x.0)
        .or(none_of("\u{005c}\u{000D}\u{2028}\u{2029}\u{000A}'".chars()))
}

// (char, String) is "cooked" and "raw"
// this is for template elements, to be able to get access to the raw string
// this makes things uglier, but oh well
fn escape_sequence<I>() -> impl Parser<Input = I, Output = (char, String)>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(character_escape_sequence()),
        try(non_escape_character_sequence()),
        try(hex_escape_sequence()),
        // TODO legacy octal escape sequence
        unicode_escape_sequence(),
    ))
}

fn character_escape_sequence<I>() -> impl Parser<Input = I, Output = (char, String)>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    token('\\')
        .and(one_of(r#"'"\bfnrtv"#.chars()))
        .map(|(t, c)| {
            let cooked = match c {
                'b' => '\u{8}',
                'f' => '\u{C}',
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                'v' => '\u{B}',
                other => other,
            };
            (cooked, format!("{}{}", t, c,))
        })
}

fn non_escape_character_sequence<I>() -> impl Parser<Input = I, Output = (char, String)>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    token('\\')
        .and(none_of(
            "'\"\\bfnrtv0123456789xu\r\n\u{2028}\u{2029}".chars(),
        ))
        .map(|(t, c)| (c, format!("{}{}", t, c)))
}

fn hex_escape_sequence<I>() -> impl Parser<Input = I, Output = (char, String)>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (token('\\'), token('x'), count::<String, _>(2, hex_digit())).map(|(t, x, hex_digits)| {
        let code_point = u32::from_str_radix(&hex_digits, 16).unwrap();
        let cooked = ::std::char::from_u32(code_point).unwrap();
        (cooked, format!("{}{}{}", t, x, hex_digits))
    })
}

// https://www.ecma-international.org/ecma-262/8.0/index.html#prod-UnicodeEscapeSequence
fn unicode_escape_sequence<I>() -> impl Parser<Input = I, Output = (char, String)>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        token('\\'),
        token('u'),
        choice((
            (token('{'), count::<String, _>(6, hex_digit()), token('}')).map(
                |(s, digits, e): (char, String, char)| s.to_string() + &digits + &e.to_string(),
            ),
            count::<String, _>(4, hex_digit()),
        )),
    ).then(|(t, u, digits_raw)| {
        let digits_cooked = if &digits_raw[0..1] == "{" {
            &digits_raw[1..digits_raw.len() - 1]
        } else {
            &digits_raw[..]
        };
        let code_point = u32::from_str_radix(digits_cooked, 16).unwrap();
        if code_point > 0x0010_FFFF {
            unexpected("code point")
                .map(|_| (' ', String::new()))
                .message("Code point too large")
                .right()
        } else {
            let cooked = ::std::char::from_u32(code_point).unwrap();
            let raw = format!("{}{}{}", t, u, digits_raw);
            value((cooked, raw)).left()
        }
    })
}

// https://www.ecma-international.org/ecma-262/8.0/index.html#sec-literals-regular-expression-literals
fn regex_literal<I>() -> impl Parser<Input = I, Output = RegexLiteral>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        between(token('/'), token('/'), regex_body()),
        many::<String, _>(id_continue()),
    ).map(|(pattern, flags)| RegexLiteral { pattern, flags })
}

fn regex_body<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (regex_first_char(), many::<String, _>(regex_char())).map(|(s, s2): (String, String)| s + &s2)
}

fn regex_first_char<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    try(regex_backslash_sequence())
        .or(try(regex_class()))
        .or(none_of("*/\\[\n\r\u{2028}\u{2029}".chars()).map(|c: char| c.to_string()))
}

fn regex_char<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    try(regex_backslash_sequence())
        .or(try(regex_class()))
        .or(none_of("/\\[\n\r\u{2028}\u{2029}".chars()).map(|c: char| c.to_string()))
}

fn regex_non_terminator<I>() -> impl Parser<Input = I, Output = char>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    none_of("\n\r\u{2028}\u{2029}".chars())
}

fn regex_backslash_sequence<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (token('\\'), regex_non_terminator()).map(|(c, s): (char, char)| c.to_string() + &s.to_string())
}

fn regex_class<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        token('['),
        many::<String, _>(
            try(regex_backslash_sequence()).or(none_of("]\\".chars()).map(|c: char| c.to_string())),
        ),
        token(']'),
    ).map(|(open, middle, end): (char, String, char)| {
        open.to_string() + &middle + &end.to_string()
    })
}

// https://www.ecma-international.org/ecma-262/8.0/index.html#sec-template-literal-lexical-components
fn template<I>() -> impl Parser<Input = I, Output = TemplateElement>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((try(no_substition_template()), template_head()))
}

fn no_substition_template<I>() -> impl Parser<Input = I, Output = TemplateElement>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    between(
        token('`'),
        token('`'),
        many::<Vec<_>, _>(template_character()),
    ).map(|pairs| {
        let cooked = pairs.iter().cloned().map(|x| x.0).collect();
        let raw = pairs.iter().cloned().map(|x| x.1).collect();
        TemplateElement { cooked, raw }
    })
}

fn template_head<I>() -> impl Parser<Input = I, Output = TemplateElement>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    between(
        token('`'),
        string("${"),
        many::<Vec<_>, _>(template_character()),
    ).map(|pairs| {
        let cooked = pairs.iter().cloned().map(|x| x.0).collect();
        let raw = pairs.iter().cloned().map(|x| x.1).collect();
        TemplateElement { cooked, raw }
    })
}

fn template_character<I>() -> impl Parser<Input = I, Output = (char, String)>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(token('$').skip(not_followed_by(token('{')))).map(|x: char| (x, x.to_string())),
        try(escape_sequence()),
        try(one_of("\r\n\u{2028}\u{2029}".chars())).map(|x: char| (x, x.to_string())),
        none_of("`\\$".chars()).map(|x: char| (x, x.to_string())),
    ))
}

fn template_substition_tail<I>() -> impl Parser<Input = I, Output = TemplateElement>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((try(template_middle()), template_tail()))
}

fn template_middle<I>() -> impl Parser<Input = I, Output = TemplateElement>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    between(
        token('}'),
        string("${"),
        many::<Vec<_>, _>(template_character()),
    ).map(|pairs| {
        let cooked = pairs.iter().cloned().map(|x| x.0).collect();
        let raw = pairs.iter().cloned().map(|x| x.1).collect();
        TemplateElement { cooked, raw }
    })
}

fn template_tail<I>() -> impl Parser<Input = I, Output = TemplateElement>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    token('}')
        .with(many::<Vec<_>, _>(template_character()))
        .map(|pairs| {
            let cooked = pairs.iter().cloned().map(|x| x.0).collect();
            let raw = pairs.iter().cloned().map(|x| x.1).collect();
            TemplateElement { cooked, raw }
        })
}

#[cfg(test)]
mod literal_tests {
    use super::*;

    #[test]
    fn test_null_literal() {
        assert_eq!(null_literal().parse("null"), Ok((NullLiteral, "")));
    }

    #[test]
    fn test_boolean_literal() {
        assert_eq!(boolean_literal().parse("true"), Ok((true, "")));
        assert_eq!(boolean_literal().parse("false"), Ok((false, "")));
    }

    #[test]
    fn test_number_literal() {
        // decimal
        assert_eq!(numeric_literal().parse("0"), Ok((0f64, "")));
        assert!(numeric_literal().parse("01").is_err());
        assert!(numeric_literal().parse("01.").is_err());
        assert_eq!(numeric_literal().parse("9"), Ok((9f64, "")));
        assert_eq!(numeric_literal().parse("10"), Ok((10f64, "")));
        assert_eq!(numeric_literal().parse("0.1"), Ok((0.1f64, "")));
        assert_eq!(numeric_literal().parse(".1"), Ok((0.1f64, "")));
        assert_eq!(numeric_literal().parse("1e1"), Ok((10f64, "")));
        assert_eq!(numeric_literal().parse(".1e1"), Ok((1f64, "")));
        assert_eq!(numeric_literal().parse("1.1e1"), Ok((11f64, "")));

        // binary
        assert_eq!(numeric_literal().parse("0b1010"), Ok((10f64, "")));
        assert_eq!(numeric_literal().parse("0B1010"), Ok((10f64, "")));
        // octal
        assert_eq!(numeric_literal().parse("0o123"), Ok((83f64, "")));
        assert_eq!(numeric_literal().parse("0O123"), Ok((83f64, "")));
        // hex
        assert_eq!(
            numeric_literal().parse("0xDEADBEEF"),
            Ok((3735928559f64, ""))
        );
        assert_eq!(
            numeric_literal().parse("0XDEADBEEF"),
            Ok((3735928559f64, ""))
        );
    }

    #[test]
    fn test_string_literal() {
        // empty
        assert_eq!(string_literal().parse(r#""""#), Ok((String::new(), "")));
        assert_eq!(string_literal().parse("''"), Ok((String::new(), "")));
        // not allowed chars
        for not_allowed_char in "\u{005c}\u{000D}\u{2028}\u{2029}\u{000A}".chars() {
            let double_quote_slice: &str = &format!("\"{}\"", not_allowed_char);
            let single_quote_slice: &str = &format!("'{}'", not_allowed_char);
            assert!(string_literal().parse(double_quote_slice).is_err());
            assert!(string_literal().parse(single_quote_slice).is_err());
        }
        // character escape sequences
        for escaped_character in r#"'"\bfnrtv"#.chars() {
            let double_quote_slice: &str = &format!("\"\\{}\"", escaped_character);
            let single_quote_slice: &str = &format!("'\\{}'", escaped_character);
            assert!(string_literal().parse(double_quote_slice).is_ok());
            assert!(string_literal().parse(single_quote_slice).is_ok());
        }
        // non character escape sequences
        assert_eq!(string_literal().parse("\"\\a\""), Ok(("a".to_string(), "")));
        assert_eq!(string_literal().parse("'\\a'"), Ok(("a".to_string(), "")));

        // hex escape sequence
        assert_eq!(
            string_literal().parse(r#""\x0A""#),
            Ok(("\n".to_string(), ""))
        );
        assert_eq!(
            string_literal().parse(r"'\x0a'"),
            Ok(("\n".to_string(), ""))
        );
        // unicode escape sequence
        assert_eq!(
            string_literal().parse(r#""\u2764""#),
            Ok(("❤".to_string(), ""))
        );
        assert_eq!(
            string_literal().parse(r"'\u2764'"),
            Ok(("❤".to_string(), ""))
        );
        assert_eq!(
            string_literal().parse(r#""\u{2764}""#),
            Ok(("❤".to_string(), ""))
        );
        assert_eq!(
            string_literal().parse(r"'\u{2764}'"),
            Ok(("❤".to_string(), ""))
        );
        assert!(string_literal().parse(r"'\u{110000}'").is_err());

        // line continuation
        for line_continuation_char in "\r\n\u{2028}\u{2029}".chars() {
            let double_quote_slice: &str = &format!("\"\\{}\"", line_continuation_char);
            let single_quote_slice: &str = &format!("'\\{}'", line_continuation_char);
            assert!(string_literal().parse(double_quote_slice).is_err());
            assert!(string_literal().parse(single_quote_slice).is_err());
        }
    }

    #[test]
    fn test_regex_literal() {
        // must be non empty
        assert!(string_literal().parse("//").is_err());

        // not allowed first chars
        for c in "*\\/[".chars() {
            let slice: &str = &format!("/{}/", c);
            assert!(string_literal().parse(slice).is_err());
        }

        // backslash as first char
        assert_eq!(
            regex_literal().parse("/\\a/"),
            Ok((
                RegexLiteral {
                    pattern: "\\a".to_string(),
                    flags: String::new()
                },
                ""
            ))
        );

        // character class as first char
        assert_eq!(
            regex_literal().parse("/[ab]/"),
            Ok((
                RegexLiteral {
                    pattern: "[ab]".to_string(),
                    flags: String::new()
                },
                ""
            ))
        );

        // not allowed second chars
        for c in "\\/[".chars() {
            let slice: &str = &format!("/a{}/", c);
            assert!(string_literal().parse(slice).is_err());
        }

        // backslash as second char
        assert_eq!(
            regex_literal().parse("/a\\a/"),
            Ok((
                RegexLiteral {
                    pattern: "a\\a".to_string(),
                    flags: String::new()
                },
                ""
            ))
        );

        // character class as second char
        assert_eq!(
            regex_literal().parse("/a[ab]/"),
            Ok((
                RegexLiteral {
                    pattern: "a[ab]".to_string(),
                    flags: String::new()
                },
                ""
            ))
        );

        // character class with unallowed chars
        for c in "\\/]".chars() {
            let slice: &str = &format!("/a[{}]/", c);
            assert!(string_literal().parse(slice).is_err());
        }

        // character class with backslash
        assert_eq!(
            regex_literal().parse("/a[ab\\]]/"),
            Ok((
                RegexLiteral {
                    pattern: "a[ab\\]]".to_string(),
                    flags: String::new()
                },
                ""
            ))
        );

        // flags
        assert_eq!(
            regex_literal().parse("/a/f"),
            Ok((
                RegexLiteral {
                    pattern: "a".to_string(),
                    flags: "f".to_string()
                },
                ""
            ))
        );
        assert_eq!(
            regex_literal().parse("/a/fi"),
            Ok((
                RegexLiteral {
                    pattern: "a".to_string(),
                    flags: "fi".to_string()
                },
                ""
            ))
        );
        assert!(regex_literal().skip(eof()).parse("/a/\\u1234").is_err());
    }

    #[test]
    fn test_template_elements() {
        // empty
        assert_eq!(
            template().parse("``"),
            Ok((
                TemplateElement {
                    cooked: "".to_string(),
                    raw: "".to_string(),
                },
                ""
            ))
        );

        // no_substitution_template
        assert_eq!(
            template().parse("`asd`"),
            Ok((
                TemplateElement {
                    cooked: "asd".to_string(),
                    raw: "asd".to_string(),
                },
                ""
            ))
        );

        // template_head
        assert_eq!(
            template().parse("`asd ${eval}`"),
            Ok((
                TemplateElement {
                    cooked: "asd ".to_string(),
                    raw: "asd ".to_string()
                },
                "eval}`"
            ))
        );

        // template_middle
        assert_eq!(
            template_substition_tail().parse("} asd ${eval}`"),
            Ok((
                TemplateElement {
                    cooked: " asd ".to_string(),
                    raw: " asd ".to_string()
                },
                "eval}`"
            ))
        );

        // template_tail
        assert_eq!(
            template_substition_tail().parse("} asd"),
            Ok((
                TemplateElement {
                    cooked: " asd".to_string(),
                    raw: " asd".to_string()
                },
                ""
            ))
        );

        // $
        assert_eq!(
            template_character().parse("$123"),
            Ok((('$', "$".to_string()), "123"))
        );
        // escape sequence
        assert_eq!(
            template_character().parse("\\n"),
            Ok((('\n', "\\n".to_string()), ""))
        );
        assert_eq!(
            template_character().parse("\\x0A"),
            Ok((('\n', "\\x0A".to_string()), ""))
        );
        assert_eq!(
            template_character().parse("\\u2764"),
            Ok((('❤', "\\u2764".to_string()), ""))
        );
        assert_eq!(
            template_character().parse("\\u{2764}"),
            Ok((('❤', "\\u{2764}".to_string()), ""))
        );
        // line continuation
        for line_continuation_char in "\r\n\u{2028}\u{2029}".chars() {
            let slice: &str = &line_continuation_char.to_string();
            assert_eq!(
                template_character().parse(slice),
                Ok((
                    (line_continuation_char, line_continuation_char.to_string()),
                    ""
                ))
            );
        }
    }
}

// https://www.ecma-international.org/ecma-262/8.0/index.html#sec-ecmascript-language-expressions

// https://www.ecma-international.org/ecma-262/8.0/index.html#sec-ecmascript-language-statements-and-declarations

// https://www.ecma-international.org/ecma-262/8.0/index.html#sec-ecmascript-language-functions-and-classes

// https://www.ecma-international.org/ecma-262/8.0/index.html#sec-ecmascript-language-scripts-and-modules

// https://facebook.github.io/jsx/

// statements
fn import_statement<I>() -> impl Parser<Input = I, Output = Statement>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        string("import"),
        skip_tokens(),
        identifier(),
        skip_tokens(),
        string("from"),
        skip_tokens(),
        string_literal(),
        skip_tokens(),
    ).map(|(_, _, id, _, _, _, string_lit, _)| {
        Statement::Import(ImportSpecifier::ImportDefault(id), string_lit)
    })
}

fn function_declaration<I>() -> impl Parser<Input = I, Output = Statement>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        string("function"),
        skip_tokens(),
        identifier(),
        skip_tokens(),
        between(
            token('(').skip(skip_tokens()),
            token(')').skip(skip_tokens()),
            sep_by::<Vec<_>, _, _>(identifier(), token(',').skip(skip_tokens())),
        ),
        between(
            token('{').skip(skip_tokens()),
            token('}'),
            many::<Vec<_>, _>(statement()),
        ),
        skip_tokens(),
    ).map(
        |(_, _, id, _, params, body, _)| Statement::FunctionDeclaration {
            declaration: FunctionDeclaration {
                id: id.clone(),
                function: Function {
                    id: Some(id),
                    params: params
                        .iter()
                        .map(|id| Pattern::Id { id: id.clone() })
                        .collect(),
                    body: body
                        .iter()
                        .map(|stmt| FunctionBodyStatement::Statement(stmt.clone()))
                        .collect(),
                    generator: false,
                },
            },
        },
    )
}

fn var_declaration_kind<I>() -> impl Parser<Input = I, Output = VariableDeclarationKind>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(string("var")).map(|_| VariableDeclarationKind::Var),
        try(string("let")).map(|_| VariableDeclarationKind::Let),
        string("const").map(|_| VariableDeclarationKind::Const),
    ))
}

fn var_declaration<I>() -> impl Parser<Input = I, Output = Statement>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        var_declaration_kind(),
        skip_tokens(),
        identifier(),
        skip_tokens(),
    ).map(|(kind, _, id, _)| Statement::VariableDeclaration {
        declaration: VariableDeclaration {
            kind,
            declarations: vec![VariableDeclarator {
                id: Pattern::Id { id: id.clone() },
                init: None,
            }],
        },
    })
}

fn statement<I>() -> impl Parser<Input = I, Output = Statement>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(import_statement()),
        try(function_declaration()),
        var_declaration(),
    ))
}

fn program<I>() -> impl Parser<Input = I, Output = Program>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    many(statement()).skip(eof()).map(|body| Program {
        source_type: SourceType::Module,
        body,
    })
}

pub fn parse(source: &str) -> Result<Program, Error> {
    let stream = State::new(source);
    let (ast, _) = program()
        .easy_parse(stream)
        .map_err(|e| failure::err_msg(e.to_string()))?;
    Ok(ast)
}
