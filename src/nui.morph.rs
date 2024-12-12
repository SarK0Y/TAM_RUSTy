use std::f32::consts::PI;
use std::i16;
use hound; 
use once_cell::sync::Lazy;
use wavers::Samples;
use wavers::{Wav, read as wav_read, ConvertTo, write as wav_write};
use Mademoiselle_Entropia::true_rnd::get_true_rnd_u8 as u8__;
use Mademoiselle_Entropia::true_rnd::__get_true_rnd_u32 as u32__;
use Mademoiselle_Entropia::true_rnd::UID_UTF8 as mk_uid;
use crate::custom_input;
use crate::custom_traits::STRN;
use crate::{errMsg0, getkey, helpful_math_ops};
use serde::{Deserialize, Serialize, Serializer};
use spectrum_analyzer::{samples_fft_to_spectrum, FrequencyLimit};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
pub fn universum_vox_morph0 (duration: u16, path_to_conf: &String) {
     let mut uv_morph: crate::enums::universum_vox_morph =
                      match load_uv_conf_morph( path_to_conf ) {Ok (json ) => json, Err (e) => {eprintln! ("{e}");
                      err_msg_morph (); return;} };
    match uv_morph.alg0 {
        2 => {mk_morph_alg2_bin_data( &uv_morph ); return; },
        7 => {mk_morph_alg7_poly( &uv_morph ); return; },
        8 => {mk_morph_alg8_poly( &uv_morph ); return; },
        9 => {mk_morph_alg9_shark_fins( &uv_morph ); return; },
        _ => { },
    }
    
    let wav: Wav<f32> = Wav::from_path( &uv_morph.file_in ).unwrap();
    // conversion happens automatically when you read
    let ( mut samples, sample_rate): (wavers::Samples< f32 >, i32) = wav_read:: <f32, _ >( &uv_morph.file_in ).unwrap();
    //let mut samples: &mut [i32] = &mut samples;
    match uv_morph.alg0 {
        1 => {mk_morph_alg1_async( &mut samples, &uv_morph ); },
        3 => {mk_morph_alg3_warp( &mut samples, &uv_morph ); },
        4 => {mk_morph_alg4_warp( &mut samples, &uv_morph ); },
        5 => {mk_morph_alg5_simple_lpf( &mut samples, &uv_morph ); },
        6 => {mk_morph_alg6_simple_lpf( &mut samples, &uv_morph ); },
        _ => {mk_morph_alg0( &mut samples, &uv_morph ); },
    }
    //let file_name = format! ( "Universum Vox.{}.wav", mk_uid( 24 ));
    let full_path = format! ( "{}", uv_morph.file_out );
    wav_write(&uv_morph.file_out, &samples, uv_morph.sample_rate, uv_morph.num_of_channels as u16 ).unwrap();
    let msg = format! ("Dear User, data was written to {full_path}\nPlease, hit any key to continue.. Thanks.");
    errMsg0( &msg );
} 
pub fn load_uv_conf_morph <P: AsRef<Path> >(path: P) -> Result<crate::enums::universum_vox_morph, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let uv_wav: crate::universum_vox_morph = serde_json::from_reader(reader)?; 
    Ok (uv_wav )
}
pub fn mk_morph_alg0 ( samples: &mut [f32], uv: &crate::enums::universum_vox_morph ){
    let mut count_frames = 0u32;
    let mut count_zeros = 0u32;
    for h in 0..samples.len() {
        if samples [ h ] == 0.0 {count_zeros += 1;}
        if h % uv.num_of_channels as usize == 0 { count_frames.inc(); }
        if count_frames % uv.step_factor == 0 {
            if (count_frames / 2 )% 2 == 1{
                let a = samples [ h ];
                samples [h] +=  samples [ h  -1 ]; 
                samples [ h ] %= uv.bar_sample;
                samples [ h -1 ] = a / 2.0;
            }
            else { 
                samples [h ] = samples [ h ] / -2.0 ;
                }
         } else { samples [h ] *= uv.fading_step; }
    } 
    //dbg! (&samples [0..113]);
    dbg!(&count_zeros);
}
pub fn mk_morph_alg1_async ( samples: &mut [f32], uv: &crate::enums::universum_vox_morph ){
    let mut ch:  Vec < Vec <f32> > = Vec::new (); ch.push ( vec! () ); ch.push ( vec! () );
    let mut count_steps = 0usize; 
    let mut switch = 0usize;
    let range = uv.step_factor as usize;
    let half_range = range >> 1;
    loop {
        let mut samples_to_morph: Vec <_> = read_chan_f32(samples, switch, 2, count_steps, range)
            .into_iter()
            .rev()
            .collect();
        let written = write_chan_f32(samples, switch, 2, count_steps, &samples_to_morph );
        dbg!(&written);
        if written < half_range { return; }
        count_steps += range;
        dbg! (&count_steps);
        switch = !switch & 1;
    }
}
pub fn mk_morph_alg3_warp (samples: &mut [f32], uv: &crate::enums::universum_vox_morph ) {
  if uv.old_freq.is_none() || uv.new_freq.is_none() || uv.range.is_none() || uv.step_freq.is_none() {err_msg_morph(); return;}
  let mut step = 0.0f32;
  let range = uv.range.unwrap();
  let step_freq = uv.step_freq.unwrap();
  while range > step {
    replace_freq(samples, uv, step);
    step += step_freq;
  }
}
pub fn mk_morph_alg4_warp (samples: &mut [f32], uv: &crate::enums::universum_vox_morph ) {
  if uv.old_freq.is_none() || uv.plus_minus_freq.is_none() || uv.range.is_none() || uv.step_freq.is_none()
  || uv.scale.is_none () {err_msg_morph(); return;}
  let mut step = 0.0f32;
  let plus_or_not = uv.plus_minus_freq.unwrap ();
  let range = uv.range.unwrap();
  let mut exclude_freq0: fn (samples: &mut [f32], uv: &crate::enums::universum_vox_morph, step: f32 ) = if plus_or_not {
    exclude_freq1 } else { exclude_freq }; 
  let step_freq = uv.step_freq.unwrap();
  while range > step {
    exclude_freq0 (samples, uv, step);
    step += step_freq;
  }
}
pub fn mk_morph_alg7_poly (uv: &crate::enums::universum_vox_morph ) -> Result <(), Box <dyn Error> > {
  if uv.bar_sample == 0.0|| uv.file_out == "" || uv.sample_rate == 0 || uv.sound_duration.is_none() ||
  uv.num_of_channels == 0 {err_msg_morph(); return Ok (() );}
  let poly = |x: i128| -> f32 {
    let y = x.pow(4) + 17 * x.pow (3) + 129 * x.pow(2) + 19 * x + 31;
    (y as f64 % uv.bar_sample as f64) as f32
  };
  let mut samples: Vec < f32 > = Vec::new ();
  let num_of_samples = (uv.sound_duration.unwrap() * uv.num_of_channels as u32 * uv.sample_rate as u32) as usize;
  let from = 53usize;
  let mut y = 0.73f32;
  let mut count_fading = 0u32;
  let mut fading = 1.0;
  let mut fading_duration = uv.fading_duration;
  let low_fading_duration = uv.fading_duration / 5;
  let mut rnd = fading_duration = u32__() % uv.fading_duration;
  for k in from..(num_of_samples + from) {
    if y > 0.41 { y = poly (k as i128) * fading; }
    else { y = poly (k as i128) * -1.0 * fading; } 
    if count_fading < fading_duration { fading *= uv.fading_step; count_fading += 1; }
    else {fading = 1.0; count_fading = 0; fading_duration = u32__() % uv.fading_duration;
        if fading_duration < low_fading_duration {fading_duration = low_fading_duration;}
    }
    //dbg! (&fading); dbg! (&y);
    samples.push (y);   
  }
  let samples: Samples<f32> = Samples::from(samples.into_boxed_slice() ).convert();
  wav_write(&uv.file_out, &samples, uv.sample_rate, uv.num_of_channels as u16 )?;
  let msg = format! ("Dear User, data was written to {}\nPlease, hit any key to continue.. Thanks.", uv.file_out);
  errMsg0( &msg );
  Ok (())
}
pub fn mk_morph_alg8_poly (uv: &crate::enums::universum_vox_morph ) -> Result <(), Box <dyn Error> > {
  if uv.bar_sample == 0.0|| uv.file_out == "" || uv.sample_rate == 0 || uv.sound_duration.is_none() ||
  uv.num_of_channels == 0 {err_msg_morph(); return Ok (() );}
  let poly = |x: i128| -> f32 {
    let y = x.pow(4) + 17 * x.pow (3) + 129 * x.pow(2) + 19 * x + 31;
    (y as f64 % uv.bar_sample as f64) as f32
  };
  let mut samples: Vec < f32 > = Vec::new ();
  let num_of_samples = (uv.sound_duration.unwrap() * uv.num_of_channels as u32 * uv.sample_rate as u32) as usize;
  let from = 53usize;
  let mut y = 0.73f32;
  let mut count_fading = 0u32;
  let mut fading = 1.0;
  let mut fading_duration = u32__() % uv.fading_duration;
  let low_fading_duration = uv.fading_duration / 5;
  for k in from..(num_of_samples + from) {
    if y > 0.41 { y = poly (k as i128) * fading; }
    else { y = poly (k as i128) * -1.0 * fading; } 
    if count_fading < fading_duration { fading *= uv.fading_step; count_fading += 1; }
    else {fading = 1.0; count_fading = 0; fading_duration = roll_num::<u32> (fading_duration) ;
        if fading_duration < low_fading_duration {fading_duration = low_fading_duration;}
    }
    //dbg! (&fading); dbg! (&y);
    samples.push (y);   
  }
  let samples: Samples<f32> = Samples::from(samples.into_boxed_slice() ).convert();
  wav_write(&uv.file_out, &samples, uv.sample_rate, uv.num_of_channels as u16 )?;
  let msg = format! ("Dear User, data was written to {}\nPlease, hit any key to continue.. Thanks.", uv.file_out);
  errMsg0( &msg );
  Ok (())
}
pub fn mk_morph_alg5_simple_lpf (samples: &mut [f32], uv: &crate::enums::universum_vox_morph ) {
  if uv.coef.is_none () || uv.old_freq.is_none () {err_msg_morph (); return }
  let mut out = 0.0f32;
  let mut coef = 0.0f32;
  let cut_freq = uv.old_freq.as_ref().unwrap();
  let rc: f32 = 1.0 / (2.0 * PI * cut_freq );
  let mut coefs = uv.coef.as_ref().unwrap();
  let alpha = coefs[0]; // / (coefs [0] + rc);
  //coef = f32::powf( coefs[0], -1.0 * 2.0 * PI * coefs [1] * cut_freq );
  let mut ch0: Vec <_> = read_chan_f32(samples, 0, 2, 0, samples.len() );
  dbg!(ch0.len());
  ch0 [0] *= alpha;
  for i in 1..ch0.len() {
    ch0 [i ] = ch0 [i] * alpha * uv.fading_step + (1.0 - alpha ) * ch0 [i - 1] * uv.fading_step;
  }
  write_chan_f32(samples, 0, 2, 0, &ch0 );
  ch0 = read_chan_f32(samples, 1, 2, 0, samples.len() );
  ch0 [0] *= alpha;
  for i in 1..ch0.len() {
    ch0 [i ] = ch0 [i] * alpha * uv.fading_step + (1.0 - alpha ) * ch0 [i - 1] * uv.fading_step;
  }
  write_chan_f32(samples, 1, 2, 0, &ch0 );
}
pub fn mk_morph_alg6_simple_lpf (samples: &mut [f32], uv: &crate::enums::universum_vox_morph ) {
  if uv.coef.is_none () || uv.old_freq.is_none () {err_msg_morph (); return }
  let mut out = 0.0f32;
  let mut coef = 0.0f32;
  let cut_freq = uv.old_freq.as_ref().unwrap();
  let rc: f32 = 1.0 / (2.0 * PI * cut_freq );
  let mut coefs = uv.coef.as_ref().unwrap();
  let alpha = coefs[0]; // / (coefs [0] + rc);
  //coef = f32::powf( coefs[0], -1.0 * 2.0 * PI * coefs [1] * cut_freq );
  let mut ch0: Vec <_> = read_chan_f32(samples, 0, 2, 0, samples.len() );
  dbg!(ch0.len());
  ch0 [0] *= alpha;
  ch0 [1] = ch0 [1] * (1.0 - alpha) + ch0 [0];
  ch0 [2] = ch0 [2] * (1.0 - alpha).powi (2 ) + ch0 [1] + ch0 [0] ;
  //ch0 [3] = ch0 [3] * (1.0 - alpha.powi(2) ) + ch0 [2] + ch0 [1] + ch0 [0];
  for i in 4..ch0.len() {
    ch0 [i ] = ch0 [i] * alpha + (1.0 - alpha ) * ch0 [i - 1] + (1.0 - alpha ).powi(2) * ch0 [i - 2]; //+ (1.0 - alpha.powi(2) ) * ch0 [i - 3];
    ch0 [i] *= uv.fading_step;
  }
  write_chan_f32(samples, 0, 2, 0, &ch0 );
  ch0 = read_chan_f32(samples, 1, 2, 0, samples.len() );
  ch0 [0] *= alpha;
  ch0 [1] = ch0 [1] * (1.0 - alpha) + ch0 [0];
  ch0 [2] = ch0 [2] * (1.0 - alpha).powi (2 ) + ch0 [1] + ch0 [0] ;
  //ch0 [3] = ch0 [3] * (1.0 - alpha.powi(2) ) + ch0 [2] + ch0 [1] + ch0 [0];
  for i in 4..ch0.len() {
    ch0 [i ] = ch0 [i] * alpha + (1.0 - alpha ) * ch0 [i - 1] + (1.0 - alpha ).powi(2) * ch0 [i - 2]; //+ (1.0 - alpha.powi(2) ) * ch0 [i - 3];
    ch0 [i] *= uv.fading_step;
  }
  write_chan_f32(samples, 1, 2, 0, &ch0 );
}

