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
                      match load_uv_conf( path_to_conf ) {Ok (json ) => json, Err (e) => {eprintln! ("{e}");
                      errMsg0( "Dear user, You need to set json properly.. Look example: {
            \"alg0\":1,\n
            \"num_of_channels\":16,\n
            \"duration\":150,\n
            \"const_duration\":false,\n
            \"deviate_duration\":0,\n
            \"velocity_level\":211,\n
            \"const_velocity\":true,\n
            \"range\":null,\n
            \"bottom\":17,\n
            \"arr\":[63,78]\n-----\nRemark: max range of notes is [0..128]
} "); return;} };
    let file_name = format! ( "Universum Vox.{}.wav", mk_uid( 24 ));
    let full_path = format! ( "{}/{file_name}", crate::cmd_keys::midi_dir ( None ) );
    let msg = format! ("Dear User, data was written to {full_path}\nPlease, hit any key to continue.. Thanks.");
    errMsg0( &msg );
}
pub fn load_uv_conf <P: AsRef<Path> >(path: P) -> Result<crate::enums::universum_vox_wav, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let uv_wav: crate::universum_vox_wav = serde_json::from_reader(reader)?; 
    Ok (uv_wav )
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