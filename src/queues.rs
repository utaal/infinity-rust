use ffi;

#[derive(Clone, Copy)]
pub struct SendOptions {
    fenced: bool,
    inlined: bool,
    local_offset: u64,
    size_in_bytes: Option<u32>,
}

impl Default for SendOptions {
    fn default() -> Self {
        SendOptions {
            fenced: false,
            inlined: false,
            local_offset: 0u64,
            size_in_bytes: None,
        }
    }
}

impl SendOptions {
    pub fn fenced(mut self, fenced: bool) -> Self {
        self.fenced = fenced;
        self
    }

    pub fn inlined(mut self, inlined: bool) -> Self {
        self.inlined = inlined;
        self
    }

    pub fn local_offset(mut self, local_offset: u64) -> Self {
        self.local_offset = local_offset;
        self
    }

    pub fn size_in_bytes(mut self, size_in_bytes: Option<u32>) -> Self {
        self.size_in_bytes = size_in_bytes;
        self
    }
}

#[derive(Clone, Copy)]
pub struct OneSidedOptions {
    fenced: bool,
    inlined: bool,
    local_offset: u64,
    remote_offset: u64,
    size_in_bytes: Option<u32>,
}

impl Default for OneSidedOptions {
    fn default() -> Self {
        OneSidedOptions {
            fenced: false,
            inlined: false,
            local_offset: 0u64,
            remote_offset: 0u64,
            size_in_bytes: None,
        }
    }
}

impl OneSidedOptions {
    pub fn fenced(mut self, fenced: bool) -> Self {
        self.fenced = fenced;
        self
    }

    pub fn inlined(mut self, inlined: bool) -> Self {
        self.inlined = inlined;
        self
    }

    pub fn local_offset(mut self, local_offset: u64) -> Self {
        self.local_offset = local_offset;
        self
    }

    pub fn remote_offset(mut self, remote_offset: u64) -> Self {
        self.remote_offset = remote_offset;
        self
    }

    pub fn size_in_bytes(mut self, size_in_bytes: Option<u32>) -> Self {
        self.size_in_bytes = size_in_bytes;
        self
    }
}

pub struct QueuePair<'a> {
    _queue_pair: *mut ffi::infinity::queues::QueuePair,
    context: &'a ::core::Context,
}

impl<'a> QueuePair<'a> {
    pub fn get_user_data(&mut self) -> &[u8] {
        unsafe {
            ::std::slice::from_raw_parts(
                (*self._queue_pair).userData as *const _,
                (*self._queue_pair).userDataSize as usize)
        }
    }

    #[inline]
    fn with_flags<T>(
        _queue_pair: *mut ffi::infinity::queues::QueuePair,
        fenced: bool,
        inlined: bool,
        inner: impl FnOnce()->T) -> T {

        unsafe {
            (*_queue_pair).defaultFlags =
                if fenced { ffi::root::ibv_send_flags_IBV_SEND_FENCE as i32 } else { 0 } |
                if inlined { ffi::root::ibv_send_flags_IBV_SEND_INLINE as i32 } else { 0 };
        }
        let result = (inner)();
        unsafe {
            (*_queue_pair).defaultFlags = 0;
        }
        result
    }

    pub fn send(
        &mut self,
        mut buffer: ::memory::Buffer,
        options: SendOptions) -> ::requests::RequestToken {

        QueuePair::with_flags(self._queue_pair, options.fenced, options.inlined, || unsafe {
            let mut _request_token = Box::new(
                ffi::infinity::requests::RequestToken::new(
                    &mut (*self.context._context.borrow_mut()) as *mut _));
            let buffer_size_in_bytes = buffer.get_size_in_bytes();
            let size_in_bytes = options.size_in_bytes.map(|x| x as u64).unwrap_or(buffer_size_in_bytes);
            assert!(size_in_bytes <= buffer_size_in_bytes,
                    "The requested operation size is larger than the buffer.");
            assert!(size_in_bytes <= (::std::u32::MAX as u64),
                "Request must be smaller or equal to UINT_32_MAX bytes. This memory region is larger. Please explicitly indicate the size of the data to transfer.");
            (*self._queue_pair).send2(
                buffer.into_raw(),
                options.local_offset,
                size_in_bytes as u32,
                &mut (*_request_token) as *mut _);
            ::requests::RequestToken {
                _request_token,
            }
        })
    }

