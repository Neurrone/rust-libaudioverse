use super::super::super::{Result, Error};
use super::super::super::libaudioverse_sys;
use std::os::raw::{ c_int };
use check;

/// used to indicate the state of a node.
#[repr(i32)]
pub enum NodeState {
    /// This node advances always.
    AlwaysPlaying = libaudioverse_sys::Lav_NODE_STATES_Lav_NODESTATE_ALWAYS_PLAYING,
    /// This node advances if other nodes need audio from it.
    Playing = libaudioverse_sys::Lav_NODE_STATES_Lav_NODESTATE_PLAYING,
    /// This node is paused.
    Paused = libaudioverse_sys::Lav_NODE_STATES_Lav_NODESTATE_PAUSED
}

/// Proxy to a NodeState property.
pub struct NodeStateProperty {
    // allow Server to construct instances of this struct
    pub(crate) index : c_int, // the index libaudioverse uses to identify this property for this node
    pub(crate) node_handle : libaudioverse_sys::LavHandle, // a handle to the parent node
}

impl NodeStateProperty {
    pub fn get(&self) -> Result<NodeState> {
        let mut value : i32 = 0;
        check(unsafe { libaudioverse_sys::Lav_nodeGetIntProperty(self.node_handle, self.index, &mut value) })?;
        match value {
            libaudioverse_sys::Lav_NODE_STATES_Lav_NODESTATE_ALWAYS_PLAYING => Ok(NodeState::AlwaysPlaying),
            libaudioverse_sys::Lav_NODE_STATES_Lav_NODESTATE_PLAYING => Ok(NodeState::Playing),
            libaudioverse_sys::Lav_NODE_STATES_Lav_NODESTATE_PAUSED => Ok(NodeState::Paused),
            _ => Err(Error {
                code: libaudioverse_sys::Lav_ERRORS_Lav_ERROR_UNKNOWN,
                message : "Invalid node state".to_string()
            })
        }
    }
    
    fn set_int(&self, value : i32) -> Result<()> {
        check(unsafe { libaudioverse_sys::Lav_nodeSetIntProperty(self.node_handle, self.index, value) })?;
        Ok(())
    }
    
    pub fn set(&self, node_state : NodeState) -> Result<()> {
        self.set_int(node_state as i32)
    }
    
    /*
    pub fn always_play(&self) -> Result<()> {
        self.set_int(NodeState::AlwaysPlaying as i32)
    }
    
    pub fn play(&self) -> Result<()> {
        self.set_int(NodeState::Playing as i32)
    }
    
    pub fn pause(&self) -> Result<()> {
        self.set_int(NodeState::Paused as i32)
    }
    */
}

