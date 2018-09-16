#![feature(nll)]
extern crate hound;
extern crate libaudioverse;
extern crate libaudioverse_sys;
use libaudioverse::{
    nodes::{Node, PullNode},
    Server,
};
use std::ffi::CString;
use std::{env, iter};
use std::{thread, time};

// Reads from a provided .wav file and plays it with a pull node, using Hound for wav decoding
fn main() {
    // Make a WavReader that reads the file provided as program argument.
    let fname = env::args().nth(1).expect("no file given");
    libaudioverse::initialize().unwrap();
    let mut reader = hound::WavReader::open(fname.clone()).unwrap();
    let spec = reader.spec();
    println!("{}: {:?}", fname, spec);
    let server = Server::construct(spec.sample_rate, 1024).unwrap();
    server
        .set_output_device_details(&CString::new("default").unwrap(), spec.channels as i32, 2)
        .expect("Could not create default audio device");

    let samples_left = reader.len() as usize;
    let duration = reader.duration() / spec.sample_rate;
    println!(
        "frames: {}, total samples: {}, duration: {}",
        reader.duration(),
        samples_left,
        duration
    );

    let mut pull_node = PullNode::new(&server, spec.sample_rate, spec.channels as u32)
        .expect("Could not create pull node");
    let send_more_audio = |_n: &PullNode, _frames: i32, _channels: i32, buf: &mut [f32]| {
        // Chain the iterator over the audio data with an infinite iterator that always returns zero.
        // Then, zip it with an iterator over the buffer that needs to be filled.
        // This has the effect of writing audio to the buffer when available, or zero otherwise.
        match spec.sample_format {
            hound::SampleFormat::Float => {
                let audio_iter = reader.samples::<f32>().chain(iter::repeat_with(|| Ok(0.0)));
                for (dst, src) in buf.iter_mut().zip(audio_iter) {
                    *dst = src.unwrap();
                }
            }
            hound::SampleFormat::Int => {
                let audio_iter = reader.samples::<i16>().chain(iter::repeat_with(|| Ok(0)));
                for (dst, src) in buf.iter_mut().zip(audio_iter) {
                    *dst = src.unwrap() as f32 / 32768.0;
                }
            }
        }
    };

    pull_node
        .set_audio_callback(send_more_audio)
        .expect("Failed to set the audio callback");

    pull_node
        .connect_server(0)
        .expect("Could not connect the pull node to the server for playback");

    thread::sleep(time::Duration::from_secs((duration + 1) as u64));
    libaudioverse::shutdown().unwrap();
}
