use crate::device_info::{AccessMode, DeviceInfo};
use crate::error::*;
use crate::frame::{Frame, FrameInfo, MvFrame};
use std::cell::Cell;
use std::convert::TryInto;
use std::ffi::{c_void, CString};
use std::mem::MaybeUninit;
use std::time::Duration;

pub struct DeviceHandle {
    raw: *mut c_void,
    payload_size: Cell<i64>,
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
        let mut raw_handle: *mut c_void = unsafe { MaybeUninit::uninit().assume_init() };
        try_unsafe! {
            mvs_sys::MV_CC_CreateHandle(&mut raw_handle, device_info.raw())
        }
        Ok(DeviceHandle {
            raw: raw_handle,
            payload_size: Cell::new(0),
        })
    }

    pub fn payload_size(&self) -> usize {
        self.payload_size.get().try_into().unwrap()
    }

    pub fn open(&self, mode: AccessMode) -> Result<()> {
        try_unsafe! {
            mvs_sys::MV_CC_OpenDevice(self.raw, mode as u32, 0)
        }
        let mut mvcc_int: mvs_sys::MVCC_INTVALUE_EX =
            unsafe { MaybeUninit::zeroed().assume_init() };
        unsafe {
            mvs_sys::MV_CC_GetIntValueEx(
                self.raw,
                CString::new("PayloadSize").unwrap().as_ptr(),
                &mut mvcc_int as *mut mvs_sys::MVCC_INTVALUE_EX,
            );
        }
        self.payload_size.set(mvcc_int.nCurValue); //One frame data size + reserved bytes (handled in SDK)
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

    pub fn get_image_buffer(&self, wait: Duration) -> Result<MvFrame> {
        let mut raw_frame: mvs_sys::MV_FRAME_OUT = unsafe { MaybeUninit::zeroed().assume_init() };
        try_unsafe! {
          mvs_sys::MV_CC_GetImageBuffer(self.raw, &mut raw_frame as *mut mvs_sys::MV_FRAME_OUT, wait.as_millis() as u32)
        }
        let frame = MvFrame::new(raw_frame, self.raw);
        Ok(frame)
    }

    pub fn get_one_frame(&self, wait: Duration) -> Result<Frame> {
        let mut frame_info = FrameInfo::new();
        let payload_size = self.payload_size() + 2048;
        let mut buf: Vec<u8> = vec![0; payload_size];
        try_unsafe! {
            mvs_sys::MV_CC_GetOneFrameTimeout(
                self.raw,
                buf.as_mut_ptr(),
                buf.len() as u32,
                frame_info.as_mut_ptr(),
                wait.as_millis() as u32,
            )
        }
        Ok(Frame::new(buf, frame_info))
    }

    pub fn get_image_for_rgb(&self, wait: Duration) -> Result<Frame> {
        let mut frame_info = FrameInfo::new();
        let payload_size = self.payload_size() / 2 * 3 + 2048;
        let mut buf: Vec<u8> = vec![0; payload_size];
        try_unsafe! {
            mvs_sys::MV_CC_GetImageForRGB(
                self.raw,
                buf.as_mut_ptr(),
                buf.len() as u32,
                frame_info.as_mut_ptr(),
                wait.as_millis() as i32,
            )
        }
        Ok(Frame::new(buf, frame_info))
    }

    pub fn get_image_for_bgr(&self, wait: Duration) -> Result<Frame> {
        let mut frame_info = FrameInfo::new();
        let payload_size = self.payload_size() / 2 * 3 + 2048;
        let mut buf: Vec<u8> = vec![0; payload_size];
        try_unsafe! {
            mvs_sys::MV_CC_GetImageForBGR(
                self.raw,
                buf.as_mut_ptr(),
                buf.len() as u32,
                frame_info.as_mut_ptr(),
                wait.as_millis() as i32,
            )
        }
        Ok(Frame::new(buf, frame_info))
    }
}
