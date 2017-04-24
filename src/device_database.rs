//! The WacomDatabase module contains information about known Wacom devices and tablets.
//! You can manually add an unknown device from a .tablet file
//! For more information, see: http://linuxwacom.sourceforge.net/wiki/index.php
extern crate wacom_sys;

use self::wacom_sys::WacomDeviceDatabase;
use device;


pub struct DeviceDatabase {
    pub db: WacomDeviceDatabase,
}

impl DeviceDatabase {
    /// Creates a new DeviceDatabase
    pub fn new() -> Option<DeviceDatabase> {
        let new_wacom_db =  unsafe { wacom_sys::libwacom_database_new() as *mut WacomDeviceDatabase };
        if new_wacom_db.is_null() { return None; }

        Some(DeviceDatabase{
            db: unsafe { *new_wacom_db },
        })
    }


}
