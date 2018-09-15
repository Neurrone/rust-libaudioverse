//!
//! Nodes represent audio transformation, genneration, and analysis.
//!

pub mod properties;
pub mod buffer_node;
pub mod environment_node;
pub mod file_streamer_node;
pub mod gain_node;
pub mod hrtf_node;
// pub mod multipanner_node;
pub mod push_node;
pub mod source_node;

use super::*;
use super::libaudioverse_sys;
use std::os::raw::{ c_uint };
use check;

pub use self::properties::{float_property, node_state_property};

/*
pub use self::buffer_node::BufferNode;
pub use self::environment_node::EnvironmentNode;
pub use self::file_streamer_node::FileStreamerNode;
pub use self::gain_node::GainNode;
pub use self::hrtf_node::HrtfNode;
pub use self::source_node::SourceNode;
*/

/// The properties and functionality described here are available on every Libaudioverse node without exception.
pub trait Node {
    /// Connect the specified output of the specified node to the specified input of the specified node.
    /// It is an error if this would cause a cycle in the graph of nodes.
    fn connect(&self, output : i32, destination : &Node, input : i32) -> Result<()> {
        check(unsafe { libaudioverse_sys::Lav_nodeConnect(self.get_handle(), output, destination.get_handle(), input) })?;
        Ok(())
    }
    
    /// Connect a node’s output to an automatable property.
    fn connect_property(&self, output : i32, destination : &Node, slot : i32) -> Result<()> {
        check(unsafe { libaudioverse_sys::Lav_nodeConnectProperty(self.get_handle(), output, destination.get_handle(), slot) })?;
        Ok(())
    }
    
    /// Connect the specified output of the specified node to the server’s input.
    /// Any node which is connected directly or indirectly to the server will remain alive even if your program lets go of it. For more details on the subject of node lifetimes, see the Libaudioverse manual.
    fn connect_server(&self, output : i32) -> Result<()> {
        check(unsafe { libaudioverse_sys::Lav_nodeConnectServer(self.get_handle(), output) })?;
        Ok(())
    }
    
    /// Disconnect the output of the specified node.
    fn disconnect(&self, output : i32, destination : &Node, input : i32) -> Result<()> {
        check(unsafe { libaudioverse_sys::Lav_nodeDisconnect(self.get_handle(), output, destination.get_handle(), input) })?;
        Ok(())
    }
    
    /// Returns the raw handle for this node.
    #[doc(hidden)]
    fn get_handle(&self) -> libaudioverse_sys::LavHandle;
    
    /// Get the number of inputs this node has.
    fn get_input_connection_count(&self) -> Result<u32> {
        let mut inputs : c_uint = 0;
        check(unsafe { libaudioverse_sys::Lav_nodeGetInputConnectionCount(self.get_handle(), &mut inputs) })?;
        Ok(inputs)
    }
    
    /// Get the number of outputs this node has.
    fn get_output_connection_count(&self) -> Result<u32> {
        let mut outputs : c_uint = 0;
        check(unsafe { libaudioverse_sys::Lav_nodeGetOutputConnectionCount(self.get_handle(), &mut outputs) })?;
        Ok(outputs)
    }
    
    /// Equivalent to disconnecting all of the outputs of this node. After a call to isolate, this node will no longer be affecting audio in any way.
    fn isolate(&self) -> Result<()> {
        check(unsafe { libaudioverse_sys::Lav_nodeIsolate(self.get_handle()) })
    }
    
    /// Reset a node. What this means depends on the node in question. Properties are not touched by node resetting.
    fn reset(&self) -> Result<()> {
        check(unsafe { libaudioverse_sys::Lav_nodeReset(self.get_handle()) })
    }
    
    /// Returns the add property.
    /// 
    /// Range: [-INFINITY, INFINITY]
    /// 
    /// Default value: 0.0
    /// 
    /// After mul is applied, we add the value to which this property is set to the node’s result.
    fn add(&self) -> float_property::FloatProperty {
        float_property::FloatProperty { index : libaudioverse_sys::lav_STANDARD_PROPERTIES_Lav_NODE_ADD,  node_handle : self.get_handle() }
    }    
    
    /// Returns the mul property.
    /// 
    /// Range: [-INFINITY, INFINITY]
    /// 
    /// Default value: 1.0
    /// 
    /// After this node processes, the value to which mul is set is used as a multiplier on the result. The most notable effect of this is to change the node’s volume. A variety of other uses exist, however, especially as regards to nodes which are connected to properties. Mul is applied before add.
    fn mul(&self) -> float_property::FloatProperty {
        float_property::FloatProperty { index : libaudioverse_sys::lav_STANDARD_PROPERTIES_Lav_NODE_MUL,  node_handle : self.get_handle() }
    }    
    
    /// Returns the state property.
    /// 
    /// range: a value from the NodeState enumeration.
    /// 
    /// Default value: NodeState::Playing
    /// 
    /// The node’s state. See the basics section in the Libaudioverse manual for details. The default is usually what you want.
    fn state(&self) -> node_state_property::NodeStateProperty {
        node_state_property::NodeStateProperty { index : libaudioverse_sys::lav_STANDARD_PROPERTIES_Lav_NODE_STATE,  node_handle : self.get_handle() }
    }    
    
}

