pub mod QHYCCDCam;

pub use self::QHYCCDCam::Control;

use self::QHYCCDCam::*;

use std::alloc::{alloc, dealloc, Layout};
use std::collections::HashMap;
use std::ffi::CStr;
use std::os;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use png::HasParameters;

#[derive(Debug)]
pub struct Camera {
    handle: *mut os::raw::c_void
}

#[derive(Debug, Copy, Clone)]
pub enum CameraError {
    QHYError, // unspecified error from the qhy sdk
    InvalidControl
}

type Result<T> = std::result::Result<T, CameraError>;

fn check(result: os::raw::c_int) -> Result<()> {
    match QHYResult::from(result as u32) {
        QHYResult::QHYCCD_SUCCESS => Ok(()),
        QHYResult::QHYCCD_ERROR => Err(CameraError::QHYError),
        a @ _ => {
            panic!("Unexpected result code from qhy sdk: {:?}", a);
        }
    }
}

static mut INITIALIZED: bool = false;

pub fn acquire(camera_idx: i32) -> Result<Camera> {
    unsafe {
        if !INITIALIZED {
            println!("Initializing QHYCCDResource");
            check(QHYCCDCam::InitQHYCCDResource())?;
            INITIALIZED = true;
        }
        let cameracount = QHYCCDCam::ScanQHYCCD();
        println!("Detected {} cameras", cameracount);
        if camera_idx >= cameracount {
            panic!("Camera id is invalid (detected {} cameras)", cameracount);
        }

        let mut id_space: [os::raw::c_char; 32] = [0; 32];
        check(QHYCCDCam::GetQHYCCDId(camera_idx, id_space.as_mut_ptr()))?;
        println!("Got camera id: {:?}", id_space);
        println!("One sec, trying again...");
        println!("How's this: {}", CStr::from_ptr(id_space.as_ptr()).to_str().unwrap());
        let handle: *mut os::raw::c_void = QHYCCDCam::OpenQHYCCD(id_space.as_mut_ptr());
        if handle == std::ptr::null_mut() {
            println!("Failed to open the device");
            return Err(CameraError::QHYError);
        }
        check(QHYCCDCam::SetQHYCCDStreamMode(handle, 0))?; // 0 means single frame mode...
        check(QHYCCDCam::InitQHYCCD(handle))?;
        check(QHYCCDCam::CancelQHYCCDExposingAndReadout(handle))?;
        Ok(Camera {
            handle: handle
        })
    }
}

impl Camera {
    pub fn set_exposure_ms(&self, ms: u32) -> Result<()> {
        self.set_param(Control::Exposure, (ms as f64) * 1000.0)
    }
    pub fn set_target_temp(&self, temp: f64) -> Result<()> {
        unsafe {
            check(QHYCCDCam::ControlQHYCCDTemp(self.handle, temp))
        }
    }
    pub fn has_param(&self, control: Control) -> bool {
        unsafe {
            match QHYResult::from(QHYCCDCam::IsQHYCCDControlAvailable(self.handle, control as i32) as u32) {
                QHYResult::QHYCCD_ERROR => {
                    false
                },
                QHYResult::QHYCCD_SUCCESS => {
                    true
                }
                a @ _ => {
                    panic!("Unexpected response when querying if control '{:?}' is available: {:?}", control, a);
                }
            }
        }
    }
    pub fn set_param(&self, control: Control, value: f64) -> Result<()> {
        unsafe {
        if self.has_param(control) {
            check(QHYCCDCam::SetQHYCCDParam(self.handle, control as i32, value))
        } else {
            println!("Cannot set control: {:?}", control);
            Ok(())
        }
        }
    }
    pub fn get_param(&self, control: Control) -> f64 {
        unsafe {
            QHYCCDCam::GetQHYCCDParam(self.handle, control as i32)
        }
    }
    pub fn release(self) -> Result<()> {
        unsafe {
        check(QHYCCDCam::CloseQHYCCD(self.handle))
        }
    }
    pub fn set_defaults(&self) -> Result<()> {
        unsafe {
        println!("Hey wait gotta get dimensions first");
        let ((chipw, chiph), (imagew, imageh), (pixelw, pixelh), bpp) = self.get_dimensions()?;
        match QHYCCDCam::IsQHYCCDControlAvailable(self.handle, Control::Color as i32) {
            1 | 2 | 3 | 4 => {
                check(QHYCCDCam::SetQHYCCDDebayerOnOff(self.handle, 1))?;
                self.set_param(Control::CONTROL_WBR, 20.0)?;
                self.set_param(Control::CONTROL_WBG, 20.0)?;
                self.set_param(Control::CONTROL_WBB, 20.0)?;
            },
            a @ _ => {
                println!("unexpected response when querying color setting: {}", a);
                return Err(CameraError::QHYError)
            }
        }
        check(QHYCCDCam::SetQHYCCDResolution(self.handle, 0, 0, imagew, imageh))?;
        check(QHYCCDCam::SetQHYCCDBinMode(self.handle, 1, 1))?;
        if self.has_param(Control::TransferBit) {
            check(QHYCCDCam::SetQHYCCDBitsMode(self.handle, 16))?;
        }
        Ok(())
        }
    }

