#[macro_export]
macro_rules! throw {
    ($e: ident) => {
        eprintln!("{}", $e);
        std::process::exit(1);
    };
}

#[macro_export]
macro_rules! enum_export {
    ($name: ident, $fun: ident, $variant: ident, $ty: ident) => {
        impl $name {
            pub fn $fun(&self) -> anyhow::Result<$ty> {
                match self {
                    $name::$variant(v) => Ok(v.clone()),
                    _ => Err(anyhow::anyhow!(
                        "Invalid variant! Expected: {}, got: {:?}",
                        stringify!($variant),
                        self
                    )),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! expect {
    ($tokens: ident, $token: expr, $expected: expr) => {
        if $token.content != $expected {
            let err = crate::tokenizer::error::Error::UnexpectedToken {
                token: format!("{}", $token.content),
                file: $tokens.cursor.file.clone(),
                data: $tokens.cursor.all_data.clone(),
                pos: $tokens.peek().unwrap().position,
            };

            $crate::throw!(err);
        }
    };
}
