use uuid::Uuid;
use crate::file::identifier::GlobalIdentifier;
use crate::parser::ast::operations::{BinaryOperation, UnaryOperator};
use crate::validation::data_type::DataType;

#[derive(Debug)]
pub enum FunctionStatement {
    Scope(Vec<FunctionStatement>),
    Return(Option<Expression>),
    If {
        condition: Expression,
        then: Box<FunctionStatement>,
        otherwise: Option<Box<FunctionStatement>>,
    },
    While {
        condition: Expression,
        then: Box<FunctionStatement>,
    },
    Expression(Expression),
}

#[derive(Debug)]
pub enum Expression {
    F32Literal(f32),
    F64Literal(f64),
    I32Literal(i32),
    I64Literal(i64),
    StringLiteral(String),
    Binary {
        lhs: Box<Expression>,
        op: BinaryOperation,
        rhs: Box<Expression>,
    },
    Unary {
        op: UnaryOperator,
        operand: Box<Expression>,
    },
    Cast {
        expr: Box<Expression>,
        ty: DataType,
    },
    LocalVariableReference(Uuid),
    GlobalVariableReference(GlobalIdentifier),
    FunctionCall(GlobalIdentifier),
    Parenthetical(Box<Expression>),
}