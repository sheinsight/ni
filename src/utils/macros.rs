#[macro_export]
macro_rules! info {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        let message = format!("🌴 {}", format_args!($($arg)*));
        print!("{}\n", message);
    }};
}

#[macro_export]
macro_rules! error {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        let message = format!("👻 {}", format_args!($($arg)*));
        print!("{}\n", message.red());
        std::process::exit(1);
    }}
}
