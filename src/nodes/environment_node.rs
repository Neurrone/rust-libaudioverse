use super::super::{buffer, libaudioverse_sys, server};
use super::properties::{
    DistanceModelProperty, Float3Property, Float6Property, FloatProperty, IntProperty,
    PanningStrategyProperty
};
use super::Node;
use check;
use std::ffi::CString;
use Result;

/// This is the entry point to the 3D simulation capabilities. Environment nodes hold the information needed to pan sources, as well as acting as an aggregate output for all sources that use this environment.
/// Note that the various properties for default values do not affect already created sources. It is best to configure these first. Any functionality to change a property on all sources needs to be implemented by the app, and is not offered by Libaudioverse.
///
/// This node has no inputs.
///
/// Outputs:
///
/// index | channels | description
/// ------|----------|------------
/// 0 | Depends on the output_channels property. | The output of the 3D environment.
pub struct EnvironmentNode {
    // allow SourceNode to access this handle
    pub(crate) handle: libaudioverse_sys::LavHandle,
}

impl Node for EnvironmentNode {
    #[doc(hidden)]
    fn get_handle(&self) -> libaudioverse_sys::LavHandle {
        self.handle
    }
}

impl EnvironmentNode {
    /// Creates a new environment node.
    pub fn new(server: &server::Server, hrtf_path: &CString) -> Result<EnvironmentNode> {
        let mut node_handle: libaudioverse_sys::LavHandle = 0;
        check(unsafe {
            libaudioverse_sys::Lav_createEnvironmentNode(
                server.handle,
                hrtf_path.as_ptr(),
                &mut node_handle,
            )
        })?;
        Ok(EnvironmentNode {
            handle: node_handle,
        })
    }

    /// Returns the default_reverb_distance property.
    ///
    /// Range: [0.0, INFINITY]
    ///
    /// Default value: 75.0
    ///
    /// The distance at which a source will be heard only in the reverb.
    /// See documentation on the SourceNode node for a specific explanation. By default, sources get the value of this property from the environment. To control this property on a per-source basis, set Lav_SOURCE_CONTROL_REVERB to true on the source.
    pub fn default_reverb_distance(&self) -> FloatProperty {
        FloatProperty { index : libaudioverse_sys::lav_STANDARD_ENVIRONMENT_PROPERTIES_Lav_ENVIRONMENT_REVERB_DISTANCE,  node_handle : self.handle }
    }

    /// Returns the default_size property.
    ///
    /// Range: [0.0, INFINITY]
    ///
    /// Default value: 0.0
    ///
    /// The default size for new sources. Sources aare approximated as spheres, with 0 being the special case of a point source. Size is used to determine the listener’s distance from a source.
    pub fn default_size(&self) -> FloatProperty {
        FloatProperty {
            index:
                libaudioverse_sys::lav_STANDARD_ENVIRONMENT_PROPERTIES_Lav_ENVIRONMENT_DEFAULT_SIZE,
            node_handle: self.handle,
        }
    }

    /// Returns the distance_model property.
    ///
    /// Range: a value from the DistanceModel enumeration
    ///
    /// Default value: DistanceModel::Linear
    ///
    /// Distance models control how quickly sources get quieter as they move away from the listener.
    /// By default, sources are configured to delegate to the environment when looking for values to use for the distance model parameters. This behavior may be changed by setting Lav_SOURCE_CONTROL_DISTANCE_MODEL to true.
    pub fn distance_model(&self) -> DistanceModelProperty {
        DistanceModelProperty { index : libaudioverse_sys::lav_STANDARD_ENVIRONMENT_PROPERTIES_Lav_ENVIRONMENT_DISTANCE_MODEL,  node_handle : self.handle }
    }

    /// Returns the max_distance property.
    ///
    /// Range: [0.0, INFINITY]
    ///
    /// Default value: 150.0
    ///
    /// The maximum distance at which a source is audible. Consider this property to be in meters.
    /// By default, sources are configured to delegate to the environment when looking for values to use for the distance model parameters. This behavior may be changed by setting Lav_SOURCE_CONTROL_DISTANCE_MODEL to true.
    pub fn max_distance(&self) -> FloatProperty {
        FloatProperty {
            index:
                libaudioverse_sys::lav_STANDARD_ENVIRONMENT_PROPERTIES_Lav_ENVIRONMENT_MAX_DISTANCE,
            node_handle: self.handle,
        }
    }

    /// Returns the max_reverb_level property.
    ///
    /// Range: [0.0, 1.0]
    ///
    /// Default value: 0.6
    ///
    /// The maximum amount of audio to be diverted to reverb sends, if any.
    /// Behavior is undefined if this property is ever less than Lav_ENVIRONMENT_MIN_REVERB_LEVEL.
    /// By default, sources look to their environmlent for the value of this property. If you wish to set it on a per-source basis, set Lav_SOURCE_CONTROL_REVERB to true on the source.
    pub fn max_reverb_level(&self) -> FloatProperty {
        FloatProperty { index : libaudioverse_sys::lav_STANDARD_ENVIRONMENT_PROPERTIES_Lav_ENVIRONMENT_MAX_REVERB_LEVEL,  node_handle : self.handle }
    }

