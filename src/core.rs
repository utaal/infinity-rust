use std::cell::RefCell;

use ffi;

pub struct ReceiveElement {
    pub buffer: (::memory::Buffer, usize),
    pub immediate: Option<u32>,
}

impl ReceiveElement {
    fn from_receive_element_t(recv: ffi::infinity::core::receive_element_t) -> Self {
        unsafe {
            ReceiveElement {
                buffer: (
                    ::memory::Buffer::from_raw(recv.buffer),
                    recv.bytesWritten as usize),
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
    pub(crate) _context: RefCell<ffi::infinity::core::Context>,
}

impl Context {
    pub fn new(device_id: u16, device_port: u16) -> Self {
        unsafe {
            Context {
                _context: RefCell::new(
                    ffi::infinity::core::Context::new(device_id, device_port)),
            }
        }
    }

    pub fn receive(&self) -> Option<ReceiveElement> {
        unsafe {
            let mut receive_element: ffi::infinity::core::receive_element_t = ::std::mem::zeroed();
            if self._context.borrow_mut().receive(&mut receive_element as *mut _) {
                Some(ReceiveElement::from_receive_element_t(receive_element))
            } else {
                None
            }
        }
    }

    pub fn post_receive_buffer(&self, buffer: ::memory::Buffer) {
        unsafe {
            let raw_buffer = buffer.into_raw();
            self._context.borrow_mut().postReceiveBuffer(raw_buffer);
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            self._context.borrow_mut().destruct();
        }
    }
}
