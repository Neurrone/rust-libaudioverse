//! Rust bindings for [Libaudioverse](https://github.com/libaudioverse/libaudioverse), a highly flexible realtime audio synthesis library.
//!

#![feature(nll)]

extern crate libaudioverse_sys;

mod buffer;
pub mod lav_ptr;
pub mod nodes;
mod server;

use std::error;
use std::fmt;
use std::ptr;
use std::result;

use std::ffi::CStr;
use std::os::raw::{c_char, c_int};

use self::libaudioverse_sys::*;

#[doc(inline)]
pub use self::{buffer::Buffer, server::Server};

#[derive(Debug)]
pub struct Error {
    pub code: i32,
    pub message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        // Displaying an `Error` simply displays the message from libaudioverse
        self.message.fmt(f)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }
}

pub type Result<T> = result::Result<T, Error>;

fn check(code: LavError) -> Result<()> {
    if code == Lav_ERRORS_Lav_ERROR_NONE {
        return Ok(());
    }

    let mut msg: *const c_char = ptr::null_mut();
    unsafe {
        // Get the message corresponding to the last error that happened on this thread.
        // The returned pointer is valid until another error occurs.
        let _error = Lav_errorGetMessage(&mut msg);
        let message = CStr::from_ptr(msg).to_string_lossy().into_owned();

        Err(Error {
            code: code as i32,
            message,
        })
    }
}

/// Initializes Libaudioverse. Failure to do so will result in crashes. You may initialize the library more than once: subsequent initializations do nothing.
pub fn initialize() -> Result<()> {
    check(unsafe { Lav_initialize() })
}

pub fn is_initialized() -> Result<bool> {
    let mut res: c_int = 0;

    check(unsafe { Lav_isInitialized(&mut res) })?;
    Ok(res != 0)
}

/// Deinitializes Libaudioverse. Failure to do so may lead to crashes, depending on what is or is not created. It is not safe to assume that Libaudioverse will properly clean itself up at process exit unless this function is called. You must deinitialize the library exactly as many times as it has been initialized.
pub fn shutdown() -> Result<()> {
    check(unsafe { Lav_shutdown() })
}

#[test]
fn initializes_and_shuts_down() {
    initialize().unwrap();
    assert!(is_initialized().unwrap());
    shutdown().unwrap();
}
