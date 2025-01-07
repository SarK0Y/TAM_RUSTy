use num::complex::Complex;
use num::Float;
use num::Zero;
use num_traits::ops::overflowing::OverflowingAdd;
use num_traits::ops::overflowing::OverflowingSub;
use std::f32::consts::PI;
use std::f32::consts::E;
use once_cell::sync::Lazy;
use num::complex::ComplexFloat;
use crate::custom_dft;
use crate::enums::freq_range;
use crate::errMsg0;
use crate::ret0;
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
    //    println!("run func simple_n_fast_dft", );
    let mut cdft = crate::enums::custom_dft {
        amplitude: Vec::<f32>::new(),
        freq: Vec::<f32>::new(),
        phase: Vec::<f32>::new(),
    };
    let mut z_sample: Complex<f32> = Complex::new (0.0, 0.0);
    //let mut unit: usize = if samples.len () == to {1} else {0};
    let unit: usize = 1_usize.overflowing_shr( (samples.len() ^ to) as u32).0;
    let norm_to = to - from - unit;
    let j: Complex<f32> = Complex::new (0.0, 1.0);
  //  dbg!(&spectre);
    for freq0 in 0..1_000_000_000 {
        let freq = (spectre.from + spectre.step * freq0 as f32);
        if freq > spectre.to {break;}
        //dbg! (&freq);
        for t in 0..norm_to {
            let coef = 1.0 / alt_e2jx(freq, t);      
            z_sample += samples[from + t] * coef;
         /*   dbg! (&coef);
            dbg!(&samples[from + t]);
            dbg!(from + t);*/
        }
        cdft.amplitude.push ((z_sample.re.powi(2) + z_sample.im.powi (2) ).sqrt() );
        if z_sample.re ==0.0 {cdft.phase.push (0.0) } else { cdft.phase.push ((z_sample.im / z_sample.re).atan() );}
        cdft.freq.push(freq);
    }
 //   println!("end func simple_n_fast_dft", );
cdft
}
pub fn simple_n_fast_idft (cdft: crate::enums::custom_dft,
    frame_len: usize, // im samples 
    ) -> Vec <f32> {
        //println!("run func simple_n_fast_idft", );
        let mut samples = Vec::<f32>::new();
        //for i in 0..frame_len { samples.push (0.0); }
        for t in 0..frame_len {
            samples.push (0.0);
            for s in 0..cdft.phase.len() {
                samples[t] += phi_sine(cdft.freq[s], t, cdft.phase[s]) * cdft.amplitude[s];
            }
        }
        //println!("end func simple_n_fast_idft", );
        samples
    }
