//! Libfabric error handling.
//!
//! This module provides [`FiError`], a wrapper around a nonzero libfabric
//! error code, along with constants for the known `FI_E*` error codes.

use std::error::Error;
use std::ffi::{CStr, c_int};
use std::fmt;
use std::num::NonZeroI32;

use ofi_libfabric_sys::bindgen as ffi;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct FiError(NonZeroI32);

impl FiError {
    const fn new(value: i32) -> Option<Self> {
        /*
         * SOMEDAY: replace with `NonZeroI32::new(value).map(Self)` once
         * `Option::map` is const-stable (rust-lang/rust#143874).
         */
        match NonZeroI32::new(value) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    const fn get(self) -> i32 {
        self.0.get()
    }

    pub(crate) const fn from_ret(rc: c_int) -> Option<Self> {
        Self::new(rc.wrapping_abs())
    }

    pub(crate) fn check(rc: c_int) -> Result<(), Self> {
        Self::from_ret(rc).map_or(Ok(()), Err)
    }
}

impl Error for FiError {}

impl fmt::Display for FiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ptr = unsafe { ffi::fi_strerror(self.get()) };
        let msg = unsafe { CStr::from_ptr(ptr) }.to_string_lossy();
        f.write_str(&msg)
    }
}

macro_rules! fi_errno_codes {
    ($($name:ident => $doc:literal),* $(,)?) => {
        $(
            #[doc = $doc]
            #[allow(clippy::cast_possible_wrap)]
            pub const $name: FiError = FiError::new(ffi::$name as i32)
                .expect(concat!(
                    "libfabric errno ",
                    stringify!($name),
                    " must be nonzero"
                ));
        )*
    };
}

fi_errno_codes! {
    FI_EPERM => "Operation not permitted",
    FI_ENOENT => "No such file or directory",
    FI_EINTR => "Interrupted system call",
    FI_EIO => "I/O error",
    FI_E2BIG => "Argument list too long",
    FI_EBADF => "Bad file number",
    FI_EAGAIN => "Try again",
    FI_ENOMEM => "Out of memory",
    FI_EACCES => "Permission denied",
    FI_EFAULT => "Bad address",
    FI_EBUSY => "Device or resource busy",
    FI_ENODEV => "No such device",
    FI_EINVAL => "Invalid argument",
    FI_EMFILE => "Too many open files",
    FI_ENOSPC => "No space left on device",
    FI_ENOSYS => "Function not implemented",
    FI_EWOULDBLOCK => "Operation would block",
    FI_ENOMSG => "No message of desired type",
    FI_ENODATA => "No data available",
    FI_EPROTO => "Protocol error",
    FI_EOVERFLOW => "Value too large for defined data type",
    FI_EMSGSIZE => "Message too long",
    FI_ENOPROTOOPT => "Protocol not available",
    FI_EOPNOTSUPP => "Operation not supported on transport endpoint",
    FI_EADDRINUSE => "Address already in use",
    FI_EADDRNOTAVAIL => "Cannot assign requested address",
    FI_ENETDOWN => "Network is down",
    FI_ENETUNREACH => "Network is unreachable",
    FI_ECONNABORTED => "Software caused connection abort",
    FI_ECONNRESET => "Connection reset by peer",
    FI_ENOBUFS => "No buffer space available",
    FI_EISCONN => "Transport endpoint is already connected",
    FI_ENOTCONN => "Transport endpoint is not connected",
    FI_ESHUTDOWN => "Cannot send after transport endpoint shutdown",
    FI_ETIMEDOUT => "Connection timed out",
    FI_ECONNREFUSED => "Connection refused",
    FI_EHOSTDOWN => "Host is down",
    FI_EHOSTUNREACH => "No route to host",
    FI_EALREADY => "Operation already in progress",
    FI_EINPROGRESS => "Operation now in progress",
    FI_ECANCELED => "Operation Canceled",
    FI_EOTHER => "Unspecified error",
    FI_ETOOSMALL => "Provided buffer is too small",
    FI_EOPBADSTATE => "Operation not permitted in current state",
    FI_EAVAIL => "Error available",
    FI_EBADFLAGS => "Flags not supported",
    FI_ENOEQ => "Missing or unavailable event queue",
    FI_EDOMAIN => "Invalid resource domain",
    FI_ENOCQ => "Missing or unavailable completion queue",
    FI_ECRC => "CRC error",
    FI_ETRUNC => "Truncation error",
    FI_ENOKEY => "Required key not available",
    FI_ENOAV => "Missing or unavailable address vector",
    FI_EOVERRUN => "Queue has been overrun",
    FI_ENORX => "Receiver not ready, no receive buffers available",
    FI_ENOMR => "No more memory registrations available",
    FI_EFIREWALLADDR => "Host unreachable because it is behind firewall",
}

#[cfg(target_os = "linux")]
fi_errno_codes! {
    FI_EREMOTEIO => "Remote I/O error",
    FI_EKEYREJECTED => "Key was rejected by service",
}
