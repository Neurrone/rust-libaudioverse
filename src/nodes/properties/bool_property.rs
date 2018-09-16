use super::super::super::libaudioverse_sys;
use super::super::super::Result;

use check;
use std::os::raw::c_int;
use Error;

/// Proxy to a bool property.
pub struct BoolProperty {
    // allow nodes to construct instances of this struct
    pub(crate) index: c_int, // the index libaudioverse uses to identify this property for this node
    pub(crate) node_handle: libaudioverse_sys::LavHandle, // a handle to the parent node
}

impl BoolProperty {
    pub fn get(&self) -> Result<bool> {
        let mut value: i32 = 0;
        check(unsafe {
            libaudioverse_sys::Lav_nodeGetIntProperty(self.node_handle, self.index, &mut value)
        })?;
        match value {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error {
                code: libaudioverse_sys::Lav_ERRORS_Lav_ERROR_UNKNOWN,
                message: "Bool property out of range.".to_string(),
            }),
        }
    }

    fn set_int(&self, value: i32) -> Result<()> {
        check(unsafe {
            libaudioverse_sys::Lav_nodeSetIntProperty(self.node_handle, self.index, value)
        })?;
        Ok(())
    }

    /// Sets the value of this property.
    pub fn set(&self, value: bool) -> Result<()> {
        match value {
            true => self.set_int(1),
            false => self.set_int(0),
        }
    }
}