pub fn replace_freq (samples: &mut [f32], uv: &crate::enums::universum_vox_morph, step: f32 ) {
    let old = (uv.old_freq.unwrap () + step) * 2.0 * PI;
    let new = (uv.new_freq.unwrap () + step) * 2.0 * PI;
    let mut fading = 1.0f32;
    let mut count_fading = 0u32;
    for s in 0..samples.len(){
        samples [ s ] -= (old * samples [s ]).sin() * fading;
        samples [ s ] += (new * samples [s ]).sin() * fading;
        if count_fading > uv.fading_duration {fading = 1.0; count_fading = 0; continue;}
        fading *= uv.fading_step;
        count_fading.inc();
    }
}
pub fn exclude_freq1 (samples: &mut [f32], uv: &crate::enums::universum_vox_morph, step: f32 ) {
    let old = (uv.old_freq.unwrap () + step) * 2.0 * PI;
    let new = (uv.new_freq.unwrap () + step) * 2.0 * PI;
    let mut fading = 1.0f32;
    let mut count_fading = 0u32;
    let scale = uv.scale.unwrap ();
    let mut dbg_from = uv.dbg_from.unwrap_or(0);
    let mut dbg_to = uv.dbg_to.unwrap_or(0);
    for s in 0..samples.len(){
        samples [ s ] += ( (old * samples [s ]).sin() * scale * fading );
        if count_fading > uv.fading_duration {fading = 1.0; count_fading = 0; continue;}
        if s > dbg_from as usize && dbg_to > 0 {crate::info::sav_dbg_msg (Some( samples [s].to_string() ) ); dbg_to.dec(); }
        fading *= uv.fading_step;
        count_fading.inc();
    }
}
pub fn exclude_freq (samples: &mut [f32], uv: &crate::enums::universum_vox_morph, step: f32 ) {
    let near_freq_step: usize = (uv.sample_rate as f32 / (uv.old_freq.unwrap() + step ) ).floor() as usize;
    let mut count = 0usize;
    let ch_num = 0usize;
    let mut samples0 = read_chan_f32(samples, ch_num, 2, 0, samples.len() );
    while samples0.len() > count{
       samples0 [count ] *= uv.fading_step;
       count += near_freq_step;
    }
    write_chan_f32(samples, ch_num, 2, 0, &samples0 );
    let ch_num = 1usize;
    count = 0;
    let mut samples0 = read_chan_f32(samples, ch_num, 2, 0, samples.len() );
    while samples0.len() > count{
       samples0 [count ] *= uv.fading_step;
       count += near_freq_step;
    }
    write_chan_f32(samples, ch_num, 2, 0, &samples0 );
}
pub fn mk_morph_alg2_bin_data (uv: &crate::enums::universum_vox_morph ) -> Result <(), Box <dyn Error> >{
    let samples: Vec <f32 > = crate::rw::read_file_to_vec::<f32>( &uv.file_in)?;
    let samples: &[f32] = &Samples::from (samples.into_boxed_slice() ).convert();
    wav_write(&uv.file_out, &samples, uv.sample_rate, uv.num_of_channels as u16 )?;
    let msg = format! ("Dear User, data was written to {}\nPlease, hit any key to continue.. Thanks.", uv.file_out);
    errMsg0( &msg );
    Ok (())
}
pub fn read_chan_f32 ( 
    samples: &mut [f32], 
    ch_num: usize, 
    num_of_channels: usize, start_from: usize, range: usize ) -> Vec < f32> {
    let mut ret: Vec < f32 > = Vec::new ();
    if start_from >= samples.len() { return ret;};
    let mut to = range;
    if to + start_from >= samples.len() { to = samples.len() - start_from; }
    let upto = to + start_from;
    dbg! (&to); dbg! (&start_from);
    for i in 0..to {
        let cursor = start_from + i * num_of_channels + ch_num;
        if cursor >= upto  { break;}
        ret.push ( samples [ cursor ] );
    }
    ret
}
pub fn write_chan_f32 ( 
    samples: &mut [f32], 
    ch_num: usize, 
    num_of_channels: usize, start_from: usize, patch: &Vec < f32 > ) -> usize {
    if start_from >= samples.len() { return 0 };
    let mut to = samples.len() - start_from;
    let upto = to + start_from;
    let mut cursor = 0usize;
    let mut prev = cursor;
    let mut cnt = 0usize;
    for i in 0..to {
        cursor = start_from + i * num_of_channels + ch_num;
        if cursor > upto  || i >= patch.len() { break;}
        samples [ cursor ] = patch [ i ];
        cnt = i;
        prev = cursor - ch_num;
    } dbg! (&cursor); dbg!(cnt); prev
}
pub fn mk_morph_alg9_shark_fins (uv: &crate::enums::universum_vox_morph ) -> Result <(), Box <dyn Error> >{
    let width = uv.step_factor as i32;
    let amplitude = uv.bar_sample;
    let fin = shark_fin(width, amplitude);
    let mut samples: Vec <f32> = Vec::new ();
    let mut count_down: u32 = uv.sound_duration.unwrap ();
    while count_down > 0{
        for j in &fin {
            samples.push ( *j );
        }
        for null in 0..uv.silent_step {
            samples.push (0.0)
        } count_down.dec();
    }
    let samples: &[f32] = &Samples::from (samples.into_boxed_slice() ).convert();
    wav_write(&uv.file_out, &samples, uv.sample_rate, uv.num_of_channels as u16 )?;
    let msg = format! ("Dear User, data was written to {}\nPlease, hit any key to continue.. Thanks.", uv.file_out);
    errMsg0( &msg );
    Ok (())
}

