use ffi;

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

    pub fn send(&mut self, buffer: ::memory::Buffer) -> ::requests::RequestToken {
        unsafe {
            let mut _request_token = Box::new(
                ffi::infinity::requests::RequestToken::new(
                    &mut (*self.context._context.borrow_mut()) as *mut _));
            (*self._queue_pair).send(buffer.into_raw(), &mut (*_request_token) as *mut _);
            ::requests::RequestToken {
                _request_token,
            }
        }
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
