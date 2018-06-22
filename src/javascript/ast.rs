#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>
}

pub type Id = String;

pub type StringLiteral = String;

#[derive(Debug)]
pub enum Statement {
    Import(Id, StringLiteral),
}
