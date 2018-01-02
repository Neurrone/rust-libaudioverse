use super::super::{libaudioverse_sys, server};
use super::Node;
use super::properties::{bool_property, buffer_property, double_property, int_property};
use check;
use Result;

/// This node plays a buffer. The output of this node will have as many channels as the buffer does, so connecting it directly to the server will have the desired effect.
/// 
/// This node has no inputs.
/// 
/// Outputs:
/// 
/// index | channels | description
/// ------|----------|------------
/// 0 | Depends on the currently playing buffer. | The output from the buffer being played.
pub struct BufferNode {
    handle: libaudioverse_sys::LavHandle
}

impl Node for BufferNode {
    #[doc(hidden)]
    fn get_handle(&self) -> libaudioverse_sys::LavHandle {
        self.handle
    }
}

impl BufferNode {
    /// Creates a new buffer node.
    pub fn new(server : &server::Server) -> Result<BufferNode> {
        let mut node_handle : libaudioverse_sys::LavHandle = 0;
        check(unsafe { libaudioverse_sys::Lav_createBufferNode(server.handle, &mut node_handle) })?;
        Ok(BufferNode { handle : node_handle })
    }
    
    /// Returns the currently playing buffer. Setting this property will reset position.
    pub fn buffer(&self) -> buffer_property::BufferProperty {
        buffer_property::BufferProperty { index : libaudioverse_sys::Lav_BUFFER_PROPERTIES_Lav_BUFFER_BUFFER,  node_handle : self.handle }
    }
    
    /// Returns the ended_count property. 
    /// 
    /// Range: [0, MAX_INT]
    /// 
    /// Default value: 0
    /// 
    /// Increments every time the buffer reaches it’s end. If the buffer is not looping, this can be used to determine when the buffer is ended, without using the callback. if the buffer is configured to loop, the counter will count up every time the end of a loop is reached. You can write to this property to reset it.
    pub fn ended_count(&self) -> int_property::IntProperty {
        int_property::IntProperty { index : libaudioverse_sys::Lav_BUFFER_PROPERTIES_Lav_BUFFER_ENDED_COUNT,  node_handle : self.handle }
    }
    
    /// Returns the looping property. 
    /// 
    /// Default value: False 
    /// 
    /// If true, this node continues playing the same buffer from the beginning after it reaches the end.
    pub fn looping(&self) -> bool_property::BoolProperty {
        bool_property::BoolProperty { index : libaudioverse_sys::Lav_BUFFER_PROPERTIES_Lav_BUFFER_LOOPING,  node_handle : self.handle }
    }
    
    /// Returns the position property.
    /// 
    /// Range: dynamic
    /// 
    /// Default value: 0.0
    /// 
    /// The position of playback, in seconds. The range of this property corresponds to the total duration of the buffer.
    pub fn position(&self) -> double_property::DoubleProperty {
        double_property::DoubleProperty { index : libaudioverse_sys::Lav_BUFFER_PROPERTIES_Lav_BUFFER_POSITION,  node_handle : self.handle }
    }
    
    /// Returns the rate property.
    /// 
    /// Range: [0, INFINITY]    
    /// 
    /// Default value: 1.0
    /// 
    /// A multiplier that applies to playback rate. 1.0 is identity. Values less than 1.0 cause a decrease in pitch and values greater than 1.0 cause an increase in pitch.
    pub fn rate(&self) -> double_property::DoubleProperty {
        double_property::DoubleProperty { index : libaudioverse_sys::Lav_BUFFER_PROPERTIES_Lav_BUFFER_RATE,  node_handle : self.handle }
    }
}