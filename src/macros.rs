#[macro_export]
macro_rules! green {
    ($($arg:tt)*) => {{
        use $crate::colors::{GREEN, NC};
        eprintln!("{GREEN}{}{NC}", format_args!($($arg)*));
    }};
}
pub use green;

#[macro_export]
macro_rules! yellow {
    ($($arg:tt)*) => {{
        use $crate::colors::{YELLOW, NC};
        eprintln!("{YELLOW}{}{NC}", format_args!($($arg)*));
    }};
}
pub use yellow;

#[macro_export]
macro_rules! red {
    ($($arg:tt)*) => {{
        use $crate::colors::{RED, NC};
        eprintln!("{RED}{}{NC}", format_args!($($arg)*));
    }};
}
pub use red;

#[macro_export]
macro_rules! error {
    (err = $err:expr, $($arg:tt)*) => {{
        $crate::red!("{}", format_args!($($arg)*));
        $crate::red!("error: {:?}", $err);
        std::process::exit(1);
    }};

    ($($arg:tt)*) => {{
        $crate::red!("{}", format_args!($($arg)*));
        std::process::exit(1);
    }};


}
pub use error;
