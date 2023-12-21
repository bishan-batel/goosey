use super::module::Module;



fn parse_from(source: &str) -> (ParserResult<Vec<UnvalidatedTopLevel>>, Box<dyn Fn() -> Trace>) {
    let source = SourceFile::new(source).rc();
    let trace = source.trace(0..0);
    let tokens = crate::lexer::tokenize(Rc::clone(&source));

    let trace = move || trace.clone();
    (Parser::new(source, tokens).parse(), Box::new(trace))
}

#[test]
fn module()  {
    let module = Module::new();
}

