use ffi;

struct QueuePair<'a> {
    _queue_pair: *mut ffi::infinity::queues::QueuePair,
    _phantom: ::std::marker::PhantomData<&'a ()>,
}

impl<'a> Drop for QueuePair<'a> {
    fn drop(&mut self) {
        unsafe {
            ffi::infinityhelpers::queues::delete_QueuePair(self._queue_pair);
        }
    }
}

struct QueuePairFactory<'a> {
    _queue_pair_factory: ffi::infinity::queues::QueuePairFactory,
    _phantom: ::std::marker::PhantomData<&'a ()>,
}

impl<'a> QueuePairFactory<'a> {
    fn new(context: &'a mut ::core::Context) -> QueuePairFactory<'a> {
        unsafe {
            QueuePairFactory {
                _queue_pair_factory: ffi::infinity::queues::QueuePairFactory::new(
                    &mut context._context as *mut _),
                _phantom: ::std::marker::PhantomData,
            }
        }
    }

    fn bind_to_port(&mut self, port: u16) {
        unsafe {
            self._queue_pair_factory.bindToPort(port);
        }
    }

    fn accept_incoming_connection<'b>(&mut self, user_data: &[u8]) -> QueuePair<'b> where 'a: 'b {
        let _queue_pair = unsafe {
            self._queue_pair_factory.acceptIncomingConnection(
                user_data.as_ptr() as *mut ::std::os::raw::c_void,
                user_data.len() as u32)
        };

        QueuePair {
            _queue_pair,
            _phantom: ::std::marker::PhantomData,
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
