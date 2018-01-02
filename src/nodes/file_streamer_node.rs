use std::ffi::{CString};
use super::super::{libaudioverse_sys, server};
use super::Node;
use super::properties::{bool_property, double_property};
use check;
use Result;

/// Streams a file, which must be specified to the constructor and cannot be changed thereafter.
/// This node is a stopgap solution, and should be considered temporary. It will likely remain for backward compatibility. Libaudioverse plans to eventually offer a more generic streaming node that also supports web addresses; such a node will have a completely different, less buffer-like interface.
/// In order to stream a file, it must be passed through a resampler. Consequentlty, the position property is slightly inaccurate and the ended property and callback are slightly delayed.
/// 
/// This node has no inputs.
/// 
/// Outputs:
/// 
/// index | channels | description
/// ------|----------|------------
/// 0 | Depends on the file. | The output of the stream.
pub struct FileStreamerNode {
    handle: libaudioverse_sys::LavHandle
}

impl Node for FileStreamerNode {
    #[doc(hidden)]
    fn get_handle(&self) -> libaudioverse_sys::LavHandle {
        self.handle
    }
}

impl FileStreamerNode {
    /// Creates a new file streamer node.
    pub fn new(server : &server::Server, path : &CString) -> Result<FileStreamerNode> {
        let mut node_handle : libaudioverse_sys::LavHandle = 0;
        check(unsafe { libaudioverse_sys::Lav_createFileStreamerNode(server.handle, path.as_ptr(), &mut node_handle) })?;
        Ok(FileStreamerNode { handle : node_handle })
    }
    
    /// Returns the ended property.
    /// This property is read-only. Switches from false to true once the stream has ended completely and gone silent. This property will never go true unless looping is false.
    pub fn ended(&self) -> bool_property::BoolProperty {
        bool_property::BoolProperty { index : libaudioverse_sys::Lav_FILE_STREAMER_PROPERTIES_Lav_FILE_STREAMER_ENDED,  node_handle : self.handle }
    }
    
    // Returns the looping property.
    /// 
    /// Default value: False
    /// 
    /// If true, this node repeats the file from the beginning once it reaches the end. Note that setting looping means that ended will never go true. If ended is already true, it may take until the end of the next processing block for ended to properly go false once more.
    pub fn looping(&self) -> bool_property::BoolProperty {
        bool_property::BoolProperty { index : libaudioverse_sys::Lav_FILE_STREAMER_PROPERTIES_Lav_FILE_STREAMER_LOOPING,  node_handle : self.handle }
    }
    
    // Returns the position property.
    /// 
    /// Range: dynamic
    /// 
    /// Default value: 0.0
    /// 
    /// The position of playback, in seconds. The range of this property corresponds to the total duration of the file. Note that this property may be slightly inaccurate because this node has to pass data through a resampler.
    pub fn position(&self) -> double_property::DoubleProperty {
        double_property::DoubleProperty { index : libaudioverse_sys::Lav_FILE_STREAMER_PROPERTIES_Lav_FILE_STREAMER_POSITION,  node_handle : self.handle }
    }
}

