//! Storage for audio data.
use std::ffi::{CString };
use std :: os :: raw;
use super::*;
use super::{libaudioverse_sys, server};
use check;

/// Buffers store un-encoded float32 audio data at the sampling rate of the server. They can be loaded from files or arrays, and will resample the data exactly once when loaded. Buffers are most commonly used with buffer nodes.
/// Save for the contained audio data, buffers are stateless; using them requires coupling them with a node. Since buffers are quite large, using a cache is recommended. Buffers may safely be used in more than one place at a time. Modifying a buffer’s audio data while it is in use will result in an error.
pub struct Buffer {
    // make handle visible for  BufferProperty's usage
    pub(crate) handle: libaudioverse_sys::LavHandle,
}

impl Buffer {
    /// Creates a new audio buffer.
    pub fn new(server : &server::Server) -> Result<Buffer> {
        let mut buf_handle : libaudioverse_sys::LavHandle = 0;
        check(unsafe { libaudioverse_sys::Lav_createBuffer(server.handle, &mut buf_handle) })?;
        Ok(Buffer { handle : buf_handle })
    }
    
    /// Get the duration of the buffer in seconds.
    pub fn get_duration(&self) -> Result<f32> {
        let mut duration : f32 = 0.0;
        check(unsafe { libaudioverse_sys::Lav_bufferGetDuration(self.handle, &mut duration) })?;
        Ok(duration)
    }
    
    /// Get the length of the specified buffer in samples. The sample rate of a buffer is the sample rate of the server for which that buffer was created. 
    /// This function is primarily useful for estimating ram usage in caching structures.
    pub fn get_length_in_samples(&self) -> Result<i32> {
        let mut samples : i32 = 0;
        check(unsafe { libaudioverse_sys::Lav_bufferGetLengthInSamples(self.handle, &mut samples) })?;
        Ok(samples)
    }
    
    /// Takes an encoded array of audio data and decodes it.
    pub fn decode_from_array(&self, data : &mut [raw::c_char]) -> Result<()> {
        check(unsafe { libaudioverse_sys::Lav_bufferDecodeFromArray(self.handle, data.as_mut_ptr(), data.len() as i32) })?;
        Ok(())
    }
    
    /// Load from an array of interleaved floats.
    pub fn load_from_array(&self, sampling_rate : i32, channels : i32, frames : i32, data : &mut [f32]) -> Result<()> {
        check(unsafe { libaudioverse_sys::Lav_bufferLoadFromArray(self.handle, sampling_rate, channels, frames, data.as_mut_ptr()) })?;
        Ok(())
    }
    
    /// Loads data into this buffer from a file. The file will be resampled to the sampling rate of the server. This will happen synchronously.
    pub fn load_from_file(&self, path : &CString) -> Result<()> {
        check(unsafe { libaudioverse_sys::Lav_bufferLoadFromFile(self.handle, path.as_ptr()) })?;
        Ok(())
    }
    
    /// Normalizes the buffer.
    pub fn normalize(&self) -> Result<()> {
        check(unsafe { libaudioverse_sys::Lav_bufferNormalize(self.handle) })
    }
    
}