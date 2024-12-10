#[macro_export]
macro_rules! as_number {
    ($t:ty, enum $enum_name:ident { $($variant:ident),* $(,)? } $(, derive($($trait:ident),*))?) => {
        $(#[derive($($trait),*)])?
        pub enum $enum_name {
            $($variant),*
        }

        impl From<$t> for $enum_name {
            fn from(value: $t) -> Self {
                match value {
                    $(x if x == $enum_name::$variant as $t => $enum_name::$variant),*,
                    _ => panic!("Invalid value for enum: {value}"),
                }
            }
        }

        impl From<$enum_name> for $t {
            fn from(variant: $enum_name) -> Self {
                variant as $t
            }
        }
    };
}