    /// Returns the min_reverb_level property.
    ///
    /// Range: [0.0, 1.0]
    ///
    /// Default value: 0.15
    ///
    /// The minimum reverb level allowed.
    /// if a send is configured to be a reverb send, this is the minimum amount of audio that will be diverted to it.
    /// Behavior is undefined if this property is ever greater than the value of Lav_ENVIRONMENT_MAX_REVERB_LEVEL.
    /// By default, sources look to their environment for the value of this property. If you wish to set it on a per-source basis, set Lav_SOURCE_CONTROL_REVERB to true on the source.
    pub fn min_reverb_level(&self) -> FloatProperty {
        FloatProperty { index : libaudioverse_sys::lav_STANDARD_ENVIRONMENT_PROPERTIES_Lav_ENVIRONMENT_MIN_REVERB_LEVEL,  node_handle : self.handle }
    }

    /// Returns the orientation property.
    ///
    /// Default Value: [0.0, 0.0, -1.0, 0.0, 1.0, 0.0]
    ///
    /// The orientation of the listener. The first three elements are a vector representing the direction in which the listener is looking and the second 3 a vector representing the direction in which a rod pointing out of the top of the listener’s head would be pointing.
    ///
    /// This property packs these vectors because they must never be modified separately. Additionally, they should both be unit vectors and must also be orthoganal.
    ///
    /// the default situates the listener such that positive x is right, positive y is up, and positive z is behind the listener. The setting (0, 1, 0, 0, 0, 1) will situate the listener such that positive x is right and positive y is forward. For those not familiar with trigonometry and who wish to consider positive x east and positivve y north, the following formula will turn the listener to face a scertain direction specified in radians clockwise of north: (sin(theta), cos(theta), 0, 0, 0, 1). As usual, note that radians=degrees*PI/180.
    pub fn orientation(&self) -> Float6Property {
        Float6Property {
            index:
                libaudioverse_sys::lav_STANDARD_ENVIRONMENT_PROPERTIES_Lav_ENVIRONMENT_ORIENTATION,
            node_handle: self.handle,
        }
    }

    /// Returns the output_channels property.
    ///
    /// Range: [0, 8]
    ///
    /// Default value: 2
    ///
    /// Environments are not smart enough to determine the number of channels their output needs to have.
    /// If you are using something greater than stereo, i.e. 5.1, you need to change this property. The specific issue solved by this property is the case in which one source is set to something different than all others, or where the app changes the panning strategies of sources after creation.
    /// Values besides 2, 4, 6, or 8 do not usually have much meaning.
    pub fn output_channels(&self) -> IntProperty {
        IntProperty { index : libaudioverse_sys::lav_STANDARD_ENVIRONMENT_PROPERTIES_Lav_ENVIRONMENT_OUTPUT_CHANNELS,  node_handle : self.handle }
    }

    /// Returns the panning_strategy property.
    ///
    /// Range: a value from the PanningStrategy enumeration
    ///
    /// Default value: PanningStrategy::Stereo
    ///
    /// The panning strategy for any source configured to delegate to the environment. All new sources delegate to the environment by default.
    /// If you want to change this property for a specific source, set Lav_SOURCE_CONTROL_PANNING on the source to true.
    pub fn panning_strategy(&self) -> PanningStrategyProperty {
        PanningStrategyProperty { index : libaudioverse_sys::lav_STANDARD_ENVIRONMENT_PROPERTIES_Lav_ENVIRONMENT_PANNING_STRATEGY,  node_handle : self.handle }
    }

    /// Returns the position property.
    ///
    /// Default value: [0.0, 0.0, 0.0]
    ///
    /// The position of the listener, in world coordinates.
    pub fn position(&self) -> Float3Property {
        Float3Property {
            index: libaudioverse_sys::lav_STANDARD_ENVIRONMENT_PROPERTIES_Lav_ENVIRONMENT_POSITION,
            node_handle: self.handle,
        }
    }

    ///  Play a buffer, using the specified position and the currently set defaults on the world for distance model and panning strategy. This is the same as creating a buffer and a source, but Libaudioverse retains control of these objects. When the buffer finishes playing, the source is automatically disposed of.
    /// If is_dry is  true, we avoid sending to the effect sends configured as defaults.
    pub fn play_async(
        &self,
        buffer: &buffer::Buffer,
        x: f32,
        y: f32,
        z: f32,
        is_dry: bool,
    ) -> Result<()> {
        check(unsafe {
            libaudioverse_sys::Lav_environmentNodePlayAsync(
                self.handle,
                buffer.handle,
                x,
                y,
                z,
                is_dry as i32,
            )
        })?;
        Ok(())
    }
}
