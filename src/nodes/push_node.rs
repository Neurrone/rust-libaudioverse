use std::{mem, ptr};
use super::super::{libaudioverse_sys, server};
use super::Node;
use super::properties::{ float_property};
use check;
use Result;
use std::os::raw::{c_void};

/// The purpose of this node is the same as the pull node, but it is used in situations wherein we do not know when we are going to get audio. Audio is queued as it is pushed to this node and then played as fast as possible. This node can be used to avoid writing a queue of audio yourself, as it essentially implements said functionality. If you need low latency audio or the ability to run something like the Opus encoder’s ability to cover for missing frames, you need a pull node.
/// 
/// This node has no inputs.
/// 
/// Outputs:
/// 
/// index | channels | description
/// ------|----------|------------
/// 0 | Depends on arguments to this node’s constructor. | Either audio from the internal queue or zero.

pub type C = FnMut(&mut PushNode);
pub type CallbackWrapper = Option<Box<Box<'node + C>>>;

pub struct PushNode<'node> {
    handle: libaudioverse_sys::LavHandle,
    low_callback: Option<Box<Box<'node + FnMut(libaudioverse_sys::LavHandle)>>>
}

impl<'node> Node for PushNode<'node> {
    #[doc(hidden)]
    fn get_handle(&self) -> libaudioverse_sys::LavHandle {
        self.handle
    }
}

impl<'node> PushNode<'node> {
    /// Creates a new gain node.
    pub fn new(server : &server::Server, sr: u32, channels: u32) -> Result<PushNode> {
        let mut node_handle : libaudioverse_sys::LavHandle = 0;
        check(unsafe { libaudioverse_sys::Lav_createPushNode(server.handle, sr, channels, &mut node_handle) })?;
        Ok(PushNode { handle : node_handle, low_callback: None })
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
    pub fn threshold(&self) -> float_property::FloatProperty {
        float_property::FloatProperty { index : libaudioverse_sys::Lav_PUSH_NODE_PROPERTIES_Lav_PUSH_THRESHOLD, node_handle : self.handle }
    }

    /// Sets the low callback, which is Called once per block and outside the audio thread when there is less than the specified threshold audio remaining.
    /*
    pub fn set_low_callback(&mut self, callback: libaudioverse_sys::LavParameterlessCallback) -> Result<()> {
        check(unsafe { libaudioverse_sys::Lav_pushNodeSetLowCallback(self.handle, callback, ptr::null_mut()) })?;
        Ok(())
    }
*/

    pub fn set_low_callback<F>(&'node mut self, callback: F) -> Result<()>
     where F: 'node + FnMut(libaudioverse_sys::LavHandle) {
        let cb: Box<Box<FnMut(libaudioverse_sys::LavHandle)>> = Box::new(Box::new(callback));
        // self.lowCallback = Some(cb);
        // let mut cb_ref = self.lowCallback.as_mut().unwrap();
        // check(unsafe { libaudioverse_sys::Lav_pushNodeSetLowCallback(self.handle, Some(lowCallbackHandler), &mut **cb_ref as *mut _) })?;
        // let cb_ptr = Box::leak(cb);
        let cb_ptr = Box::into_raw(cb);
        self.low_callback = Some(unsafe { Box::from_raw(cb_ptr) });
        check(unsafe { libaudioverse_sys::Lav_pushNodeSetLowCallback(self.handle, Some(low_callback_handler), cb_ptr as *mut _) })?;
        Ok(())
    }
    
    /// Sets the underrun callback, which is Called exactly once and outside the audio thread when the node runs out of audio completely.
    pub fn set_underrun_callback(&mut self, callback: libaudioverse_sys::LavParameterlessCallback) -> Result<()> {
        check(unsafe { libaudioverse_sys::Lav_pushNodeSetUnderrunCallback(self.handle, callback, ptr::null_mut()) })?;
        Ok(())
    }

    /// Feed more audio data into the internal queue.
    pub fn feed(&self, length_in_samples: u32, frames: *mut f32) -> Result<()> {
        check(unsafe { libaudioverse_sys::Lav_pushNodeFeed(self.handle, length_in_samples, frames) })?;
        Ok(())
    }

}

pub extern "C" fn low_callback_handler(node_handle: libaudioverse_sys::LavHandle, userdata: *mut c_void){ 
    let closure: &mut Box<FnMut(libaudioverse_sys::LavHandle)> = unsafe { mem::transmute(userdata) };
    closure(node_handle)
}