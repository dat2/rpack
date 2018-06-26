use std::collections::HashMap;

// https://www.ecma-international.org/ecma-262/8.0/index.html#sec-identifier-names
pub type Id = String;

// https://www.ecma-international.org/ecma-262/8.0/index.html#sec-ecmascript-language-lexical-grammar-literals
pub type StringLiteral = String;

pub type BooleanLiteral = bool;

#[derive(Debug, Clone, PartialEq)]
pub struct NullLiteral;

pub type NumberLiteral = f64;

#[derive(Debug, Clone, PartialEq)]
pub struct RegexLiteral {
    pub pattern: String,
    pub flags: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TemplateElement {
    pub cooked: String,
    pub raw: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    NullLiteral(NullLiteral),
    BooleanLiteral(BooleanLiteral),
    NumberLiteral(NumberLiteral),
    StringLiteral(StringLiteral),
}

// https://www.ecma-international.org/ecma-262/8.0/index.html#sec-ecmascript-language-expressions
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    // https://www.ecma-international.org/ecma-262/8.0/index.html#sec-primary-expression
    This,
    IdReference {
        id: Id,
    },
    Literal {
        literal: Literal,
    },
    ArrayLiteral {
        // ES2015: Can be spread
        elements: Vec<Expression>,
    },
    ObjectLiteral {
        properties: Vec<Property>,
    },
    Function {
        id: Option<Id>,
        params: Vec<Pattern>,
        body: Vec<Statement>,
        async: bool,
        generator: bool,
    },
    Class,
    RegexLiteral {
        regex: RegexLiteral,
    },
    TemplateLiteral {
        quasis: Vec<TemplateElement>,
        expressions: Vec<Expression>,
    },
    Spread {
        expression: Box<Expression>,
    },
    // https://www.ecma-international.org/ecma-262/8.0/index.html#sec-left-hand-side-expressions
    Member {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
        computed: bool, // lhs[rhs]
    },
    Super,
    MetaProperty,
    New {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
    },
    Call {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
    },
    TaggedTemplate {
        tag: Box<Expression>,
        // quasi can only be a TemplateLiteral
        quasi: Box<Expression>,
    },
    // https://www.ecma-international.org/ecma-262/8.0/index.html#sec-update-expressions
    // ++ or --
    Update {
        operator: UpdateOperator,
        argument: Box<Expression>,
        prefix: bool,
    },
    // https://www.ecma-international.org/ecma-262/8.0/index.html#sec-unary-operators
    Unary {
        operator: UnaryOperator,
        prefix: bool,
        argument: Box<Expression>,
    },
    // https://www.ecma-international.org/ecma-262/8.0/index.html#sec-exp-operator
    // https://www.ecma-international.org/ecma-262/8.0/index.html#sec-multiplicative-operators
    // https://www.ecma-international.org/ecma-262/8.0/index.html#sec-additive-operators
    // https://www.ecma-international.org/ecma-262/8.0/index.html#sec-bitwise-shift-operators
    // https://www.ecma-international.org/ecma-262/8.0/index.html#sec-relational-operators
    // https://www.ecma-international.org/ecma-262/8.0/index.html#sec-equality-operators
    // https://www.ecma-international.org/ecma-262/8.0/index.html#sec-binary-bitwise-operators
    // https://www.ecma-international.org/ecma-262/8.0/index.html#sec-binary-logical-operators
    Binary {
        operator: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    // https://www.ecma-international.org/ecma-262/8.0/index.html#sec-conditional-operator
    Conditional {
        test: Box<Expression>,
        alternate: Box<Expression>,
        consequent: Box<Expression>,
    },
    // https://www.ecma-international.org/ecma-262/8.0/index.html#sec-assignment-operators
    Assignment {
        operator: AssignmentOperator,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    ArrowFunction {
        body: Box<ArrowFunctionBody>,
        expression: bool,
        async: bool, // async () =>
    },
    Yield {
        argument: Option<Box<Expression>>,
        delegate: bool, // yield *
    },
    // https://www.ecma-international.org/ecma-262/8.0/index.html#sec-comma-operator
    Comma {
        expressions: Vec<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Property {
    pub key: Expression,
    pub value: Expression,
    pub kind: PropertyKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PropertyKind {
    Init,
    Get,
    Set,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UpdateOperator {
    // ++
    Increment,
    // --
    Decrement,
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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
    // **
    Exponent,
}

#[derive(Debug, Clone, PartialEq)]
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

// https://www.ecma-international.org/ecma-262/8.0/index.html#sec-ecmascript-language-statements-and-declarations
pub type BlockStatement = Vec<Statement>;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Block {
        body: BlockStatement,
    },
    Expression {
        expression: Expression,
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

// https://www.ecma-international.org/ecma-262/8.0/index.html#sec-ecmascript-language-scripts-and-modules
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub source_type: SourceType,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SourceType {
    Script,
    Module,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Directive {
    pub expression: Literal,
    pub directive: String,
}

// statements
#[derive(Debug, Clone, PartialEq)]
pub struct SwitchCase {
    // default has no test
    test: Option<Expression>,
    consequent: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CatchClause {
    param: Pattern,
    body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ForInit {
    VariableDeclaration(VariableDeclaration),
    Expression(Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ForInLeft {
    VariableDeclaration(VariableDeclaration),
    Pattern(Pattern),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImportSpecifier {
    // import {foo, bar, baz as qux} from './mod'
    Import(HashMap<Id, Id>),
    // import foo from './mod'
    ImportDefault(Id),
    // import * as foo from './mod'
    ImportNamespaced(Id),
}

// declarations
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub id: Id,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration {
    pub kind: VariableDeclarationKind,
    pub declarations: Vec<VariableDeclarator>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariableDeclarationKind {
    Var,
    // ES2015
    Let,
    Const,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclarator {
    pub id: Pattern,
    pub init: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArrowFunctionBody {
    Expression(Expression),
}

// patterns
#[derive(Debug, Clone, PartialEq)]
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
