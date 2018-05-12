use ffi;

struct ReceiveElement {
    _receive_element: ffi::infinity::core::receive_element_t,
}

struct Context {
    _context: ffi::infinity::core::Context,
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
                Some(ReceiveElement {
                    _receive_element: receive_element,
                })
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
