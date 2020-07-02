use anyhow::Result;
use std::time::Duration;
use std::time::Instant;

fn print_frame_info(frame: &mvs::Frame, fps: f64) {
    let frame_info = frame.as_ref();
    println!(
        "{:.4} fps, width: {}, height: {}, pixel format: {:?}, payload size: {}",
        fps,
        frame_info.width(),
        frame_info.height(),
        frame_info.pixel_format(),
        frame_info.payload_size(),
    )
}

fn main() -> Result<()> {
    let tls = dbg!(mvs::enumerate_tls());
    let device_info_list = mvs::enumerate_devices(&tls)?;
    println!("device num: {}", device_info_list.len());
    for device_info in device_info_list {
        assert!(device_info.is_device_accesible(mvs::AccessMode::Exclusive));
        let handle = mvs::DeviceHandle::new(device_info)?;
        handle.open(mvs::AccessMode::Exclusive)?;
        handle.start_grabbing()?;
        let start = Instant::now();
        println!("payload size: {}", handle.payload_size());
        for num in 1..10 {
            let frame = handle.get_one_frame(Duration::from_millis(1000))?;
            // let frame = handle
            //     .get_image_for_bgr(Duration::from_millis(1000))
            //     .unwrap();
            let fps = num as f64 / start.elapsed().as_secs_f64();
            print_frame_info(&frame, fps);
        }
        handle.stop_grabbing()?;
        handle.close()?;
    }
    Ok(())
}
