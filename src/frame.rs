use std::ffi::c_void;
use std::mem::MaybeUninit;
use std::ops::Deref;

pub struct Frame {
    buf: Vec<u8>,
    frame_info: FrameInfo,
}

pub struct FrameInfo {
    raw: mvs_sys::MV_FRAME_OUT_INFO_EX,
}

impl Frame {
    pub(crate) fn new(buf: Vec<u8>, frame_info: FrameInfo) -> Self {
        Self { buf, frame_info }
    }
}

impl AsRef<FrameInfo> for Frame {
    fn as_ref(&self) -> &FrameInfo {
        &self.frame_info
    }
}

impl Deref for Frame {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.buf
    }
}

impl FrameInfo {
    pub(crate) fn new() -> Self {
        let raw = unsafe { MaybeUninit::zeroed().assume_init() };
        FrameInfo { raw }
    }

    pub(crate) fn as_mut_ptr(&mut self) -> *mut mvs_sys::MV_FRAME_OUT_INFO_EX {
        &mut self.raw as *mut mvs_sys::MV_FRAME_OUT_INFO_EX
    }

    pub fn width(&self) -> u32 {
        self.raw.nWidth as u32
    }

    pub fn height(&self) -> u32 {
        self.raw.nHeight as u32
    }

    pub fn pixel_format(&self) -> i64 {
        self.raw.enPixelType
    }
}

pub struct MvFrame {
    raw: mvs_sys::MV_FRAME_OUT,
    raw_handle: *mut c_void,
}

impl MvFrame {
    pub fn display(&self) {
        todo!()
    }

    pub(crate) fn new(raw: mvs_sys::MV_FRAME_OUT, raw_handle: *mut c_void) -> Self {
        Self { raw, raw_handle }
    }
}

impl Drop for MvFrame {
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
