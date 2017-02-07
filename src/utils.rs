#[macro_export]
macro_rules! fail {
    ($($args: expr),*) => {{
        println!($($args),*);
        ::std::process::exit(1);
    }};
}

#[macro_export]
macro_rules! fail_with {
    ($e: expr) => { fail!("{}", $e); };
}

#[macro_export]
macro_rules! try_unwrap_opt {
    ($e: expr, $msg: expr) => {
        match $e {
            Some(v) => v,
            None => fail_with!($msg),
        }
    }
}
