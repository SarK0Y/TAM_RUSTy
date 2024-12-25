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
    let mut z_sample = Complex::new (0.0, 0.0);
    let norm_to = to - from;
    let j = Complex::new (0.0, 1.0);
    for freq0 in 0..1_000_000_000 {
        let freq = spectre.from + spectre.step * freq0 as f32;
        if freq > spectre.to {break;}
        for k in 0..norm_to {
            let coef = alt_const.powc (2.0 * PI * freq * k as f32 * j/ norm_to as f32 );
        }
    }
cdft
}
//fn