use std::collections::HashMap;

// TODO source tracking
#[derive(Debug, Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

// identifiers
pub type Id = String;

// literals
pub type StringLiteral = String;

#[derive(Debug, Clone)]
pub enum BooleanLiteral {
    True,
    False,
}

#[derive(Debug, Clone)]
pub struct NullLiteral;

pub type NumberLiteral = f64;

#[derive(Debug, Clone)]
pub struct RegexLiteral {
    pub pattern: String,
    pub flags: String,
}

#[derive(Debug, Clone)]
pub enum Literal {
    StringLiteral(StringLiteral),
    BooleanLiteral(BooleanLiteral),
    NullLiteral(NullLiteral),
    NumberLiteral(NumberLiteral),
    RegexLiteral(RegexLiteral),
}

// program
#[derive(Debug, Clone)]
pub struct Program {
    pub source_type: SourceType,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum SourceType {
    Script,
    Module,
}

// functions
#[derive(Debug, Clone)]
pub struct Function {
    pub id: Option<Id>,
    pub params: Vec<Pattern>,
    pub body: FunctionBody,
    // ES2015
    pub generator: bool,
}

#[derive(Debug, Clone)]
pub struct Directive {
    pub expression: Literal,
    pub directive: String,
}

pub type BlockStatement = Vec<Statement>;

#[derive(Debug, Clone)]
pub enum FunctionBodyStatement {
    Statement(Statement),
    Directive(Directive),
}

pub type FunctionBody = Vec<FunctionBodyStatement>;

// statements
#[derive(Debug, Clone)]
pub enum Statement {
    // ES5
    Expression {
        expression: Expression,
    },
    Block {
        body: BlockStatement,
    },
    Empty,
    Debugger,
    With {
        object: Expression,
        body: Box<Statement>,
    },
    Return {
        argument: Option<Expression>,
    },
    Label {
        label: Id,
        body: Box<Statement>,
    },
    Break {
        label: Option<Id>,
    },
    Continue {
        label: Option<Id>,
    },
    If {
        test: Expression,
        consequent: Box<Statement>,
        alternate: Option<Box<Statement>>,
    },
    Switch {
        discriminant: Expression,
        cases: Vec<SwitchCase>,
    },
    Throw {
        argument: Expression,
    },
    Try {
        block: BlockStatement,
        handler: Option<CatchClause>,
        finalizer: Option<BlockStatement>,
    },
    While {
        test: Expression,
        body: Box<Statement>,
    },
    DoWhile {
        body: Box<Statement>,
        test: Expression,
    },
    For {
        init: Option<ForInit>,
        test: Option<Expression>,
        update: Option<Expression>,
        body: Box<Statement>,
    },
    // For In and For Of (ES2015)
    ForInOf {
        left: ForInLeft,
        right: Expression,
        body: Box<Statement>,
    },
    FunctionDeclaration {
        declaration: FunctionDeclaration,
    },
    VariableDeclaration {
        declaration: VariableDeclaration,
    },
    // ES2015
    Import(ImportSpecifier, StringLiteral),
}

#[derive(Debug, Clone)]
pub struct SwitchCase {
    // default has no test
    test: Option<Expression>,
    consequent: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct CatchClause {
    param: Pattern,
    body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum ForInit {
    VariableDeclaration(VariableDeclaration),
    Expression(Expression),
}

#[derive(Debug, Clone)]
pub enum ForInLeft {
    VariableDeclaration(VariableDeclaration),
    Pattern(Pattern),
}

#[derive(Debug, Clone)]
pub enum ImportSpecifier {
    // import {foo, bar, baz as qux} from './mod'
    Import(HashMap<Id, Id>),
    // import foo from './mod'
    ImportDefault(Id),
    // import * as foo from './mod'
    ImportNamespaced(Id),
}

// declarations
#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    id: Id,
    function: Function,
}

#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub kind: VariableDeclarationKind,
    pub declarations: Vec<VariableDeclarator>,
}

#[derive(Debug, Clone)]
pub enum VariableDeclarationKind {
    Var,
    // ES2015
    Let,
    Const,
}

#[derive(Debug, Clone)]
pub struct VariableDeclarator {
    pub id: Pattern,
    pub init: Option<Expression>,
}

// expressions
#[derive(Debug, Clone)]
pub enum Expression {
    Id {
        id: Id,
    },
    Literal {
        literal: Literal,
    },
    This,
    // ES2015
    Super,
    Array {
        // ES2015: Can be spread
        elements: Vec<Expression>,
    },
    Object {
        properties: Vec<Property>,
    },
    Function {
        function: Function,
    },
    Unary {
        operator: UnaryOperator,
        prefix: bool,
        argument: Box<Expression>,
    },
    // ++ or --
    Update {
        operator: UpdateOperator,
        argument: Box<Expression>,
        prefix: bool,
    },
    // also includes LogicalExpression
    Binary {
        operator: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Assignment {
        operator: AssignmentOperator,
        left: Box<AssignmentLeft>,
        right: Box<Expression>,
    },
    Member {
        // ES2015, object can be super
        object: Box<Expression>,
        property: Box<Expression>,
        computed: bool,
    },
    // Ternary
    Conditional {
        test: Box<Expression>,
        alternate: Box<Expression>,
        consequent: Box<Expression>,
    },
    Call {
        // ES2015, callee can be super
        callee: Box<Expression>,
        // ES2015, arguments can be spread
        arguments: Vec<Expression>,
    },
    New {
        callee: Box<Expression>,
        // ES2015: arguments can be spread
        arguments: Vec<Expression>,
    },
    // Comma separated expressions
    Sequence {
        expressions: Vec<Expression>,
    },
    // ES2015
    Spread {
        expression: Box<Expression>,
    },
    // ES2015
    ArrowFunction {
        body: Box<ArrowFunctionBody>,
        expression: bool,
    },
    // ES2015
    Yield {
        argument: Option<Box<Expression>>,
        delegate: bool,
    },
    // ES2015
    TemplateLiteral {
        quasis: Vec<TemplateElement>,
        expressions: Vec<Expression>,
    },
    TaggedTemplate {
        tag: Box<Expression>,
        // quasi can only be a TemplateLiteral
        quasi: Box<Expression>,
    },
}

#[derive(Debug, Clone)]
pub struct Property {
    pub key: PropertyKey,
    pub value: Expression,
    pub kind: PropertyKind,
}

#[derive(Debug, Clone)]
pub enum PropertyKey {
    Literal(Literal),
    Id(Id),
    // ES2015
    Expression(Expression),
}

#[derive(Debug, Clone)]
pub enum PropertyKind {
    Init,
    Get,
    Set,
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    // -
    Minus,
    // +
    Plus,
    // !
    Not,
    // ~
    BitwiseNot,
    Typeof,
    Void,
    Delete,
}

#[derive(Debug, Clone)]
pub enum UpdateOperator {
    // ++
    Increment,
    // --
    Decrement,
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    // ==
    EqEq,
    // !=
    NotEq,
    // ===
    EqEqEq,
    // !==
    NotEqEq,
    // <
    Lt,
    // <=
    Lte,
    // >
    Gt,
    // >=
    Gte,
    // <<
    Shl,
    // >>
    Shr,
    // >>>
    UnsignedShr,
    // +
    Plus,
    // -
    Minus,
    // *
    Multiply,
    // /
    Divide,
    // %
    Mod,
    // |
    BitwiseOr,
    // ||
    Or,
    // ^
    BitwiseXor,
    // &
    BitwiseAnd,
    // &&
    And,
    In,
    InstanceOf,
}

#[derive(Debug, Clone)]
pub enum AssignmentOperator {
    // =
    Eq,
    // +=
    PlusEq,
    // -=
    MinusEq,
    // *=
    MultiplyEq,
    // /=
    DivideEq,
    // %=
    ModEq,
    // <<=
    ShlEq,
    // >>=
    ShrEq,
    // >>>=
    UnsignedShrEq,
    // |=
    BitwiseOrEq,
    // ^=
    BitwiseXorEq,
    // &=
    BitwiseAndEq,
}

#[derive(Debug, Clone)]
pub enum AssignmentLeft {
    Pattern(Pattern),
    Expression(Expression),
}

#[derive(Debug, Clone)]
pub enum ArrowFunctionBody {
    FunctionBody(FunctionBody),
    Expression(Expression),
}

#[derive(Debug, Clone)]
pub struct TemplateElement {
    tail: bool,
    value_cooked: String,
    value_raw: String,
}

// patterns
#[derive(Debug, Clone)]
pub enum Pattern {
    Id {
        id: Id,
    },
    Object {
        properties: Vec<Pattern>,
    },
    Array {
        elements: Vec<Pattern>,
    },
    Rest,
    Assignment {
        left: Box<Pattern>,
        right: Expression,
    },
}

// visitors
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