    pub fn set_bin_mode(&self, bin: u8) -> Result<()> {
        match bin {
            1 => if !self.has_param(Control::Bin1x1Mode) { return Err(CameraError::InvalidControl); }
            2 => if !self.has_param(Control::Bin2x2Mode) { return Err(CameraError::InvalidControl); }
            3 => if !self.has_param(Control::Bin3x3Mode) { return Err(CameraError::InvalidControl); }
            4 => if !self.has_param(Control::Bin4x4Mode) { return Err(CameraError::InvalidControl); }
            _ => { return Err(CameraError::InvalidControl); }
        }
        unsafe {
        check(QHYCCDCam::SetQHYCCDBinMode(self.handle, bin as i32, bin as i32))
        }
    }

    pub fn get_exposure_remaining(&self) -> u32 {
        unsafe {
            QHYCCDCam::GetQHYCCDExposureRemaining(self.handle)
        }
    }

    pub fn display_camera_dimensions(&self) -> Result<()> {
        let (overscan_start_X, overscan_start_Y, overscan_size_X, overscan_size_Y) = self.get_overscan_area()?;
        println!("Overscan area:");
        println!("  startX x startY : {:05} x {:05}", overscan_start_X, overscan_start_Y);
        println!("  sizeX  x sizeY  : {:05} x {:05}", overscan_size_X, overscan_size_Y);
        let (effective_start_X, effective_start_Y, effective_size_X, effective_size_Y) = self.get_effective_area()?;
        println!("Effective area:");
        println!("  startX x startY : {:05} x {:05}", effective_start_X, effective_start_Y);
        println!("  sizeX  x sizeY  : {:05} x {:05}", effective_size_X, effective_size_Y);
        let ((chipw, chiph), (imagew, imageh), (pixelw, pixelh), bpp) = self.get_dimensions()?;
        println!("Chip dimensions:");
        println!("Chip size (w/h):      {:05} x {:05} [mm]", chipw, chiph);
        println!("Pixel size (w/h):     {:05} x {:05} [um]", pixelw, pixelh);
        println!("Image size (w/h):     {:05} x {:05} [pixels]", imagew, imageh);
        println!("   bpp:               {}", bpp);
        Ok(())
    }

