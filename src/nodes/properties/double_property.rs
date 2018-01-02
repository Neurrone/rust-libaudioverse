use super::super::super::Result;
use super::super::super::libaudioverse_sys;

use std::os::raw::{ c_int };
use check;

/// Proxy to a double property.
pub struct DoubleProperty {
    // allow nodes to construct instances of this struct
    pub(crate) index : c_int, // the index libaudioverse uses to identify this property for this node
    pub(crate) node_handle : libaudioverse_sys::LavHandle, // a handle to the parent node
}

impl DoubleProperty {
    pub fn get(&self) -> Result<f64> {
        let mut value : f64 = 0.0;
        check(unsafe { libaudioverse_sys::Lav_nodeGetDoubleProperty(self.node_handle, self.index, &mut value) })?;
        Ok(value)
    }
    
    /// Returns the range of the node's property
    pub fn get_range(&self) -> Result<(f64, f64)> {
        let mut min : f64 = 0.0;
        let mut max : f64 = 0.0;
        check(unsafe { libaudioverse_sys::Lav_nodeGetDoublePropertyRange(self.node_handle, self.index, &mut min, &mut max) })?;
        Ok((min, max))
    }
    
    pub fn set(&self, value : f64) -> Result<()> {
        check(unsafe { libaudioverse_sys::Lav_nodeSetDoubleProperty(self.node_handle, self.index, value) })?;
        Ok(())
    }
}

