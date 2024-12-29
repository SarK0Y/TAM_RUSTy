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
pub fn speedy_sine (init_freq: Option <f32>, sample_rate: u32, out_freq: f32, value: f32) 
    -> (Option <f32>, Option <Vec<f32> > ) {
    static mut precalc: Lazy< Vec<f32> > = Lazy::new(|| {Vec::new()});
    static mut base0: Option <f32> = None;
    let mut base = 0.0f32;
    let mut precalc_len =0usize;
    unsafe {
        if let Some (x) = init_freq {base0 = Some (x); }
        if base0.is_none() {errMsg0("speedy_sine needs an init freq. Thanks."); return (None, None);}
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
    new_sine.push ( get_precalc (0));
    for s in 1..precalc_len {
        let step = get_precalc (s) - get_precalc (s - 1);

    }
    (None, None)
} 
//fn
// https://www.ece.virginia.edu/~ffh8x/moi/compression.html
//https://alg0z.blogspot.com/2024/12/very-flaw-of-fft.html