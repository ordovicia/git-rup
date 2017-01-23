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
