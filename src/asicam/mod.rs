pub mod ASICamera2;

use self::ASICamera2::{CameraInfo, ControlCaps, ControlType, ExposureStatus, ImageType};

use std::alloc::{alloc, dealloc, Layout};
use std::collections::HashMap;
use std::ffi::CStr;
use std::os;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use png::HasParameters;

#[derive(Debug)]
pub struct Control {
    pub name: String,
    pub description: String,
    pub max: i64,
    pub min: i64,
    pub default: i64,
    pub can_auto: bool,
    pub is_writable: bool,
    pub control_type: ASICamera2::ControlType
}

#[derive(Debug)]
pub struct Camera {
    id: i32,
    pub width: u32,
    pub height: u32,
    curr_width: u32,
    curr_height: u32,
    bin: u8,
    color_format: ASICamera2::ImageType,
    image_buffer: *mut u8,
    controls: HashMap<ASICamera2::ControlType, Control>
}

impl Camera {
    pub fn new(id: i32) -> Camera {
        Camera {
            id: id,
            controls: HashMap::new(),
            width: 0,
            height: 0,
            curr_width: 0,
            curr_height: 0,
            bin: 1,
            image_buffer: std::ptr::null_mut(),
            color_format: ASICamera2::ImageType::END
        }
    }

    pub fn get_control_value(&self, control: ASICamera2::ControlType) -> Result<i64> {
        let mut current: os::raw::c_long = 0;
        let mut is_auto: os::raw::c_int = 0;
        let res =
            unsafe {
                ASICamera2::ASIGetControlValue(
                    self.id,
                    control as i32,
                    &mut current as *mut os::raw::c_long,
                    &mut is_auto as *mut os::raw::c_int
                )
            };
        build_result(current, res)
    }

    pub fn set_control_value(&mut self, control: ASICamera2::ControlType, value: i64) -> Result<()> {
        let res =
            unsafe {
                ASICamera2::ASISetControlValue(
                    self.id,
                    control as i32,
                    value,
                    0
                )
            };
        build_result((), res)?;
        match control {
            ControlType::HardwareBin => {
                if value == 0 {
                    self.curr_width *= 2;
                    self.curr_height *= 2;
                    Ok(())
                } else if value == 1 {
                    self.curr_width /= 2;
                    self.curr_height /= 2;
                    Ok(())
                } else {
                    // pretty sure this is unreachable,
                    // would be an out of band value and fail in `build_result`
                    unreachable!();
                }
            }
            _ => Ok(())
        }
    }

    pub fn set_exposure_ms(&mut self, ms: u64) -> Result<()> {
        self.set_control_value(ControlType::Exposure, ms as i64 * 1000)
    }