    pub fn read(
        &mut self,
        mut buffer: ::memory::Buffer,
        region_token: &::memory::RegionToken,
        options: OneSidedOptions) -> ::requests::RequestToken {

        QueuePair::with_flags(self._queue_pair, options.fenced, options.inlined, || unsafe {
            let mut _request_token = Box::new(
                ffi::infinity::requests::RequestToken::new(
                    &mut (*self.context._context.borrow_mut()) as *mut _));
            let buffer_size_in_bytes = buffer.get_size_in_bytes();
            let size_in_bytes = options.size_in_bytes.map(|x| x as u64).unwrap_or(buffer_size_in_bytes);
            assert!(size_in_bytes <= buffer_size_in_bytes,
                    "The requested operation size is larger than the buffer.");
            assert!(size_in_bytes <= (::std::u32::MAX as u64),
                "Request must be smaller or equal to UINT_32_MAX bytes. This memory region is larger. Please explicitly indicate the size of the data to transfer.");
            (*self._queue_pair).read2(
                buffer.into_raw(),
                options.local_offset,
                region_token._region_token,
                options.remote_offset,
                size_in_bytes as u32,
                &mut (*_request_token) as *mut _);
            ::requests::RequestToken {
                _request_token,
            }
        })
    }

    pub fn write(
        &mut self,
        mut buffer: ::memory::Buffer,
        region_token: &::memory::RegionToken,
        options: OneSidedOptions) -> ::requests::RequestToken {

        QueuePair::with_flags(self._queue_pair, options.fenced, options.inlined, || unsafe {
            let mut _request_token = Box::new(
                ffi::infinity::requests::RequestToken::new(
                    &mut (*self.context._context.borrow_mut()) as *mut _));
            let buffer_size_in_bytes = buffer.get_size_in_bytes();
            let size_in_bytes = options.size_in_bytes.map(|x| x as u64).unwrap_or(buffer_size_in_bytes);
            assert!(size_in_bytes <= buffer_size_in_bytes,
                    "The requested operation size is larger than the buffer.");
            assert!(size_in_bytes <= (::std::u32::MAX as u64),
                "Request must be smaller or equal to UINT_32_MAX bytes. This memory region is larger. Please explicitly indicate the size of the data to transfer.");
            (*self._queue_pair).write2(
                buffer.into_raw(),
                options.local_offset,
                region_token._region_token,
                options.remote_offset,
                size_in_bytes as u32,
                &mut (*_request_token) as *mut _);
            ::requests::RequestToken {
                _request_token,
            }
        })
    }
}

impl<'a> Drop for QueuePair<'a> {
    fn drop(&mut self) {
        unsafe {
            ffi::infinityhelpers::queues::delete_QueuePair(self._queue_pair);
        }
    }
}

pub struct QueuePairFactory<'a> {
    _queue_pair_factory: ffi::infinity::queues::QueuePairFactory,
    context: &'a ::core::Context,
}

impl<'a> QueuePairFactory<'a> {
    pub fn new(context: &'a ::core::Context) -> QueuePairFactory<'a> {
        unsafe {
            QueuePairFactory {
                _queue_pair_factory: ffi::infinity::queues::QueuePairFactory::new(
                    &mut (*context._context.borrow_mut()) as *mut _),
                context,
            }
        }
    }

    pub fn bind_to_port(&mut self, port: u16) {
        unsafe {
            self._queue_pair_factory.bindToPort(port);
        }
    }

    pub fn accept_incoming_connection<'b>(&'b mut self, user_data: &[u8]) -> QueuePair<'b> where 'a: 'b {
        let _queue_pair = unsafe {
            self._queue_pair_factory.acceptIncomingConnection(
                user_data.as_ptr() as *mut ::std::os::raw::c_void,
                user_data.len() as u32)
        };

        QueuePair {
            _queue_pair,
            context: self.context,
        }
    }

    pub fn connect_to_remote_host<'b>(
        &mut self,
        addr: impl ::std::net::ToSocketAddrs,
        user_data: &[u8]) -> QueuePair<'b> where 'a: 'b {

        let addr = addr.to_socket_addrs().expect("Invalid socket address")
            .next().expect("Missing socket address");

        let _queue_pair = unsafe {
            self._queue_pair_factory.connectToRemoteHost(
                ::std::ffi::CString::new(format!("{}", addr.ip())).unwrap().as_ptr(),
                addr.port(),
                user_data.as_ptr() as *mut ::std::os::raw::c_void,
                user_data.len() as u32)
        };

        QueuePair {
            _queue_pair,
            context: self.context,
        }
    }

}

impl<'a> Drop for QueuePairFactory<'a> {
    fn drop(&mut self) {
        unsafe {
            self._queue_pair_factory.destruct();
        }
    }
}
