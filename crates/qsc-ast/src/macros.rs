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
