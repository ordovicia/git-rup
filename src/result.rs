use std::result;

extern crate git2;

pub type Result<T> = result::Result<T, git2::Error>;

#[macro_export]
macro_rules! try_unwrap {
    ($e: expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => fail_with!(e),
        }
    }
}
