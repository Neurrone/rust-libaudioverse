//!
//! Properties control aspects of nodes in the manner that their name suggests.
//!

mod bool_property;
mod buffer_property;
mod distance_model_property;
mod double_property;
mod float3_property;
mod float6_property;
mod float_property;
mod int_property;
mod node_state_property;
mod panning_strategy_property;

#[doc(inline)]
pub use self::{
    bool_property::BoolProperty,
    buffer_property::BufferProperty,
    distance_model_property::{DistanceModel, DistanceModelProperty},
    double_property::DoubleProperty,
    float3_property::Float3Property,
    float6_property::Float6Property,
    float_property::FloatProperty,
    int_property::IntProperty,
    node_state_property::{NodeState, NodeStateProperty},
    panning_strategy_property::{PanningStrategy, PanningStrategyProperty},
};
