use ffi;

pub struct ReceiveElement {
    buffer: Option<(::memory::Buffer, usize)>,
    immediate: Option<u32>,
}

impl ReceiveElement {
    fn from_receive_element_t(recv: ffi::infinity::core::receive_element_t) -> Self {
        unsafe {
            ReceiveElement {
                buffer: Some((
                    ::memory::Buffer::from_raw(recv.buffer),
                    recv.bytesWritten as usize)),
                immediate: if recv.immediateValueValid {
                    Some(recv.immediateValue)
                } else {
                    None
                },
            }
        }
    }
}

pub struct Context {
    pub(crate) _context: ffi::infinity::core::Context,
}

impl Context {
    pub fn new(device_id: u16, device_port: u16) -> Self {
        unsafe {
            Context {
                _context: ffi::infinity::core::Context::new(device_id, device_port),
            }
        }
    }

    pub fn receive(&mut self) -> Option<ReceiveElement> {
        unsafe {
            let mut receive_element: ffi::infinity::core::receive_element_t = ::std::mem::zeroed();
            if self._context.receive(&mut receive_element as *mut _) {
                Some(ReceiveElement::from_receive_element_t(receive_element))
            } else {
                None
            }
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            self._context.destruct();
        }
    }
}
