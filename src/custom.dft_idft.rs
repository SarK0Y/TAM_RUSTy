use num::complex::Complex;
use num::Float;
use std::f32::consts::PI;
use std::f32::consts::E;
use once_cell::sync::Lazy;
use num::complex::ComplexFloat;
use crate::enums::freq_range;
use crate::errMsg0;
pub fn custom_dft (samples: &mut [f32], 
    from: usize,
    to: usize,
    spectre: freq_range, alt_const_e: Option <f32>) -> crate::enums::custom_dft{
    let alt_const = if let Some (e) = alt_const_e {e} else {E};
    let mut cdft = crate::enums::custom_dft {
        amplitude: Vec::<f32>::new(),
        freq: Vec::<f32>::new(),
        phase: Vec::<f32>::new(),
    };
    let mut z_sample: Complex<f32> = Complex::new (0.0, 0.0);
    let norm_to = to - from;
    let j: Complex<f32> = Complex::new (0.0, 1.0);
    for freq0 in 0..1_000_000_000 {
        let freq = (spectre.from + spectre.step * freq0 as f32);
        if freq > spectre.to {break;}
        for t in 0..norm_to {
            let coef = alt_const.powc ((-2.0 * PI * t as f32 * j/ norm_to as f32) * freq );
            z_sample += samples[from + t] * coef;
        }
        cdft.amplitude.push ((z_sample.re.powi(2) + z_sample.im.powi (2) ).sqrt() );
        cdft.phase.push ((z_sample.im / z_sample.re).atan() );
        cdft.freq.push(freq);
    }
cdft
}
pub fn custom_idft (cdft: crate::enums::custom_dft,
    frame_len: usize, // im samples 
    alt_const_e: Option <f32>) -> Vec <f32> {
        let alt_const = if let Some (e) = alt_const_e {e} else {E};
        let mut samples = Vec::<f32>::new();
        for i in 0..frame_len { samples.push (0.0); }
        for t in 0..frame_len {
            for s in 0..cdft.phase.len() {
                samples[t] += (2.0 *PI * cdft.freq [s] * t as f32 + cdft.phase [s]).sin () * cdft.amplitude[s];
            }
        }
        samples
    }
pub fn simple_n_fast_dft (samples: &mut [f32], 
    from: usize,
    to: usize,
    spectre: freq_range ) -> crate::enums::custom_dft{
    let mut cdft = crate::enums::custom_dft {
        amplitude: Vec::<f32>::new(),
        freq: Vec::<f32>::new(),
        phase: Vec::<f32>::new(),
    };
    let mut z_sample: Complex<f32> = Complex::new (0.0, 0.0);
    let norm_to = to - from;
    let j: Complex<f32> = Complex::new (0.0, 1.0);
    for freq0 in 0..1_000_000_000 {
        let freq = (spectre.from + spectre.step * freq0 as f32);
        if freq > spectre.to {break;}
        for t in 0..norm_to {
            let coef = 1.0 / alt_e2jx(freq, t);      
            z_sample += samples[from + t] * coef;
        }
        cdft.amplitude.push ((z_sample.re.powi(2) + z_sample.im.powi (2) ).sqrt() );
        cdft.phase.push ((z_sample.im / z_sample.re).atan() );
        cdft.freq.push(freq);
    }
cdft
}
pub fn simple_n_fast_idft (cdft: crate::enums::custom_dft,
    frame_len: usize, // im samples 
    alt_const_e: Option <f32>) -> Vec <f32> {
        let alt_const = if let Some (e) = alt_const_e {e} else {E};
        let mut samples = Vec::<f32>::new();
        for i in 0..frame_len { samples.push (0.0); }
        for t in 0..frame_len {
            for s in 0..cdft.phase.len() {
                samples[t] += phi_sine(cdft.freq[s], t, cdft.phase[s]) * cdft.amplitude[s];
            }
        }
        samples
    }
pub fn init_speedy_sine (init_freq: Option <f32>, sample_rate: u32, out_freq: f32) 
    -> Option <Vec<f32> > {
    static mut precalc: Lazy< Vec<f32> > = Lazy::new(|| {Vec::new()});
    static mut base0: Option <f32> = None;
    let mut base = 0.0f32;
    let mut precalc_len =0usize;
    unsafe {
        if let Some (x) = init_freq {base0 = Some (x); }
        else {return Some (precalc.clone() ); }
        if base0.is_none() {errMsg0("speedy_sine needs an init freq. Thanks."); return None;}
        else {base = base0.unwrap();}
        match precalc.len() {
            0 => {
                for s in 0..sample_rate{
                    precalc.push ( 2.0 * PI * base * s as f32 / sample_rate as f32);
                }
            },
            _ => {}
        }
        precalc_len = precalc.len();
    }
    let mut get_precalc = |s: usize| -> f32 {unsafe {return precalc[s] } };
    let mut new_sine: Vec <f32> = Vec::new();
    let mut coef_to_scale = (base / out_freq);
    if 1.0 - (coef_to_scale - coef_to_scale.floor() ) > 0.5 {coef_to_scale = coef_to_scale.ceil(); } else {coef_to_scale = coef_to_scale.floor(); }
    let fill_num = coef_to_scale - 2.0;
    for s in 1..precalc_len {
        let mut from = get_precalc (s - 1);
        new_sine.push ( from );
        let step = (get_precalc (s) - get_precalc (s - 1) ) / fill_num;
        for f in 0..fill_num as usize{
            from += step;
            new_sine.push ( from );
        }

    }
    mem_sample_rate(sample_rate);
    mem_init_freq(init_freq.unwrap() );
Some (new_sine)
}
pub fn init_speedy_sine_1hz ( sample_rate: u32 ) 
    -> Option <Vec<f32> > {
    static mut precalc: Lazy< Vec<f32> > = Lazy::new(|| {Vec::new()});
    let mut precalc_len =0usize;
    unsafe {
        match precalc.len() {
            0 => {
                for s in 0..sample_rate{
                    precalc.push ( (2.0 * PI * s as f32 / sample_rate as f32).sin() );
                }
            },
            _ => {}
        }
    }
    mem_sample_rate(sample_rate);
unsafe { Some (precalc.clone() ) }
}
pub fn init_speedy_cos_1hz ( sample_rate: u32 ) 
    -> Option <Vec<f32> > {
    static mut precalc: Lazy< Vec<f32> > = Lazy::new(|| {Vec::new()});
    let mut precalc_len =0usize;
    unsafe {
        match precalc.len() {
            0 => {
                for s in 0..sample_rate{
                    precalc.push ( (2.0 * PI * s as f32 / sample_rate as f32).cos() );
                }
            },
            _ => {}
        }
    }
   mem_sample_rate(sample_rate);
unsafe { Some (precalc.clone() ) }
}