pub fn hybrid_dft_recursion (samples: &mut [f32], 
    from: usize,
    to: usize,
    spectre: &freq_range ) -> crate::enums::custom_dft{
    let mut cdft = crate::enums::custom_dft {
    amplitude: Vec::<f32>::new(),
    freq: Vec::<f32>::new(),
    phase: Vec::<f32>::new(),
    };
    let mut over_cdft: *mut custom_dft = & mut cdft;
    crate::faav::over_cdft( Some (over_cdft ));
    let mut over_samples: *mut [f32] = samples;
    crate::faav::over_samples( Some (over_samples) );
    let spectre_ = spectre.clone();
    let mut dft = crate::thread::spawn ( move || {
        unsafe {
            let over_cdft_ = crate::faav::over_cdft( None).unwrap();
            let over_samples: *mut [f32] = crate::faav::over_samples( None).unwrap();
            *over_cdft_ = dft_recursion(&mut *over_samples, Vec::<*mut f32>::new() , from, to, &spectre_ , 0).1;
        }
      }
    );
    dft.join();
    let mut z_sample: Complex<f32> = Complex::new (0.0, 0.0);
    cdft
}
pub fn dft_recursion (samples: &mut [f32], subset: Vec <*mut f32>, 
    from: usize,
    to: usize,
    spectre: &freq_range, depth: usize ) -> (Complex<f32>, crate::enums::custom_dft){
    let mut cdft = crate::enums::custom_dft {
    amplitude: Vec::<f32>::new(),
    freq: Vec::<f32>::new(),
    phase: Vec::<f32>::new(),
    };
    let mut ret:(Complex<f32>, crate::enums::custom_dft) = (Complex::zero(), cdft.clone() ); 
    if samples.len() == 1 {return ret}
    let mut z_sample: Complex<f32> = Complex::new (0.0, 0.0);
    let mut samples_even: Vec <*mut f32> = Vec::new();
    let mut samples_odd = Vec::<*mut f32>::new();
    if subset.len () == 0{
        dbg!(&depth);
        let unit: usize = 1_usize.overflowing_shr( (samples.len() ^ to) as u32).0;
        let from = from + unit;
        let norm_to = to.overflowing_sub( from ); // possible error
        if to == 0 {return ret}
        /*if norm_to.1 == true {
            dbg!(&from); dbg!(&to);
        }*/
        for t in 0..norm_to.0 /2 {
            let indx = 2 * t;
            let even: *mut f32 = &mut samples [indx ];
            let odd: *mut f32 = &mut samples [indx + 1];
            samples_even.push (even);
            samples_odd.push (odd);
        }
    } else {
        for t in 0..subset.len() /2 {
            let indx = 2 * t;
            let even: *mut f32 = subset [indx ];
            let odd: *mut f32 = subset [indx + 1];
            samples_even.push (even);
            samples_odd.push (odd);
        }
    }
    let z_const = Complex::new(0.83, 0.14);
    let len = samples_odd.len();
    let depth_ = depth + 1;
    let z_odd: Complex<f32> = if len > 1 {dft_recursion( samples, samples_odd, 0, len, spectre, depth_).0} else { z_const };
    let len = samples_even.len();
    let z_even = if len > 1 {dft_recursion( samples, samples_even, 0, len, spectre, depth_ ).0} else {z_const };
    let zero_point: *mut f32 = &mut samples [0];
    let sample_rate = mem_sample_rate( 0 ) as usize;
    let mut z_sample_odd: Complex<f32> = Complex::new (0.0, 0.0);
    let unit: usize = 1_usize.overflowing_shr( (samples.len() ^ to) as u32).0;
    let norm_to = to - from - unit;
    if subset.len() <=1 {return ret;}
    let mut freq: f32 = spectre.from;
    for freq0 in 0..1_000_000_000 {
        if freq > spectre.to {break;}
        let coef1 = 1.0 / alt_e2jx(freq, 1);
      //  dbg! (&freq);
        for t in 0..norm_to / 2 {
            let indx = 2 * t;
            let time_odd = unsafe {subset [indx + 1].offset_from (zero_point ) as usize % sample_rate};    
            let coef_odd = 1.0 / alt_e2jx(freq, time_odd);        
            let time_even = unsafe {subset [indx ].offset_from (zero_point ) as usize % sample_rate};    
            let coef_even = 1.0 / alt_e2jx(freq, time_even);        
            if norm_to < 2 {
                z_sample += unsafe { *subset[indx] * coef_even };
                z_sample_odd += unsafe { *subset [ indx + 1] * coef_odd };
            } else {
                z_sample = z_even + z_odd * coef1 }
         
        }      
        z_sample += z_sample_odd * coef1;
        cdft.amplitude.push ((z_sample.re.powi(2) + z_sample.im.powi (2) ).sqrt() );
        if z_sample.re ==0.0 {cdft.phase.push (0.0) } else { cdft.phase.push ((z_sample.im / z_sample.re).atan() );}
        cdft.freq.push(freq);
        freq += spectre.step;
    }

 //   println!("end func simple_n_fast_dft", );
ret.0 = z_sample;
ret.1 = cdft;
dbg! (&depth);
ret
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
    let phi_to_sample_num = (phi * out_freq / (2.0 * PI ) ).round() as usize * sample_rate as usize;
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
/*
https://www.w3computing.com/articles/how-to-implement-a-fast-fourier-transform-fft-in-cpp/
using namespace std;
using Complex = complex<double>;
using CArray = vector<Complex>;

const double PI = acos(-1);

void fft(CArray &x) {
    const size_t N = x.size();
    if (N <= 1) return;

    // Divide
    CArray even(N / 2);
    CArray odd(N / 2);
    for (size_t i = 0; i < N / 2; ++i) {
        even[i] = x[i * 2];
        odd[i] = x[i * 2 + 1];
    }

    // Conquer
    fft(even);
    fft(odd);

    // Combine
    for (size_t k = 0; k < N / 2; ++k) {
        Complex t = polar(1.0, -2 * PI * k / N) * odd[k];
        x[k] = even[k] + t;
        x[k + N / 2] = even[k] - t;
    }
}
*/