#[macro_export]
macro_rules! info {
    ($($args: expr),*) => {{
        // print!("[git-rup] ");
        println!($($args),*);
    }};
}

#[macro_export]
macro_rules! warn {
    ($($args: expr),*) => { info!($($args),*); };
}

#[macro_export]
macro_rules! fail {
    ($($args: expr),*) => {{
        info!($($args),*);
        ::std::process::exit(1);
    }};
}
