// use crate::ASICamera2::BayerPattern;
//
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![feature(alloc_layout_extra)]
mod asicam;


use crate::asicam::ASICamera2::{ControlType, ImageType};
use crate::asicam::Camera;

fn main() {
    operate_qhy();
}

fn operate_qhy() {
    println!("Operating on qhy camera ... or i'll die trying");
}

fn operate_asi() {
    println!("Operating on asi camera ... or i'll die trying");
    let mut camera = asicam::acquire(0).unwrap();

    println!("{:?}", camera);
    camera.set_control_value(ControlType::TargetTemp, -100).unwrap();
    camera.set_control_value(ControlType::CoolerOn, 1).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(500));
    println!("Camera temperature is currently {:?}", camera.get_control_value(ControlType::Temperature).unwrap());

    /*
    for exposure in [2000, 5000, 10000, 30000].iter() {
        camera.set_control_value(ControlType::Exposure, *exposure).unwrap();
        for gain in [450, 375, 325, 250, 200].iter() {
            camera.set_control_value(ControlType::Gain, *gain).unwrap();
            for offset in [100, 80, 60, 40, 20, 0].iter() {
                camera.set_control_value(ControlType::Offset, *offset).unwrap();
                take_calibration_images(&camera, 1, &format!("roof_gain_{:03}_offset_{:03}_exposure_{:06}", gain, offset, exposure));
            }
        }
    }
    */
    camera.set_exposure_ms(45000).unwrap();
//    camera.set_control_value(ControlType::Exposure, 70000000).unwrap();
    camera.set_control_value(ControlType::Gain, 350).unwrap();
    camera.set_control_value(ControlType::Offset, 0).unwrap();
    camera.set_control_value(ControlType::HardwareBin, 0).unwrap();
    camera.set_roi_format(camera.width, camera.height, 1, ImageType::RGB24).unwrap();
    take_calibration_images(&camera, 40, "dark_gain_350_exposure_45000");
    /*
    for exposure in [1000 * 1000 * 10].iter() {
        camera.set_control_value(ControlType::Exposure, *exposure).unwrap();
        for gain in [450, 375, 325, 250, 200].iter() {
            camera.set_control_value(ControlType::Gain, *gain).unwrap();
            for offset in [100, 80, 70, 60, 40, 0].iter() {
                camera.set_control_value(ControlType::Offset, *offset).unwrap();
                take_calibration_images(
                    &camera,
                    30,
                    &format!("images/gain_{:03}_offset_{:03}_exposure_{:06}", gain, offset, exposure));
            }
        }
    }
    */

    println!("Done!");
}

fn take_calibration_images(camera: &Camera, count: u32, path_fragment: &str) {
    for i in 0..count {
        println!("{} image {:06}", path_fragment,  i);
        let temp = camera.get_control_value(ControlType::Temperature).unwrap();
        println!("Camera temperature is currently {:?}", temp);
        camera.take_image(&format!("{}_{:06}_temp_{:03}.png", path_fragment, i, temp)).unwrap();
    }
}
