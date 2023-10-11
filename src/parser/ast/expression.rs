use crate::file::identifier::{Identifier, Namespace};
use crate::file::trace::Trace;
use crate::parser::ast::function::UnvalidatedFunctionExpression;
use crate::parser::ast::operations::BinaryOperation;

#[derive(Debug, PartialEq)]
pub enum UnvalidatedExpression {
    // if else chains can evaluated to an expression
    IfElse {
        condition: Box<UnvalidatedExpression>,
        then: Box<UnvalidatedFunctionExpression>,
        otherwise: Box<UnvalidatedFunctionExpression>,
    },
    Scope(Vec<UnvalidatedFunctionExpression>, Trace),

    BoolLiteral(bool, Trace),
    I32Literal(f32, Trace),
    I64Literal(f64, Trace),
    StringLiteral(String, Trace),

    Parenthetical {
        expr: Box<UnvalidatedExpression>,
    },
    BinaryExpression {
        /// Left hand Side
        lhs: Box<UnvalidatedExpression>,

        /// Binary Operator
        op: BinaryOperation,
        rhs: Box<UnvalidatedExpression>,
        trace: Trace,
    },
    FunctionCall {
        explicit_namespace: Namespace,
        ident: Identifier,
        trace: Trace,
    },

    VariableReference {
        explicit_namespace: Namespace,
        ident: Identifier,
        trace: Trace,
    },
    ObjectProperty {
        object: Box<UnvalidatedExpression>,
        identifier: Identifier,
        trace: Trace,
    },
    ObjectMethod {},
}