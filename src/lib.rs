extern crate mvs_sys;

#[macro_use]
pub mod error;

use crate::error::*;
use std::ffi::c_void;
use std::mem;
use std::os::raw::c_uint;
use std::time::Duration;

pub fn get_sdk_version() -> c_uint {
    unsafe { mvs_sys::MV_CC_GetSDKVersion() }
}

#[derive(Debug, Clone, Copy)]
pub enum TransportLayerType {
    UnknwonDevice,
    GigeDevice,
    UsbDevice,
    CameraLinkDevice,
}

pub fn enumerate_tls() -> Vec<TransportLayerType> {
    unsafe {
        let supports = mvs_sys::MV_CC_EnumerateTls();
        todo!()
    }
}

pub struct DeviceInfo {
    raw: *mut mvs_sys::MV_CC_DEVICE_INFO,
}

impl DeviceInfo {
    pub fn is_device_accesible(&self, mode: AccessMode) -> bool {
        unsafe { mvs_sys::MV_CC_IsDeviceAccessible(self.raw, mode as u32).is_positive() }
    }
}

pub fn enumerate_devices(tlayer_types: &[TransportLayerType]) -> Result<Vec<DeviceInfo>> {
    // TODO: support other devices
    let mut raw: mvs_sys::MV_CC_DEVICE_INFO_LIST =
        unsafe { mem::MaybeUninit::zeroed().assume_init() };
    try_unsafe!(mvs_sys::MV_CC_EnumDevices(
        mvs_sys::MV_USB_DEVICE,
        &mut raw as *mut mvs_sys::MV_CC_DEVICE_INFO_LIST,
    ));
    let mut device_info_list = Vec::new();
    for device_num in 0..raw.nDeviceNum {
        let device_info = DeviceInfo {
            raw: raw.pDeviceInfo[device_num as usize],
        };
        device_info_list.push(device_info);
    }
    Ok(device_info_list)
}

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

pub enum AccessMode {
    Exclusive = mvs_sys::MV_ACCESS_Exclusive as isize,
    ExclusiveWithSwitch = mvs_sys::MV_ACCESS_ExclusiveWithSwitch as isize,
    Control = mvs_sys::MV_ACCESS_Control as isize,
    ControlWithSwitch = mvs_sys::MV_ACCESS_ControlWithSwitch as isize,
    ControlSwitchEnable = mvs_sys::MV_ACCESS_ControlSwitchEnable as isize,
    ControlSwitchEnableWithKey = mvs_sys::MV_ACCESS_ControlSwitchEnableWithKey as isize,
    Monitor = mvs_sys::MV_ACCESS_Monitor as isize,
}

pub struct Frame {
    raw: mvs_sys::MV_FRAME_OUT,
    raw_handle: *mut c_void,
}

impl Frame {}

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

impl DeviceHandle {
    pub fn new(device_info: DeviceInfo) -> Result<Self> {
        let mut raw_handle: *mut c_void = unsafe { mem::MaybeUninit::uninit().assume_init() };
        try_unsafe! {
            mvs_sys::MV_CC_CreateHandle(&mut raw_handle, device_info.raw)
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
        let frame = Frame {
            raw_handle: self.raw,
            raw: raw_frame,
        };
        Ok(frame)
    }

    pub fn display_one_frame(&self) -> Result<()> {
        // try_unsafe! {
        //     mvs_sys::MV_CC_DisplayOneFrame(handle: *mut ::std::os::raw::c_void, pstDisplayInfo: *mut MV_DISPLAY_FRAME_INFO)
        // }
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn print_sdk_version() {
        println!("{}", get_sdk_version())
    }
}
