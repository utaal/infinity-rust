use std::cell::UnsafeCell;

use ffi;

pub struct RegionToken {
    _region_token: *mut ffi::infinity::memory::RegionToken,
    cxx_delete: bool,
}

impl RegionToken {
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            ::std::slice::from_raw_parts(
                ::std::mem::transmute(self._region_token),
                ::std::mem::size_of::<ffi::infinity::memory::RegionToken>())
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != ::std::mem::size_of::<ffi::infinity::memory::RegionToken>() {
            None
        } else {
            let _region_token: Box<ffi::infinity::memory::RegionToken> = unsafe {
                Box::new(::std::ptr::read(
                        ::std::mem::transmute::<_, *const ffi::infinity::memory::RegionToken>(bytes.as_ptr())))
            };
            Some(RegionToken {
                _region_token: Box::leak(_region_token),
                cxx_delete: false,
            })
        }
    }
}

impl Drop for RegionToken {
    fn drop(&mut self) {
        unsafe {
            if self.cxx_delete {
                ffi::infinityhelpers::memory::delete_RegionToken(self._region_token);
            } else {
                ::std::mem::drop(Box::from_raw(self._region_token));
            }
        }
    }
}

pub struct Buffer {
    _buffer: UnsafeCell<Box<ffi::infinity::memory::Buffer>>,
}

impl Buffer {
    pub fn new(context: & ::core::Context, size: u64) -> Self {
        unsafe {
            Buffer {
                _buffer: UnsafeCell::new(Box::new(ffi::infinity::memory::Buffer::new(
                    &mut (*context._context.borrow_mut()) as *mut _, size))),
            }
        }
    }

    pub(crate) unsafe fn from_raw(buffer: *mut ffi::infinity::memory::Buffer) -> Self {
        Buffer {
            _buffer: UnsafeCell::new(Box::from_raw(buffer)),
        }
    }

    pub(crate) unsafe fn into_raw(mut self) -> *mut ffi::infinity::memory::Buffer {
        // HERE BE DRAGONS
        let _buffer = ::std::mem::replace(
            &mut self._buffer,
            UnsafeCell::new(Box::from_raw(::std::ptr::null_mut())));
        ::std::mem::forget(self);
        Box::into_raw(_buffer.into_inner())
    }

    pub fn region_token(mut self) -> (UnsafeBuffer, RegionToken) {
        let region_token = unsafe {
            RegionToken {
                _region_token: (*self.as_region_ptr()).createRegionToken(),
                cxx_delete: true,
            }
        };
        let unsafe_buffer = unsafe {
            // HERE BE DRAGONS
            let _buffer = ::std::mem::replace(
                &mut self._buffer,
                UnsafeCell::new(Box::from_raw(::std::ptr::null_mut())));
            ::std::mem::forget(self);
            UnsafeBuffer {
                _buffer,
            }
        };
        (unsafe_buffer, region_token)
    }

    unsafe fn as_region_ptr(&self) -> *mut ffi::infinity::memory::Region {
        ::std::mem::transmute::<_, *mut ffi::infinity::memory::Region>(
            (*self._buffer.get()).as_mut())
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

pub struct UnsafeBuffer {
    _buffer: UnsafeCell<Box<ffi::infinity::memory::Buffer>>,
}

impl Drop for UnsafeBuffer {
    fn drop(&mut self) {
        unsafe {
            ffi::infinity::memory::Buffer_Buffer_destructor((*self._buffer.get()).as_mut() as *mut _);
        }
    }
}
