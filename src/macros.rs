#[macro_export]
macro_rules! throw {
    ($e: ident) => {
        eprintln!("{}", $e);
        std::process::exit(1);
    };
}
