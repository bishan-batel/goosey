use crate::file::identifier::{Identifier, Namespace};
use crate::file::trace::Trace;
use crate::parser::ast::data::UnvalidatedType;
use crate::parser::ast::function::UnvalidatedFunctionExpression;
use crate::parser::ast::operations::{BinaryOperation, UnaryOperator};
use crate::parser::ast::UnvalidatedSymbol;

#[derive(Debug, PartialEq)]
pub enum UnvalidatedExpression {
    // if else chains can evaluated to an expression
    IfElse {
        condition: Box<UnvalidatedExpression>,
        then: Box<UnvalidatedFunctionExpression>,
        otherwise: Box<UnvalidatedFunctionExpression>,
        trace: Trace,
    },
    Scope(Vec<UnvalidatedFunctionExpression>, Trace),

    BoolLiteral(bool, Trace),
    F32Literal(f32, Trace),
    F64Literal(f64, Trace),
    I32Literal(i32, Trace),
    I64Literal(i64, Trace),
    StringLiteral(String, Trace),

    Parenthetical(Box<UnvalidatedExpression>, Trace),
    Binary {
        /// Left hand Side
        lhs: Box<UnvalidatedExpression>,

        /// Binary Operator
        op: BinaryOperation,
        rhs: Box<UnvalidatedExpression>,
        trace: Trace,
    },
    Unary {
        expr: Box<UnvalidatedExpression>,
        op: UnaryOperator,
        trace: Trace,
    },
    Cast {
        expr: Box<UnvalidatedExpression>,
        ty: UnvalidatedType,
        trace: Trace,
    },
    FunctionCall {
        symbol: UnvalidatedSymbol,
        arguments: Vec<UnvalidatedExpression>,
        trace: Trace,
    },
    VariableReference {
        symbol: UnvalidatedSymbol,
        trace: Trace,
    },
    ObjectProperty {
        object: Box<UnvalidatedExpression>,
        identifier: Identifier,
        trace: Trace,
    },
}

impl Into<UnvalidatedFunctionExpression> for UnvalidatedExpression {
    fn into(self) -> UnvalidatedFunctionExpression {
        UnvalidatedFunctionExpression::Expression(self)
    }
}

impl UnvalidatedExpression {
    pub fn trace(&self) -> Trace {
        (match self {
            UnvalidatedExpression::IfElse {
                condition: _,
                then: _,
                otherwise: _,
                trace
            } => trace,
            UnvalidatedExpression::Scope(_, trace) => trace,
            UnvalidatedExpression::BoolLiteral(_, trace) => trace,
            UnvalidatedExpression::I32Literal(_, trace) => trace,
            UnvalidatedExpression::I64Literal(_, trace) => trace,
            UnvalidatedExpression::StringLiteral(_, trace) => trace,
            UnvalidatedExpression::Parenthetical(_, trace) => trace,
            UnvalidatedExpression::F32Literal(_, trace) => trace,
            UnvalidatedExpression::F64Literal(_, trace) => trace,
            UnvalidatedExpression::Binary { trace, .. } => trace,
            UnvalidatedExpression::FunctionCall { trace, .. } => trace,
            UnvalidatedExpression::VariableReference { trace, .. } => trace,
            UnvalidatedExpression::ObjectProperty { .. } => todo!(),
            UnvalidatedExpression::Cast { trace, .. } => trace,
            UnvalidatedExpression::Unary { trace, .. } => trace,
        }).clone()
    }
}