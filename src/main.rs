extern crate mvs;

use std::thread::sleep;
use std::time::Duration;

fn main() {
    let a = Vec::new();
    let device_info_list = mvs::enumerate_devices(&a).unwrap();
    println!("device num: {}", device_info_list.len());
    for device_info in device_info_list {
        let handle = mvs::DeviceHandle::new(device_info).unwrap();
        handle.open(mvs::AccessMode::Exclusive).unwrap();
        handle.start_grabbing().unwrap();
        let frame = handle
            .get_image_buffer(Duration::from_millis(1000))
            .unwrap();
        sleep(Duration::from_secs(5));
        handle.stop_grabbing().unwrap();
        handle.close().unwrap();
    }
}
