use std::ops::Range;
use std::rc::Rc;

use crate::file::source_file::SourceFile;

#[derive(Debug, PartialEq, Clone)]
pub struct Trace {
    pub source: Rc<SourceFile>,
    pub range: Range<usize>,
}