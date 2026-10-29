//! Internal macros shared across the crate.

/// Declares a `bitflags` type from a set of libfabric `FI_*` flag constants,
/// along with a top-level constant for each flag.
macro_rules! fi_flags {
    (
        $name:ident: $ty:ty {
            $(
                $(#[$($flag_meta:tt)*])*
                $flag:ident
            ),+ $(,)?
        }
    ) => {
        bitflags::bitflags! {
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
            pub struct $name: $ty {
                $(
                    $(#[$($flag_meta)*])*
                    const $flag = ofi_libfabric_sys::bindgen::$flag as $ty;
                )+
            }
        }

        $(
            $(#[$($flag_meta)*])*
            pub const $flag: $name = $name::$flag;
        )+
    };
}

pub(crate) use fi_flags;
