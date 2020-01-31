#[derive(Debug, Clone, Copy)]
pub enum AccessMode {
    Exclusive,
    ExclusiveWithSwitch,
    Control,
    ControlWithSwitch,
    ControlSwitchEnable,
    ControlSwitchEnableWithKey,
    Monitor,
}

impl From<u32> for AccessMode {
    fn from(value: u32) -> Self {
        match value {
            mvs_sys::MV_ACCESS_Exclusive => AccessMode::Exclusive,
            mvs_sys::MV_ACCESS_ExclusiveWithSwitch => AccessMode::ExclusiveWithSwitch,
            mvs_sys::MV_ACCESS_Control => AccessMode::Control,
            mvs_sys::MV_ACCESS_ControlWithSwitch => AccessMode::ControlWithSwitch,
            mvs_sys::MV_ACCESS_ControlSwitchEnable => AccessMode::ControlSwitchEnable,
            mvs_sys::MV_ACCESS_ControlSwitchEnableWithKey => AccessMode::ControlSwitchEnableWithKey,
            mvs_sys::MV_ACCESS_Monitor => AccessMode::Monitor,
            code => panic!("Undefined access mode code: {}", code),
        }
    }
}

impl From<AccessMode> for u32 {
    fn from(access_mode: AccessMode) -> Self {
        match access_mode {
            AccessMode::Exclusive => mvs_sys::MV_ACCESS_Exclusive,
            AccessMode::ExclusiveWithSwitch => mvs_sys::MV_ACCESS_ExclusiveWithSwitch,
            AccessMode::Control => mvs_sys::MV_ACCESS_Control,
            AccessMode::ControlWithSwitch => mvs_sys::MV_ACCESS_ControlWithSwitch,
            AccessMode::ControlSwitchEnable => mvs_sys::MV_ACCESS_ControlSwitchEnable,
            AccessMode::ControlSwitchEnableWithKey => mvs_sys::MV_ACCESS_ControlSwitchEnableWithKey,
            AccessMode::Monitor => mvs_sys::MV_ACCESS_Monitor,
        }
    }
}
