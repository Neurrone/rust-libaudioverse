use super::super::{libaudioverse_sys, server};
use super::{Node, environment_node};
use super::properties::{ bool_property, distance_model_property, float_property,  float3_property, panning_strategy_property };
use check;
use Result;

/// The source node allows the spatialization of sound that passes through it. Sources have one input which is mono, to which a node should be connected. The audio from the input is spatialized according both to the source’s properties and those on its environment, and passed directly to the environment. Sources have no outputs. To hear a source, you must connect its environment to something instead.
/// Since the source communicates with the environment through a nonstandard mechanism, environments do not keep their sources alive. If you are in a garbage collected language, failure to hold on to the source nodes will cause them to go silent.
/// 
/// Inputs:
/// 
/// index | channels | description
/// ------|----------|------------
/// 0 | 1 | The audio to enter the 3D environment.
/// 
/// This node has no outputs.
pub struct SourceNode {
    handle: libaudioverse_sys::LavHandle
}

impl Node for SourceNode {
    #[doc(hidden)]
    fn get_handle(&self) -> libaudioverse_sys::LavHandle {
        self.handle
    }
}

impl SourceNode {
    /// Creates a new source node.
    pub fn new(server : &server::Server, environment_node : &environment_node::EnvironmentNode) -> Result<SourceNode> {
        let mut node_handle : libaudioverse_sys::LavHandle = 0;
        check(unsafe { libaudioverse_sys::Lav_createSourceNode(server.handle, environment_node.handle, &mut node_handle) })?;
        Ok(SourceNode { handle : node_handle })
    }
    
    /// Returns the control_distance_model property.
    /// 
    /// Default value: False
    /// 
    /// In order to make working with sources easier for simple applications, some properties of source objects are ignored in favor of values on the environment. This property is used to disable this behavior for properties related to the distance model.
    pub fn control_distance_model(&self) -> bool_property::BoolProperty {
        bool_property::BoolProperty { index : libaudioverse_sys::Lav_SOURCE_PROPERTIES_Lav_SOURCE_CONTROL_DISTANCE_MODEL,  node_handle : self.handle }
    }
    
    /// Returns the control_panning property.
    /// 
    /// Default value: False
    /// 
    /// In order to make working with sources easier for simple applications, some properties of source objects are ignored in favor of values on the environment. This property is used to disable this behavior for properties related to panning.
    pub fn control_panning(&self) -> bool_property::BoolProperty {
        bool_property::BoolProperty { index : libaudioverse_sys::Lav_SOURCE_PROPERTIES_Lav_SOURCE_CONTROL_PANNING,  node_handle : self.handle }
    }
    
    /// Returns the control_reverb property.
    /// 
    /// Default value: False
    /// 
    /// In order to make working with sources easier for simple applications, some properties of source objects are ignored in favor of values on the environment. This property is used to disable this behavior for properties related to reverb.
    pub fn control_reverb(&self) -> bool_property::BoolProperty {
        bool_property::BoolProperty { index : libaudioverse_sys::Lav_SOURCE_PROPERTIES_Lav_SOURCE_CONTROL_REVERB,  node_handle : self.handle }
    }
    
    /// Returns the distance_model property.
    /// 
    /// Range: a value from the DistanceModel enumeration
    /// 
    /// Default value: DistanceModel::Linear
    /// 
    /// The distance model determines how quickly sources get quieter as they move away from the listener. The default value of this property is set from the corresponding property on the environment at source creation. By default, sources ignore this property in favor of the value provided by their environment. Set Lav_SOURCE_CONTROL_DISTANCE_MODEL to true to control it yourself.
    pub fn distance_model(&self) -> distance_model_property::DistanceModelProperty {
        distance_model_property::DistanceModelProperty { index : libaudioverse_sys::Lav_SOURCE_PROPERTIES_Lav_SOURCE_DISTANCE_MODEL,  node_handle : self.handle }
    }
    
    /// Returns the head_relative property.
    /// 
    /// Default value: False
    /// 
    /// Whether or not to consider this source’s position to always be relative to the listener.
    /// Sources which are head relative interpret their positions in the default coordinate system, relative to the listener. Positive x is right, positive y is up, and positive z is behind the listener. The orientation and position properties of an environment do not affect head relative sources, making them ideal for such things as footsteps and/or HUD effects that should be panned.
    pub fn head_relative(&self) -> bool_property::BoolProperty {
        bool_property::BoolProperty { index : libaudioverse_sys::Lav_SOURCE_PROPERTIES_Lav_SOURCE_HEAD_RELATIVE,  node_handle : self.handle }
    }
    
    /// Returns the max_distance property.
    /// 
    /// Range: [0.0, INFINITY]
    /// 
    /// Default value: 150.0
    /// 
    /// The maximum distance from the listener at which the source will be audible. The default value of this property is set from the corresponding property on the environment at source creation. By default, sources do not respect this property and use the corresponding value from their environment. Set Lav_SOURCE_CONTROL_DISTANCE_MODEL to true to control it yourself.
    pub fn max_distance(&self) -> float_property::FloatProperty {
        float_property::FloatProperty { index : libaudioverse_sys::Lav_SOURCE_PROPERTIES_Lav_SOURCE_MAX_DISTANCE,  node_handle : self.handle }
    }
    