pub fn shark_fin (width: i32, amplitude: f32) -> Vec <f32> {
    let mut fin: Vec <f32> = Vec::new ();
    let fst_part = 2 * width / 3; 
    let nd_part = width / 3; 
    let mut amplitude0 = 2.0 * amplitude / 3.0;
    let mut growing = 1.0 / calc_fading_coef(amplitude0, amplitude, 0.006, fst_part);
    let mut step = amplitude / 6.0;
    for s in 0..fst_part{
        fin.push (amplitude0);
        let tmp = amplitude0 * growing;
        if tmp < amplitude {amplitude0 = tmp}
        else { break; }
    }
    let mut fading = calc_fading_coef(0.001, amplitude0, 0.006, nd_part);
    for s in 0..nd_part {
        amplitude0 *= fading;
        fin.push (amplitude0);
    }
    dbg!(&amplitude0);
    fin
}
pub fn calc_fading_coef (bar: f32, amplitude: f32, err: f32, pow: i32 ) -> f32 {
    let mut fading = 0.5f32;
    let mut res = amplitude * fading.powi (pow);
    let mut count_down = 100;
    let mut down = 2.0f32;
    let hi_err = 1.0 + err;
    loop {
        if count_down == 0 {break;}
        if (res - bar).abs() >= err {break;}
        if res > bar {
            fading -= fading / 2.0;
        } else {
            let tmp = fading + fading / down;
            if tmp >= 1.0 {down += 1.0;}
            else { fading = tmp; }
         }
        res = amplitude * fading.powi (pow);
        count_down.dec();
    }
    dbg!( &fading );
    fading
}
pub fn calc_fading_step (bar: f32, amplitude: f32, err: f32 ) -> f32 {
    let mut fading = amplitude / bar;
    fading
}
pub fn err_msg_morph (){
    errMsg0( "Dear user, You need to set json properly.. Look example: {
            \"type_\":\"wav\",\n
            \"alg0\":1,\n
            \"num_of_channels\":2,\n
            \"num_of_rnd_samples\":233(null),\n
            \"sound_duration\":150,\n
            \"sample_rate\":44100,\n
            \"sample_format\":\"f32\"(or \"i32\"/\"i16\"),\n
            \"bar_sample\":0.94,\n
            \"scale\":1.54,\n
            \"fading_duration\":1110,\n
            \"step_factor\":8,\n
            \"fading_step\":0.83,\n
            \"silent_step\":113,\n
            \"old_freq\":5287.7,\n
            \"new_freq\":7287.7,\n
            \"step_freq\":2.3,\n
            \"range\":75.8,\n
            \"coef\":[1.97,0.94],\n
            \"plus_minus_freq\":false,\n
            \"file_in\":\"/tmp/in.wav\",\n
            \"file_out\":\"/tmp/out.wav\",\n
} "); 
}
pub fn roll_num <P: 
std::ops::Mul + 
num_traits::Pow < usize, Output = P> +
std::ops::BitAnd +
std::ops::Add +
std::ops::BitAndAssign +
std::ops::AddAssign +
std::marker::Copy > (x: P) -> P {
    let mut y = x.pow(2);
    y &= x;
    y += x; y
}
//fn
//let tan = (PI * uv.old_freq.unwrap() / uv.sample_rate as f32).tan();
  //  let shift_coef = (tan - 1.0) / (tan + 1.0);
    
//https://docs.rs/spectrum-analyzer/latest/spectrum_analyzer/
/*
for i in (0..10).map(|x| x as f64 * 0.1) {
    println!("{:.1}", i);
}
...............
use std::iter::successors;

let iter = successors(Some(0.1), |&x| {
    if x < 1.0 {
        Some(x + 0.1)
    } else {
        None
    }
});

for i in iter {
    println!("{:.1}", i);
}
-----------
let vec = vec![1, 2, 3, 4, 5];
let reversed: Vec<_> = vec.into_iter().rev().collect();
-------------
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