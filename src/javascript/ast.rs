use std::collections::HashMap;

// TODO source tracking
#[derive(Debug)]
pub struct Position {
    line: usize,
    column: usize,
}

// identifiers
pub type Id = String;

// literals
pub type StringLiteral = String;

#[derive(Debug)]
pub enum BooleanLiteral {
    True,
    False,
}

#[derive(Debug)]
pub struct NullLiteral;

pub type NumberLiteral = f64;

#[derive(Debug)]
pub struct RegexLiteral {
    pattern: String,
    flags: String,
}

#[derive(Debug)]
pub enum Literal {
    StringLiteral(StringLiteral),
    BooleanLiteral(BooleanLiteral),
    NullLiteral(NullLiteral),
    NumberLiteral(NumberLiteral),
    RegexLiteral(RegexLiteral),
}

// program
#[derive(Debug)]
pub struct Program {
    pub source_type: SourceType,
    pub body: Vec<Statement>,
}

#[derive(Debug)]
pub enum SourceType {
    Script,
    Module,
}

#[derive(Debug)]
pub enum Statement {
    Import(ImportSpecifier, StringLiteral),
}

#[derive(Debug)]
pub enum ImportSpecifier {
    // import {foo, bar, baz as qux} from './mod'
    Import(HashMap<Id, Id>),
    // import foo from './mod'
    ImportDefault(Id),
    // import * as foo from './mod'
    ImportNamespaced(Id),
}

// Visitor stuff
pub trait Data {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Result;
}

pub trait Visitor {
    type Result;

    fn visit_program(&mut self, program: &Program) -> Self::Result;
    fn visit_statement(&mut self, statement: &Statement) -> Self::Result;
}

impl Data for Program {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Result {
        visitor.visit_program(self)
    }
}

impl Data for Statement {
    fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Result {
        visitor.visit_statement(self)
    }
}
