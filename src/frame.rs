use std::ffi::c_void;
use std::mem::MaybeUninit;
use std::path::Path;

pub struct Frame {
    raw: mvs_sys::MV_FRAME_OUT,
    raw_handle: *mut c_void,
}

impl Frame {
    pub fn display(&self) {
        let mut display_info: mvs_sys::MV_DISPLAY_FRAME_INFO =
            unsafe { MaybeUninit::zeroed().assume_init() };
        todo!()
    }

    pub(crate) fn new(raw: mvs_sys::MV_FRAME_OUT, raw_handle: *mut c_void) -> Self {
        Self { raw, raw_handle }
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        // TODO: error handling???
        unsafe {
            mvs_sys::MV_CC_FreeImageBuffer(
                self.raw_handle,
                &mut self.raw as *mut mvs_sys::MV_FRAME_OUT,
            );
        }
    }
}
