//fn
//https://github.com/RustAudio/cpal/blob/master/examples/enumerate.rs

/*
use cpal::{Sample};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() {
    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
    let host = cpal::default_host();
    let device = host.default_output_device().expect("no output device available");
    let supported_config = device.default_output_config().unwrap();

    println!("Device: {}, Using config: {:?}", device.name().expect("flute"), supported_config);

    let config = supported_config.into();

    let stream = device.build_output_stream(&config, write_silence, err_fn).unwrap();
    stream.play().unwrap();

    std::thread::sleep(std::time::Duration::from_millis(3000));
}

fn write_silence(data: &mut [f32], _: &cpal::OutputCallbackInfo) {
    let mut counter = 0;
    for sample in data.iter_mut() {
        let s = if (counter / 20) % 2 == 0 { &1.0 } else { &0.0 };
        counter = counter + 1;
        *sample = Sample::from(s);
    }
    println!("{:?}", data);
}
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Sample, SampleFormat};
use std::sync::{Arc, Mutex};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = cpal::default_host();
    let input_device = host.default_input_device().expect("no input device available");
    let output_device = host.default_output_device().expect("no output device available");
    let config = input_device.default_input_config()?;

    let buffer = Arc::new(Mutex::new(Vec::new()));
    let buffer_clone = buffer.clone();

    let input_stream = input_device.build_input_stream(
        &config.into(),
        move |data: &[f32], _: &_| {
            let mut buffer = buffer_clone.lock().unwrap();
            buffer.extend_from_slice(data);
        },
        |err| eprintln!("an error occurred on input stream: {}", err),
        None,
    )?;

    let output_stream = output_device.build_output_stream(
        &config.into(),
        move |data: &mut [f32], _: &_| {
            let mut buffer = buffer.lock().unwrap();
            let len = data.len().min(buffer.len());
            data[..len].copy_from_slice(&buffer[..len]);
            buffer.drain(..len);
        },
        |err| eprintln!("an error occurred on output stream: {}", err),
        None,
    )?;

    input_stream.play()?;
    output_stream.play()?;
    std::thread::sleep(std::time::Duration::from_secs(10));
    Ok(())
}
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Sample, SampleFormat};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = cpal::default_host();
    let device = host.default_input_device().expect("no input device available");
    let config = device.default_input_config()?;

    let stream = device.build_input_stream(
        &config.into(),
        move |data: &[f32], _: &_| {
            // Process the input data here
            println!("Received {} samples", data.len());
        },
        |err| eprintln!("an error occurred on stream: {}", err),
        None,
    )?;

    stream.play()?;
    std::thread::sleep(std::time::Duration::from_secs(5));
    Ok(())
}
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Sample, SampleFormat};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("no output device available");
    let config = device.default_output_config()?;

    let sample_format = config.sample_format();
    let config = config.into();

    let stream = match sample_format {
        SampleFormat::F32 => device.build_output_stream(&config, write_sine::<f32>, err_fn, None)?,
        SampleFormat::I16 => device.build_output_stream(&config, write_sine::<i16>, err_fn, None)?,
        _ => return Err("Unsupported sample format".into()),
    };

    stream.play()?;
    std::thread::sleep(std::time::Duration::from_secs(1));
    Ok(())
}

fn write_sine<T: Sample>(data: &mut [T], _: &cpal::OutputCallbackInfo) {
    for (i, sample) in data.iter_mut().enumerate() {
        let x = i as f32 / 44100.0;
        *sample = Sample::from(&(x * 440.0 * 2.0 * std::f32::consts::PI).sin());
    }
}

fn err_fn(err: cpal::StreamError) {
    eprintln!("an error occurred on stream: {}", err);
}
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Sample, SampleFormat};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("no output device available");
    let config = device.default_output_config()?;

    let stream = match config.sample_format() {
        SampleFormat::F32 => device.build_output_stream(&config.into(), write_sine::<f32>, err_fn, None)?,
        SampleFormat::I16 => device.build_output_stream(&config.into(), write_sine::<i16>, err_fn, None)?,
        _ => return Err("Unsupported sample format".into()),
    };

    stream.play()?;
    std::thread::sleep(std::time::Duration::from_secs(1));
    Ok(())
}

fn write_sine<T: Sample>(data: &mut [T], _: &cpal::OutputCallbackInfo) {
    for (i, sample) in data.iter_mut().enumerate() {
        let x = i as f32 / 44100.0;
        *sample = Sample::from(&(x * 440.0 * 2.0 * std::f32::consts::PI).sin());
    }
}

fn err_fn(err: cpal::StreamError) {
    eprintln!("an error occurred on stream: {}", err);
}
use cpal::traits::{DeviceTrait, HostTrait};
let mut supported_configs_range = device.supported_output_configs()
    .expect("error while querying configs");
let supported_config = supported_configs_range.next()
    .expect("no supported config?!")
    .with_max_sample_rate();
Now that we have everything for the stream, we are ready to create it from our selected device:

use cpal::Data;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
let stream = device.build_output_stream(
    &config,
    move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        // react to stream events and read or write stream data here.
    },
    move |err| {
        // react to errors here.
    },
    None // None=blocking, Some(Duration)=timeout
);
*/