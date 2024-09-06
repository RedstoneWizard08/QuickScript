#[macro_export]
macro_rules! is_enum_variant {
    ($var: ident == $enum: ident::$variant: ident) => {
        if let $enum::$variant(_) = $var {
            return true;
        }

        return false;
    };
}

#[macro_export]
macro_rules! is_enum_variant_impl {
    ($name: ident -> $enum: ident::$variant: ident) => {
        impl $enum {
            pub fn $name(&self) -> bool {
                $crate::is_enum_variant!(self == $enum::$variant);
            }
        }
    };
}

#[macro_export]
macro_rules! is_enum_variant_no_field {
    ($var: ident == $enum: ident::$variant: ident) => {
        if let $enum::$variant = $var {
            return true;
        }

        return false;
    };
}

#[macro_export]
macro_rules! is_enum_variant_no_field_impl {
    ($name: ident -> $enum: ident::$variant: ident) => {
        impl $enum {
            pub fn $name(&self) -> bool {
                $crate::is_enum_variant_no_field!(self == $enum::$variant);
            }
        }
    };
}

#[macro_export]
macro_rules! get_enum_variant_value {
    ($var: ident -> $enum: ident::$variant: ident) => {
        if let $enum::$variant(val) = $var {
            return Ok(val.clone());
        }

        return Err($crate::miette::miette!(
            "Incorrect enum variant! Expected {}, got: {:?}",
            stringify!($variant),
            $var
        ));
    };
}

#[macro_export]
macro_rules! get_enum_variant_value_impl {
    ($name: ident -> $enum: ident::$variant: ident: $ty: ident) => {
        impl $enum {
            pub fn $name(&self) -> $crate::miette::Result<$ty> {
                $crate::get_enum_variant_value!(self -> $enum::$variant);
            }
        }
    }
}

#[macro_export]
macro_rules! string_enum {
    ($name: ident = { $($item: ident: $value: expr,)+ }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        pub enum $name {
            $($item,)+
        }

        impl $name {
            pub fn to_string(&self) -> &'static str {
                match self.clone() {
                    $(
                        Self::$item => $value,
                    )+
                }
            }

            pub fn from_str(val: impl AsRef<str>) -> Option<Self> {
                let val = val.as_ref();

                match val {
                    $(
                        $value => Some(Self::$item),
                    )+

                    _ => None,
                }
            }

            pub fn values() -> Vec<Self> {
                vec![
                    $(Self::$item,)+
                ]
            }
        }
    }
}
