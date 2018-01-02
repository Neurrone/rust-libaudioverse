use super::super::super::Result;
use super::super::super::libaudioverse_sys;

use std::os::raw::{ c_int };
use check;

/// Proxy to an int property.
pub struct IntProperty {
    // allow Server to construct instances of this struct
    pub(crate) index : c_int, // the index libaudioverse uses to identify this property for this node
    pub(crate) node_handle : libaudioverse_sys::LavHandle, // a handle to the parent node
}

impl IntProperty {
    pub fn get(&self) -> Result<i32> {
        let mut value : i32 = 0;
        check(unsafe { libaudioverse_sys::Lav_nodeGetIntProperty(self.node_handle, self.index, &mut value) })?;
        Ok(value)
    }
    
    /// Returns the range of the node's property
    pub fn get_range(&self) -> Result<(i32, i32)> {
        let mut min : i32 = 0;
        let mut max : i32 = 0;
        check(unsafe { libaudioverse_sys::Lav_nodeGetIntPropertyRange(self.node_handle, self.index, &mut min, &mut max) })?;
        Ok((min, max))
    }
    
    pub fn set(&self, value : i32) -> Result<()> {
        check(unsafe { libaudioverse_sys::Lav_nodeSetIntProperty(self.node_handle, self.index, value) })?;
        Ok(())
    }
}

