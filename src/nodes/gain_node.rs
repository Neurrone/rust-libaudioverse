use super::super::{libaudioverse_sys, server};
use super::Node;
use check;
use Result;

/// This node is essentially in instantiated generic node, offering only the functionality therein. Its purpose is to allow changing the gain or adding offset to a large collection of nodes. One possible use is as a simple mixer: point all the nodes to be mixed at the input, set mul, and then point the output at the destination for the mixed audio.
/// 
/// Inputs:
/// 
/// index | channels | description
/// ------|----------|------------
/// 0 | Depends on arguments to this node’s constructor. | The signal whose gain is to be changed.
/// 
/// Outputs:
/// 
/// index | channels | description
/// ------|----------|------------
/// 0 | Depends on arguments to this node’s constructor. | The signal with its gain changed.

pub struct GainNode {
    handle: libaudioverse_sys::LavHandle
}

impl Node for GainNode {
    #[doc(hidden)]
    fn get_handle(&self) -> libaudioverse_sys::LavHandle {
        self.handle
    }
}

impl GainNode {
    /// Creates a new gain node.
    pub fn new(server : &server::Server, channels : i32) -> Result<GainNode> {
        let mut node_handle : libaudioverse_sys::LavHandle = 0;
        check(unsafe { libaudioverse_sys::Lav_createGainNode(server.handle, channels, &mut node_handle) })?;
        Ok(GainNode { handle : node_handle })
    }
}