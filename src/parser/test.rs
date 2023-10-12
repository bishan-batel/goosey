use std::rc::Rc;
use crate::file::identifier::{Identifier, Namespace};
use crate::file::source_file::SourceFile;
use crate::file::trace::Trace;
use crate::ir::visibility::Visibility;
use crate::parser::ast::data::{UnparsedVariableInfo, UnvalidatedType};
use crate::parser::ast::expression::UnvalidatedExpression;
use crate::parser::ast::function::{UnparsedFunctionPrototype, UnvalidatedFunctionExpression};
use crate::parser::ast::operations::BinaryOperation;
use crate::parser::ast::top_level::UnparsedTopLevel;
use crate::parser::ast::UnvalidatedSymbol;
use crate::parser::error::ParserResult;
use crate::parser::parser::Parser;

fn parse_from(source: &str) -> (ParserResult<Vec<UnparsedTopLevel>>, Trace) {
    let source = SourceFile::new(source).rc();
    let trace = source.trace(0..0);
    let tokens = crate::lexer::tokenize(Rc::clone(&source));

    (Parser::new(source, tokens).parse(), trace)
}

#[test]
fn function_proto() {
    let (vecs, trace) = parse_from(r"
        fun no_arg() {}
        pub fun args(_0: i32, _1: i64) {}
        fun returns(): ref[i32] {}
    ");

    assert_eq!(vecs, Ok(vec![
        UnparsedTopLevel::FunctionDefinition {
            proto: UnparsedFunctionPrototype {
                name: Identifier("no_arg".into()),
                arguments: vec![],
                returns: UnvalidatedType::Unit,
                visibility: Visibility::Private,
            },
            body: UnvalidatedExpression::Scope(vec![], trace.clone()).into(),
            trace: trace.clone(),
        },
        UnparsedTopLevel::FunctionDefinition {
            proto: UnparsedFunctionPrototype {
                name: Identifier("args".into()),
                arguments: vec![
                    UnparsedVariableInfo {
                        ident: "_0".into(),
                        ty: UnvalidatedType::Type("i32".into()),
                        mutable: false,
                    },
                    UnparsedVariableInfo {
                        ident: "_1".into(),
                        ty: UnvalidatedType::Type("i64".into()),
                        mutable: false,
                    },
                ],
                returns: UnvalidatedType::Unit,
                visibility: Visibility::Public,
            },
            body: UnvalidatedExpression::Scope(vec![], trace.clone()).into(),
            trace: trace.clone(),
        },
        UnparsedTopLevel::FunctionDefinition {
            proto: UnparsedFunctionPrototype {
                name: Identifier("returns".into()),
                arguments: vec![],
                returns: UnvalidatedType::Reference(Box::new(UnvalidatedType::Type("i32".into()))),
                visibility: Visibility::Private,
            },
            body: UnvalidatedExpression::Scope(vec![], trace.clone()).into(),
            trace: trace.clone(),
        },
    ]))
}

#[test]
fn function_oneline() {
    let (vecs, trace) = parse_from(r"
        fun funny() => 69
    ");

    assert_eq!(vecs, Ok(vec![
        UnparsedTopLevel::FunctionDefinition {
            proto: UnparsedFunctionPrototype {
                name: Identifier("funny".into()),
                arguments: vec![],
                returns: UnvalidatedType::Unit,
                visibility: Visibility::Private,
            },
            body: UnvalidatedExpression::I32Literal(69, trace.clone()).into(),
            trace: trace.clone(),
        },
    ]))
}

#[test]
fn function_operator_precedence() {
    let (vecs, trace_templ) = parse_from(r"
        fun funny() =>
        2 = 1*2 / 4 + (2 + 5)
    ");

    let trace = || trace_templ.clone();

    use UnvalidatedExpression as E;

    assert_eq!(vecs, Ok(vec![
        UnparsedTopLevel::FunctionDefinition {
            proto: UnparsedFunctionPrototype {
                name: Identifier("funny".into()),
                arguments: vec![],
                returns: UnvalidatedType::Unit,
                visibility: Visibility::Private,
            },
            body: E::Binary {
                lhs: Box::new(E::I32Literal(2, trace())),
                op: BinaryOperation::Assign,
                rhs: Box::new(E::Binary {
                    lhs: Box::new(E::Binary {
                        lhs: Box::new(E::Binary {
                            lhs: Box::new(E::I32Literal(1, trace())),
                            op: BinaryOperation::Multiply,
                            rhs: Box::new(E::I32Literal(2, trace())),
                            trace: trace(),
                        }),
                        op: BinaryOperation::Divide,
                        rhs: Box::new(E::I32Literal(4, trace())),
                        trace: trace(),
                    }),
                    op: BinaryOperation::Plus,
                    rhs: Box::new(E::Parenthetical(
                        Box::new(E::Binary {
                            lhs: Box::new(E::I32Literal(2, trace())),
                            op: BinaryOperation::Plus,
                            rhs: Box::new(E::I32Literal(5, trace())),
                            trace: trace(),
                        }), trace())),
                    trace: trace(),
                }),
                trace: trace(),
            }.into(),
            trace: trace(),
        },
    ]))
}

#[test]
fn function_symbols() {
    let (vecs, trace) = parse_from(r##"
        fun funny() {
            printf("hello")
            std::io::printf("hello")
        }
    "##);

    let trace = || trace.clone();

    use UnvalidatedExpression as E;

    if let Err(ref e) = vecs {
        eprintln!("{e}");
    }

    assert_eq!(vecs, Ok(vec![
        UnparsedTopLevel::FunctionDefinition {
            proto: UnparsedFunctionPrototype {
                name: Identifier("funny".into()),
                arguments: vec![],
                returns: UnvalidatedType::Unit,
                visibility: Visibility::Private,
            },
            body: E::Scope(vec![
                E::FunctionCall {
                    symbol: UnvalidatedSymbol {
                        explicit_namespace: Namespace { chain: vec![] },
                        identifier: "printf".into(),
                    },
                    arguments: vec![
                        E::StringLiteral("hello".into(), trace())
                    ],
                    trace: trace(),
                }.into(),
                E::FunctionCall {
                    symbol: UnvalidatedSymbol {
                        explicit_namespace: Namespace { chain: vec!["std".into(), "io".into()] },
                        identifier: "printf".into(),
                    },
                    arguments: vec![
                        E::StringLiteral("hello".into(), trace())
                    ],
                    trace: trace(),
                }.into(),
            ], trace()).into(),
            trace: trace(),
        },
    ]))
}
