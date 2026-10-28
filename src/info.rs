use std::ffi::CStr;
use std::ptr;
use std::ptr::NonNull;

use bitflags::bitflags;

use crate::errno::FiError;

use foreign_types::ForeignType;
use foreign_types::ForeignTypeRef;
use foreign_types::foreign_type;

use ofi_libfabric_sys::bindgen as ffi;

macro_rules! fi_flags {
    (
        $name:ident: $ty:ty {
            $(
                $(#[$($flag_meta:tt)*])*
                $flag:ident
            ),+ $(,)?
        }
    ) => {
        bitflags! {
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
            pub struct $name: $ty {
                $(
                    $(#[$($flag_meta)*])*
                    const $flag = ffi::$flag as $ty;
                )+
            }
        }

        $(
            $(#[$($flag_meta)*])*
            pub const $flag: $name = $name::$flag;
        )+
    };
}

// TODO: replace the C names in these flag docs with the Rust wrapper names.
fi_flags!(FiGetInfoFlags: u64 {
    /// Indicates that the node parameter is a numeric string representation of a fabric address, such as a dotted decimal IP address. Use of this flag will suppress any lengthy name resolution protocol.
    FI_NUMERICHOST,
    /// Indicates that the caller is only querying for what providers are potentially available. All providers will return exactly one `fi_info` struct, regardless of whether that provider is usable on the current platform or not. The returned `fi_info` struct will contain default values for all members, with the exception of `fabric_attr`. The `fabric_attr` member will have the `prov_name` and `prov_version` values filled in.
    FI_PROV_ATTR_ONLY,
    /// Indicates that the node and service parameters specify the local source address to associate with an endpoint. If specified, either the node and/or service parameter must be non-NULL. This flag is often used with passive endpoints.
    FI_SOURCE,
    /// Indicates that the provider should rescan available network interfaces. This operation may be computationally expensive.
    FI_RESCAN,
});

foreign_type! {
    pub unsafe type FiInfo
    {
        type CType = ffi::fi_info;
        fn drop = ffi::fi_freeinfo;
    }
}

impl FiInfo {
    #[must_use]
    pub fn alloc() -> Option<Self> {
        let ptr = unsafe { ffi::fi_allocinfo() };
        NonNull::new(ptr).map(Self)
    }

    /// # Errors
    ///
    /// Returns [`FiError`] on error. The following errors may be returned:
    ///
    /// - [`FI_EBADFLAGS`]: The specified endpoint or domain capability or operation flags are invalid.
    /// - [`FI_ENODATA`]: Indicates that no providers could be found which support the requested fabric information.
    /// - [`FI_ENOMEM`]: Indicates that there was insufficient memory to complete the operation.
    /// - [`FI_ENOSYS`]: Indicates that requested version is newer than the library being used.
    ///
    /// [`FI_EBADFLAGS`]: crate::errno::FI_EBADFLAGS
    /// [`FI_ENODATA`]: crate::errno::FI_ENODATA
    /// [`FI_ENOMEM`]: crate::errno::FI_ENOMEM
    /// [`FI_ENOSYS`]: crate::errno::FI_ENOSYS
    pub fn getinfo(
        version: u32,
        node: Option<&CStr>,
        service: Option<&CStr>,
        flags: FiGetInfoFlags,
        hints: Option<&FiInfoRef>,
    ) -> Result<Self, FiError> {
        let node_ptr = node.map_or(ptr::null(), CStr::as_ptr);
        let service_ptr = service.map_or(ptr::null(), CStr::as_ptr);
        let hints_ptr = hints.map_or(ptr::null(), |hints| hints.as_ptr().cast_const());
        let mut info_ptr: *mut ffi::fi_info = ptr::null_mut();

        let rc = unsafe {
            ffi::fi_getinfo(
                version,
                node_ptr,
                service_ptr,
                flags.bits(),
                hints_ptr,
                &raw mut info_ptr,
            )
        };

        FiError::check(rc)?;
        Ok(unsafe { Self::from_ptr(info_ptr) })
    }
}

impl FiInfoRef {
    #[must_use]
    pub fn dup(&self) -> Option<FiInfo> {
        let ptr = unsafe { ffi::fi_dupinfo(self.as_ptr()) };
        NonNull::new(ptr).map(FiInfo)
    }
}
