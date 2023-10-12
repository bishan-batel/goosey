use std::fmt::{Debug, Display, Formatter};
use std::ops::Range;
use std::rc::Rc;

use crate::file::source_file::SourceFile;

#[derive(Debug)]
pub struct Trace {
    pub source: Rc<SourceFile>,
    pub range: Range<usize>,
}

impl Eq for Trace {}

impl PartialEq for Trace {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Clone for Trace {
    fn clone(&self) -> Self {
        Self {
            source: Rc::clone(&self.source),
            range: self.range.clone(),
        }
    }
}

impl Display for Trace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let src = self.source.source.as_str();
        let start = self.range.start.max(0);
        let end = (self.range.end - 1).min(self.source.source.len() - 1);
        f.write_fmt(format_args!("Trace {{ '{}' }}", &src[start..end]))?;
        f.write_fmt(format_args!("{:?}", self.range))
    }
}