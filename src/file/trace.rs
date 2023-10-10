use std::ops::Range;
use std::rc::Rc;
use crate::file::source_file::SourceFile;

pub struct Trace {
    pub source: Rc<SourceFile>,
    pub range: Range<usize>,
}