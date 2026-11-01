use std::ffi::CStr;
use std::ptr;
use std::ptr::NonNull;

use crate::errno::FiError;
use crate::macros::fi_flags;

use foreign_types::ForeignType;
use foreign_types::ForeignTypeRef;
use foreign_types::foreign_type;

use ofi_libfabric_sys::bindgen as ffi;

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

fi_flags!(FiCaps: u64 {
    /// Specifies that the endpoint supports some set of atomic operations. Endpoints supporting this capability support operations defined by struct `fi_ops_atomic`. In the absence of any relevant flags, [`FI_ATOMIC`](Self::FI_ATOMIC) implies the ability to initiate and be the target of remote atomic reads and writes. Applications can use the [`FI_READ`](Self::FI_READ), [`FI_WRITE`](Self::FI_WRITE), [`FI_REMOTE_READ`](Self::FI_REMOTE_READ), and [`FI_REMOTE_WRITE`](Self::FI_REMOTE_WRITE) flags to restrict the types of atomic operations supported by an endpoint.
    FI_ATOMIC,
    /// Requests that the provider support the association of a user specified identifier with each address vector (AV) address. User identifiers are returned with completion data in place of the AV address. See `fi_domain(3)` and `fi_av(3)` for more details.
    FI_AV_USER_ID,
    /// Requests support for collective operations. Endpoints that support this capability support the collective operations defined in `fi_collective(3)`.
    FI_COLLECTIVE,
    /// Requests that the communication endpoint use the source address of an incoming message when matching it with a receive buffer. If this capability is not set, then the `src_addr` parameter for msg and tagged receive operations is ignored.
    FI_DIRECTED_RECV,
    /// Similar to [`FI_DIRECTED_RECV`](Self::FI_DIRECTED_RECV), but only applies to tagged receive operations.
    FI_TAGGED_DIRECTED_RECV,
    /// Similar to [`FI_DIRECTED_RECV`](Self::FI_DIRECTED_RECV), but requires the source address to be exact, i.e., `FI_ADDR_UNSPEC` is not allowed. This capability can be used alone, or in conjunction with [`FI_DIRECTED_RECV`](Self::FI_DIRECTED_RECV) or [`FI_TAGGED_DIRECTED_RECV`](Self::FI_TAGGED_DIRECTED_RECV) as a modifier to disallow `FI_ADDR_UNSPEC` being used as the source address.
    FI_EXACT_DIRECTED_RECV,
    /// Indicates that the endpoint support the [`FI_FENCE`](Self::FI_FENCE) flag on data transfer operations. Support requires tracking that all previous transmit requests to a specified remote endpoint complete prior to initiating the fenced operation. Fenced operations are often used to enforce ordering between operations that are not otherwise guaranteed by the underlying provider or protocol.
    FI_FENCE,
    /// Specifies that the endpoint should support transfers to and from device memory.
    FI_HMEM,
    /// Indicates that the endpoint support host local communication. This flag may be used in conjunction with [`FI_REMOTE_COMM`](Self::FI_REMOTE_COMM) to indicate that local and remote communication are required. If neither [`FI_LOCAL_COMM`](Self::FI_LOCAL_COMM) or [`FI_REMOTE_COMM`](Self::FI_REMOTE_COMM) are specified, then the provider will indicate support for the configuration that minimally affects performance. Providers that set [`FI_LOCAL_COMM`](Self::FI_LOCAL_COMM) but not [`FI_REMOTE_COMM`](Self::FI_REMOTE_COMM), for example a shared memory provider, may only be used to communication between processes on the same system.
    FI_LOCAL_COMM,
    /// Specifies that an endpoint should support sending and receiving messages or datagrams. Message capabilities imply support for send and/or receive queues. Endpoints supporting this capability support operations defined by struct `fi_ops_msg`.
    ///
    /// The caps may be used to specify or restrict the type of messaging operations that are supported. In the absence of any relevant flags, [`FI_MSG`](Self::FI_MSG) implies the ability to send and receive messages. Applications can use the [`FI_SEND`](Self::FI_SEND) and [`FI_RECV`](Self::FI_RECV) flags to optimize an endpoint as send-only or receive-only.
    FI_MSG,
    /// Indicates that the endpoint support multicast data transfers. This capability must be paired with [`FI_MSG`](Self::FI_MSG). Applications can use [`FI_SEND`](Self::FI_SEND) and [`FI_RECV`](Self::FI_RECV) to optimize multicast as send-only or receive-only.
    FI_MULTICAST,
    /// Specifies that the endpoint must support the [`FI_MULTI_RECV`](Self::FI_MULTI_RECV) flag when posting receive buffers.
    FI_MULTI_RECV,
    /// Specifies that the endpoint must support the [`FI_MULTI_RECV`](Self::FI_MULTI_RECV) flag when posting tagged receive buffers.
    FI_TAGGED_MULTI_RECV,
    /// Requests that endpoints which support multiple receive contexts allow an initiator to target (or name) a specific receive context as part of a data transfer operation.
    FI_NAMED_RX_CTX,
    /// Specifies that the provider must support being used as a peer provider in the peer API flow. The provider must support importing `owner_ops` when opening a CQ, counter, and shared receive queue.
    FI_PEER,
    /// Indicates that the user requires an endpoint capable of initiating reads against remote memory regions. This flag requires that [`FI_RMA`](Self::FI_RMA) and/or [`FI_ATOMIC`](Self::FI_ATOMIC) be set.
    FI_READ,
    /// Indicates that the user requires an endpoint capable of receiving message data transfers. Message transfers include base message operations as well as tagged message functionality.
    FI_RECV,
    /// Indicates that the endpoint support communication with endpoints located at remote nodes (across the fabric). See [`FI_LOCAL_COMM`](Self::FI_LOCAL_COMM) for additional details. Providers that set [`FI_REMOTE_COMM`](Self::FI_REMOTE_COMM) but not [`FI_LOCAL_COMM`](Self::FI_LOCAL_COMM), for example NICs that lack loopback support, cannot be used to communicate with processes on the same system.
    FI_REMOTE_COMM,
    /// Indicates that the user requires an endpoint capable of receiving read memory operations from remote endpoints. This flag requires that [`FI_RMA`](Self::FI_RMA) and/or [`FI_ATOMIC`](Self::FI_ATOMIC) be set.
    FI_REMOTE_READ,
    /// Indicates that the user requires an endpoint capable of receiving write memory operations from remote endpoints. This flag requires that [`FI_RMA`](Self::FI_RMA) and/or [`FI_ATOMIC`](Self::FI_ATOMIC) be set.
    FI_REMOTE_WRITE,
    /// Specifies that the endpoint should support RMA read and write operations. Endpoints supporting this capability support operations defined by struct `fi_ops_rma`. In the absence of any relevant flags, [`FI_RMA`](Self::FI_RMA) implies the ability to initiate and be the target of remote memory reads and writes. Applications can use the [`FI_READ`](Self::FI_READ), [`FI_WRITE`](Self::FI_WRITE), [`FI_REMOTE_READ`](Self::FI_REMOTE_READ), and [`FI_REMOTE_WRITE`](Self::FI_REMOTE_WRITE) flags to restrict the types of RMA operations supported by an endpoint.
    FI_RMA,
    /// Requests that an endpoint support the generation of completion events when it is the target of an RMA and/or atomic operation. This flag requires that [`FI_REMOTE_READ`](Self::FI_REMOTE_READ) and/or [`FI_REMOTE_WRITE`](Self::FI_REMOTE_WRITE) be enabled on the endpoint.
    FI_RMA_EVENT,
    /// Indicates that the provider is ‘persistent memory aware’ and supports RMA operations to and from persistent memory. Persistent memory aware providers must support registration of memory that is backed by non- volatile memory, RMA transfers to/from persistent memory, and enhanced completion semantics. This flag requires that [`FI_RMA`](Self::FI_RMA) be set. This capability is experimental.
    FI_RMA_PMEM,
    /// Indicates that the user requires an endpoint capable of sending message data transfers. Message transfers include base message operations as well as tagged message functionality.
    FI_SEND,
    /// Requests or indicates support for address vectors which may be shared among multiple processes.
    FI_SHARED_AV,
    /// Requests that the endpoint return source addressing data as part of its completion data. This capability only applies to connectionless endpoints. Note that returning source address information may require that the provider perform address translation and/or look-up based on data available in the underlying protocol in order to provide the requested data, which may adversely affect performance. The performance impact may be greater for address vectors of type `FI_AV_TABLE`.
    FI_SOURCE,
    /// Must be paired with [`FI_SOURCE`](Self::FI_SOURCE). When specified, this requests that raw source addressing data be returned as part of completion data for any address that has not been inserted into the local address vector. Use of this capability may require the provider to validate incoming source address data against addresses stored in the local address vector, which may adversely affect performance.
    FI_SOURCE_ERR,
    /// Specifies that the endpoint should handle tagged message transfers. Tagged message transfers associate a user-specified key or tag with each message that is used for matching purposes at the remote side. Endpoints supporting this capability support operations defined by struct `fi_ops_tagged`. In the absence of any relevant flags, [`FI_TAGGED`](Self::FI_TAGGED) implies the ability to send and receive tagged messages. Applications can use the [`FI_SEND`](Self::FI_SEND) and [`FI_RECV`](Self::FI_RECV) flags to optimize an endpoint as send-only or receive-only.
    FI_TAGGED,
    /// Indicates that the endpoint should support triggered operations. Endpoints support this capability must meet the usage model as described by `fi_trigger(3)`.
    FI_TRIGGER,
    /// Indicates that the user requires an endpoint capable of initiating writes against remote memory regions. This flag requires that [`FI_RMA`](Self::FI_RMA) and/or [`FI_ATOMIC`](Self::FI_ATOMIC) be set.
    FI_WRITE,
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
    pub fn new() -> Option<Self> {
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
    pub fn try_clone(&self) -> Option<FiInfo> {
        let ptr = unsafe { ffi::fi_dupinfo(self.as_ptr()) };
        NonNull::new(ptr).map(FiInfo)
    }

    fn as_ref(&self) -> &ffi::fi_info {
        // SAFETY: `self` borrows a valid, initialized `fi_info`.
        unsafe { &*self.as_ptr() }
    }

    #[must_use]
    pub fn caps(&self) -> FiCaps {
        FiCaps::from_bits_retain(self.as_ref().caps)
    }
}
