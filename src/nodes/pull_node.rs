use super::super::{libaudioverse_sys, server};
use super::Node;
use check;
use std::mem;
use std::os::raw::c_void;
use std::slice;
use Result;

/// This node calls the audio callback whenever it needs more audio. The purpose of this node is to inject audio from an external source that Libaudioverse does not support, for example a custom network protocol. If you need low latency audio or the ability to run something like the Opus encoder’s ability to cover for missing frames, you need a pull node.
///
/// This node has no inputs.
///
/// Outputs:
///
/// index | channels | description
/// ------|----------|------------
/// 0 | Depends on arguments to this node’s constructor. | The result of the configured callback.

pub struct PullNode<'node> {
    handle: libaudioverse_sys::LavHandle,
    audio_callback: Option<Box<Box<'node + FnMut(&PullNode, i32, i32, &mut [f32])>>>,
}

impl<'node> Node for PullNode<'node> {
    #[doc(hidden)]
    fn get_handle(&self) -> libaudioverse_sys::LavHandle {
        self.handle
    }
}

impl<'node> PullNode<'node> {
    /// Creates a new Pull node.
    pub fn new(server: &server::Server, sr: u32, channels: u32) -> Result<PullNode> {
        let mut node_handle: libaudioverse_sys::LavHandle = 0;
        check(unsafe {
            libaudioverse_sys::Lav_createPullNode(server.handle, sr, channels, &mut node_handle)
        })?;
        Ok(PullNode {
            handle: node_handle,
            audio_callback: None,
        })
    }

    /// Sets the audio callback used when the node needs more audio.
    ///
    /// Callback parameters:
    ///
    /// Parameter | description
    /// ------|------------
    /// node: &PullNode | The node which called this callback.
    /// frames: i32 | The number of frames of audio needed. This is not guaranteed to be the same on every call.
    /// channels: i32 | The number of channels as set when the pull node is created.
    /// buffer: &mut [f32] | The destination to which audio should be written. This is a buffer that is frames*channels long. Write interleaved audio data to it and use zeroes when the callback has less data than requested. Do not assume that it is zeroed.
    pub fn set_audio_callback<F>(&mut self, callback: F) -> Result<()>
    where
        F: 'node + FnMut(&PullNode, i32, i32, &mut [f32]),
    {
        let cb: Box<Box<FnMut(&PullNode, i32, i32, &mut [f32])>> = Box::new(Box::new(callback));
        let cb_ptr = Box::into_raw(cb);
        self.audio_callback = Some(unsafe { Box::from_raw(cb_ptr) });
        check(unsafe {
            libaudioverse_sys::Lav_pullNodeSetAudioCallback(
                self.handle,
                Some(callback_handler),
                cb_ptr as *mut _,
            )
        })?;
        Ok(())
    }
}

/// Handles callbacks from Libaudioverse, allowing closures to be used as callbacks.
extern "C" fn callback_handler(
    node_handle: libaudioverse_sys::LavHandle,
    frames: i32,
    channels: i32,
    buffer: *mut f32,
    userdata: *mut c_void,
) {
    let buf = unsafe { slice::from_raw_parts_mut(buffer, (frames * channels) as usize) };
    let closure: &mut Box<FnMut(&PullNode, i32, i32, &mut [f32])> =
        unsafe { mem::transmute(userdata) };
    let node = PullNode {
        handle: node_handle,
        audio_callback: None,
    };
    closure(&node, frames, channels, buf)
}
