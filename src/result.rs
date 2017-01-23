use std::{result, fmt};

extern crate git2;

pub struct Error {
    msg: &'static str,
    err: Option<git2::Error>,
}

impl Error {
    pub fn new(msg: &'static str) -> Error {
        Error {
            msg: msg,
            err: None,
        }
    }

    pub fn with_err(msg: &'static str, err: git2::Error) -> Error {
        Error {
            msg: msg,
            err: Some(err),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.err {
            Some(ref e) => {
                write!(f, "{}: ", self.msg).unwrap();
                e.fmt(f)
            }
            None => write!(f, "{}", self.msg),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.err {
            Some(ref e) => {
                write!(f, "{}: ", self.msg).unwrap();
                e.fmt(f)
            }
            None => write!(f, "{}", self.msg),
        }
    }
}

pub type Result<T> = result::Result<T, Error>;

#[macro_export]
macro_rules! try_unwrap {
    ($e: expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => fail_with!(e),
        }
    }
}

pub fn with_msg<T>(result: result::Result<T, git2::Error>, msg: &'static str) -> Result<T> {
    match result {
        Ok(v) => Ok(v),
        Err(e) => {
            Err(Error {
                msg: msg,
                err: Some(e),
            })
        }
    }
}
