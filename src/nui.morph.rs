use std::f32::consts::PI;
use std::i16;
use hound; 
use once_cell::sync::Lazy;
use wavers::Samples;
use wavers::{Wav, read as wav_read, ConvertTo, write as wav_write_};
use Mademoiselle_Entropia::true_rnd::get_true_rnd_u8 as u8__;
use Mademoiselle_Entropia::true_rnd::__get_true_rnd_u32 as u32__;
use Mademoiselle_Entropia::true_rnd::__get_true_rnd_i32 as i32__;
use Mademoiselle_Entropia::true_rnd::UID_UTF8 as mk_uid;
use crate::custom_input;
use crate::custom_traits::STRN;
use crate::faav::{read_saved_geom, unset_geom};
use crate::{errMsg0, getkey, helpful_math_ops};
use serde::{Deserialize, Serialize, Serializer};
use spectrum_analyzer::{samples_fft_to_spectrum, FrequencyLimit};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
pub fn universum_vox_morph0 (duration: u16, path_to_conf: &String) {
    crate::faav::get_morph_state();
     let mut uv_morph: crate::enums::universum_vox_morph =
                      match load_uv_conf_morph( path_to_conf ) {Ok (json ) => json, Err (e) => {eprintln! ("{e}");
                      err_msg_morph (); return;} };
    match uv_morph.alg0 {
        2 => {mk_morph_alg2_bin_data( &uv_morph ); return; },
        7 => {mk_morph_alg7_poly( &uv_morph ); return; },
        8 => {mk_morph_alg8_poly( &uv_morph ); return; },
        9 => {mk_morph_alg9_shark_fins( &uv_morph ); return; },
        10 => {mk_morph_alg10_shark_fins( &uv_morph ); return; },
        11 => {mk_morph_alg11_ellipse_like( &uv_morph ); return; },
        12 => {mk_morph_alg12_tria( &uv_morph ); return; },
        13 => {mk_morph_alg13_tria_full( &uv_morph ); return; },
        _ => { },
    }
    
    let wav: Wav<f32> = Wav::from_path( &uv_morph.file_in ).unwrap();
    // conversion happens automatically when you read
    if let Some( uv_path ) = &uv_morph.sub_config {
        universum_vox_morph0(duration, uv_path);
    }
    let ( mut samples, sample_rate): (wavers::Samples< f32 >, i32) = if crate::faav::get_morph_state().is_none(){
        let data = wav_read:: <f32, _ >( &uv_morph.file_in ).unwrap();
        crate::faav::set_morph_state(&Some(data.clone() ) );
        data
    } else { crate::faav::get_morph_state().unwrap() };
    //let mut samples: &mut [i32] = &mut samples;
    match uv_morph.alg0 {
        1 => {mk_morph_alg1_async( &mut samples, &uv_morph ); },
        3 => {mk_morph_alg3_warp( &mut samples, &uv_morph ); },
        4 => {mk_morph_alg4_warp( &mut samples, &uv_morph ); },
        5 => {mk_morph_alg5_simple_lpf( &mut samples, &uv_morph ); },
        6 => {mk_morph_alg6_simple_lpf( &mut samples, &uv_morph ); },
        14 => {mk_morph_alg14_abval( &mut samples, &uv_morph ); },
        15 => {mk_morph_alg15_rot_ampl( &mut samples, &uv_morph ); },
        16 => {mk_morph_alg16_half_elliptic_sound( &mut samples, &uv_morph ); },
        17 => {mk_morph_alg17_shaped_frame( &mut samples, &uv_morph ); },
        _ => {mk_morph_alg0( &mut samples, &uv_morph ); },
    }
    //let file_name = format! ( "Universum Vox.{}.wav", mk_uid( 24 ));
    let full_path = format! ( "{}", uv_morph.file_out );
    wav_write(&uv_morph.file_out, &samples, uv_morph.sample_rate, uv_morph.num_of_channels as u16 ).unwrap();
    let msg = format! ("Dear User, data was written to {full_path}\nPlease, hit any key to continue.. Thanks.");
    if uv_morph.file_out.len() > 0 { errMsg0( &msg ); crate::faav::unset_morph_state();}
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
pub fn mk_morph_alg14_abval (samples: &mut [f32], uv: &crate::enums::universum_vox_morph ) {
    let sign = if uv.plus_minus_freq.unwrap_or ( true ) { 1.0f32 } else { -1.0};
    for i in 0..samples.len (){
        samples [i] = samples [i].abs () * sign
    }
}
pub fn mk_geom (uv: &crate::enums::universum_vox_morph ) {
    use crate::enums::geom;
    let geoms = if let Some (shapes ) = &uv.geoms { shapes.clone() }else {
        dbg! (&uv.geoms); err_msg_morph();  return;};
    let mut shaped_frame = Vec::<f32>::new();
    let mut shape = Vec::<f32>::new();
    for j in geoms{
        match j {
            geom::half_ellipse { from, to, lb, n } => {shape = half_ellipse_like(from, to, lb, n as usize);},
            geom::tria { a, b, bar, step, direct } => {shape = if direct {tria(a, b, step, bar)} else {
                tria(a, b, step, bar).into_iter().rev().collect()
            }},
            geom::tria_full { a, b, a1, b1, step, bar } => {shape = trias_full(a, b, a1, b1, step, bar)},
            geom::shark_fin { w, h, lb } => {shape = if lb.is_some() {shark_fin3(w, h, lb ) } else {
                shark_fin2(w, h) }
            }
            _ => {}
        }
        for s in &shape{ shaped_frame.push(*s); }
       }
    crate::faav::pocket_geom( &shaped_frame );
}
pub fn mk_morph_alg15_rot_ampl (samples: &mut [f32], uv: &crate::enums::universum_vox_morph ) {
    let mut roll = i32__();
    for i in 0..samples.len (){
        samples [i] = samples [i] * -1.0_f32.powi( roll & 1 );
        roll = roll_num::<i32> ( roll );
    }
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
pub fn mk_morph_alg18_acute_freq (samples: &mut [f32], uv: &crate::enums::universum_vox_morph ) {
    let main_freq = if let Some (x) = uv.old_freq {x} else {err_msg_morph(); return;};
    let mut main_freq = if uv.sample_rate <= main_freq as i32 {errMsg0("You need to set frequency less than sample rate."); return;} 
    else {uv.sample_rate as usize / main_freq as usize};
    let ch_num = uv.select_channel.unwrap_or(0) as usize;
    let domain_complete_on = uv.coef.clone().unwrap_or(vec![0.77, 0.0])[0];
    let mut freq_step = uv.coef.clone().unwrap_or(vec![0.77, 0.0])[1];
    if freq_step == 0.0 {
        
        if main_freq > 1 {freq_step = 1.0 / (1.0 - 1.0 / main_freq as f32);} else {
            freq_step = 1.0 / (1.0 - 1.0 / 1.1);
        }
    }
    let mut sign = 1.0f32;
    let ampl: [f32; 2] = [freq_step, 1.0 / freq_step ];
    let mut up_domain_complete_on = 0usize;
    let mut down_domain_complete_on = 0usize;
    let mut prev_up_down = false;
    let mut cur_up_down = false;
    let ch0 = read_chan_f32(samples, ch_num, uv.num_of_channels.into(), 0, samples.len() );
    for i in 0..samples.len() / main_freq  {

        for j in 0..main_freq {
            let adr = i * main_freq + j;
            if adr >= samples.len() {break;}
            if samples [adr ] < 0.0 { down_domain_complete_on.inc(); } else {up_domain_complete_on.inc();}
        }
        if down_domain_complete_on > up_domain_complete_on { sign = -1.0; } else {sign = 1.0; }
        for j in 0..main_freq {
            let adr = i * main_freq + j;
            if adr >= samples.len() {break;}
            match sign {
                -1.0 => {
                    samples [adr] *= ampl [samples[adr].is_sign_negative() as usize ];
                },
                1.0 => {
                    samples [adr] *= ampl [samples[adr].is_sign_negative() as usize ];
                },
                _ => {}
            }
        }
        
    }
    write_chan_f32(samples, ch_num, uv.num_of_channels.into(), 0, &ch0);
}
pub fn mk_morph_alg17_shaped_frame (samples: &mut [f32], uv: &crate::enums::universum_vox_morph ) {
    let len = uv.step_factor as usize;
    let sign: f32 = if uv.plus_minus_freq.unwrap_or (true) == true { 1.0 }else { -1.0 };
    /*let mut tst = uv.clone();
    tst.geoms = Some( vec![crate::enums::geom::tria {a:1.5, b: 0.7, bar: 0.97, step: 0.04, direct: false},
                            crate::enums::geom::shark_fin{w:150, h:0.9, lb:Some(19.1) }] );
    println!("{}", serde_json::to_string (&tst).unwrap() ); */
    dbg!(&sign);
    let to = uv.bar_sample;
    let base = uv.scale.unwrap();
    unset_geom();
    mk_geom(uv);
    let frame = read_saved_geom(); 
    let frame_len = frame.len();
    if frame_len == 0 {return;}
    let num_of_channels = 2usize;
    let ch_num = uv.select_channel.unwrap_or (0) as usize;
    let frame_tst = frame.clone();
    //wav_write_("/tst/shapes.wav", &Samples::from (frame_tst).convert::<f32>(), uv.sample_rate, num_of_channels as u16);
    let ch0 = read_chan_f32(samples, ch_num, num_of_channels, 0, samples.len() );
    for i in 0..samples.len() {
        if samples [i] * sign < 0.0 { continue;}
        samples [i] *= frame [i % frame_len ];
    }
    write_chan_f32(samples, ch_num, num_of_channels, 0, &ch0);
}
pub fn mk_morph_alg16_half_elliptic_sound (samples: &mut [f32], uv: &crate::enums::universum_vox_morph ) {
    let len = uv.step_factor as usize;
    let sign: f32 = if uv.plus_minus_freq.unwrap_or (true) == true { 1.0 }else { -1.0 };
    dbg!(&sign);
    let to = uv.bar_sample;
    let base = uv.scale.unwrap();
    let frame = half_ellipse_like(0.0, to, base, len);
    let frame_len = frame.len();
    let num_of_channels = 2usize;
    let ch_num = 0usize;
    let ch0 = read_chan_f32(samples, ch_num, num_of_channels, 0, samples.len() );
    for i in 0..samples.len() {
        if samples [i] * sign < 0.0 { continue;}
        samples [i] *= frame [i % frame_len ];
    }
    write_chan_f32(samples, ch_num, num_of_channels, 0, &ch0);
    let ch_num = 1usize;
    let ch0 = read_chan_f32(samples, ch_num, num_of_channels, 0, samples.len() );
    for i in 0..samples.len() {
        if samples [i] * sign < 0.0 { continue;}
        samples [i] *= frame [i % frame_len ];
    }
    write_chan_f32(samples, ch_num, num_of_channels, 0, &ch0);
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
pub fn mk_morph_alg13_tria_full (uv: &crate::enums::universum_vox_morph ) -> Result <(), Box <dyn Error> >{
    let mut coefs: Vec <f32> = if let Some (x) = &uv.coef { x.clone() }else {err_msg_morph(); return Ok (());};
    let bar = uv.bar_sample;
    let a = coefs [0];
    let b = coefs [1];
    let a0 = coefs [2];
    let b0 = coefs [3];
    let step = coefs [4];
    let fin = trias_full(a, b, a0, b0, step, bar);
    let mut samples: Vec <f32> = Vec::new ();
    let mut count_down: u32 = uv.sound_duration.unwrap ();
    while count_down > 0{
        for j in &fin {
            samples.push ( *j );
        }
        if uv.plus_minus_freq.unwrap() == true {
            for null in 0..uv.silent_step {
                samples.push (0.0)
            }
        } else {
            for j in &fin {
               samples.push ( 0.0 - *j );
            }   
        }
         count_down.dec();
    }
   let samples: &[f32] = &Samples::from (samples.into_boxed_slice() ).convert();
    wav_write(&uv.file_out, &samples, uv.sample_rate, uv.num_of_channels as u16 )?;
    let msg = format! ("Dear User, data was written to {}\nPlease, hit any key to continue.. Thanks.", uv.file_out);
    errMsg0( &msg );
    Ok (())
}
pub fn mk_morph_alg12_tria (uv: &crate::enums::universum_vox_morph ) -> Result <(), Box <dyn Error> >{
    let mut coefs: Vec <f32> = if let Some (x) = &uv.coef { x.clone() }else {err_msg_morph(); return Ok (());};
    let bar = uv.bar_sample;
    let a = coefs [0];
    let b = coefs [1];
    let step = coefs [2];
    let fin = trias(a, b, step, bar);
    let mut samples: Vec <f32> = Vec::new ();
    let mut count_down: u32 = uv.sound_duration.unwrap ();
    while count_down > 0{
        for j in &fin {
            samples.push ( *j );
        }
        if uv.plus_minus_freq.unwrap() == true {
            for null in 0..uv.silent_step {
                samples.push (0.0)
            }
        } else {
            for j in &fin {
               samples.push ( 0.0 - *j );
            }   
        }
         count_down.dec();
    }
   let samples: &[f32] = &Samples::from (samples.into_boxed_slice() ).convert();
    wav_write(&uv.file_out, &samples, uv.sample_rate, uv.num_of_channels as u16 )?;
    let msg = format! ("Dear User, data was written to {}\nPlease, hit any key to continue.. Thanks.", uv.file_out);
    errMsg0( &msg );
    Ok (())
}
pub fn mk_morph_alg11_ellipse_like (uv: &crate::enums::universum_vox_morph ) -> Result <(), Box <dyn Error> >{
    let len = uv.step_factor as usize;
    let amplitude = uv.bar_sample;
    let base = uv.scale.unwrap();
    let fin = half_ellipse_like(0.0, amplitude, base, len);
    let mut samples: Vec <f32> = Vec::new ();
    let mut count_down: u32 = uv.sound_duration.unwrap ();
    while count_down > 0{
        for j in &fin {
            samples.push ( *j );
        }
        if uv.plus_minus_freq.unwrap() == true {
            for null in 0..uv.silent_step {
                samples.push (0.0)
            }
        } else {
            for j in &fin {
               samples.push ( 0.0 - *j );
            }   
        }
         count_down.dec();
    }
    let samples: &[f32] = &Samples::from (samples.into_boxed_slice() ).convert();
    wav_write(&uv.file_out, &samples, uv.sample_rate, uv.num_of_channels as u16 )?;
    let msg = format! ("Dear User, data was written to {}\nPlease, hit any key to continue.. Thanks.", uv.file_out);
    errMsg0( &msg );
    Ok (())
}
pub fn mk_morph_alg10_shark_fins (uv: &crate::enums::universum_vox_morph ) -> Result <(), Box <dyn Error> >{
    let width = uv.step_factor as i32;
    let amplitude = uv.bar_sample;
    let fin = shark_fin2(width, amplitude);
    let mut samples: Vec <f32> = Vec::new ();
    let mut count_down: u32 = uv.sound_duration.unwrap ();
    while count_down > 0{
        for j in &fin {
            samples.push ( *j );
        }
        if uv.plus_minus_freq.unwrap() == true {
            for null in 0..uv.silent_step {
                samples.push (0.0)
            }
        } else {
            for j in &fin {
               samples.push ( 0.0 - *j );
            }   
        }
         count_down.dec();
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
        if tmp < amplitude && tmp > 0.0 {amplitude0 = tmp}
        else { break; }
    }
    //fin = fin.into_iter().rev().collect();
    let mut fading = calc_fading_coef(0.001, amplitude0, 0.006, nd_part);
    for s in 0..nd_part {
        amplitude0 *= fading;
        fin.push (amplitude0);
    }
    dbg!(&amplitude0);
    fin
}
pub fn shark_fin2 (width: i32, amplitude: f32) -> Vec <f32> {
    let mut fin: Vec <f32> = Vec::new ();
    let fst_part = 2 * width / 3; 
    let nd_part = width / 3; 
    let mut amplitude0 = amplitude / 3.0;
    let mut growing = log_grow_up_to(amplitude0, amplitude, 17.0, fst_part as usize);
    let mut step = amplitude / 6.0;
    for s in growing{
        fin.push ( s );
        
    }
    //fin = fin.into_iter().rev().collect();
    let mut amplitude0 = amplitude;
    let mut fading = calc_fading_coef(0.001, amplitude0, 0.006, nd_part);
    for s in 0..nd_part {
        amplitude0 *= fading;
        fin.push (amplitude0);
    }
    dbg!(&amplitude0);
    fin
}
pub fn shark_fin3 (width: i32, amplitude: f32, lb: Option < f32>) -> Vec <f32> {
    let mut fin: Vec <f32> = Vec::new ();
    let fst_part = 2 * width / 3; 
    let nd_part = width / 3; 
    let mut amplitude0 = amplitude / 3.0;
    let base = if let Some (lb0) = lb { lb0 } else {17.0}; 
    let mut growing = log_grow_up_to(amplitude0, amplitude, 17.0, fst_part as usize);
    let mut step = amplitude / 6.0;
    for s in growing{
        fin.push ( s );
        
    }
    //fin = fin.into_iter().rev().collect();
    let mut amplitude0 = amplitude;
    let mut fading = calc_fading_coef(0.001, amplitude0, 0.006, nd_part);
    for s in 0..nd_part {
        amplitude0 *= fading;
        fin.push (amplitude0);
    }
    dbg!(&amplitude0);
    fin
}

pub fn half_ellipse_like (from: f32, to: f32, base: f32, range: usize) -> Vec <f32> {
let mut ret = Vec::<f32>::new();
let fst_4th: Vec <f32> = log_grow_up_to (from, to, base, range); 
let nd_4th: Vec <f32> = fst_4th.clone().into_iter().rev().collect();
for j in 0..fst_4th.len(){ ret.push( fst_4th [j] ); }
for j in 0..nd_4th.len(){ ret.push( nd_4th [j] ); }
ret
}
pub fn trias_full (a: f32, b: f32, a0: f32, b0: f32, step: f32, bar: f32) -> Vec <f32> {
let mut ret = Vec::<f32>::new();
let fst_tria: Vec <f32> = tria(a, b, step, bar); 
let nd_tria: Vec <f32> = tria(a0, b0, step, bar).into_iter().rev().collect();
for j in 0..fst_tria.len(){ ret.push( fst_tria [j] ); }
for j in 0..nd_tria.len(){ ret.push( nd_tria [j] ); }
ret
}
pub fn trias (a: f32, b: f32, step: f32, bar: f32) -> Vec <f32> {
let mut ret = Vec::<f32>::new();
let fst_tria: Vec <f32> = tria(a, b, step, bar); 
let nd_tria: Vec <f32> = fst_tria.clone().into_iter().rev().collect();
for j in 0..fst_tria.len(){ ret.push( fst_tria [j] ); }
for j in 0..nd_tria.len(){ ret.push( nd_tria [j] ); }
ret
}
pub fn tria (a: f32, b: f32, step: f32, bar: f32) -> Vec <f32> {
    let mut ret = Vec::<f32>::new();
    let mut x = 0.0f32;
    for j in 0..1_000_000_000{
        let y = a * x + b;
        if y >= 0.0 && y < bar {ret.push (y );}
        if y > bar {break} x += step;
    }
    ret
}
pub fn log_grow_up_to (from: f32, to: f32, base: f32, range: usize) -> Vec <f32> {
    let mut ret = Vec::<f32>::new();
    for j in 1..range{
        let sample = (j as f32).log (base);
        if sample >= from && sample < to {ret.push (sample);}
        if sample > to {break}
    }
    ret
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
pub fn wav_write(file_out: &String, samples: &[f32], sample_rate: i32, num_of_channels: u16 ) -> Result <(), Box <dyn Error> >{
    if file_out != "" { wav_write_(file_out, samples, sample_rate, num_of_channels )?;}
    else {crate::faav::set_morph_state(&Some( (Samples::from(samples).convert(), sample_rate) ) );}
    Ok (())
}
pub fn err_msg_morph (){
    errMsg0( "Dear user, You need to set json properly.. Look example: {
            \"type_\":\"morph\",\n
            \"alg0\":17,\n
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
            \"select_channel\":1,\n
            \"geoms\":[{\"tria\":{\"a\":1.2,\"b\":0.1,\"step\":0.04,\"bar\":0.94,\"direct\":false}},
                       {\"shark_fin\":{\"w\":541,\"h\":0.91,\"lb\":16.4} }],

            \"sub_config\":\"/tst/sub_config01.uv(or null)\",\n
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
num_traits::PrimInt +
std::marker::Copy +
std::fmt::Debug > (x: P) -> P {
    let cur_type = std::any::type_name:: <P> ();
    let mut y = x;
    match cur_type {
    "i32" | "u32" | "i64" | "u64" => { 
        let pow = P::from( 7 ).unwrap ();
        y =num_traits::checked_pow(x, 2).unwrap_or( pow ); },
    _ => {errMsg0("fn roll_num failed"); return x;}
}
    y.rotate_left( x.to_u32().unwrap_or(3) );
    y += x; 
    //dbg! (&y);
    y
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