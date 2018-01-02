use std::ffi::{CString};
use super::super::{libaudioverse_sys, server};
use super::Node;
use super::properties::{ bool_property, float_property, panning_strategy_property };
use check;
use Result;

/// A panner which can have the algorithm it uses changed at runtime. The use for multipanners is for applications in which we may wish to change the speaker configuration at runtime. Capabilities include switching from HRTF to stereo and back, a useful property for games wherein the user might or might not be using headphones.
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
/// 0 | Depends on the currently set panning strategy. | The signal, panned according to the configured panning strategy.
pub struct MultipannerNode {
    handle: libaudioverse_sys::LavHandle
}

impl Node for MultipannerNode {
    #[doc(hidden)]
    fn get_handle(&self) -> libaudioverse_sys::LavHandle {
        self.handle
    }
}

impl MultipannerNode {
    /*
    /// Creates a new multipanner node.
    ///  You can use either Libaudioverse’s internal HRTF (The Diffuse MIT Kemar Dataset) by passing “default” as the HRTf file name, or an HRTF of your own.
    pub fn new(server : &server::Server, hrtf_path : &mut CString) -> Result<MultipannerNode> {
        let mut node_handle : libaudioverse_sys::LavHandle = 0;
        check(unsafe { libaudioverse_sys::Lav_createMultipannerNode(server.handle, hrtf_path.as_ptr(), &mut node_handle) })?;
        Ok(MultipannerNode { handle : node_handle })
    }
    */
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
    /// Whether or not this panner should crossfade. Lack of crossfading introduces audible artifacts when the panner is moved. You usually want this on.
    pub fn should_crossfade(&self) -> bool_property::BoolProperty {
        bool_property::BoolProperty { index : libaudioverse_sys::Lav_PANNER_PROPERTIES_Lav_PANNER_SHOULD_CROSSFADE,  node_handle : self.handle }
    }
    
    /// Returns the strategy property. 
    /// 
    /// Range: a value from the PanningStrategy enumeration
    /// 
    /// Default value: PanningStrategy::Stereo
    /// 
    /// What type of panning to use. Possibilities include HRTF, stereo, 5.1, and 7.1 speaker configurations. For something more nontraditional, use an amplitude panner.
    pub fn strategy(&self) -> panning_strategy_property::PanningStrategyProperty {
        panning_strategy_property::PanningStrategyProperty { index : libaudioverse_sys::Lav_PANNER_PROPERTIES_Lav_PANNER_STRATEGY,  node_handle : self.handle }
    }
}