pub fn mem_sample_rate (set_sample_rate: u32 ) -> u32 {
    static mut sample_rate: u32 = 0;
    unsafe {
        if set_sample_rate > 0 {sample_rate = set_sample_rate;} sample_rate
    }
} 
pub fn mem_init_freq (set_init_freq: f32 ) -> f32 {
    static mut init_freq: f32 = 0.0;
    unsafe {
        if init_freq > 0.0 {init_freq = set_init_freq;} init_freq
    }
} 
pub fn mock_sine ( out_freq: f32, time: usize) -> f32 {
    static mut sine_approx: Vec <f32> = Vec::new();
    let sample_rate = mem_sample_rate(0);
    if sample_rate == 0 {errMsg0("Dear User, Please, set sample rate"); return 0.0 }
    unsafe {
        if let Some (approx) = init_speedy_sine_1hz(sample_rate ) {sine_approx = approx;}
        else {errMsg0( "Dear User, no init freq has been set."); return 0.0;}
    };
    let mut coef_to_scale = out_freq;
    if 1.0 - (coef_to_scale - coef_to_scale.floor() ) > 0.5 {coef_to_scale = coef_to_scale.ceil(); } else {coef_to_scale = coef_to_scale.floor(); }
    let mut csin = |s: usize| -> f32 {unsafe {return sine_approx[s] } };
    let mut sample_id = time * coef_to_scale as usize;
    csin ((sample_id as usize) % sample_rate as usize)
}
pub fn phi_sine ( out_freq: f32, time: usize, phi: f32) -> f32 {
    static mut sine_approx: Vec <f32> = Vec::new();
    let sample_rate = mem_sample_rate(0);
    if sample_rate == 0 {errMsg0("Dear User, Please, set sample rate"); return 0.0 }
    unsafe {
        if let Some (approx) = init_speedy_sine_1hz(sample_rate ) {sine_approx = approx;}
        else {errMsg0( "Dear User, no init freq has been set."); return 0.0;}
    };
    let mut coef_to_scale = out_freq;
    if 1.0 - (coef_to_scale - coef_to_scale.floor() ) > 0.5 {coef_to_scale = coef_to_scale.ceil(); } else {coef_to_scale = coef_to_scale.floor(); }
    let mut csin = |s: usize| -> f32 {unsafe {return sine_approx[s] } };
    let phi_to_sample_num = (phi / (2.0 * PI)).round() as usize * sample_rate as usize;
    let mut sample_id = time * coef_to_scale as usize + phi_to_sample_num;
    csin ((sample_id as usize) % sample_rate as usize)
}
pub fn table_cos ( out_freq: f32, time: usize) -> f32 {
    static mut cos_approx: Vec <f32> = Vec::new();
    let sample_rate = mem_sample_rate(0);
    if sample_rate == 0 {errMsg0("Dear User, Please, set sample rate"); return 0.0 }
    /*let init_freq = if mem_init_freq(0.0 ) <= 0.0 {errMsg0("Dear User, init freq must be set."); return 0.0;} else {
        mem_init_freq(0.0)
    };*/ 
    unsafe {
        if let Some (approx) = init_speedy_cos_1hz(sample_rate ) {cos_approx = approx;}
        else {errMsg0( "Dear User, no init freq has been set."); return 0.0;}
    };
    let mut coef_to_scale = out_freq;
    if 1.0 - (coef_to_scale - coef_to_scale.floor() ) > 0.5 {coef_to_scale = coef_to_scale.ceil(); } else {coef_to_scale = coef_to_scale.floor(); }
    let mut ccos = |s: usize| -> f32 {unsafe {return cos_approx[s] } };
    let mut sample_id = time * coef_to_scale as usize;
    ccos ((sample_id as usize) % sample_rate as usize)
} 
pub fn alt_e2jx ( out_freq: f32, time: usize) -> Complex<f32> {
    Complex::new( table_cos(out_freq, time), mock_sine(out_freq, time) )
}
//fn
// https://www.ece.virginia.edu/~ffh8x/moi/compression.html
//https://alg0z.blogspot.com/2024/12/very-flaw-of-fft.html