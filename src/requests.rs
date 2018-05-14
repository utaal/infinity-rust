use ffi;

pub struct RequestResult {
    pub buffer: ::memory::Buffer,
    pub immediate: Option<u32>,
}

pub struct RequestToken {
    pub(crate) _request_token: Box<ffi::infinity::requests::RequestToken>,
}

impl RequestToken {
    pub fn wait_until_completed(mut self) -> RequestResult {
        unsafe {
            self._request_token.waitUntilCompleted();
            assert!(self._request_token.wasSuccessful());
            let buffer = ::memory::Buffer::from_raw(
                ::std::mem::transmute(self._request_token.getRegion()));
            let immediate = if self._request_token.hasImmediateValue() {
                Some(self._request_token.getImmediateValue())
            } else {
                None
            };
            RequestResult {
                buffer,
                immediate,
            }
        }
    }
}
