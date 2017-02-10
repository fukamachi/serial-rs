pub use self::com::*;

mod com;
mod error;
mod ffi;

use std::ffi::OsStr;

pub fn open_async<T: AsRef<OsStr> + ?Sized>(port: &T) -> ::Result<COMPort> {
    COMPort::open_async(port)
}