    pub fn take_image(&self, path: &str) -> Result<()> {
        unsafe {
        let exposure_duration = self.get_param(Control::Exposure);
        let exposure_ms = exposure_duration / 1000.0;
        println!("Exposure duration: {}", exposure_ms);
        let result = QHYCCDCam::ExpQHYCCDSingleFrame(self.handle);
        match QHYCCDCam::QHYResult::from(result as u32) {
            QHYResult::QHYCCD_SUCCESS => {
                println!("Didn't expect this result...");
                std::thread::sleep(std::time::Duration::from_millis(1000));
            },
            QHYResult::QHYCCD_READ_DIRECTLY => {
                println!("Exp complete, example sleeps so i'll sleep too");
                std::thread::sleep(std::time::Duration::from_millis(1000));
            },
            a @ _ =>{
                println!("exp err: {:?}", a);
                return Err(CameraError::QHYError);
            }
        }

        let bufsize = QHYCCDCam::GetQHYCCDMemLength(self.handle);
        println!("Ok, we'll need {} bytes...", bufsize);
        let data_layout = Layout::from_size_align(bufsize as usize, 8).unwrap();
        let data = alloc(data_layout);

        let mut counter: i64 = (self.get_param(Control::Exposure) as u64 / 1000) as i64;

        while counter > 0 {
            println!("I think there's about {}ms remaining", counter);
            std::thread::sleep(std::time::Duration::from_millis(500));
            println!("Camera temp is currently: {}", self.get_param(Control::CurTemp));
            counter -= 500;
        }

        let mut castediw = 0i32;
        let mut castedih = 0i32;
        let mut castedbpp = 0i32;
        let mut channels = 0;
        println!("Getting data...");
        check(QHYCCDCam::GetQHYCCDSingleFrame(self.handle, &mut castediw, &mut castedih, &mut castedbpp, &mut channels, data))?;
        println!("Ok, guess we got it?");
        println!("image: {} x {}", castediw, castedih);
        println!("bpp: {}", castedbpp);
        println!("channels: {}", channels);

        let dest = Path::new(path);
        let file = File::create(dest).unwrap();
        let ref mut w = BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, castediw as u32, castedih as u32);
        encoder.set(png::ColorType::RGB).set(png::BitDepth::Sixteen);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(
            unsafe {
                std::slice::from_raw_parts(
                    data,
                    bufsize as usize
                )
            }
        ).unwrap();
        dealloc(data as *mut u8, data_layout);
        Ok(())
    }
    }
    pub fn get_overscan_area(&self) -> Result<(u32, u32, u32, u32)> {
        unsafe {
        let mut startX: i32 = 0;
        let mut startY: i32 = 0;
        let mut sizeX: i32 = 0;
        let mut sizeY: i32 = 0;
        check(QHYCCDCam::GetQHYCCDOverScanArea(
            self.handle,
           &mut startX as *mut os::raw::c_int,
           &mut startY as *mut os::raw::c_int,
           &mut sizeX as *mut os::raw::c_int,
           &mut sizeY as *mut os::raw::c_int
        ))?;
        Ok((startX as u32, startY as u32, sizeX as u32, sizeY as u32))
        }
    }
    pub fn get_effective_area(&self) -> Result<(u32, u32, u32, u32)> {
        unsafe {
        let mut startX: i32 = 0;
        let mut startY: i32 = 0;
        let mut sizeX: i32 = 0;
        let mut sizeY: i32 = 0;
        check(QHYCCDCam::GetQHYCCDEffectiveArea(
            self.handle,
            &mut startX as *mut os::raw::c_int,
            &mut startY as *mut os::raw::c_int,
            &mut sizeX as *mut os::raw::c_int,
            &mut sizeY as *mut os::raw::c_int
        ))?;
        Ok((startX as u32, startY as u32, sizeX as u32, sizeY as u32))
        }
    }
    pub fn get_dimensions(&self) -> Result<((f64, f64), (u32, u32), (f64, f64), u32)> {
        unsafe {
        let mut chipw: f64 = 0.0;
        let mut chiph: f64 = 0.0;
        let mut imagew: i32 = 0;
        let mut imageh: i32 = 0;
        let mut pixelw: f64 = 0.0;
        let mut pixelh: f64 = 0.0;
        let mut bpp: i32 = 0;
        check(QHYCCDCam::GetQHYCCDChipInfo(
            self.handle,
            &mut chipw as *mut os::raw::c_double,
            &mut chiph as *mut os::raw::c_double,
            &mut imagew as *mut os::raw::c_int,
            &mut imageh as *mut os::raw::c_int,
            &mut pixelw as *mut os::raw::c_double,
            &mut pixelh as *mut os::raw::c_double,
            &mut bpp as *mut os::raw::c_int))?;
        Ok((
            (chipw, chiph),
            (imagew as u32, imageh as u32),
            (pixelw, pixelh),
            bpp as u32
        ))
        }
    }
}
