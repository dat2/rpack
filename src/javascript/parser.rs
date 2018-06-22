use combine::error::ParseError;
use combine::parser::char::{char, letter, spaces, string, tab};
use combine::parser::error::unexpected;
use combine::parser::item::{none_of, token, value};
use combine::parser::repeat::{many, skip_many};
use combine::parser::sequence::between;
use combine::stream::state::State;
use combine::{eof, Parser, Stream};
use failure::{self, Error};
use javascript::ast::*;

fn ws<I>() -> impl Parser<Input = I, Output = ()>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    skip_many(char(' ').or(tab()))
}

fn eol<I>() -> impl Parser<Input = I, Output = ()>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    spaces()
}

fn reserved<I>(word: &'static str) -> impl Parser<Input = I, Output = &'static str>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    string(word).skip(ws())
}

fn id<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    many(letter())
        .map(|vec: Vec<char>| vec.into_iter().collect())
        .then(|s: String| match s.as_ref() {
            "import" | "from" => unexpected("reserved word").map(|_| String::new()).right(),
            _ => value(s).left(),
        })
        .skip(ws())
}

fn string_literal<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    between(token('\''), token('\''), many(none_of("'".chars())))
}

fn import_statement<I>() -> impl Parser<Input = I, Output = Statement>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        reserved("import"),
        id(),
        reserved("from"),
        string_literal(),
        eol(),
    ).map(|(_, id, _, string_lit, _)| Statement::Import(id, string_lit))
}

fn program<I>() -> impl Parser<Input = I, Output = Program>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    many(import_statement())
        .skip(eof())
        .map(|statements| Program {
            statements: statements,
        })
}

pub fn parse(source: &str) -> Result<Program, Error> {
    let stream = State::new(source);
    let (ast, _) = program()
        .easy_parse(stream)
        .map_err(|e| failure::err_msg(e.to_string()))?;
    Ok(ast)
}
