use std::ffi::{CString};
use super::super::{libaudioverse_sys, server};
use super::Node;
use super::properties::{ bool_property, float_property };
use check;
use Result;

/// This node implements an HRTF panner.
/// 
/// Inputs:
/// 
/// index | channels | description
/// ------|----------|------------
/// 0 | 1 | The signal to pan.
/// 
/// Outputs:
/// 
/// index | channels | description
/// ------|----------|------------
/// 0 | 2 | The signal with the HRTF applied.
pub struct HrtfNode {
    handle: libaudioverse_sys::LavHandle
}

impl Node for HrtfNode {
    #[doc(hidden)]
    fn get_handle(&self) -> libaudioverse_sys::LavHandle {
        self.handle
    }
}

impl HrtfNode {
    /// Creates a new HRTF node.
    ///  You can use either Libaudioverse’s internal HRTF (The Diffuse MIT Kemar Dataset) by passing “default” as the HRTf file name, or an HRTF of your own.
    pub fn new(server : &server::Server, hrtf_path : &CString) -> Result<HrtfNode> {
        let mut node_handle : libaudioverse_sys::LavHandle = 0;
        check(unsafe { libaudioverse_sys::Lav_createHrtfNode(server.handle, hrtf_path.as_ptr(), &mut node_handle) })?;
        Ok(HrtfNode { handle : node_handle })
    }
    
    /// Returns the azimuth property.
    /// 
    /// Range: [-INFINITY, INFINITY]
    /// 
    /// Default value: 0.0
    /// 
    /// The horizontal angle of the panner in degrees. 0 is straight ahead and positive values are clockwise.
    pub fn azimuth(&self) -> float_property::FloatProperty {
        float_property::FloatProperty { index : libaudioverse_sys::Lav_PANNER_PROPERTIES_Lav_PANNER_AZIMUTH,  node_handle : self.handle }
    }
    
    /// Returns the elevation property.
    /// 
    /// Range: [-90.0, 90.0]
    /// 
    /// Default value: 0.0
    /// 
    /// The vertical angle of the panner in degrees. 0 is horizontal and positive values move upward.
    pub fn elevation(&self) -> float_property::FloatProperty {
        float_property::FloatProperty { index : libaudioverse_sys::Lav_PANNER_PROPERTIES_Lav_PANNER_ELEVATION,  node_handle : self.handle }
    }
    
    /// Returns the should_crossfade property.
    /// 
    /// Default value: True
    /// 
    /// By default, panners crossfade their output. This property allows such functionality to be disabled. Note that for HRTF nodes, crossfading is more important than for other panner types. Unlike other panner types, the audio artifacts produced by disabling crossfading are noticeable, even for updates of only a few degrees.
    pub fn should_crossfade(&self) -> bool_property::BoolProperty {
        bool_property::BoolProperty { index : libaudioverse_sys::Lav_PANNER_PROPERTIES_Lav_PANNER_SHOULD_CROSSFADE,  node_handle : self.handle }
    }
}