//! The main entry point to Libaudioverse.
use std::ffi::{CString};
use super::*;
use super::libaudioverse_sys;
use check;

/// Represents a server, the main entry point to Libaudioverse. All libaudioverse nodes must be passed a server at creation time as the first argument to their constructor and cannot migrate between them. Furthermore, it is an error to try to connect objects from different servers.
/// By default, Libaudioverse will use one thread per core on the current system for audio mixing. This may be changed via Lav_serverSetThreads.
/// For full details of this class, see the Libaudioverse manual.
pub struct Server {
    pub(crate) handle: libaudioverse_sys::LavHandle,
}

impl Server {
    /// Creates a new server with a default sampling rate of 44,100 and a block size of 1024. This is sufficient and performant for most applications.
    pub fn new() -> Result<Server> {
        Server::construct(44100, 1024)
    }
    
    /// create a new server with the specified sampling rate and block size. The block size is the number of samples to process at once, and must be a multiple of 4.
    pub fn construct(sampling_rate : u32, block_size : u32) -> Result<Server> {
        let mut handle: libaudioverse_sys::LavHandle = 0;
        check(unsafe { libaudioverse_sys::Lav_createServer(sampling_rate, block_size, &mut handle) })?;
        Ok(Server { handle })
    }
    
    /// Set the output  of the server to the system's default audio device with 2 channels and 2 mixahead.
    pub fn set_output_device(&self) -> Result<()> {
        let default = CString::new("default").unwrap();
        self.set_output_device_details(&default, 2, 2)
    }
    
    /// Set the output device of the server. 
    /// Use the literal string "default" for the default audio device. Note that it is possible to change the output device of a server even after it has been set.
    /// After the output device has been set, calls to Lav_serverGetBlock will error.
    pub fn set_output_device_details(&self, identifier : &CString, channels : i32, mixahead : i32) -> Result<()> {
        check(unsafe { Lav_serverSetOutputDevice(self.handle, identifier.as_ptr(), channels, mixahead) })
    }
 
/* 
    pub fn create_buffer(&self) -> Result<buffer::Buffer> {
        let mut buf_handle : libaudioverse_sys::LavHandle = 0;
        check(unsafe { Lav_createBuffer(self.handle, &mut buf_handle) })?;
        Ok(buffer::Buffer { handle : buf_handle })
    }
  
    pub fn create_buffer_node(&self) -> Result<buffer_node::BufferNode> {
        let mut node_handle : libaudioverse_sys::LavHandle = 0;
        check(unsafe { libaudioverse_sys::Lav_createBufferNode(self.handle, &mut node_handle) })?;
        Ok(buffer_node::BufferNode { handle : node_handle })
    }
  */  
}

#[test]
fn can_create_server_with_default_audio_device() {
    initialize().unwrap();
    let server = Server::new().unwrap();
    server.set_output_device().expect("Could not create default audio device");
    shutdown().unwrap();
}