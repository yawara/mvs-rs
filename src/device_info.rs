use crate::error::*;
use std::mem;

#[derive(Debug, Clone, Copy)]
pub enum TransportLayerType {
    UnknwonDevice,
    GigeDevice,
    UsbDevice,
    CameraLinkDevice,
}

pub struct DeviceInfo {
    raw: *mut mvs_sys::MV_CC_DEVICE_INFO,
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

impl DeviceInfo {
    pub fn is_device_accesible(&self, mode: AccessMode) -> bool {
        unsafe { mvs_sys::MV_CC_IsDeviceAccessible(self.raw, mode as u32).is_positive() }
    }

    pub(crate) fn raw(&self) -> *mut mvs_sys::MV_CC_DEVICE_INFO {
        self.raw
    }
}

pub fn enumerate_tls() -> Vec<TransportLayerType> {
    unsafe {
        let supports = mvs_sys::MV_CC_EnumerateTls();
        todo!()
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
