extern crate mvs;

use std::time::Duration;
use std::time::Instant;

fn print_frame_info(frame: &mvs::Frame, fps: f64) {
    let frame_info = frame.as_ref();
    println!(
        "{} fps, width: {}, height: {}",
        fps,
        frame_info.width(),
        frame_info.height()
    )
}

fn main() {
    let a = Vec::new();
    let device_info_list = mvs::enumerate_devices(&a).unwrap();
    println!("device num: {}", device_info_list.len());
    for device_info in device_info_list {
        assert!(device_info.is_device_accesible(mvs::AccessMode::Exclusive));
        let handle = mvs::DeviceHandle::new(device_info).unwrap();
        handle.open(mvs::AccessMode::Exclusive).unwrap();
        handle.start_grabbing().unwrap();
        let start = Instant::now();
        let mut num = 0;
        for _ in 0..100 {
            //let frame = handle.get_one_frame(Duration::from_millis(1000)).unwrap();
            let frame = handle
                .get_image_for_bgr(Duration::from_millis(1000))
                .unwrap();
            num += 1;
            let fps = num as f64 / start.elapsed().as_secs_f64();
            print_frame_info(&frame, fps);
        }
        handle.stop_grabbing().unwrap();
        handle.close().unwrap();
    }
}
