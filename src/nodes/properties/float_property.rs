use super::super::super::Result;
use super::super::super::libaudioverse_sys;

use std::os::raw::{ c_int };
use check;

/// Proxy to a float property.
pub struct FloatProperty {
    // allow Server to construct instances of this struct
    pub(crate) index : c_int, // the index libaudioverse uses to identify this property for this node
    pub(crate) node_handle : libaudioverse_sys::LavHandle, // a handle to the parent node
}

impl FloatProperty {
    pub fn get(&self) -> Result<f32> {
        let mut value : f32 = 0.0;
        check(unsafe { libaudioverse_sys::Lav_nodeGetFloatProperty(self.node_handle, self.index, &mut value) })?;
        Ok(value)
    }
    
    /// Returns the range of the node's property
    pub fn get_range(&self) -> Result<(f32, f32)> {
        let mut min : f32 = 0.0;
        let mut max : f32 = 0.0;
        check(unsafe { libaudioverse_sys::Lav_nodeGetFloatPropertyRange(self.node_handle, self.index, &mut min, &mut max) })?;
        Ok((min, max))
    }
    
    pub fn set(&self, value : f32) -> Result<()> {
        check(unsafe { libaudioverse_sys::Lav_nodeSetFloatProperty(self.node_handle, self.index, value) })?;
        Ok(())
    }
}

