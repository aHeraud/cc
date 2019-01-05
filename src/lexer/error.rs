use std::ffi::OsString;
use std::error::Error;
use std::fmt::{Display, Formatter, Error as FmtError};

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidToken {
    pub source_filename: OsString,
    pub line: usize,
    pub col: usize,
    pub line_text: String
}

impl InvalidToken {
    pub fn from_offset(filename: OsString, source: &str, offset: usize) -> InvalidToken {
        let mut lines: usize = 0;
        let mut col: usize = 1;
        for line in source[0..offset].lines() {
            lines += 1;
            col = line.len() + 1;
        }
        if lines == 0 {
            lines = 1;
        }

        InvalidToken {
            source_filename: filename,
            line: lines,
            col: col,
            line_text: source.lines().skip(lines - 1).next().unwrap().to_string()
        }
    }
}

impl Display for InvalidToken {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "Invalid token in source file {} at position {}:{}", 
            self.source_filename.to_string_lossy(), self.line, self.col)
    }
}

//TODO: add cause for invalid token error (ex: invalid int literal suffix)
impl Error for InvalidToken {}
