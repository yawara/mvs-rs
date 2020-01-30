use std::os::raw::c_uint;

pub fn get_sdk_version() -> c_uint {
    unsafe { mvs_sys::MV_CC_GetSDKVersion() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn print_sdk_version() {
        println!("{}", get_sdk_version())
    }
}
