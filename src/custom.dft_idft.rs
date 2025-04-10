use num::complex::Complex;
use num::Float;
use num::Zero;
use num_traits::ops::overflowing::OverflowingAdd;
use num_traits::ops::overflowing::OverflowingSub;
use num_traits::Signed;
use crate::custom_traits::helpful_math_ops;
use std::io::Write;
use crate::STRN;
use std::f32::consts::PI;
use std::f32::consts::E;
use once_cell::sync::Lazy;
use num::complex::ComplexFloat;
use crate::custom_dft;
use crate::enums::freq_range;
use crate::errMsg0;
use crate::ret0;
pub fn get_freq_component_of_sample (sample: f32, time: usize, freq: f32) -> f32 {
    let z_sample = sample * E.powc (-2.0 * Complex::<f32>::i() * PI * freq * time as f32);
    let amplitude = (z_sample.re.powi (2) + z_sample.im.powi (2) ).sqrt() ;
    let phase = (z_sample.re / amplitude).acos();
    phi_sine(freq, time, phase ) * amplitude
}
pub fn exclude_freq_component_from_sample (sample: f32, time: usize, freq: f32) -> f32 {
    let z_sample = sample * E.powc (-2.0 * Complex::<f32>::i() * PI * freq * time as f32);
    let amplitude = (z_sample.re.powi (2) + z_sample.im.powi (2) ).sqrt() ;
    let phase = (z_sample.re / amplitude).acos();
    //sample - phi_sine(freq, time, phase ) * amplitude
    let ret = sample - ( 2.0 * PI * freq * time as f32 + phase ).sin() * amplitude;
    if ret == f32::nan() {return sample} ret
}
pub fn exclude_freqs_component_from_sample (sample: f32, time: usize, freqs: &Vec <f32> ) -> f32 {
    let mut ret = sample;
    for freq in freqs {
        let z_sample = sample * E.powc (-2.0 * Complex::<f32>::i() * PI * freq * time as f32);
        let amplitude = (z_sample.re.powi (2) + z_sample.im.powi (2) ).sqrt() ;
        let phase = (z_sample.re / amplitude).acos();
    ret -= phi_sine(*freq, time, phase ) * amplitude;
    }  ret
}
pub fn replace_freqs_component_from_sample_ (
    sample: &mut f32, time: usize, 
    freqs: &crate::enums::freq_range, new_freqs: &crate::enums::freq_range, scale: f32, bar_amplitude: f32) {
    //if new_freqs.len() != freqs.len() {errMsg0("Dear User, lists of old & new freqs should have the same length. Thanks"); return f32::nan() }
    let mut new_freq: f32 = new_freqs.from;
    let mut freq: f32 = freqs.from;
    let Pi2: f32 = 2.0 * PI;
    let mut tmp = 0.0f32;
    while freq <= freqs.to {
        let z_sample = *sample * E.powc (-2.0 * Complex::<f32>::i() * PI * freq * time as f32);
        let amplitude = (z_sample.re.powi (2) + z_sample.im.powi (2) ).sqrt() ;
        if amplitude == f32::nan() || amplitude == 0.0 {
            new_freq += new_freqs.step;
            freq += freqs.step;
            continue;}
        let phase = (z_sample.re / amplitude).acos();
        tmp = *sample;
        if tmp.abs() < bar_amplitude {
            tmp -=  (Pi2 * freq * time as f32 + phase).sin() * amplitude;//phi_sine(freq, time, phase ) * amplitude;
   // *sample += phi_sine(new_freq, time, phase ) * amplitude;
            tmp +=  (Pi2 * new_freq * time as f32 + phase).sin() * amplitude * scale;
        } else {
            new_freq += new_freqs.step;
            freq += freqs.step; return;
        }
        if tmp.abs() < bar_amplitude {*sample = tmp;}
        else {*sample *= -1.0;}
        new_freq += new_freqs.step;
        freq += freqs.step;
   //if *sample > bar_amplitude {*sample = bar_amplitude };
    }
}
pub fn replace_freqs_component_from_sample (sample: f32, time: usize, freqs: &Vec <f32>, new_freqs: &Vec<f32>) -> f32 {
    if new_freqs.len() != freqs.len() {errMsg0("Dear User, lists of old & new freqs should have the same length. Thanks"); return f32::nan() }
    let mut ret = sample;
    for j in 0..freqs.len() {
        let z_sample = sample * E.powc (-2.0 * Complex::<f32>::i() * PI * freqs[j] * time as f32);
        let amplitude = (z_sample.re.powi (2) + z_sample.im.powi (2) ).sqrt() ;
        let phase = (z_sample.re / amplitude).acos();
    ret -= phi_sine(freqs[j], time, phase ) * amplitude;
    ret += phi_sine(new_freqs[j], time, phase ) * amplitude;
    }  ret
}
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
            let coef =  E.powc (-2.0 * Complex::<f32>::i() * PI * freq * t as f32);//1.0 / alt_e2jx(freq, t);      
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
        let mut samples = Vec::<f32>::with_capacity( frame_len );
        //for i in 0..frame_len { samples.push (0.0); }
     //   dbg! (&cdft);
        for t in 0..frame_len {
            samples.push (0.0);
            for s in 0..cdft.phase.len() {
                let sample = phi_sine(cdft.freq[s], t, cdft.phase[s]) * cdft.amplitude[s];
                let sample_phi = mock_sine(cdft.freq[s], t ) * cdft.amplitude[s];
                samples[t] += sample; 
                let z_sample = samples [t] * E.powc (-2.0 * Complex::<f32>::i() * PI * cdft.freq[s] * t as f32);
                let amplitude = (z_sample.re.powi (2) + z_sample.im.powi (2) ).sqrt() ;
                dbg! (cdft.amplitude[s] );
                dbg! (cdft.freq[s] );
                dbg! (t);
                dbg! (z_sample );
                dbg! (sample );
                dbg! (amplitude );
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
   // println!("{:?}", cdft.clone() );
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
    //dbg! (samples.len() );
    let mut z_sample: Complex<f32> = Complex::new (0.0, 0.0);
    let mut samples_even: Vec <*mut f32> = Vec::new();
    let mut samples_odd = Vec::<*mut f32>::new();
  //  dbg! (&subset);
    if subset.len () == 0{
  //      dbg!(&depth);
        let unit: usize = 1_usize.overflowing_shr( (samples.len() ^ to) as u32).0;
        let from = from + unit;
        let norm_to = to.overflowing_sub( from ); // possible error
        //if to == 0 {return ret}
        /*if norm_to.1 == true {
            dbg!(&from); dbg!(&to);
        }*/
      //  dbg! (&norm_to);
        for t in 0..norm_to.0 /2 {
            let indx = from + 2 * t;
            let even: *mut f32 = &mut samples [indx ];
            let odd: *mut f32 = &mut samples [indx + 1];
            samples_even.push (even);
            samples_odd.push (odd);
          //  unsafe { dbg! (*samples_odd [ t ] ); dbg! (indx ) };

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
   // dbg! (subset.len());
    let z_const = Complex::new(0.83, 0.14);
    let len = samples_odd.len();
    let depth_ = depth + 1;
    let ret_odd = if len > 1 {dft_recursion( samples, samples_odd, 0, len, spectre, depth_)} else { ret.clone()};
    let z_odd: Complex<f32> = ret_odd.0;
    let len = samples_even.len();
    let z_even = if len > 1 {dft_recursion( samples, samples_even, 0, len, spectre, depth_ ).0} else {z_const };
    let zero_point: *mut f32 = &mut samples [0];
    let sample_rate = mem_sample_rate( 0 ) as usize;
    let mut z_sample_odd: Complex<f32> = Complex::new (0.0, 0.0);
    //let unit: usize = 1_usize.overflowing_shr( (samples.len() ^ to) as u32).0;
    let norm_to = to - from; //- unit;
    //dbg! (&norm_to);
   // if subset.len() == 1 {return ret;}
    let mut freq: f32 = spectre.from;
    for freq0 in 0..1_000_000_000 {
        if freq > spectre.to {break;}
        let coef1 = 1.0 / alt_e2jx(freq, 1);
      //  dbg! (&freq);
        for t in 0..norm_to / 2 {
            if norm_to == 2 {
                let indx = 2 * t;
               //unsafe { dbg! (*subset [indx + 1] ); dbg! (subset [indx ] ) };
                let time_odd = unsafe {subset [indx + 1].offset_from (zero_point ) as usize % sample_rate};    
                let coef_odd = E.powc (-2.0 * Complex::<f32>::i() * PI * freq * time_odd as f32);//1.0 / alt_e2jx(freq, time_odd);        
                let time_even = unsafe {subset [indx ].offset_from (zero_point ) as usize % sample_rate};    
                let coef_even = 1.0 / alt_e2jx(freq, time_even);        
                z_sample += unsafe { *subset[indx] * coef_even };
                z_sample_odd += unsafe { *subset [ indx + 1] * coef_odd };
            } else {
                z_sample = z_even + z_odd * coef1 }
         
        }      
        z_sample += z_sample_odd * coef1;
        cdft.amplitude.push ((z_sample.re.powi(2) + z_sample.im.powi (2) ).sqrt() / norm_to as f32 );
        if z_sample.re ==0.0 {cdft.phase.push (0.0) } else { cdft.phase.push ((z_sample.im / z_sample.re).atan() );}
        cdft.freq.push(freq);
        freq += spectre.step;
    }

 //   println!("end func simple_n_fast_dft", );
ret.0 = z_sample;
ret.1 = cdft; //.clone();
//if depth == 0 {dbg! (&cdft); }
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
    //let mut csin = |s: usize| -> f32 {unsafe {return sine_approx[s] } };
    let mut sample_id = time * coef_to_scale as usize;
    crate::C!( sine_approx [(sample_id as usize) % sample_rate as usize ] )
}
pub fn phi_sine ( out_freq: f32, time: usize, phi: f32) -> f32 {
    static mut sine_approx: Vec <f32> = Vec::new();
    let sample_rate = mem_sample_rate(0);
    if sample_rate == 0 {errMsg0("Dear User, Please, set sample rate"); return 0.0 }
    unsafe {
        if let Some (approx) = init_speedy_sine_1hz(sample_rate ) {sine_approx = approx;}
        else {errMsg0( "Dear User, no init freq has been set."); return 0.0;}
    };
    let mut coef_to_scale = out_freq.round () as usize;
    //let mut csin = |s: usize| -> f32 {unsafe {return sine_approx[s] } };
    let phi_to_sample_num = ( (phi * out_freq / (2.0 * PI ) ) * sample_rate as f32 ).round () as usize;
    let mut sample_id = time * coef_to_scale + phi_to_sample_num;
    crate::C!( sine_approx [(sample_id as usize) % sample_rate as usize ] )
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
//    let mut ccos = |s: usize| -> f32 {unsafe {return cos_approx[s] } };
    let mut sample_id = time * coef_to_scale as usize;
    crate::C!( cos_approx [(sample_id as usize) % sample_rate as usize ] )
} 
pub fn alt_e2jx ( out_freq: f32, time: usize) -> Complex<f32> {
    Complex::new( table_cos(out_freq, time), mock_sine(out_freq, time) )
}
pub fn calc_sub_range (from: usize, to: usize, gap_ratio: f32) -> (usize, usize) {
    let width = ((to - from) as f32 * gap_ratio).round() as usize;
    let centre: usize = (to - from) / 2;
    let from_ = from + centre - width / 2;
    let to_ = from + centre + width / 2;
    if to_ - from_ == 0 {return (from, to );}
    (from_, to_ )
}
pub fn tune_wave_energy2_ (samples: &mut [f32], uv: &crate::enums::universum_vox_morph) {
    let mut base = samples [0];
    let mut base1 = samples [ 1 ];
    let energy_dt = uv.step_freq.unwrap_or (0.23);
    let scale = uv.scale.unwrap_or (0.31) * -1.0;
    for j in 1..samples.len() {
        base1 = samples [ j ];
        samples [j] = base - base1 + energy_dt;
        samples [ j ] *= scale;
        base = base1;
    }
}
pub fn smooth_wave_energy (samples: &mut [f32], uv: &crate::enums::universum_vox_morph) {
    if uv.input_u64.is_none() {errMsg0("'input_u64' in Universum Vox must be set"); return}
    if uv.input_f32.is_none() {errMsg0("'input_f32' in Universum Vox must be set"); return}
    let input_u64 = uv.input_u64.clone().unwrap();
    if input_u64.len() < 3 {errMsg0("'input_u64' in Universum Vox sets 'step_width', 'from' & 'to'"); return}
    let input_f32 = uv.input_f32.clone().unwrap();
    if input_f32.len() < 3 {errMsg0("'input_f32' in Universum Vox sets 'logic_zero', 'scale' & 'ceil'"); return}
    let step_width = input_u64 [0] as usize;
    let from = input_u64 [1] as usize;
    let to = if input_u64 [2] == 0 { samples.len() } else {input_u64 [2] as usize };
    let mut j = from + step_width;
    let ceil: f32 = input_f32 [ 2 ];
    let scale = input_f32 [ 1 ];
    let logic_zero = input_f32 [ 0 ];
    let mut coefs: Vec <f32> = uv.coef.clone ().unwrap_or (vec! [1.0, 1.0]);
    let coefs_len: usize = coefs.len();
    let mut tmp = 0_f32;
    if samples [j - step_width ].abs () > ceil {samples [j - step_width ] *= scale }
    while j < to {
        if samples [ j ].abs () > ceil {samples [ j ] *= scale }
        tmp = (samples [j - step_width ] + samples [ j ] ) / 2.0;
         if samples [j - step_width ].abs () > logic_zero {
            for i in j - step_width + 1..j {
                samples [ i ] = tmp * coefs [ i % coefs_len ];
            }
        } j += step_width;
    }
}
pub fn smooth_wave_energy1_ (samples: &mut [f32], uv: &crate::enums::universum_vox_morph) {
    if uv.input_u64.is_none() {errMsg0("'input_u64' in Universum Vox must be set"); return}
    if uv.input_f32.is_none() {errMsg0("'input_f32' in Universum Vox must be set"); return}
    let input_u64 = uv.input_u64.clone().unwrap();
    if input_u64.len() < 3 {errMsg0("'input_u64' in Universum Vox sets 'step_width', 'from' & 'to'"); return}
    let input_f32 = uv.input_f32.clone().unwrap();
    if input_f32.len() < 3 {errMsg0("'input_f32' in Universum Vox sets 'logic_zero', 'scale' & 'ceil'"); return}
    let step_width = input_u64 [0] as usize + 1;
    let from = input_u64 [1] as usize;
    let to = if input_u64 [2] == 0 { samples.len() } else {input_u64 [2] as usize };
    let mut j = from + step_width;
    let ceil: f32 = input_f32 [ 2 ];
    let scale = input_f32 [ 1 ];
    let logic_zero = input_f32 [ 0 ];
    let mut tmp = 0_f32;
    if samples [j - step_width ].abs () > ceil {samples [j - step_width ] *= scale }
    while j < to {
        if samples [ j ].abs () > ceil {samples [ j ] *= scale }
        tmp = (samples [j - step_width ] + samples [ j ] ) / 2.0;
         if samples [j - step_width ].abs () > logic_zero {
            for i in j - step_width + 1..j {
                samples [ i ] = tmp;
            }
        } j += step_width;
    }
}
pub fn wave_energy_norma (samples: &mut [f32], uv: &crate::enums::universum_vox_morph) {
    if uv.input_u64.is_none() {errMsg0("'input_u64' in Universum Vox must be set"); return}
    if uv.input_f32.is_none() {errMsg0("'input_f32' in Universum Vox must be set"); return}
    let input_u64 = uv.input_u64.clone().unwrap();
    if input_u64.len() < 2 {errMsg0("'input_u64' in Universum Vox sets 'from' & 'to'"); return}
    let input_f32 = uv.input_f32.clone().unwrap();
    if input_f32.len() < 2 {errMsg0("'input_f32' in Universum Vox sets 'scale' & 'ceil'"); return}
    let from = input_u64 [0] as usize;
    let to = if input_u64 [1] == 0 { samples.len() } else {input_u64 [2] as usize };
    let scale = input_f32 [ 0 ];
    let ceil: f32 = input_f32 [ 1 ];
    for j in from..to {
        while samples [ j ].abs () > ceil {
            samples [ j ] *= scale;
        }
    }
}
pub fn wave_energy_vox (samples: &mut [f32], uv: &crate::enums::universum_vox_morph) {
    if uv.input_u64.is_none() {errMsg0("'input_u64' in Universum Vox must be set"); return}
    if uv.input_f32.is_none() {errMsg0("'input_f32' in Universum Vox must be set"); return}
    let input_u64 = uv.input_u64.clone().unwrap();
    if input_u64.len() < 2 {errMsg0("'input_u64' in Universum Vox sets 'from' & 'to'"); return}
    let input_f32 = uv.input_f32.clone().unwrap();
    if input_f32.len() < 3 {errMsg0("'input_f32' in Universum Vox sets 'scale' & 'min_top', 'max_bottom'"); return}
    let from = input_u64 [0] as usize + 1;
    let to = if input_u64 [1] == 0 { samples.len() } else {input_u64 [2] as usize };
    let scale = input_f32 [ 0 ];
    let min_top: f32 = input_f32 [ 1 ];
    let max_bottom: f32 = input_f32 [ 2 ];
    let mut new_top = min_top;
    for j in from..to {
        if samples [ j ].abs () > new_top.abs () {
            while samples [ j ].abs () > new_top {
                samples [ j ] *= scale;
            } new_top = samples [j];
        } 
        if samples [ j ].abs () < max_bottom.abs () || samples [j].sign () != new_top.sign () {new_top = min_top;}
    }
}
pub fn tune_wave_energy2 (samples: &mut [f32], uv: &crate::enums::universum_vox_morph) {
    let mut base = samples [0];
    let mut base1 = samples [ 1 ];
    let energy_dt = uv.step_freq.unwrap_or (0.23);
    let scale = uv.scale.unwrap_or (0.31);
    for j in 1..samples.len() {
        base1 = samples [ j ];
        samples [j] = base - base1 + energy_dt;
        samples [ j ] *= scale;
        base = base1;
    }
}
pub fn tune_wave_energy_mix (mut samples: &mut [f32], uv: &crate::enums::universum_vox_morph) {
    use crate::faav::over_uv;
    use crate::faav::over_samples0;
    let mut samples0 = samples.clone();
    let over_uv_: *const crate::enums::universum_vox_morph = uv;
    over_samples0(Some (samples0.as_mut_slice() ) );
    crate::faav::over_uv( Some (over_uv_ ) );
    let mut thr1 = std::thread::spawn (move|| {
        crate::cdsp::tune_wave_energy3 (
           unsafe { &mut *over_samples0(None).unwrap() },
           unsafe { &*over_uv(None).unwrap() } ); 
        });
    crate::cdsp::tune_wave_energy4 (
           samples,
           unsafe { &*over_uv(None).unwrap() } ); 
    thr1.join();
    for i in 0..samples0.len(){
        samples [ i ] -= samples0 [ i ];
    }
}
pub fn tune_wave_energy_mix1 (mut samples: &mut [f32], uv: &crate::enums::universum_vox_morph) {
    let mut samples0 = samples.clone();
    crate::cdsp::tune_wave_energy5 ( &mut samples0, &uv ); 
    for i in 0..samples0.len(){
        samples [ i ] -= samples0 [ i ];
    }
}
pub fn shuffle (mut samples: &mut [f32], uv: &crate::enums::universum_vox_morph) {
    use Mademoiselle_Entropia::true_rnd::__get_true_rnd_u32 as u32__;
    let cursor = u32__() as usize;
    let mut tmp = cursor;
    let mut tmp_sample = 0f32;
    let len = samples.len() - 1;
    for i in 0..samples.len(){
       // tmp &= len; curious side-kick
        //println! ("tst"); dbg! (&tmp);
        tmp %= len; //tmp.rem_euclid( len );
       // dbg! (&tmp);
        //swap_samples(samples, tmp, i, 1);
        tmp_sample = samples [ i ];
        samples [ i ] = samples [ tmp ];
        samples [ tmp ] = tmp_sample;
        //dbg! (&tmp);
        tmp *= cursor;
        tmp += i;
        //dbg! (&len);
        //dbg! (&cursor);
    }
}
pub fn wave_energy_stat (samples: &mut [f32], uv: &crate::enums::universum_vox_morph) {
    if uv.input_u64.is_none() {errMsg0("'input_u64' in Universum Vox must be set"); return}
    if uv.input_f32.is_none() {errMsg0("'input_f32' in Universum Vox must be set"); return}
    let input = uv.input_u64.clone().unwrap();
    if input.len() < 3 {errMsg0("'input_u64' in Universum Vox sets 'window_width', 'from' & 'to'"); return}
    let input_f32 = uv.input_f32.clone().unwrap();
    let window_width = input [0];
    let from = input [1] as usize;
    let to = if input [2] == 0 { samples.len() } else {input [2] as usize };
    let sum = input_f32 [0];
    let mut if_keys: Vec <u8> = Vec::with_capacity (window_width as usize);
    for t in 0..window_width as usize - 1 {
        if_keys.push(1);
    }
    if_keys.push (0);
    let mut fns: Vec <fn (String)> = Vec::new();
    for t in 0..window_width as usize - 1 {
        fns.push(nop);
    }
    fns.push(printIt);
    let mut now_sum = 0.0_f32;
    for j in from..to {
        let sub_j = j % if_keys.len();
        let if_key = if_keys [ sub_j ];
        now_sum *= if_key as f32;
        now_sum += samples [ j ];
        fns [ sub_j ] ( now_sum.to_string() );
    }
}
pub fn wave_energy_stat_fading (samples: &mut [f32], uv: &crate::enums::universum_vox_morph) {
    if uv.input_u64.is_none() {errMsg0("'input_u64' in Universum Vox must be set"); return}
    if uv.input_f32.is_none() {errMsg0("'input_f32' in Universum Vox must be set"); return}
    let input = uv.input_u64.clone().unwrap();
    if input.len() < 3 {errMsg0("'input_u64' in Universum Vox sets 'window_width', 'from' & 'to'"); return}
    let input_f32 = uv.input_f32.clone().unwrap();
    let window_width = input [0];
    let from = input [1] as usize + 1;
    let to = if input [2] == 0 || input [2] as usize > samples.len() { samples.len() } else {input [2] as usize };
    let mut sign: usize = 0;
    let mut sign1: usize = 0;
    let mut count_fading_len = 0_isize;
    let mut prev_fading_len = 0_isize;
    let mut fns: Vec <fn (String)> = Vec::new();
    let mut fn_set_max: Vec <fn (&mut isize, &mut isize)> = Vec::new();
    fn_set_max.push(set_nop);
    fn_set_max.push (set_max);
    fns.push (printIt); // 0
    fns.push (nop); // 1
    for j in from..to {
        sign = (samples [ j ] - samples [j - 1]).is_negative() as usize;
        fns [ sign ] ( format!( "j: {j} fading length: {}", count_fading_len.to_string() ) );
        count_fading_len.inc();
        count_fading_len *= sign as isize;
        sign1 = (count_fading_len - prev_fading_len).is_positive() as usize;
        fn_set_max [sign1] (&mut prev_fading_len, &mut count_fading_len);
    }
    println! ("max fading length: {prev_fading_len}");
}
pub fn wave_energy_stat_gaps (samples: &mut [f32], uv: &crate::enums::universum_vox_morph) {
    if uv.input_u64.is_none() {errMsg0("'input_u64' in Universum Vox must be set"); return}
    if uv.input_f32.is_none() {errMsg0("'input_f32' in Universum Vox must be set"); return}
    let input = uv.input_u64.clone().unwrap();
    if input.len() < 3 {errMsg0("'input_u64' in Universum Vox sets 'window_width', 'from', 'to' & 'gap'"); return}
    let input_f32 = uv.input_f32.clone().unwrap();
    let window_width = input [0];
    let from = input [1] as usize;
    let to = if input [2] as usize > samples.len() { samples.len() } else {input [2] as usize };
    let gap = input [3] as usize;
    let sum = input_f32 [0];
    let mut if_keys: Vec <u8> = Vec::with_capacity (window_width as usize);
    for t in 0..window_width as usize - 1 {
        if_keys.push(1);
    }
    if_keys.push (0);
    let mut fns: Vec <fn (String)> = Vec::new();
    for t in 0..window_width as usize - 1 {
        fns.push(nop);
    }
    fns.push(printIt);
    let mut now_sum = 0.0_f32;
    let mut j = from;
    while j < to {
        let sub_j = j % if_keys.len();
        let if_key = if_keys [ sub_j ];
        now_sum *= if_key as f32;
        now_sum += samples [ j ];
        fns [ sub_j ] ( format!("j: {j}, sum: {}",now_sum.to_string() ) );
        j += gap;
    }
}
pub fn printIt (it: String){
    let path_to_save_log = crate::faav::log_file_printIt(None).unwrap_or ("/tmp/log_printIt".strn() );
    let mut file = match Mademoiselle_Entropia::help_funcs::get_file_append(&format!("{path_to_save_log}")){Ok(f) => f, 
                                                            Err(e) => return println!("Sorry, can't open {path_to_save_log}: {e:?}")};
    file.write_all (it.as_bytes() );
    println!("{}", it);
}
pub fn nop (it: String) {
   /* dbg! ("tst"); */
}
pub fn tune_wave_energy3 (samples: &mut [f32], uv: &crate::enums::universum_vox_morph) {
    let mut K1 = 1.0f32;
    let pi = std::f32::consts::PI;
    let e = std::f32::consts::E;
    let energy_dt = uv.step_freq.unwrap_or (0.23);
    let bar = uv.bar_sample;
    let scale = uv.scale.unwrap_or (0.31);
    for j in 3..samples.len() {
        K1 = samples [ j ] - samples [ j - 3];
        let K = (samples [j - 2 ] + energy_dt) * scale;
        //dbg! (&K);
        //samples [ j ] = pi.powf ( K ) + e.powf ( 2.0 * K );
        let shift = K * samples [j - 1] + K1; 
        if shift.abs() == std::f32::INFINITY || shift.abs() == std::f32::NAN {continue;}
        samples [ j ] =  shift;
        samples [ j ] %= bar;
      // dbg! (&j); dbg! (&samples [ j ]);
    }
}
pub fn tune_wave_energy4 (samples: &mut [f32], uv: &crate::enums::universum_vox_morph) {
    let mut K1 = 1.0f32;
    let pi = std::f32::consts::PI;
    let e = std::f32::consts::E;
    let energy_dt = uv.step_freq.unwrap_or (0.23);
    let bar = uv.bar_sample;
    let scale = uv.scale.unwrap_or (0.31);
    for j in 4..samples.len() {
        K1 = samples [ j ] - samples [ j - 4];
        let K = (samples [j - 2 ] + energy_dt) * samples [ j - 3];
        //dbg! (&K);
        //samples [ j ] = pi.powf ( K ) + e.powf ( 2.0 * K );
        let shift = K * samples [j - 1] + K1; 
        if shift.abs() == std::f32::INFINITY || shift.abs() == std::f32::NAN {continue;}
        samples [ j ] =  shift;
        samples [ j ] %= bar;
      // dbg! (&j); dbg! (&samples [ j ]);
    }
}
pub fn tune_wave_energy5 (samples: &mut [f32], uv: &crate::enums::universum_vox_morph) {
    let mut coefs: Vec <f32> = Vec::new();
    let scale = uv.scale.unwrap_or (0.31);
    let fading_duration = uv.fading_duration as i32;
    let fading: f32 = uv.fading_step;
    if let Some (x) = &uv.coef { coefs = x.clone();}
    else {errMsg0("Needs to set coef in json"); return}
    let len = coefs.len();
    for i in 0..samples.len(){
        samples [ i ] *= coefs [ i % len ] * scale * fading.powi (i as i32 % fading_duration);
    }
}
pub fn tune_wave_energy (samples: &mut [f32], energy_dt: f32) {
    let mut base = samples [0];
    let mut base1 = samples [ 1 ];
    for j in 1..samples.len() {
        base1 = samples [ j ];
        samples [j] = base - base1 + energy_dt;
        base = base1;
    }
}
pub fn tune_wave_energy1 (samples: &mut [f32], energy_dt: f32) {
    let mut base = samples [0];
    let mut base1 = samples [ 1 ];
    for j in 1..samples.len() {
        base1 = samples [ j ];
        samples [j] = (base - base1) * base + energy_dt + base1;
        base = base1;
    }
}
#[inline(never)]
pub fn swap_samples (samples: &mut [f32], i: usize, tmp: usize ) {    
    let tmp_sample = samples [ i ];
    samples [ i ] = samples [ tmp ];
    samples [ tmp ] = tmp_sample;
    dbg! (&tmp);
    dbg! (&i);
}
pub trait Clone_Slice <Rhs = Self> {
    type S;
    fn clone (&mut self) -> Vec <Self::S>;

}
impl Clone_Slice <Self> for &mut [f32] {
    type S = f32;
    fn clone (&mut self) -> Vec <Self::S > {
    let mut ret: Vec <Self::S > = Vec::with_capacity (self.len() ); 
        for item in 0..self.len() {
            ret.push ( self[item].clone () );
        }
        ret
    }
}
pub fn set_max <T: Copy>(old: &mut T, new: &mut T) {
    *old = *new;
}
pub fn set_nop <T>(old: &mut T, new: &mut T) {
}
pub trait Num_Sign <T = Self >{
    type S;
    fn sign (&mut self) -> Self::S;
}
impl Num_Sign <Self> for f32 {
    type S = f32;
    fn sign (&mut self) -> Self::S {
        if self.is_positive() { return 1 as Self::S}
        -1 as Self::S
    }
}
//fn
// Зри в Корень (с) Козьма Прутков ;D
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
------------------
E= 0.5 * ρ * v * ω2 * A2
 
where:
ρ = density of the medium (kg/m³),
v = speed of sound in the medium (m/s),
ω = angular frequency ( ω = 2πf),
A = amplitude of the wave.

I= P / A 
where:
I = intensity (measured in watts per square meter, W/m2),
P = power (energy per unit time, measured in watts, W),
A = area (measured in square meters, m2).
Intensity is also proportional to the square of the amplitude: I∝A2

For a single sound wave, the energy is also proportional to the square of its frequency: E∝f2
where 
f is frequency.

dB=10 * log 10 (I / I0 )
where:
I = intensity of the sound,
I0 = reference intensity (usually the threshold of human hearing, 10E−12 W/m2 ).
A 10 dB increase represents a 10-fold increase in sound intensity, but humans perceive this as roughly a doubling of loudness.
------------
dK = (1/2) * μ * dx * (∂y/∂t)^2
K = (1/2) * μ * ∫[0 to L] (∂y/∂t)^2 dx
dU = (1/2) * T * dx * (∂y/∂x)^2
U = (1/2) * T * ∫[0 to L] (∂y/∂x)^2 dx
Frequencies of standing waves on a string fixed at both ends:
f_n = (n / (2L)) * sqrt(T / μ)
where:
    n = mode number (1, 2, 3, ...),
    L = length of the string,
    T = tension in the string,
    μ = linear mass density.
Speed of waves on the string:
    v = sqrt(T / μ)
Sure! Here are the formulas for the energy of a vibrating string, written in a simple, typable format that you can easily use in text editors or notes:

---

### 1. **Kinetic Energy (K)**
For a small segment of the string:
```
dK = (1/2) * μ * dx * (∂y/∂t)^2
```
Total kinetic energy of the string:
```
K = (1/2) * μ * ∫[0 to L] (∂y/∂t)^2 dx
```

---

### 2. **Potential Energy (U)**
For a small segment of the string:
```
dU = (1/2) * T * dx * (∂y/∂x)^2
```
Total potential energy of the string:
```
U = (1/2) * T * ∫[0 to L] (∂y/∂x)^2 dx
```

---

### 3. **Total Energy (E)**
Total energy of the vibrating string:
```
E = K + U
```
Substituting the expressions for K and U:
```
E = (1/2) * μ * ∫[0 to L] (∂y/∂t)^2 dx + (1/2) * T * ∫[0 to L] (∂y/∂x)^2 dx
```

---

### 4. **Standing Wave Frequencies**
Frequencies of standing waves on a string fixed at both ends:
```
f_n = (n / (2L)) * sqrt(T / μ)
```
where:
- `n` = mode number (1, 2, 3, ...),
- `L` = length of the string,
- `T` = tension in the string,
- `μ` = linear mass density.

---

### 5. **Wave Speed on the String**
Speed of waves on the string:
```
v = sqrt(T / μ)

### 6. **Energy in Terms of Amplitude**
For a sinusoidal wave with amplitude `A` and frequency `f`, the total energy per unit length is approximately:
```
E ≈ (1/2) * μ * (2πfA)^2

### Summary of Variables
- `μ` = linear mass density (mass per unit length, kg/m),
- `T` = tension in the string (newtons, N),
- `L` = length of the string (meters, m),
- `y` = displacement of the string (meters, m),
- `∂y/∂t` = transverse velocity of the string (m/s),
- `∂y/∂x` = slope of the string (dimensionless),
- `f` = frequency of vibration (Hz),
- `A` = amplitude of vibration (meters, m).
. Total Energy of a Vibrating String (Approximation)
The total energy 
𝐸
E of a vibrating string can be approximated as:
E ≈ 2 * π² * μ * f² * A² * L
where:
    μ = linear mass density of the string (mass per unit length, kg/m),
    f = frequency of vibration (Hz),
    A = amplitude of vibration (maximum displacement, meters),
    L = length of the string (meters).

3. Example Calculation
Let’s say you have a guitar string with:

Linear mass density, μ=0.01kg/m,
Frequency, f=440Hz (A4 note),

Amplitude, A=0.001m (1 mm),
Length, L=0.65m.
Plugging into the formula:
E ≈ 2 * π² * (0.01) * (440)² * (0.001)² * (0.65)
E ≈ 2 * 9.87 * 0.01 * 193600 * 0.000001 * 0.65
E ≈ 0.025 Joules
So, the string has roughly 0.025 Joules of energy.

4. Energy Proportionality
The energy of the string is proportional to:

Square of the amplitude (A2): Doubling the amplitude quadruples the energy.

Square of the frequency (f2): Doubling the frequency quadruples the energy.
Length (L): Doubling the length doubles the energy.
Linear mass density (μ): Doubling the mass density doubles the energy.

5. Wave Speed
The wave speed 

v on the string is given by:
v = sqrt(T / μ)
where 
𝑇
T is the tension in the string.

Summary
For a rough approximation of the energy of a vibrating string:

E ≈ 2 * π2 * μ * f2 * A2 * L

*/