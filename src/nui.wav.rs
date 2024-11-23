use std::f32::consts::PI;
use std::i16;
use hound; 
use Mademoiselle_Entropia::true_rnd::get_true_rnd_u8 as u8__;
use Mademoiselle_Entropia::true_rnd::__get_true_rnd_i32 as i32__;
use Mademoiselle_Entropia::true_rnd::UID_UTF8 as mk_uid;
use crate::custom_traits::STRN;
use crate::{errMsg0, getkey, helpful_math_ops};
use serde::{Deserialize, Serialize, Serializer};

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