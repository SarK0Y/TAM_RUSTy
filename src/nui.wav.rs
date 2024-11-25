use std::f32::consts::PI;
use std::i16;
use hound; 
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
pub fn universum_vox_wav0 (duration: u16, path_to_conf: &String) {
     let mut uv_wav: crate::enums::universum_vox_wav =
                      match load_uv_conf_wav( path_to_conf ) {Ok (json ) => json, Err (e) => {eprintln! ("{e}");
                      errMsg0( "Dear user, You need to set json properly.. Look example: {
            \"type_\":\"wav\",\n
            \"alg0\":1,\n
            \"num_of_channels\":2,\n
            \"sound_duration\":150,\n
            \"num_of_rnd_samples\":233,\n
            \"sample_rate\":44100,\n
            \"bits_per_sample\":32,\n
            \"sample_format\":SampleFormat::Int(SampleFormat::Float),
            \"bar_sample\":0.94,\n
            \"amplitude\":2077,\n
} "); return;} };
    let spec = hound::WavSpec {
        channels: uv_wav.num_of_channels,
        sample_rate: uv_wav.sample_rate,
        bits_per_sample: uv_wav.bits_per_sample,
        sample_format: uv_wav.sample_format.conv(),
    };
    let mut samples = gen_rnd_vals_f32( uv_wav.num_of_rnd_samples);
    let num_of_samples_to_gen = uv_wav.sample_rate as u64 * duration as u64;
    match uv_wav.alg0 {
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
pub fn mk_samples_alg0 <'a> ( arr: &'a mut Vec <f32>, len: u64, uv: &crate::enums::universum_vox_wav ) -> &'a mut Vec < f32 >{
    for j in 0..arr.len() { arr [j] *= uv.amplitude; }
    let num_of_rnd_samples = arr.len (); let mut count_samples = num_of_rnd_samples;
    for i in 0..len {
        let sample = (arr [i as usize ].powf( 3.3 ) + arr [ arr.len () - 1 ].powf (2.1) + 1.0 ) % uv.bar_sample;
        arr.push ( sample );
    }
    arr
}
pub fn load_uv_conf_wav <P: AsRef<Path> >(path: P) -> Result<crate::enums::universum_vox_wav, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let uv_wav: crate::universum_vox_wav = serde_json::from_reader(reader)?; 
    Ok (uv_wav )
}
pub trait Conv {
    fn conv (&self) -> hound::SampleFormat;
}
impl Conv for crate::enums::SampleFormat {
    fn conv (&self) -> hound::SampleFormat {
        match self {
            crate::SampleFormat::Float => return hound::SampleFormat::Float,
            crate::SampleFormat::Int => return hound::SampleFormat::Int,
        }
    }
}
pub fn gen_rnd_vals_f32 (num_of_vals: u32) -> Vec < f32 > {
    let mut rnd_f32s = Vec:: <f32>::new ();
    for j in 0..num_of_vals {
        let rnd = 1.0 / i32__() as f32;
        rnd_f32s.push ( rnd );
    } rnd_f32s
}
//fn
/*
use std::f32::consts::PI;
use std::i16;
use hound;

let spec = hound::WavSpec {
    channels: 1,
    sample_rate: 44100,
    bits_per_sample: 16,
    sample_format: hound::SampleFormat::Int,
};

let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();

for t in (0..44100).map(|x| x as f32 / 44100.0) {
    let sample = (t * 440.0 * 2.0 * PI).sin();
    let amplitude = i16::MAX as f32;
    writer.write_sample((sample * amplitude) as i16).unwrap();
}

writer.finalize().unwrap();
*/