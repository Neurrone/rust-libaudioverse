use super::super::super::Result;
use super::super::super::libaudioverse_sys;
use super::super::super::buffer;
use std::os::raw::{ c_int };
use check;

/// Proxy to a buffer property.
pub struct BufferProperty {
    // allow Server to construct instances of this struct
    pub(crate) index : c_int, // the index libaudioverse uses to identify this property for this node
    pub(crate) node_handle : libaudioverse_sys::LavHandle, // a handle to the parent node
}

impl BufferProperty {
    pub fn set(&self, buffer : &buffer::Buffer) -> Result<()> {
        check(unsafe { libaudioverse_sys::Lav_nodeSetBufferProperty(self.node_handle, self.index, buffer.handle) })?;
        Ok(())
    }
}

