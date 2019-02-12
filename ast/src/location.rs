use std::fmt;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::ffi::OsString;

/// The location of an ast node in a source file
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Location {
    pub filename: Rc<OsString>,
    pub line: usize,
    pub column: usize,
    pub byte_offset: usize
}

impl Location {
    pub fn new(filename: Rc<OsString>, line: usize, column: usize, byte_offset: usize) -> Location {
        Location {
            filename,
            line,
            column,
            byte_offset
        }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}", self.filename.to_string_lossy(), self.line, self.column)
    }
}

impl Default for Location {
    fn default() -> Self {
        Location {
            filename: Rc::new(OsString::default()),
            line: 0,
            column: 0,
            byte_offset: 0
        }
    }
}
