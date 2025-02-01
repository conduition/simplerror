#![doc = include_str!("../README.md")]

/// For internal use only.
#[macro_export]
macro_rules! internal_impl_enum_from {
    (
            $enum_name:ident,
            $member:ident($inner_type1:ty, $($inner_type2:ty),+)
        ) => {
        // Don't implement `From` automatically for error enum members which have more than
        // one inner type. Caller has to figure that out.
    };

    (
            $enum_name:ident,
            $member:ident($inner_type:ty)
        ) => {
        impl From<$inner_type> for $enum_name {
            fn from(inner: $inner_type) -> Self {
                Self::$member(inner)
            }
        }
    };
}

/// Declare error types with automatic error wrapping and trait implementations handled for you.
#[macro_export]
macro_rules! declare {
    {
        $(
            $(#[$enum_attribute:meta])*
            $visibility:vis enum $enum_name:ident {
                $(
                    $(#[$member_attribute:meta])*
                    $member:ident$(($($inner_name:ident: $inner_type:ty),+))? $(=> $message:literal)?,
                )+
            }
        )*
    } => {
        $(
            $(#[$enum_attribute])*
            #[derive(Debug)]
            $visibility enum $enum_name {
                $($member$(($($inner_type),+))?),+
            }

            impl core::fmt::Display for $enum_name {
                fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                    #[allow(unreachable_code)]
                    match self {
                        $(
                            $(#[$member_attribute])*
                            Self::$member$(($($inner_name),+))? => {
                                $(
                                    return write!(f, $message);
                                )?
                                write!(f, "{}::{}", stringify!($enum_name), stringify!($member))
                            }
                        )+
                    }
                }
            }

            $(
                // Implement From<$inner_type> for $enum_name::$member where applicable
                $($crate::internal_impl_enum_from!($enum_name, $member($($inner_type),+));)?
            )+
        )*
    }
}
