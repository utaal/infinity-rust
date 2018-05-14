use std::cell::UnsafeCell;

use ffi;

pub struct Buffer {
    _buffer: UnsafeCell<Box<ffi::infinity::memory::Buffer>>,
}

impl Buffer {
    pub fn new(context: &mut ::core::Context, size: u64) -> Self {
        unsafe {
            Buffer {
                _buffer: UnsafeCell::new(Box::new(ffi::infinity::memory::Buffer::new(
                    &mut context._context as *mut _, size))),
            }
        }
    }

    pub(crate) unsafe fn from_raw(buffer: *mut ffi::infinity::memory::Buffer) -> Self {
        Buffer {
            _buffer: UnsafeCell::new(Box::from_raw(buffer)),
        }
    }

    unsafe fn as_region_ptr(&self) -> *mut ffi::infinity::memory::Region {
        ::std::mem::transmute::<_, *mut ffi::infinity::memory::Region>(
            self._buffer.get())
    }
}

impl ::std::ops::Deref for Buffer {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        unsafe {
            // let this = ::std::mem::transmute::<_, &mut Self>(self);
            ::std::slice::from_raw_parts(
                ::std::mem::transmute::<_, *const u8>((*self._buffer.get()).getData()),
                (*self.as_region_ptr()).getSizeInBytes() as usize)
        }
    }
}

impl ::std::ops::DerefMut for Buffer {
    fn deref_mut(&mut self) -> &mut[u8] {
        unsafe {
            ::std::slice::from_raw_parts_mut(
                ::std::mem::transmute::<_, *mut u8>((*self._buffer.get()).getData()),
                (*self.as_region_ptr()).getSizeInBytes() as usize)
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            ffi::infinity::memory::Buffer_Buffer_destructor((*self._buffer.get()).as_mut() as *mut _);
        }
    }
}

