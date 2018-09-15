#![feature(nll)]
extern crate hound;
extern crate libaudioverse;
extern crate libaudioverse_sys;
use std::env;
use std::{thread, time};
use std::ffi::CString;
use libaudioverse::{Node, nodes::push_node::PushNode, Server};

fn main() {
    // Make a WavReader that reads the file provided as program argument.
    let fname = env::args().nth(1).expect("no file given");
    libaudioverse::initialize().unwrap();
    let mut reader = hound::WavReader::open(fname.clone()).unwrap();
    let spec = reader.spec();
    println!("{}: {:?}", fname, spec);
    let server = Server::construct(spec.sample_rate, 1024).unwrap();
    server.set_output_device_details(&CString::new("default").unwrap(), spec.channels as i32, 2).expect("Could not create default audio device");
    
    let samples_left = reader.len() as usize;
    let duration = reader.duration() / spec.sample_rate;
    println!("frames: {}, total samples: {}, duration: {}", reader.duration(), samples_left, duration);
    let frames = 4096;
    
    let mut push_node = PushNode::new(&server, spec.sample_rate, spec.channels as u32).expect("Could not create push node");
    let mut send_more_audio = |n : &mut PushNode| {
        let mut buf : Vec<f32> = vec![0.0; frames*spec.channels as usize];
        let mut i = 0;
        match spec.sample_format {
            hound::SampleFormat::Float => {
                for (dst, src) in buf.iter_mut().zip(reader.samples::<f32>()) {
                    *dst = src.unwrap();
                    i = i + 1;
                }
            }
            hound::SampleFormat::Int => {
                for (dst, src) in buf.iter_mut().zip(reader.samples::<i16>()) {
                    *dst = src.unwrap() as f32 / 32768.0;
                    i = i + 1;
                }
            }
        }
        println!("{}", i);
        n.feed(frames as u32 * spec.channels as u32, buf.as_mut_ptr()).expect("Failed to feed data to push node");
    };
    
    send_more_audio(&mut push_node);
    push_node.set_low_callback(|n: &mut PushNode| send_more_audio(n)).expect("Failed to set the low callback");
    
    push_node.connect_server(0).expect("Could not connect the push node to the server for playback");
    
    thread::sleep(time::Duration::from_secs((duration+1) as u64));
    libaudioverse::shutdown().unwrap();
}
