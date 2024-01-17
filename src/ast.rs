// Represents a C program, which consists of a list of external declarations (functions and global variables)
pub struct Program {
    // List of functions and global variables
    pub external_declarations: Vec<ExternalDeclaration>,
}

// External declarations can be either a function or a global variable
pub enum ExternalDeclaration {
    FunctionDef(FunctionDefinition),
    GlobalVar(VariableDeclaration),
    // TODO: add struct, typedef, etc
}
#[derive(Debug)]
pub struct FunctionParameter {
    pub type_specifier: TypeSpecifier,
    pub name: String,
}

// A function definition consists of a return type, name, parameters, and a body (compound statement)
#[derive(Debug)]
pub struct FunctionDefinition {
    pub return_type: TypeSpecifier,
    pub name: String,
    pub parameters: Vec<FunctionParameter>,
    pub body: CompoundStatement,
}

#[derive(Debug)]
// A compound statement is a sequence of statements enclosed in braces
pub struct CompoundStatement {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
// Statements can be of various types
pub enum Statement {
    ExpressionStmt(Expression),
    CompoundStmt(CompoundStatement),
    ReturnStmt(Expression),
    IfThenStmt {
        condition: Box<Expression>,
        body: Box<Statement>,
        else_body: Option<Box<Statement>>,
    }
    // TODO: add more statement types like if, while, for, etc.
}

#[derive(Debug)]
pub enum Expression {
    Identifier(String),
    Constant(i32),
    StringLiteral(String),
    FunctionCall { name: String, arguments: Vec<Expression> },
    BinaryOp { left: Box<Expression>, operator: BinaryOperator, right: Box<Expression> },
    UnaryOp { operator: UnaryOperator, operand: Box<Expression> },
    EqualityOp {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    // TODO: add more expression types like ternary conditional, sizeof, cast, etc.
}

// Variable declarations consist of a type specifier and a list of init declarators
pub struct VariableDeclaration {
    pub type_specifier: TypeSpecifier,
    pub init_declarators: Vec<InitDeclarator>,
}

// An init declarator is a variable name and an optional initializer
pub struct InitDeclarator {
    pub name: String,
    pub initializer: Option<Expression>,
}

// Type specifiers define the type of a variable or return type of a function
#[derive(Debug)]
pub enum TypeSpecifier {
    Void,
    Char,
    Int,
    Float
    // TODO: add more types like float, double, custom structs, pub enums, etc.
}

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equals,
    LogicalAnd,
    LogicalOr
    // TODO: add more operators like modulo, logical and/or, bitwise operators, etc.
}

#[derive(Debug)]
pub enum UnaryOperator {
    Negate,
    LogicalNot,
}