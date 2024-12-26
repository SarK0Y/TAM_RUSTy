use num::complex::Complex;
use std::f32::consts::PI;
use std::f32::consts::E;
use num::complex::ComplexFloat;
use crate::enums::freq_range;
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
//fn