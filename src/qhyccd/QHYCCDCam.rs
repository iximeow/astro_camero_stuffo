use std::os;

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum QHYResult {
    QHYCCD_SUCCESS = 0,
    QHYCCD_READ_DIRECTLY = 0x2001,
    QHYCCD_DELAY_200MS = 0x2000,
    QHYCCD_ERROR = 0xffffffff
}

impl From<u32> for QHYResult {
    fn from(u: u32) -> QHYResult {
        match u {
            0 => QHYResult::QHYCCD_SUCCESS,
            0x2000 => QHYResult::QHYCCD_READ_DIRECTLY,
            0x2001 => QHYResult::QHYCCD_DELAY_200MS,
            0xffffffff => QHYResult::QHYCCD_ERROR,
            _ => {
                panic!("Unexpected result code from qhy sdk: {:08x}", u);
            }
        }
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum Control {
    Brightness = 0, // !< image brightness
    Contrast = 1,       //1 image contrast 
    CONTROL_WBR = 2,            //2 red of white balance 
    CONTROL_WBB = 3,            //3 blue of white balance
    CONTROL_WBG = 4,            //4 the green of white balance 
    Gamma = 5,          //5 screen gamma 
    Gain = 6,           //6 camera gain 
    Offset = 7,         //7 camera offset 
    Exposure = 8,       //8 expose time (us)
    Speed = 9,          //9 transfer speed 
    TransferBit = 10,    //10 image depth bits 
    Channels = 11,       //11 image channels 
    USBTraffic = 12,     //12 hblank 
    RowNoiseRe = 13,     //13 row denoise 
    CurTemp = 14,        //14 current cmos or ccd temprature 
    CurPWM = 15,         //15 current cool pwm 
    ManulPwm = 16,       //16 set the cool pwm 
    CFWPort = 17,        //17 control camera color filter wheel port 
    Cooler = 18,         //18 check if camera has cooler
    St4port = 19,        //19 check if camera has st4port
    Color = 20,              //20   
    Bin1x1Mode = 21,         //21 check if camera has bin1x1 mode 
    Bin2x2Mode = 22,         //22 check if camera has bin2x2 mode 
    Bin3x3Mode = 23,         //23 check if camera has bin3x3 mode 
    Bin4x4Mode = 24,         //24 check if camera has bin4x4 mode 
    CAM_MECHANICALSHUTTER = 25,                  //25 mechanical shutter  
    CAM_TRIGER_INTERFACE = 26,                   //26 triger  
    CAM_TECOVERPROTECT_INTERFACE = 27,           //27 tec overprotect
    CAM_SINGNALCLAMP_INTERFACE = 28,             //28 singnal clamp 
    CAM_FINETONE_INTERFACE = 29,                 //29 fine tone 
    CAM_SHUTTERMOTORHEATING_INTERFACE = 30,      //30 shutter motor heating 
    CAM_CALIBRATEFPN_INTERFACE = 31,             //31 calibrated frame 
    CAM_CHIPTEMPERATURESENSOR_INTERFACE = 32,    //32 chip temperaure sensor
    CAM_USBREADOUTSLOWEST_INTERFACE = 33,        //33 usb readout slowest 
    CAM_8BITS = 34,                              //34 8bit depth 
    CAM_16BITS = 35,                             //35 16bit depth
    CAM_GPS = 36,                                //36 check if camera has gps 
    CAM_IGNOREOVERSCAN_INTERFACE = 37,           //37 ignore overscan area 
    QHYCCD_3A_AUTOBALANCE = 38,                 //38
    QHYCCD_3A_AUTOEXPOSURE = 39,                    //39
    QHYCCD_3A_AUTOFOCUS = 40,                   //40
    CONTROL_AMPV = 41,                           //41 ccd or cmos ampv
    CONTROL_VCAM = 42,                           //42 Virtual Camera on off 
    CAM_VIEW_MODE = 43,                         //43
    CONTROL_CFWSLOTSNUM = 44,                   //44 check CFW slots number
    IS_EXPOSING_DONE = 45,                      //45
    ScreenStretchB = 46,                            //46
    ScreenStretchW = 47,                            //47
    CONTROL_DDR = 48,                           //47
    CAM_LIGHT_PERFORMANCE_MODE = 49,                //49
    CAM_QHY5II_GUIDE_MODE = 50,                 //50
    DDR_BUFFER_CAPACITY = 51,                   //51
    DDR_BUFFER_READ_THRESHOLD = 52,             //52
    DefaultOffset = 53,                         //53
    OutputDataActualBits = 54,                  //54
    OutputDataAlignment = 55                        //55
}

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum Bayer
{
    GB = 1,
    GR = 2,
    BG = 3,
    RG = 4
}


extern "C" {
    pub fn ScanQHYCCD() -> os::raw::c_int;
    pub fn InitQHYCCDResource() -> os::raw::c_int;
    pub fn GetQHYCCDId(id: os::raw::c_int, id: *mut os::raw::c_char) -> os::raw::c_int;
    pub fn GetQHYCCDModel(id: *mut os::raw::c_char, model: *mut os::raw::c_char) -> os::raw::c_int;
    pub fn OpenQHYCCD(id: *mut os::raw::c_char) -> *mut os::raw::c_void;
    pub fn SetQHYCCDStreamMode(handle: *mut os::raw::c_void, mode: os::raw::c_char) -> os::raw::c_int;
    pub fn SetQHYCCDResolution(handle: *mut os::raw::c_void, x: os::raw::c_uint, y: os::raw::c_uint, xsize: os::raw::c_uint, ysize: os::raw::c_uint) -> os::raw::c_int;
    pub fn InitQHYCCD(id: *mut os::raw::c_void) -> os::raw::c_int;
    pub fn IsQHYCCDControlAvailable(handle: *mut os::raw::c_void, control: os::raw::c_int) -> os::raw::c_int;
    pub fn SetQHYCCDParam(handle: *mut os::raw::c_void, control: os::raw::c_int, value: os::raw::c_double) -> os::raw::c_int;
    pub fn GetQHYCCDParam(handle: *mut os::raw::c_void, control: os::raw::c_int) -> os::raw::c_double;
    pub fn GetQHYCCDEffectiveArea(handle: *mut os::raw::c_void, startx: *mut os::raw::c_int, starty: *mut os::raw::c_int, sizex: *mut os::raw::c_int, sizey: *mut os::raw::c_int) -> os::raw::c_int;
    pub fn GetQHYCCDOverScanArea(handle: *mut os::raw::c_void, startx: *mut os::raw::c_int, starty: *mut os::raw::c_int, sizex: *mut os::raw::c_int, sizey: *mut os::raw::c_int) -> os::raw::c_int;
    pub fn GetQHYCCDChipInfo(
        handle: *mut os::raw::c_void,
        chipw: *mut os::raw::c_double, chiph: *mut os::raw::c_double,
        imagew: *mut os::raw::c_int, imageh: *mut os::raw::c_int,
        pixelw: *mut os::raw::c_double, pixelh: *mut os::raw::c_double,
        bpp: *mut os::raw::c_int) -> os::raw::c_int;
    pub fn CancelQHYCCDExposingAndReadout(handle: *mut os::raw::c_void) -> os::raw::c_int;
    pub fn ControlQHYCCDTemp(handle: *mut os::raw::c_void, target: os::raw::c_double) -> os::raw::c_int;
    pub fn SetQHYCCDDebayerOnOff(handle: *mut os::raw::c_void, onoff: os::raw::c_int) -> os::raw::c_int;
    pub fn SetQHYCCDBinMode(handle: *mut os::raw::c_void, wbin: os::raw::c_int, hbin: os::raw::c_int) -> os::raw::c_int;
    pub fn SetQHYCCDBitsMode(handle: *mut os::raw::c_void, bits: os::raw::c_int) -> os::raw::c_int;
    pub fn ExpQHYCCDSingleFrame(handle: *mut os::raw::c_void) -> os::raw::c_int;
    pub fn GetQHYCCDExposureRemaining(handle: *mut os::raw::c_void) -> os::raw::c_uint;
    pub fn GetQHYCCDMemLength(handle: *mut os::raw::c_void) -> os::raw::c_int;
    pub fn GetQHYCCDSingleFrame(handle: *mut os::raw::c_void, w: *mut os::raw::c_int, h: *mut os::raw::c_int, bpp: *mut os::raw::c_int, channels: *mut os::raw::c_int, data: *mut os::raw::c_uchar) -> os::raw::c_int;
    pub fn CloseQHYCCD(handle: *mut os::raw::c_void) -> os::raw::c_int;
    pub fn ReleaseQHYCCDResource() -> os::raw::c_int;
}
