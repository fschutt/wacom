extern crate wacom_sys;

use self::wacom_sys::*;
use std::os::raw::*;
use std::ffi::CString;
use std::path::Path;
use error::Error;

pub struct Device {
    device: WacomDevice
}

impl Device {

    /// Create a new device reference by name
    pub fn new_from_name(db: &DeviceDatabase, name: &str) -> Result<Device, Error> {
        if let Ok(device_name) = CString::new(name) {
            let err_ptr = unsafe{ libwacom_error_new() };
            if err_ptr.is_null() { 
                return Err(Error{ code: WacomErrorCode::WERROR_BAD_ALLOC, msg: String::new() }); 
            }

            let mut err = unsafe{ *err_ptr };
            let device_ptr = unsafe { libwacom_new_from_name(&db.db, device_name.as_ptr(), &mut err)
                                      as *mut WacomDevice};
            
            if device_ptr.is_null() {
                if err.is_null() {                 
                    return Err(Error { code: WacomErrorCode::WERROR_BAD_ALLOC, msg: String::new() }); 
                }
                return Err(Error {
                    code: unsafe { libwacom_get_error_code(&err) },
                    msg:  unsafe { libwacom_get_error_message(&err) },
                }
            }

            Ok(Device{device: unsafe { *device_ptr }});

        } else {
             return Err(Error{ code: WacomErrorCode::WERROR_BAD_PATH, msg: String::new() }); 
        }
    }

    pub fn new_from_path() -> Option<Device> {

    }

    pub fn new_from_usb_id() -> Option<Device> {

    }

    pub fn get_bus_type(&self) -> WacomBusType {
        unsafe{ wacom_sys::libwacom_get_bustype(&self.device) as WacomBusType}
    }

    /// Given the ID of the button to check for (between 'A' and 'Z') returns the type of button
    pub fn get_button_flag(&self, button: &str) -> Option<WacomButtonFlags> {
        if let Ok(key_code) = CString::new(button) {
            let button = unsafe{ wacom_sys::libwacom_get_button_flag(&self.device, *key_code.as_ptr()) as WacomButtonFlags};
            if button != WacomButtonFlags::WACOM_BUTTON_NONE { return Some(button); }
        }

        None
    }

    /// Given the ID of the button to check for (between 'A' and 'Z'), returns the status LED group id to use
    pub fn get_button_led_group(&self, button: &str) -> Option<i32> {
        if let Ok(key_code) = CString::new(button) {
            let led_id = unsafe{ wacom_sys::libwacom_get_button_led_group(&self.device, *key_code.as_ptr()) as c_int};
            if led_id != -1_i32 { return Some(led_id); }
        }

        None
    }

    /// Get the class of the device (Intuos, Cintiq, etc.)
    pub fn get_class(&self) -> WacomClass{
        unsafe { wacom_sys::libwacom_get_class(&self.device) as WacomClass}
    }

    /// Get the height of the usable area of the device in inch
    pub fn get_height(&self) -> i32 {
        unsafe { libwacom_get_height(&self.device) as i32 }   
    }

    
    /// Get the width of the usable area of the device in inch
    pub fn get_width(&self) -> i32 {
        unsafe { libwacom_get_width(&self.device) as i32 }   
    }

    /// Get the integration flags (integrated display, etc)
    pub fn get_integration_flags(&self) -> WacomIntegrationFlags {
        unsafe { libwacom_get_integration_flags(&self.device) as WacomIntegrationFlags }
    }
    
    /// Get the path to the SVG layout of the device
    pub fn get_layout_filepath(&self) -> Option<Path> {
        let path_str = unsafe { libwacom_get_layout_filename(&self.device) };
        Path::new(path_str) 
    }

}

impl PartialEq for Device {
        fn eq(&self, other: &Device) -> bool {
            let equal = unsafe {wacom_sys::libwacom_compare(&self.device, &other.device, WacomCompareFlags::WCOMPARE_NORMAL)
                        	as *mut c_int };

            return unsafe { *equal } == 0_i32;
        }
}

#[test]
fn device_eq_device() {

}
