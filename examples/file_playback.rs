use std::ffi::CString;
use std::{env, thread, time};
extern crate libaudioverse;
use libaudioverse::{
    nodes::{BufferNode, Node},
    Buffer, Server,
};

// Plays the provided .ogg file
fn main() {
    let fname = env::args().nth(1).expect("no file given");
    libaudioverse::initialize().unwrap();

    // server initialization
    let server = Server::new().unwrap();
    server
        .set_output_device()
        .expect("Could not create default audio device");

    // create a new buffer
    let buf = Buffer::new(&server).unwrap();
    let path = CString::new(fname).expect("The file name is not a valid C string");

    // load audio data from test.ogg into this buffer
    buf.load_from_file(&path).unwrap();

    // create a Buffer node, associate it with the buffer just created, and play it
    let buf_node = BufferNode::new(&server).unwrap();
    buf_node.buffer().set(&buf).unwrap();
    buf_node.connect_server(0).unwrap();

    // wait for the whole file to finish playing
    let duration = buf_node.position().get_range().unwrap().1.ceil() as u64;
    thread::sleep(time::Duration::from_secs(duration));

    libaudioverse::shutdown().unwrap();
}
