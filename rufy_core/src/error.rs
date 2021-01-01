use std::fmt;
use std::result::Result as IResult;
use std::fmt::{Formatter, Debug};
use std::io;

#[derive(Debug, Clone)]
pub enum Error {
    IOError(String),
    LuaError(String),
}

pub type Result<T, Error> = IResult<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Error::IOError(ref message) => write!(f, "io error: {}", message),
            Error::LuaError(ref message) => write!(f, "lua error: {}", message),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IOError(err.to_string())
    }
}

impl From<rlua::Error> for Error {
    fn from(err: rlua::Error) -> Self {
        Error::LuaError(err.to_string())
    }
}