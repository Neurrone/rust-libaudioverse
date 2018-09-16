use super::super::{libaudioverse_sys, server};
use super::properties::FloatProperty;
use super::Node;
use check;
use std::mem;
use std::os::raw::c_void;
use Result;

/// The purpose of this node is the same as the pull node, but it is used in situations wherein we do not know when we are going to get audio. Audio is queued as it is pushed to this node and then played as fast as possible. This node can be used to avoid writing a queue of audio yourself, as it essentially implements said functionality. If you need low latency audio or the ability to run something like the Opus encoder’s ability to cover for missing frames, you need a pull node.
///
/// This node has no inputs.
///
/// Outputs:
///
/// index | channels | description
/// ------|----------|------------
/// 0 | Depends on arguments to this node’s constructor. | Either audio from the internal queue or zero.
pub struct PushNode<'node> {
    pub handle: libaudioverse_sys::LavHandle,
    pub low_callback: Option<Box<Box<'node + FnMut(&mut PushNode)>>>,
    pub underrun_callback: Option<Box<Box<'node + FnMut(&mut PushNode)>>>,
}

impl<'node> Node for PushNode<'node> {
    #[doc(hidden)]
    fn get_handle(&self) -> libaudioverse_sys::LavHandle {
        self.handle
    }
}

impl<'node> PushNode<'node> {
    /// Creates a new push node.
    pub fn new(server: &server::Server, sr: u32, channels: u32) -> Result<PushNode> {
        let mut node_handle: libaudioverse_sys::LavHandle = 0;
        check(unsafe {
            libaudioverse_sys::Lav_createPushNode(server.handle, sr, channels, &mut node_handle)
        })?;
        Ok(PushNode {
            handle: node_handle,
            low_callback: None,
            underrun_callback: None,
        })
    }

    /// Returns the threshold property.
    ///
    /// Range: [0.0, 'INFINITY']
    ///
    /// Default Value: 0.03
    ///
    /// Rate: k
    ///
    /// When the remaining audio in the push node has a duration less than this property, the low callback is called.
    pub fn threshold(&self) -> FloatProperty {
        FloatProperty {
            index: libaudioverse_sys::Lav_PUSH_NODE_PROPERTIES_Lav_PUSH_THRESHOLD,
            node_handle: self.handle,
        }
    }

    /// Sets the low callback, which is Called once per block and outside the audio thread when there is less than the specified threshold audio remaining.
    pub fn set_low_callback<F>(&mut self, callback: F) -> Result<()>
    where
        F: 'node + FnMut(&mut PushNode),
    {
        let cb: Box<Box<FnMut(&mut PushNode)>> = Box::new(Box::new(callback));
        let cb_ptr = Box::into_raw(cb);
        self.low_callback = Some(unsafe { Box::from_raw(cb_ptr) });
        check(unsafe {
            libaudioverse_sys::Lav_pushNodeSetLowCallback(
                self.handle,
                Some(callback_handler),
                cb_ptr as *mut _,
            )
        })?;
        Ok(())
    }

    /// Sets the underrun callback, which is Called exactly once and outside the audio thread when the node runs out of audio completely.
    pub fn set_underrun_callback<F>(&mut self, callback: F) -> Result<()>
    where
        F: 'node + FnMut(&mut PushNode),
    {
        let cb: Box<Box<FnMut(&mut PushNode)>> = Box::new(Box::new(callback));
        let cb_ptr = Box::into_raw(cb);
        self.underrun_callback = Some(unsafe { Box::from_raw(cb_ptr) });
        check(unsafe {
            libaudioverse_sys::Lav_pushNodeSetUnderrunCallback(
                self.handle,
                Some(callback_handler),
                cb_ptr as *mut _,
            )
        })?;
        Ok(())
    }

    /// Feed more audio data into the internal queue.
    pub fn feed(&self, length_in_samples: u32, frames: *mut f32) -> Result<()> {
        check(unsafe {
            libaudioverse_sys::Lav_pushNodeFeed(self.handle, length_in_samples, frames)
        })?;
        Ok(())
    }
}

/// Handles callbacks from Libaudioverse, allowing closures to be used as callbacks.
extern "C" fn callback_handler(node_handle: libaudioverse_sys::LavHandle, userdata: *mut c_void) {
    let closure: &mut Box<FnMut(&mut PushNode)> = unsafe { mem::transmute(userdata) };
    let mut node = PushNode {
        handle: node_handle,
        low_callback: None,
        underrun_callback: None,
    };
    closure(&mut node)
}