    /// Returns the max_reverb_level property.
    /// 
    /// Range: [0.0, 1.0]
    /// 
    /// Default value: 0.6
    /// 
    /// The maximum amount of audio to be diverted to reverb sends, if any.
    /// Behavior is undefined if this property is ever less than Lav_SOURCE_MIN_REVERB_LEVEL.
    /// The default value of this property is set from the corresponding property on the environment at source creation. By default, this property is ignored in favor of the value provided by this source’s environment. Set Lav_SOURCE_CONTROL_REVERB to true to control it yourself.
    pub fn max_reverb_level(&self) -> float_property::FloatProperty {
        float_property::FloatProperty { index : libaudioverse_sys::Lav_SOURCE_PROPERTIES_Lav_SOURCE_MAX_REVERB_LEVEL,  node_handle : self.handle }
    }
    
    /// Returns the min_reverb_level property.
    /// 
    /// Range: [0.0, 1.0]
    /// 
    /// Default value: 0.15
    /// 
    /// The minimum reverb level allowed.
    /// if a send is configured to be a reverb send, this is the minimum amount of audio that will be diverted to it.
    /// Behavior is undefined if this property is ever greater than the value you give to Lav_SOURCE_MAX_REVERB_LEVEL.
    /// The default value of this property is set from the corresponding property on the environment at source creation. By default, this property is ignored in favor of the value provided by this source’s environment. Set Lav_SOURCE_CONTROL_REVERB to true to control it yourself.
    pub fn min_reverb_level(&self) -> float_property::FloatProperty {
        float_property::FloatProperty { index : libaudioverse_sys::Lav_SOURCE_PROPERTIES_Lav_SOURCE_MIN_REVERB_LEVEL,  node_handle : self.handle }
    }
    
    /// Returns the occlusion property.
    /// 
    /// Range: [0.0, 1.0]
    /// 
    /// Default value: 0.0
    /// 
    /// A scalar representing how occluded this source is.
    /// This property controls internal filters of the source that make occluded objects sound muffled. A value of 1.0 is a fully occluded source, which will be all but silent; a value of 0.0 has no effect.
    /// It is extremely difficult to map occlusion to a physical quantity. As a consequence, this property is unitless.
    pub fn occlusion(&self) -> float_property::FloatProperty {
        float_property::FloatProperty { index : libaudioverse_sys::Lav_SOURCE_PROPERTIES_Lav_SOURCE_OCCLUSION,  node_handle : self.handle }
    }
    
    /// Returns the panning_strategy property. 
    /// 
    /// Range: a value from the PanningStrategy enumeration
    /// 
    /// Default value: PanningStrategy::Stereo
    /// 
    /// The strategy for the internal multipanner. The default value of this property is set from the corresponding property on the environment at source creation. By default, this property is ignored and sources use the value provided by their environment. Set Lav_SOURCE_CONTROL_PANNING to true to control it yourself.
    pub fn panning_strategy(&self) -> panning_strategy_property::PanningStrategyProperty {
        panning_strategy_property::PanningStrategyProperty { index : libaudioverse_sys::Lav_SOURCE_PROPERTIES_Lav_SOURCE_PANNING_STRATEGY,  node_handle : self.handle }
    }
    
    /// Returns the position property.
    /// 
    /// Default value: [0.0, 0.0, 0.0]
    /// 
    /// The position of the source in world coordinates.
    pub fn position(&self) -> float3_property::Float3Property {
        float3_property::Float3Property { index : libaudioverse_sys::Lav_SOURCE_PROPERTIES_Lav_SOURCE_POSITION,  node_handle : self.handle }
    }
    
    /// Returns the reverb_distance property.
    /// 
    /// Range: [0.0, INFINITY]
    /// 
    /// Default value: 75.0
    /// 
    /// The distance at which the source will only be heard through the reverb effect sends.
    /// If this source is not feeding any effect sends configured as reverbs, this property has no effect.
    /// For values greater than Lav_SOURCE_MAX_DISTANCE, the source will always be heard at least somewhat in the dry path. Lav_SOURCE_DISTANCE_MODEL controls how this crossfading takes place.
    /// The default value of this property is set from the corresponding property on the environment at source creation. By default, sources ignore this property in favor of the value provided by their environment. Set Lav_SOURCE_CONTROL_REVERB to true to control it yourself.
    pub fn reverb_distance(&self) -> float_property::FloatProperty {
        float_property::FloatProperty { index : libaudioverse_sys::Lav_SOURCE_PROPERTIES_Lav_SOURCE_REVERB_DISTANCE,  node_handle : self.handle }
    }
    
    /// Returns the size property.
    /// 
    /// Range: [0.0, INFINITY]
    /// 
    /// Default value: 0.0
    /// 
    /// The size of the source. Sources are approximated as spheres. The size is used to determine the closest point on the source to the listener, and is the radius of this sphere. Size currently has no other effect.
    /// The default value of this property is set from the corresponding property on the environment at source creation.
    pub fn size(&self) -> float_property::FloatProperty {
        float_property::FloatProperty { index : libaudioverse_sys::Lav_SOURCE_PROPERTIES_Lav_SOURCE_SIZE,  node_handle : self.handle }
    }
}