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
/*pub fn universum_vox_wav0 (duration: u16, path_to_conf: &String) {
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
            \"sample_format\":\"Float\"(or \"Int\"),\n
            \"bar_sample\":0.94,\n
            \"amplitude\":2077,\n
            \"fading_duration\":1110,\n
            \"fading_step\":0.83,\n
            \"silent_step\":113,\n
} "); return;} };
    let spec = hound::WavSpec {
        channels: uv_wav.num_of_channels,
        sample_rate: uv_wav.sample_rate,
        bits_per_sample: uv_wav.bits_per_sample,
        sample_format: uv_wav.sample_format.conv(),
    };
    let mut samples = gen_rnd_vals_f32( uv_wav.num_of_rnd_samples, &uv_wav);
    let num_of_samples_to_gen = uv_wav.sample_rate as u64 * duration as u64;
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
} */
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


 */