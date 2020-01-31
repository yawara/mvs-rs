use std::convert::TryInto;

#[derive(Debug, Clone, Copy)]
pub enum TransportLayerType {
    UnknwonDevice,
    GigeDevice,
    UsbDevice,
    CameraLinkDevice,
}

macro_rules! check_tls {
    ($x: expr, $tl: ident, $tls: expr) => {
        if is_support($x, TransportLayerType::$tl) {
            $tls.push(TransportLayerType::$tl)
        }
    };
}

pub fn enumerate_tls() -> Vec<TransportLayerType> {
    unsafe {
        let mut tls = Vec::new();
        let raw: u32 = mvs_sys::MV_CC_EnumerateTls().try_into().unwrap();
        check_tls!(raw, UnknwonDevice, tls);
        check_tls!(raw, GigeDevice, tls);
        check_tls!(raw, UsbDevice, tls);
        check_tls!(raw, CameraLinkDevice, tls);
        tls
    }
}

fn is_support(value: u32, tlayer_type: TransportLayerType) -> bool {
    value & Into::<u32>::into(tlayer_type) != 0u32
}

impl From<u32> for TransportLayerType {
    fn from(value: u32) -> Self {
        match value {
            mvs_sys::MV_UNKNOW_DEVICE => Self::UnknwonDevice,
            mvs_sys::MV_USB_DEVICE => Self::UsbDevice,
            mvs_sys::MV_GIGE_DEVICE => Self::GigeDevice,
            mvs_sys::MV_CAMERALINK_DEVICE => Self::CameraLinkDevice,
            code => panic!("Undefined transport layer type code: {}", code),
        }
    }
}

impl From<TransportLayerType> for u32 {
    fn from(tlayer_type: TransportLayerType) -> Self {
        match tlayer_type {
            TransportLayerType::UnknwonDevice => mvs_sys::MV_UNKNOW_DEVICE,
            TransportLayerType::GigeDevice => mvs_sys::MV_GIGE_DEVICE,
            TransportLayerType::UsbDevice => mvs_sys::MV_USB_DEVICE,
            TransportLayerType::CameraLinkDevice => mvs_sys::MV_CAMERALINK_DEVICE,
        }
    }
}
