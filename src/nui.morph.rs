use std::f32::consts::PI;
use std::i16;
use hound; 
use wavers::{Wav, read, ConvertTo};
use Mademoiselle_Entropia::true_rnd::get_true_rnd_u8 as u8__;
use Mademoiselle_Entropia::true_rnd::__get_true_rnd_i32 as i32__;
use Mademoiselle_Entropia::true_rnd::UID_UTF8 as mk_uid;
use crate::custom_traits::STRN;
use crate::{errMsg0, getkey, helpful_math_ops};
use serde::{Deserialize, Serialize, Serializer};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
pub fn universum_vox_morph0 (duration: u16, path_to_conf: &String) {
     let mut uv_wav: crate::enums::universum_vox_morph =
                      match load_uv_conf_morph( path_to_conf ) {Ok (json ) => json, Err (e) => {eprintln! ("{e}");
                      errMsg0( "Dear user, You need to set json properly.. Look example: {
            \"type_\":\"wav\",\n
            \"alg0\":1,\n
            \"num_of_channels\":2,\n
            \"num_of_rnd_samples\":233(null),\n
            \"sample_rate\":44100,\n
            \"sample_format\":\"f32\"(or \"i32\"/\"i16\"),\n
            \"bar_sample\":0.94,\n
            \"amplitude\":2077,\n
            \"fading_duration\":1110,\n
            \"fading_step\":0.83,\n
            \"silent_step\":113,\n
            \"file_in\":\"/tmp/in.wav\",\n
            \"file_out\":\"/tmp/out.wav\",\n
} "); return;} };
   
    match uv_wav.alg0 {
        1 => {mk_samples_alg1( &mut samples, num_of_samples_to_gen, &uv_wav ); },
        _ => {mk_samples_alg0( &mut samples, num_of_samples_to_gen, &uv_wav ); },
    }
    let file_name = format! ( "Universum Vox.{}.wav", mk_uid( 24 ));
    let full_path = format! ( "{}/{file_name}", crate::cmd_keys::midi_dir ( None ) );
    let mut writer = hound::WavWriter::create(&full_path, spec).unwrap();
    for j in samples {
        writer.write_sample( j );
    }
    let msg = format! ("Dear User, data was written to {full_path}\nPlease, hit any key to continue.. Thanks.");
    errMsg0( &msg );
} 
pub fn load_uv_conf_morph <P: AsRef<Path> >(path: P) -> Result<crate::enums::universum_vox_morph, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let uv_wav: crate::universum_vox_morph = serde_json::from_reader(reader)?; 
    Ok (uv_wav )
}
//fn
/*
use hound;

fn main() {
    let mut reader = hound::WavReader::open("path/to/file.wav").unwrap();
    let samples: Vec<i16> = reader.samples().map(|s| s.unwrap()).collect();
    
    println!("Number of samples: {}", samples.len());
    println!("Sample rate: {}", reader.spec().sample_rate);
}

use wavers::{Wav, read};

fn main() {
    let fp = "path/to/file.wav";
    let (samples, sample_rate) = read::<i16, _>(fp).unwrap();
    
    println!("Number of samples: {}", samples.len());
    println!("Sample rate: {}", sample_rate);
}

use hound;

// Reading samples from a specific channel
let mut reader = hound::WavReader::open("input.wav").unwrap();
let channel = 0; // 0 for left, 1 for right in stereo files
let samples: Vec<i16> = reader.samples::<i16>()
    .step_by(reader.spec().channels as usize)
    .skip(channel)
    .map(|s| s.unwrap())
    .collect();

// Writing samples to a specific channel
let spec = hound::WavSpec {
    channels: 2,
    sample_rate: 44100,
    bits_per_sample: 16,
    sample_format: hound::SampleFormat::Int,
};
let mut writer = hound::WavWriter::create("output.wav", spec).unwrap();
for sample in samples {
    writer.write_sample(sample).unwrap();
    writer.write_sample(0).unwrap(); // Write 0 to the other channel
}
writer.finalize().unwrap();

use wavers::{Wav, read, write};

// Reading samples from a specific channel
let (samples, sample_rate) = read::<i16, _>("input.wav").unwrap();
let channel = 0;
let channel_samples: Vec<i16> = samples.iter()
    .skip(channel)
    .step_by(samples.channels() as usize)
    .cloned()
    .collect();

// Writing samples to a specific channel
let mut output_samples = vec![0i16; samples.len()];
for (i, &sample) in channel_samples.iter().enumerate() {
    output_samples[i * 2 + channel] = sample;
}
write("output.wav", &output_samples, sample_rate, 2).unwrap();

use wavers::{Wav, read};
use std::path::Path;

fn main() {
	let fp = "path/to/wav.wav";
    // creates a Wav file struct, does not read the audio data. Just the header information.
    let wav: Wav<i16> = Wav::from_path(fp).unwrap();
    // or to read the audio data directly
    let (samples, sample_rate): (Samples<i16>, i32) = read::<i16, _>(fp).unwrap();
    // samples can be derefed to a slice of samples
    let samples: &[i16] = &samples;
}

use wavers::{Wav, read, ConvertTo};
use std::path::Path;

fn main() {
    // Two ways of converted a wav file
    let fp: "./path/to/i16_encoded_wav.wav";
    let wav: Wav<f32> = Wav::from_path(fp).unwrap();
    // conversion happens automatically when you read
    let samples: &[f32] = &wav.read().unwrap();

    // or read and then call the convert function on the samples.
    let (samples, sample_rate): (Samples<i16>, i32) = read::<i16, _>(fp).unwrap();
    let samples: &[f32] = &samples.convert();
}

use wavers::Wav;
use std::path::Path;

fn main() {
	let fp: &Path = &Path::new("path/to/wav.wav");
	let out_fp: &Path = &Path::new("out/path/to/wav.wav");

    // two main ways, read and write as the type when reading
    let wav: Wav<i16> = Wav::from_path(fp).unwrap();
    wav.write(out_fp).unwrap();

    // or read, convert, and write
    let (samples, sample_rate): (Samples<i16>,i32) = read::<i16, _>(fp).unwrap();
    let sample_rate = wav.sample_rate();
    let n_channels = wav.n_channels();

    let samples: &[f32] = &samples.convert();
    write(out_fp, samples, sample_rate, n_channels).unwrap();
}

 #[test]
    fn write_sin_wav() {
        let fp = "./wav.wav";
        let sr: i32 = 16000;
        let duration = 10;
        let mut samples: Vec<f32> = (0..sr * duration).map(|x| (x as f32 / sr as f32)).collect();
        for sample in samples.iter_mut() {
            *sample *= 440.0 * 2.0 * std::f32::consts::PI;
            *sample = sample.sin();
            *sample *= i16::MAX as f32;
        }
        let samples: Samples<f32> = Samples::from(samples.into_boxed_slice().convert_slice());

        write(fp, &samples, sr, 1).unwrap();
        std::fs::remove_file(fp).unwrap();
    }
 */