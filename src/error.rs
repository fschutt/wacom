
extern crate wacom_sys;

use self::wacom_sys::WacomErrorCode;

pub struct Error {
    code: WacomErrorCode,
    msg:  String,
}

impl Error {
    

}
