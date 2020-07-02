use crate::access_mode::AccessMode;
use crate::error::*;
use crate::transport_layer::TransportLayerType;

use std::mem;

#[derive(Debug)]
pub struct DeviceInfo {
    raw: *mut mvs_sys::MV_CC_DEVICE_INFO,
}

impl DeviceInfo {
    pub fn is_device_accesible(&self, mode: AccessMode) -> bool {
        unsafe { mvs_sys::MV_CC_IsDeviceAccessible(self.raw, mode.into()).is_positive() }
    }

    pub(crate) fn raw(&self) -> *mut mvs_sys::MV_CC_DEVICE_INFO {
        self.raw
    }
}

pub fn enumerate_devices(tlayer_types: &[TransportLayerType]) -> Result<Vec<DeviceInfo>> {
    let mut targets: u32 = 0;
    for tlayer_type in tlayer_types {
        targets |= Into::<u32>::into(*tlayer_type);
    }
    let mut raw: mvs_sys::MV_CC_DEVICE_INFO_LIST =
        unsafe { mem::MaybeUninit::zeroed().assume_init() };
    try_unsafe!(mvs_sys::MV_CC_EnumDevices(
        targets,
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
