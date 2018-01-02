use super::super::super::Result;
use super::super::super::libaudioverse_sys;

use std::os::raw::{ c_int };
use check;

/// Proxy to a float3 property.
pub struct Float3Property {
    // allow Server to construct instances of this struct
    pub(crate) index : c_int, // the index libaudioverse uses to identify this property for this node
    pub(crate) node_handle : libaudioverse_sys::LavHandle, // a handle to the parent node
}

impl Float3Property {
    pub fn get(&self) -> Result<[f32; 3]> {
        let mut values : [f32; 3] = [0.0, 0.0, 0.0];
        check(unsafe { libaudioverse_sys::Lav_nodeReadFloatArrayProperty(self.node_handle, self.index, 0, &mut values[0]) })?;
        check(unsafe { libaudioverse_sys::Lav_nodeReadFloatArrayProperty(self.node_handle, self.index, 1, &mut values[1]) })?;
        check(unsafe { libaudioverse_sys::Lav_nodeReadFloatArrayProperty(self.node_handle, self.index, 2, &mut values[2]) })?;
        Ok(values)
    }
    
    pub fn set(&self, v1 : f32, v2 : f32, v3 : f32) -> Result<()> {
        check(unsafe { libaudioverse_sys::Lav_nodeSetFloat3Property(self.node_handle, self.index, v1, v2, v3) })?;
        Ok(())
    }
}