    pub fn take_image(&self, path: &str) -> Result<()> {
        let exposure_duration = self.get_control_value(ControlType::Exposure).unwrap();
        let exposure_ms = exposure_duration / 1000;
        unsafe {
            let res = ASICamera2::ASIStartExposure(self.id, 0); // isDark == false, doesnt matter really
            build_result((), res)?;
        }

        println!("Sleeping {}ms", exposure_ms + 2500);
        std::thread::sleep(std::time::Duration::from_millis(exposure_ms as u64 + 2500));

        let res = unsafe {
            ASICamera2::ASIGetDataAfterExp(
                self.id,
                self.image_buffer,
                self.curr_width as i64 * self.curr_height as i64 * 3
            )
        };
        build_result((), res)?;

        let dest = Path::new(path);
        let file = File::create(dest).unwrap();
        let ref mut w = BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, self.curr_width, self.curr_height);
        encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(
            unsafe {
                std::slice::from_raw_parts(
                    self.image_buffer,
                    self.curr_width as usize * self.curr_height as usize* 3
                )
            }
        ).unwrap();
        Ok(())
    }

    pub fn exposure_status(&self) -> Result<ExposureStatus> {
        let mut exposure_status = ExposureStatus::Failed;
        let res = unsafe {
            ASICamera2::ASIGetExpStatus(self.id, &mut exposure_status as *mut ExposureStatus)
        };
        build_result(exposure_status, res)
    }

    pub fn set_roi_format(&mut self, width: u32, height: u32, binning: u8, image_type: ImageType) -> Result<()> {
        self.curr_width = width - (width % 8);
        self.curr_height = height - (height % 8);
        self.bin = binning;
        let res = unsafe {
            ASICamera2::ASISetROIFormat(
                self.id,
                self.curr_width as i32,
                self.curr_height as i32,
                self.bin as i32,
                image_type as i32)
        };
        build_result((), res)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum CameraError {
    InvalidIndex = 1,
    InvalidId = 2,
    InvalidControlType = 3,
    CameraClosed = 4,
    CameraRemoved = 5,
    InvalidPath = 6,
    InvalidFileformat = 7,
    InvalidSize = 8,
    InvalidImgtype = 9,
    OutofBoundary = 10,
    Timeout = 11,
    InvalidSequence = 12,
    BufferTooSmall = 13,
    VideoModeActive = 14,
    ExposureInProgress = 15,
    GeneralError = 16,
    InvalidMode = 17,
    End = 18
}

fn build_result<T>(value: T, err: ASICamera2::ErrorCode) -> Result<T> {
    match err {
        ASICamera2::ErrorCode::Success => { Ok(value) }
        ASICamera2::ErrorCode::InvalidIndex => { Err(CameraError::InvalidIndex) }
        ASICamera2::ErrorCode::InvalidId => { Err(CameraError::InvalidId) }
        ASICamera2::ErrorCode::InvalidControlType => { Err(CameraError::InvalidControlType) }
        ASICamera2::ErrorCode::CameraClosed => { Err(CameraError::CameraClosed) }
        ASICamera2::ErrorCode::CameraRemoved => { Err(CameraError::CameraRemoved) }
        ASICamera2::ErrorCode::InvalidPath => { Err(CameraError::InvalidPath) }
        ASICamera2::ErrorCode::InvalidFileformat => { Err(CameraError::InvalidFileformat) }
        ASICamera2::ErrorCode::InvalidSize => { Err(CameraError::InvalidSize) }
        ASICamera2::ErrorCode::InvalidImgtype => { Err(CameraError::InvalidImgtype) }
        ASICamera2::ErrorCode::OutofBoundary => { Err(CameraError::OutofBoundary) }
        ASICamera2::ErrorCode::Timeout => { Err(CameraError::Timeout) }
        ASICamera2::ErrorCode::InvalidSequence => { Err(CameraError::InvalidSequence) }
        ASICamera2::ErrorCode::BufferTooSmall => { Err(CameraError::BufferTooSmall) }
        ASICamera2::ErrorCode::VideoModeActive => { Err(CameraError::VideoModeActive) }
        ASICamera2::ErrorCode::ExposureInProgress => { Err(CameraError::ExposureInProgress) }
        ASICamera2::ErrorCode::GeneralError => { Err(CameraError::GeneralError) }
        ASICamera2::ErrorCode::InvalidMode => { Err(CameraError::InvalidMode) }
        ASICamera2::ErrorCode::End => { Err(CameraError::End) }
    }
}

type Result<T> = std::result::Result<T, CameraError>;

pub fn acquire(camera_id: i32) -> Result<Camera> {
    unsafe {
        let cameracount = ASICamera2::ASIGetNumOfConnectedCameras();
        if camera_id >= cameracount {
            panic!("Camera id is invalid (detected {} cameras)", camera_id);
        }
        let props_layout = Layout::array::<CameraInfo>(cameracount as usize).unwrap();
        let props = alloc(props_layout) as *mut CameraInfo;
        let res = ASICamera2::ASIGetCameraProperty(props, camera_id);
        build_result((), res)?;
        println!("Got properties");

        let res = ASICamera2::ASIOpenCamera(camera_id);
        build_result((), res)?;
        println!("Opened camera");

        let res = ASICamera2::ASIInitCamera(camera_id);
        build_result((), res)?;
        println!("Init'd camera");

        let mut control_count: i32 = 0;
        let res = ASICamera2::ASIGetNumOfControls(camera_id, &mut control_count as *mut os::raw::c_int);
        build_result((), res)?;
        println!("Got control count");

        let control_layout = Layout::array::<ControlCaps>(1).unwrap();
        let control = alloc(control_layout) as *mut ControlCaps;

        let mut camera = Camera::new(camera_id);

        let camera_props: CameraInfo = *props.offset(camera_id as isize);
        camera.width = camera_props.max_width as u32;
        camera.height = camera_props.max_height as u32;
        camera.curr_width = camera_props.max_width as u32;
        camera.curr_height = camera_props.max_height as u32;
        camera.color_format = ImageType::RGB24;
        camera.image_buffer = alloc(
            Layout::from_size_align(camera.curr_width as usize * camera.curr_height as usize * 3, 8).unwrap()
        );

        let res = ASICamera2::ASISetROIFormat(camera_id, camera.width as i32, camera.height as i32, 1, ImageType::RGB24 as i32);
        build_result((), res)?;
        println!("Set ROI/Format");

        for c in 0..control_count {
            let res = ASICamera2::ASIGetControlCaps(0, c, control);
            build_result((),  res)?;
            println!("Got control {:?}", c);

            let control = Control {
                name: CStr::from_ptr((*control).name.as_ptr()).to_str().unwrap().to_owned(),
                description: CStr::from_ptr((*control).description.as_ptr()).to_str().unwrap().to_owned(),
                max: (*control).max_value,
                min: (*control).min_value,
                default: (*control).default_value,
                can_auto: bool::from((*control).is_auto_supported),
                is_writable: bool::from((*control).is_writable),
                control_type: (*control).control_type
            };

            camera.controls.insert(control.control_type, control);
        }

        dealloc(control as *mut u8, control_layout);
        dealloc(props as *mut u8, props_layout);

        Ok(camera)
    }
}
