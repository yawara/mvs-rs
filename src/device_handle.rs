use crate::device_info::{AccessMode, DeviceInfo};
use crate::error::*;
use crate::frame::Frame;
use std::ffi::c_void;
use std::mem;
use std::time::Duration;

pub struct DeviceHandle {
    raw: *mut c_void,
}

impl Drop for DeviceHandle {
    fn drop(&mut self) {
        // TODO: error handling???
        unsafe {
            mvs_sys::MV_CC_DestroyHandle(self.raw);
        }
    }
}

impl DeviceHandle {
    pub fn new(device_info: DeviceInfo) -> Result<Self> {
        let mut raw_handle: *mut c_void = unsafe { mem::MaybeUninit::uninit().assume_init() };
        try_unsafe! {
            mvs_sys::MV_CC_CreateHandle(&mut raw_handle, device_info.raw())
        }
        Ok(DeviceHandle { raw: raw_handle })
    }

    pub fn open(&self, mode: AccessMode) -> Result<()> {
        try_unsafe! {
            mvs_sys::MV_CC_OpenDevice(self.raw, mode as u32, 0)
        }
        Ok(())
    }
    pub fn close(&self) -> Result<()> {
        try_unsafe! {
            mvs_sys::MV_CC_CloseDevice(self.raw)
        }
        Ok(())
    }

    pub fn start_grabbing(&self) -> Result<()> {
        try_unsafe! {
            mvs_sys::MV_CC_StartGrabbing(self.raw)
        }
        Ok(())
    }

    pub fn stop_grabbing(&self) -> Result<()> {
        try_unsafe! {
            mvs_sys::MV_CC_StopGrabbing(self.raw)
        }
        Ok(())
    }

    pub fn get_image_buffer(&self, wait: Duration) -> Result<Frame> {
        let mut raw_frame: mvs_sys::MV_FRAME_OUT =
            unsafe { mem::MaybeUninit::zeroed().assume_init() };
        try_unsafe! {
          mvs_sys::MV_CC_GetImageBuffer(self.raw, &mut raw_frame as *mut mvs_sys::MV_FRAME_OUT, wait.as_millis() as u32)
        }
        let frame = Frame::new(raw_frame, self.raw);
        Ok(frame)
    }
}
