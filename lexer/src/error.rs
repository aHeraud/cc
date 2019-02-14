use std::ffi::OsString;
use std::error::Error;
use std::fmt::{Display, Formatter, Error as FmtError};
use std::rc::Rc;

use ast::Location;

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidToken {
    pub location: Location,
    pub line_text: String
}

impl InvalidToken {
    pub fn from_offset(location: Location, source: &str) -> InvalidToken {
        let line_text = source.lines().skip(location.line - 1).next().unwrap_or("").to_string();
        InvalidToken {
            location,
            line_text
        }
    }
}

impl Display for InvalidToken {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}: invalid token", self.location)
    }
}

//TODO: add cause for invalid token error (ex: invalid int literal suffix)
impl Error for InvalidToken {}
