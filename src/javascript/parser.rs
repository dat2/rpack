use combine::error::ParseError;
use combine::parser::char::{char, letter, spaces, string, tab};
use combine::parser::choice::choice;
use combine::parser::combinator::try;
use combine::parser::error::unexpected;
use combine::parser::item::{none_of, token, value};
use combine::parser::repeat::{many, sep_by, skip_many};
use combine::parser::sequence::between;
use combine::stream::state::State;
use combine::{eof, Parser, Stream};
use failure::{self, Error};
use javascript::ast::*;

// whitespace utils
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

// TODO comments

// lexer things
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
    many(letter().or(token('_')))
        .map(|vec: Vec<char>| vec.into_iter().collect())
        .then(|s: String| match s.as_ref() {
            "function" | "import" | "from" => {
                unexpected("reserved word").map(|_| String::new()).right()
            }
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

// statements
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
    ).map(|(_, id, _, string_lit, _)| {
        Statement::Import(ImportSpecifier::ImportDefault(id), string_lit)
    })
}

fn function_declaration<I>() -> impl Parser<Input = I, Output = Statement>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        reserved("function"),
        id(),
        between(
            token('(').skip(ws()),
            token(')').skip(ws()),
            sep_by::<Vec<_>, _, _>(id(), token(',').skip(ws())),
        ),
        between(
            token('{').skip(eol()),
            token('}'),
            many::<Vec<_>, _>(import_statement()),
        ),
        eol(),
    ).map(|(_, id, params, body, _)| Statement::FunctionDeclaration {
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
    })
}

fn statement<I>() -> impl Parser<Input = I, Output = Statement>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((try(import_statement()), function_declaration()))
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
