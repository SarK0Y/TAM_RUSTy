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