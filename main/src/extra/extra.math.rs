//use chrono::round;
use rug::float::Round;
use rug::ops::{AddAssignRound, DivAssignRound, MulAssignRound, PowAssign as rugPowAssign, PowAssignRound, SubAssignRound, Pow as rugpow};
use rug::{Assign, Integer as rugint, float::Constant as rugconst, Float as rugfloat, ops::SubFrom};
use num::Float;
use std::f64::consts::E;
#[cfg(feature="meps")]
use min_err_per_step::trig::Trig;
#[cfg(feature="meps")]
use min_err_per_step::nth_root::__2rt;
#[cfg(feature="meps")]
use min_err_per_step::base::{glob_precision, Pi};
use Mademoiselle_Entropia::minio::InterruptMsg;
const PREC: u64 = 1024;
const PREC0: u64 = 6000;
pub fn simple_Pi (step: f64) -> f64 {
    let num_of_step = (1.0 as f64 / step) as usize;
    let mut x: f64 = 0.0;
    let mut ret: f64  = x;
    let mut iteration = x;
    for s in 0..num_of_step {
        x += step;
        iteration = 2.0 * x - x.powi(2);
        iteration = iteration.sqrt() * step;
        ret += iteration;
    } ret * 4.0
}
pub fn simple_Pi_vs_std_Pi (step: String) -> (f64, f64) {
    dbg!(&step);
    let (_, step) = crate::split_once(&step, ":");
    let step = step.parse::<f64>().unwrap_or(0.001);
    let std_Pi = std::f64::consts::PI;
    let tst_Pi = simple_Pi (step);
    crate::krunner (Some (&tst_Pi.to_string()) );
    let msg = format! ("deviation from std Pi {}\ntst Pi {}", &(std_Pi - tst_Pi).to_string(), tst_Pi);
    crate::errMsg0( &msg);
    (tst_Pi, std_Pi - tst_Pi )
}
pub fn tst_Pi (ceil: f64) -> f64 {
    let mut x = 1.0_f64;
    let mut y = x - x;
    let ret: f64 = Gauss_Legendre_Pi(ceil);
    ret
}
pub fn fast_n_simple_Pi (error: f64 ) -> f64 {
    let mut dx: f64 = 2.0.sqrt();
    //let mut x = 0.0f64;
    dx = dx / 2.0;
    let mut y = (1.0 - dx.powi(2) ).sqrt();
    let mut dy = 1.0 - y;
    dx = ( dx.powi(2) + dy.powi(2) ).sqrt();
    let mut num_of_pts = 4u64;
    while dx > error {
        dx = dx / 2.0;
        y = (1.0 - dx.powi(2) ).sqrt();
        dy = 1.0 - y;
        dx = ( dx.powi(2) + dy.powi(2) ).sqrt();
        num_of_pts *= 2;
    } num_of_pts as f64 * dx
}
pub fn fast_n_simple_long_Pi (err: usize ) -> rugfloat {
    let _2 = rugfloat::with_val_64 (PREC0, 2);
    let _1 = rugfloat::with_val_64 (PREC0, 1);
    let err = _1.clone () / _2.clone().pow(err);
    let mut dx: rugfloat = _2.clone();
    dx = dx.sqrt ();
    //let mut x = 0.0f64;
    dx /= 2;
    let mut y: rugfloat = (_1.clone() - dx.clone().pow(2) );
    y = y.sqrt ();
    let mut dy: rugfloat = _1.clone() - y;
    dx = ( dx.clone().pow(2) + dy.clone().pow(2) );
    dx = dx.sqrt ();
    let mut num_of_pts: rugfloat = _1.clone() * 4;
    while dx > err {
        dx = dx / 2;
        y = (_1.clone() - dx.clone().pow (2) );
        y = y.sqrt ();
        dy = _1.clone() - y;
        dx = ( dx.clone().pow (2) + dy.clone().pow (2) );
        dx = dx.sqrt ();
        num_of_pts *= 2;
    } num_of_pts * dx
}

pub fn arc_val (from: f64, to: f64) -> f64 {
    let x = from;
    let From = ((x - 1.0) *(-(x - 2.0).sqrt() * x) + (x - 1.0).asin() ) / 2.0;
    let x = to;
    let To = ((x - 1.0) *(-(x - 2.0).sqrt() * x) + (x - 1.0).asin() ) / 2.0;
    To - From
}
#[cfg(not(feature="meps"))]
pub fn __2rt (x: &rugfloat, err: u64 ) -> rugfloat {
    let _2 = rugfloat::with_val_64 (PREC0, 2);
    let _1 = rugfloat::with_val_64 (PREC0, 1);
    let mut start_x: rugfloat = _1.clone();
    start_x.assign (x >> 3); ///
    let no_less = _1.clone () / _2.clone().pow (err);
    let mut b = _1.clone();
   // dbg! (&start_x);
    let mut step: usize = 0;
    //sin_x = 2 * start_x.clone ();
    while (start_x.clone() - b.clone() ).abs () > no_less {
        b = x.clone () / start_x.clone ();
	start_x = (start_x.clone() + b.clone () ) / 2;
    }
   // dbg! (&start_x);
    return b
}
pub fn nthrt (x: &rugfloat, err: usize ) -> rugfloat {
    let _2 = rugfloat::with_val_64 (PREC0, 2);
    let _1 = rugfloat::with_val_64 (PREC0, 1);
    let mut start_x: rugfloat = _1.clone();
    start_x.assign (x >> 3); ///
    let no_less = _1.clone () / _2.clone().pow (err);
    let mut b = _1.clone();
   // dbg! (&start_x);
    let mut step: usize = 0;
    //sin_x = 2 * start_x.clone ();
    while (start_x.clone() - b.clone() ).abs () > no_less {
        b = x.clone () / start_x.clone ();
	start_x = (start_x.clone() + b.clone () ) / 2;
    }
   // dbg! (&start_x);
    return b
}
pub fn fast_n_simple_sin (x: &rugfloat, err: usize ) -> rugfloat {
    let _2 = rugfloat::with_val_64 (PREC0, 2);
    let _1 = rugfloat::with_val_64 (PREC0, 1);
    let mut start_x: rugfloat = x / _2.pow (err);
    let mut step: usize = 0;
    let mut sin_x: rugfloat = start_x.clone();
    //sin_x = 2 * start_x.clone ();
    let mut cos_x: rugfloat = ( _1.clone () - start_x.clone().pow (2) );
    /*sin_x *= cos_x.sqrt ();
    start_x *= 2; */
    while start_x < *x {
        cos_x = ( _1.clone () - sin_x.clone().pow (2) );
        sin_x *= 2;
        sin_x *= __2rt (&cos_x, PREC0 );//cos_x.sqrt ();
        start_x *= 2;
    }
    dbg! (&start_x);
    return sin_x
}
pub fn fast_n_simple_cos (x: &rugfloat, err: usize ) -> rugfloat {
    let _2 = rugfloat::with_val_64 (2*PREC0, 2);
    let _1 = rugfloat::with_val_64 (2*PREC0, 1);
    let mut start_x: rugfloat = _1.clone();
    start_x.assign (x >> err); /// _2.pow (err);
    dbg! (&start_x);
    let mut step: usize = 0;
    //sin_x = 2 * start_x.clone ();
    let mut cos_x: rugfloat = _1.clone ();
    cos_x = ( _1.clone () - start_x.clone() * start_x.clone() );
    cos_x = cos_x.sqrt ();
    while start_x < *x {
        cos_x = 2 * cos_x.clone () * cos_x - _1.clone ();
        //dbg! (&cos_x); break;
        start_x *= 2;
    }
    dbg! (&start_x);
    return cos_x
}
pub fn fast_n_simple_cos3 (x: &rugfloat, err: usize ) -> rugfloat {
    let _3 = rugfloat::with_val_64 (PREC0, 3);
    let _1 = rugfloat::with_val_64 (PREC0, 1);
    let mut start_x: rugfloat = x / _3.pow (err);
    dbg! (&start_x);
    let mut step: usize = 0;
    //sin_x = 2 * start_x.clone ();
    let mut cos_3x: rugfloat = _1.clone ();
    cos_3x = ( _1.clone () - start_x.clone() * start_x.clone() );
    cos_3x = __2rt (&cos_3x, PREC0);
    dbg! (&cos_3x);
    while start_x < *x {
        cos_3x = 4 * cos_3x.clone ().pow (3) - 3 * cos_3x.clone ();
        start_x *= 3;
    }
    dbg! (&cos_3x);
    return cos_3x
}
pub fn fast_n_simple_sin3 (x: &rugfloat, err: usize ) -> rugfloat {
    let _3 = rugfloat::with_val_64 (PREC0, 3);
    let _1 = rugfloat::with_val_64 (PREC0, 1);
    let mut start_x: rugfloat = x / _3.pow (err);
    dbg! (&start_x);
    let mut step: usize = 0;
    //sin_x = 2 * start_x.clone ();
    let mut sin_3x: rugfloat = _1.clone ();
    sin_3x = start_x.clone();
    dbg! (&sin_3x);
    while start_x < *x {
        sin_3x = 3 * sin_3x.clone () - 4* sin_3x.clone ().pow (3);
        start_x *= 3;
    }
    dbg! (&sin_3x);
    return sin_3x
}
pub fn tst_Pi_ (error: f64) -> f64 { // failed
    let mut x = 1.0_f64;
    let mut y = x - x;
    while (x - y).abs() > error {
        x /= 2.0;
        y = (2.0 * x - x.powi(2) ).sqrt();
    }
    let n = (0.5/ x);
     dbg! ((x - y).abs() );
    y = y / 2.0;
    let ret = 8.0 * n * y;
    dbg! (y);
    ret
}
pub fn tst_Pi_vs_std_Pi (step: String) -> (f64, f64) {
    dbg!(&step);
    let _1 = rugfloat::with_val_64 (PREC0, 1);
    let _05 = rugfloat::with_val_64 (2*PREC0, 0.5);
    let _2 = rugfloat::with_val_64 (PREC0, 2);
    let (_, step) = crate::split_once(&step, ":");
    let error = step.parse::<f64>().unwrap_or(31.0);
    println!("rounds: {error}\n");
    let std_Pi = std::f64::consts::PI;
    let _std_Pi: rugfloat = rugfloat::with_val_64(PREC0, rugconst::Pi);    let control_tst_Pi = tst_Pi (error + 1.0);
    let tst_Pi = tst_Pi (error);
    crate::krunner (Some (&tst_Pi.to_string()) );
    let epi__ = __epi(100);
    let msg = format! ("deviation from std Pi {}\ntst Pi {}\nCos(std): {} \nCos(tst): {}\nstd_Cos(tst): {}\n
    std_Cos(800 000*tst): {}\nCos(800 000*tst): {}\ntricked_Cos(800 000*tst): {}\ntricked_Cos(811 000*tst): {}   ",
    &(std_Pi - tst_Pi).to_string(), tst_Pi, tst_Cos (std_Pi) , tst_Cos(tst_Pi),
    tst_Pi.cos(), (800_000.0*tst_Pi).cos(), tst_Cos(800_000.0*tst_Pi ),
    tricked_Cos(800_000.0*tst_Pi, error ), tricked_Cos(811_000.0*tst_Pi, error ));
    let msg1 = format! ("{}\nfast & Simple Pi(0.5^35): {}\ncontrol_tst(step + 1): {}\nstd_Pi - 2*tst_asin: {}\nstd_Pi - 2*almost_asin: {}
    \nstd_Pi - __epi {}"
    , msg, std_Pi - fast_n_simple_Pi (0.5.powi(35) ), control_tst_Pi, std_Pi - 2.0 * tst_asin(1.0, 11),
     std_Pi - 2.0 * almost_asin(1.0, 75), _std_Pi - epi__.clone()  );
     dbg!( epi() );
     dbg! (&epi__);
     fast_real_e(1.0);
     dbg! (big_exp_Taylor( rugfloat::with_val_64( PREC0, 1.0 ), 100));
     let err: usize = 4000;
     let mut _45deg = fast_n_simple_long_Pi ( err );
     let err_pi = rugfloat::with_val_64 (PREC0, rugconst::Pi) - _45deg.clone ();
     dbg! (&err_pi);
     #[cfg(feature="meps")]
     glob_precision (Some (6000));
     #[cfg(feature="meps")]
    { _45deg = Pi();}
    _45deg /= 4;
     let mut rug_sin3_err = __2rt (&_05, 4300);
     let mut rug_sin_err = rug_sin3_err.clone();//_45deg.clone().sin();
     let mut rug_cos_err = rug_sin3_err.clone();// _45deg.clone().cos();
     //let mut rug_cos_err =  _45deg.clone().cos();
     let mut rug_cos3_err = rug_sin3_err.clone();//_45deg.clone().cos();
     //let sin_45deg = _45deg.sin();
     dbg! ("sqrt(2) for tst");
     __2rt(&_2, 2200);
     dbg! ("end sqrt(2) for tst");
     let mut sin_45deg =fast_n_simple_sin ( &_45deg.clone (), 5000);
     #[cfg(feature="meps")]
     //let mut cos_45deg = _45deg.__cos(4200);//fast_n_simple_cos ( &_45deg, 5100);
     let mut cos_45deg = _45deg.__cos();
     #[cfg(not(feature="meps"))]
     let mut cos_45deg = fast_n_simple_cos ( &_45deg, 5100);
     let mut cos3_45deg =fast_n_simple_cos3 ( &_45deg, 2200);
     let mut sin3_45deg =fast_n_simple_sin3 ( &_45deg, 3750);
     #[cfg(feature="meps")]
     use min_err_per_step::base::ext_const_E;
     #[cfg(feature="meps")]
     {
        let mut cos_extra = _45deg.cos_extra_prec ();
        let cos_extra_vs__sin = _45deg.__sin () / cos_extra.clone ();
        cos_extra /= _45deg.__cos();
        let __cos_vs__sin  = _45deg.__sin () / _45deg.__cos();
        let power = rugfloat::with_val_64 (PREC0, 0.693147181);
        let ext_const_e = ext_const_E (&power);
        dbg! (&cos_extra);
        dbg! (&cos_extra_vs__sin);
        dbg! (&__cos_vs__sin);
        dbg! (&ext_const_e);
     }
     dbg! (&cos_45deg);
     let _2_sqrt = _2.clone().sqrt();
     rug_sin_err /= sin_45deg.clone();
     rug_cos_err /= cos_45deg.clone();
     rug_cos3_err /= cos3_45deg.clone();
     rug_sin3_err /= sin3_45deg.clone();
     let sin2x_vs_sin3x = sin_45deg.clone () / sin3_45deg.clone ();
     dbg! (&rug_sin_err);
     dbg! (&rug_cos_err);
     dbg! (&rug_cos3_err);
     dbg! (&rug_sin3_err);
     sin_45deg *= 2;
     cos_45deg *= 2;
     cos3_45deg *= 2;
     let fast_n_simple_sin_err = sin_45deg.clone() - _2_sqrt.clone ();
     let fast_n_simple_cos_err = cos_45deg.clone() - _2_sqrt.clone ();
     let fast_n_simple_cos3_err = cos3_45deg.clone() - _2_sqrt.clone();
     dbg! (&fast_n_simple_sin_err);
     dbg! (&fast_n_simple_cos_err);
     dbg! (&fast_n_simple_cos3_err);
     dbg! (&sin2x_vs_sin3x);
     let sqrt_err = _2_sqrt / __2rt (&_2, 3000);
     dbg! (&sqrt_err);
    InterruptMsg( &msg1);
    (tst_Pi, std_Pi - tst_Pi )
}
// term git  remote set-url --add origin  https://[token]@github.com/SarK0Y/Mademoiselle_Entropia.git
pub fn Gauss_Legendre_Pi (rounds: f64) -> f64 {
    let mut a = 1.0f64;
    let mut b: f64 =1.0 / 2.0.sqrt();
    let mut t: f64 = 1.0 / 4.0;
    let mut p = 1.0 as f64;
    let mut a_nxt = a;
    let mut b_nxt = a;
    let mut t_nxt = a;
    let mut p_nxt = a;
    for r in 0..rounds as usize {
        a_nxt = (a + b) / 2.0;
        b_nxt = (a * b).sqrt();
        t_nxt = t - p * (a - a_nxt).powi(2);
        p = 2.0 * p;
        if a == a_nxt {dbg! (a_nxt); dbg!(r);}
        if b == b_nxt {dbg! (b_nxt); dbg!(r);}
        if t == t_nxt {dbg! (t_nxt); dbg!(r); break;}
        a = a_nxt;
        b = b_nxt;
        t = t_nxt;
    }
    dbg!(t); dbg!(t_nxt);
(a + b).powi(2) / (4.0 * t)
}
pub fn sigma_ln (from: f64, to: f64) -> f64 {
    let x = from;
    let From = x * (x.ln() - 1.0);
    let x = to;
    let To = x * (x.ln() - 1.0);
    To - From
}
pub fn sigma_log (base: f64, from: f64, to: f64) -> f64 {
    let x = from;
    let From = x * (x.ln() - 1.0);
    let x = to;
    let To = x * (x.ln() - 1.0);
    (To - From) / base.ln()
}
pub fn tst_Cos (x: f64) -> f64 {
// cos (0) -sin(0)(1) - cos(0)(2) + sin(0)(3) + cos(0)(4) - 0(5) - 1(6) +0(7) + 1(8)
    1.0 - (x.powi(2) / 2u64.factorial()) + (x.powi(4) / 4.factorial() ) -  (x.powi(6) / 6.factorial() ) + (x.powi(8) / 8.factorial() )
}
pub fn tricked_Cos (x: f64, rounds_to_calc_pi: f64) -> f64 {
    let pi = Gauss_Legendre_Pi(rounds_to_calc_pi);
    let rotations = x / pi;
    if rotations.floor() == rotations {
        if (rotations as usize) % 2 == 0 {return 1.0}
    }
    let x = x % pi;
// cos (0) -sin(0)(1) - cos(0)(2) + sin(0)(3) + cos(0)(4) - 0(5) - 1(6) +0(7) + 1(8) - 0(9) - 1(10)
    let ret = 1.0 - (x.powi(2) / 2u64.factorial()) + (x.powi(4) / 4.factorial() ) -  (x.powi(6) / 6.factorial() ) + (x.powi(8) / 8.factorial() )
    - (x.powi(10) / 10.factorial() ) + (x.powi(12) / 12.factorial() );
    if rotations.floor() as usize % 2 == 1 {return -1.0 * ret ;}
    ret

}
pub fn almost_asin (x: f64, rounds: usize) -> f64 {
     let mut ret = 1f64;
    for n in 1..rounds{
        let n = n as f64;
        let numerator = (2.0 * n).fuzzy_factorial();
        let denominator:f64 = 4.0.powi(n as i32) * n.fuzzy_factorial().powi (2) * (2.0 * n as f64 + 1.0);
        ret += (numerator as f64 / denominator as f64) * (x.powi (2 * n as i32 + 1) );
    }
    dbg!(ret);
    ret
}
pub fn tst_asin (x: f64, rounds: usize) -> f64 {
     let mut ret = 1f64;
    for n in 1..rounds{
        let numerator = (2 * n).factorial_();
        let denominator:f64 = 4.0.powi(n as i32) * n.factorial_().powi (2) * (2.0 * n as f64 + 1.0);
        ret += (numerator as f64 / denominator as f64) * (x.powi (2 * n as i32 + 1) );
    }
    dbg!(ret);
    ret
}
pub trait fuzzy_Factorial {
    fn fuzzy_factorial (&self) -> f64;
}
impl fuzzy_Factorial for f64 {
    fn fuzzy_factorial (&self) -> f64 {
        let mut ret = 1.0f64;
        let mut x = *self;
        while x > 1.0{
            ret *= x;
            x -= 1.0;
        } ret
    }
}
pub trait Factorial {
    fn factorial (&mut self) -> f64;
    fn factorial_ (&self) -> f64;
}
impl Factorial for u64 {
    fn factorial (&mut self) -> f64 {
        let mut ret = 1u64;
        while *self > 1{
            ret *= *self;
            *self -= 1;
        } ret as f64
    }
    fn factorial_ (&self) -> f64 {
        let mut ret: u64 = 1;
        let mut x = *self;
        while x > 1{
            ret *= x;
            x -= 1;
        } ret as f64
    }
}
impl Factorial for usize {
    fn factorial (&mut self) -> f64 {
        let mut ret = 1usize;
        while *self > 1{
            ret *= *self;
            *self -= 1;
        } ret as f64
    }
   fn factorial_ (&self) -> f64 {
        let mut ret = 1usize;
        let mut x = *self;
        while x > 1{
            ret *= x;
            x -= 1;
        } ret as f64
    }
}
impl Factorial for i32 {
    fn factorial (&mut self) -> f64 {
        let mut ret = 1i32;
        while *self > 1{
            ret *= *self;
            *self -= 1;
        } ret as f64
    }
   fn factorial_ (&self) -> f64 {
        let mut ret = 1i32;
        let mut x = *self;
        while x > 1{
            ret *= x;
            x -= 1;
        } ret as f64
    }
}
pub fn epi () -> f64 {
   let n: f64 =587124671.0;
   let m=768614336.0;
   let ret = E.powf( (m/n).sqrt() );
   ret
}
pub fn __epi (terms: usize) -> rugfloat {
   let PREC_ = PREC as u32;
   let PREC0_ = PREC0 as u32;
   let n  = rugfloat::with_val_64(PREC0,587124671u64);
   let m =rugfloat::with_val_64(PREC0,768614336u64);
   //let coef: Rational = Rational::const_from_unsigneds(m, n);
   let _x =  rugfloat::with_val_64(PREC0,m / n );
   let x =  rugfloat::with_val_64(PREC0, _x.sqrt());
 //  let ret =big_exp_Taylor( rugfloat::with_val(PREC0_, x), terms);
    let ret = __fast_real_e( x.clone() );
  /* let n0: f64 =587124671.0;
   let m0=768614336.0;
   let ret = fast_real_e_orig( m0, n0);*/
   let orig_epi = __fast_real_e (rugfloat::with_val (PREC0_, 1.0 ) ).pow(x);
   dbg!(orig_epi);
   ret
}
fn exp_Taylor(x: f64, terms: usize) -> rugfloat { // todo it
    let mut sum = rugfloat::with_val_64 (PREC0, 1);
    let mut term = sum.clone();
    let mut over_term: *mut rugfloat = &mut term;
    over_rugfloat( Some (over_term ) );
    let mut x =rugfloat::with_val_64 (PREC0, x);
    let mut __x: *mut rugfloat = &mut x;
    over_rugfloat1( Some (__x ) );
    for n in 1..=terms {
     //   let mut over_term =unsafe { &mut *over_bigfloat(None).unwrap() };
       // let over_term1 =unsafe { &mut *over_bigfloat(None).unwrap() };
       term *= x.clone() / rugfloat::with_val_64 (PREC0, n); // Calculate x^n / n!
       sum += term.clone();
    } sum
}
fn big_exp_Taylor_(x: rugfloat, terms: usize) -> rugfloat {
    let mut sum = rugfloat::with_val_64 (PREC0, 1);
    let mut term = sum.clone();
    let mut over_term: *mut rugfloat = &mut term;
    over_rugfloat( Some (over_term ) );
     for n in 1..=terms {
     //   let mut over_term =unsafe { &mut *over_bigfloat(None).unwrap() };
       // let over_term1 =unsafe { &mut *over_bigfloat(None).unwrap() };
       term *= x.clone() / rugfloat::with_val_64 (PREC0, n); // Calculate x^n / n!
       sum += term.clone();
    }
dbg!(&sum);
    sum
}
fn big_exp_Taylor(x: rugfloat, terms: usize) -> rugfloat {
    let PREC_ = PREC as u32;
    let PREC0_ = PREC0 as u32;
    let mut sum = rugfloat::with_val(PREC0_, 1.0); // Start with the first term of the series
    let mut term = sum.clone(); // This will hold each term value
    let mut __factorial = sum.clone();
    let mut xn = sum.clone();
    let rm = Round::Down;
    //let mut over_term: *mut BigFloat = &mut term;
    //over_bigfloat( Some (over_term ) );
    for n in 1..=terms {
     //   let mut over_term =unsafe { &mut *over_bigfloat(None).unwrap() };
       // let over_term1 =unsafe { &mut *over_bigfloat(None).unwrap() };
       xn.mul_assign_round (rugfloat::with_val(PREC0_, x.clone() ), rm);
       __factorial.mul_assign_round (rugfloat::with_val(PREC0_, n.clone() ), rm);
       term = rugfloat::with_val_64 (PREC0, xn.clone() / __factorial.clone() ); // Calculate x^n / n!
       sum.add_assign_round( term.clone(), rm);
    }
dbg!(&sum);
    sum
}

pub fn fast_real_e (exp: f64) -> rugfloat {
    let PREC_ = PREC as u32;
    let PREC0_ = PREC0 as u32;
    let one = rugfloat::with_val(PREC0_, 1.0);
    let mut one_div_by = one.clone();
    let rm = Round::Down;
    let exponent: u32 = PREC0_ / 2;
    let mut const_e_base = rugfloat::with_val(PREC0_, 2.0);
    //const_e_base.pow_assign_round(exponent, rm);
    const_e_base.pow_assign(exponent);
    dbg! (&const_e_base);
    let (new_coef, canceled_coef) = num_n_den_from_float64( exp );
    let mut big_exp = const_e_base.clone ();
    big_exp *= rugfloat::with_val(PREC0_, new_coef);
    const_e_base.mul_assign_round(rugfloat::with_val(PREC0_ , canceled_coef), rm);
    dbg! (&const_e_base);
    one_div_by.div_assign_round(const_e_base, rm);
    const_e_base = one_div_by;
    const_e_base += one;
    dbg! (&big_exp);
    dbg! (&exponent);
    dbg! (&new_coef);
    let mut const_e = const_e_base.clone();
    const_e.pow_assign_round( big_exp, rm );
    dbg! (&const_e_base );
    dbg!(&const_e);
    const_e.clone()
}
pub fn __fast_real_e (exp: rugfloat) -> rugfloat {
    let PREC_ = PREC as u32;
    let PREC0_ = PREC0 as u32;
    let one = rugfloat::with_val(PREC0_, 1.0);
    let mut one_div_by = one.clone();
    let rm = Round::Down;
    let exponent: u32 = PREC0_ / 2;
    let mut const_e_base = rugfloat::with_val(PREC0_, 2.0);
    //const_e_base.pow_assign_round(exponent, rm);
    const_e_base.pow_assign(exponent);
    dbg! (&const_e_base);
    let (new_coef, canceled_coef) = num_n_den_from_rugfloat( exp );
    let mut big_exp = const_e_base.clone ();
    big_exp *= rugfloat::with_val_64(PREC0, &new_coef);
    const_e_base.mul_assign_round(rugfloat::with_val(PREC0_ , &canceled_coef), rm);
    dbg! (&const_e_base);
    one_div_by.div_assign_round(&const_e_base, rm);
    dbg! (&const_e_base);
    const_e_base = one_div_by;
    const_e_base += one.clone();
    dbg! (&big_exp);
    dbg! (&canceled_coef);
    dbg! (&new_coef);
    let mut const_e = const_e_base.clone();
    const_e.pow_assign_round( big_exp, rm );
    dbg! (&const_e_base );
    dbg!(&const_e);
    if const_e == one {
        std::thread::spawn (move || {
            let prec_ = (PREC0_ as f64 + PREC0_  as f64 * 0.1) as u64;
            crate::faav::real_e (Some( re_fast_real_e(new_coef, canceled_coef, prec_) ), prec_ );
        }).join();
    }
    let saved_real_e = crate::faav::real_e(None, 0);
    if saved_real_e > rugfloat::with_val(3, 0.0) {return saved_real_e; }
    const_e.clone()
}
pub fn re_fast_real_e (new_coef: rugfloat, canceled_coef: rugfloat, prec_: u64) -> rugfloat {
    let PREC_ = prec_;
    let PREC0_ = prec_;
    let one = rugfloat::with_val_64(PREC0_, 1.0);
    let mut one_div_by = one.clone();
    let rm = Round::Down;
    let exponent: u64 = PREC0_ / 2;
    let mut const_e_base = rugfloat::with_val_64(PREC0_, 2.0);
    //const_e_base.pow_assign_round(exponent, rm);
    const_e_base.pow_assign(exponent); // = 2 ^ m
    dbg! (&const_e_base);
    let mut big_exp = const_e_base.clone (); // = 2 ^ m
    big_exp *= rugfloat::with_val_64(PREC0_, &new_coef); // = new_coef * 2 ^ m
    const_e_base.mul_assign_round(rugfloat::with_val_64(PREC0_ , &canceled_coef), rm);
    dbg! (&const_e_base);
    one_div_by.div_assign_round(&const_e_base, rm);
    dbg! (&const_e_base);
    const_e_base = one_div_by;
    const_e_base += one.clone();
    dbg! (&big_exp);
    dbg! (&canceled_coef);
    dbg! (&new_coef);
    let mut const_e = const_e_base.clone();
    const_e.pow_assign_round( big_exp, rm );
    dbg! (&const_e_base );
    dbg!(&const_e);
    if const_e == one {
        let prec_ = (PREC0_ as f64 + PREC0_  as f64 * 0.1) as u64;
            return re_fast_real_e(new_coef, canceled_coef, prec_);
    }
    const_e.clone()
}
pub fn fast_real_e_orig (num: f64, den: f64) -> rugfloat {
    let PREC_ = PREC as u32;
    let PREC0_ = PREC0 as u32;
    let one = rugfloat::with_val(PREC0_, 1.0);
    let mut one_div_by = one.clone();
    let rm = Round::Down;
    let exponent: u32 = PREC0_ / 2;
    let mut const_e_base = rugfloat::with_val(PREC0_, 2.0);
    //const_e_base.pow_assign_round(exponent, rm);
    const_e_base.pow_assign(exponent);
    dbg! (&const_e_base);
    let new_coef = num;
    let canceled_coef = den;
    let mut big_exp = const_e_base.clone ();
    big_exp *= rugfloat::with_val(PREC0_, new_coef);
    const_e_base.mul_assign_round(rugfloat::with_val(PREC0_ , canceled_coef), rm);
    dbg! (&const_e_base);
    one_div_by.div_assign_round(const_e_base, rm);
    const_e_base = one_div_by;
    const_e_base += one;
    dbg! (&big_exp);
    dbg! (&exponent);
    dbg! (&new_coef);
    let mut const_e = const_e_base.clone();
    const_e.pow_assign_round( big_exp, rm );
    dbg! (&const_e_base );
    dbg!(&const_e);
    const_e.clone()
}

pub fn num_n_den_from_float64 (x: f64) -> (i64, i64) {
    let mut floor = x.floor();
    let mut mantissa = x - floor;
    if mantissa == 0.0 { return (x as i64, 1)}
    let mut den = 10.0f64;
    let mut num = den;
    let mut epsilon = 2.0f64;
    epsilon = epsilon.powi (40);
    while (mantissa - num / (den - 1.0 ) ).abs() > epsilon {
        num = (den - 1.0 ) * mantissa;
        den *= 10.0;
    }
    den -= 1.0;
    num += floor * den;
    (num as i64, den as i64)
}
pub fn num_n_den_from_rugfloat (x: rugfloat) -> (rugfloat, rugfloat) {
    let mut floor = rugfloat::with_val_64(PREC0, x.to_integer().unwrap_or(rugint::new()) );
    dbg!(&floor);
    let mut mantissa = rugfloat::with_val_64(PREC0, &x - &floor );
    dbg! (&mantissa);
    let mut den = rugfloat::with_val_64(PREC0, 10u64);
    let mut num = rugfloat::with_val_64(PREC0, 1u64);
    let one = rugfloat::with_val_64(PREC0, 1u64);
    let ten = rugfloat::with_val_64(PREC0, 10u64);
    let mut err = rugfloat::with_val_64(PREC0, 0.5);
    let mut mid_res = rugfloat::with_val_64(PREC0, 1.0);
    err.pow_assign(PREC0 / 2u64);
    let mut cnt = 200;
    den.pow_assign( cnt );
    //while mid_res.clone( ) != err.clone() {
   // for _ in 0..=cnt {
        //mid_res = mantissa.clone() - num.clone() / (den.clone() - one.clone() );
        num = (den.clone() - one.clone() ) * mantissa.clone();
        num = rugfloat::with_val_64(PREC0, num.to_integer().unwrap_or (rugint::new() ) );
       // den *= ten.clone();
    //}
    den.sub_assign_round(1.0, Round::Down);
    dbg! (&den);
    let num0 = rugfloat::with_val_64(PREC0,  (&den - &one) );
    num += rugfloat::with_val_64(PREC0,  &floor * &num0 );
    (num, den)
}
use once_cell::sync::Lazy;

use crate::errMsg0;
pub fn sum_exp_Taylor (set: Option <(*mut rugfloat, *mut rugfloat) >){
    static mut sum: Lazy < *mut rugfloat > = Lazy::new (|| {&mut rugfloat::with_val_64(PREC0, 1u64) });
    static mut term: Lazy < *mut rugfloat > = Lazy::new (|| {&mut rugfloat::with_val_64(PREC0, 1u64) });
    unsafe {
        if set.is_some() {
            *sum = set.unwrap().0;
            *term = set.unwrap().1;
            return;
         }
       //  BigFloat::
         //sum.as_mut().expect("extra.math 264").add_prec_assign( *term.as_mut().expect("extra.math 264"), PREC);
    }
}
pub fn over_rugfloat (pointer: Option <*mut rugfloat > ) -> Option <*mut rugfloat > {
    static mut state: Lazy < Option <*mut rugfloat > > = Lazy::new (|| {None});
    unsafe {
        if pointer.is_some() { *state = pointer} state.clone()
    }
}
pub fn over_rugfloat1 (pointer: Option <*mut rugfloat > ) -> Option <*mut rugfloat > {
    static mut state: Lazy < Option <*mut rugfloat > > = Lazy::new (|| {None});
    unsafe {
        if pointer.is_some() { *state = pointer} state.clone()
    }
}
pub fn base_num_sys (num: u32, rdx: u32) -> Vec <u32> {
    let mut residue = 1u32;
    let mut conv: Vec <u32> = Vec::new();
    let mut rdx = rdx;
    let mut num = num;
    while num > 0 {
        residue = num % rdx;
        conv.push ( residue );
        num -= residue;
        num /= rdx;
    } return conv
}
//fn
// 9999999999999999999999999999999
// https://math.stackexchange.com/questions/197874/maclaurin-expansion-of-arcsin-x
//https://gitlab.com/tspiteri/gmp-mpfr-sys/-/blob/master/build.rs?ref_type=heads
/*
e^(x^2) = sum_{n=0}^∞ (x^(2n)) / (n!)
∫ e^(x^2) dx = sum_{n=0}^∞ (x^(2n+1)) / ((2n+1) * n!) + C
 use rug::{Assign, Integer};
let mut buffer = Integer::new();
// ... buffer can be used and reused ...
let (a, b) = (Integer::from(10), Integer::from(20));
let incomplete = &a - &b;
buffer.assign(incomplete);
assert_eq!(buffer, -10);
-------
use rug::float::Constant;
use rug::Float;
// x has a precision of 10 bits
let x = Float::with_val(10, 180);
// y has a precision of 50 bits
let y = Float::with_val(50, Constant::Pi);
let incomplete = &x / &y;
// z has a precision of 45 bits
let z = Float::with_val(45, incomplete);
assert!(57.295 < z && z < 57.296);
------
use rug::ops::SubFrom;
use rug::Integer;
let mut rhs = Integer::from(10);
// set rhs = 100 - rhs
rhs.sub_from(100);
assert_eq!(rhs, 90);
-----
There are two main reasons why operations like &a - &b do not perform a complete computation and return a Rug type:

Sometimes we need to assign the result to an object that already exists. Since Rug types require memory allocations, this can help reduce the number of allocations. (While the allocations might not affect performance noticeably for computationally intensive functions, they can have a much more significant effect on faster functions like addition.)
For the Float and Complex number types, we need to know the precision when we create a value, and the operation itself does not convey information about what precision is desired for the result.
There are two things that can be done with incomplete-computation values:

Assign them to an existing object without unnecessary allocations. This is usually achieved using the Assign trait or a similar method, for example int.assign(incomplete) and float.assign_round(incomplete, Round::Up).
Convert them to the final value using the Complete trait, the From trait or a similar method. For example incomplete integers can be completed using incomplete.complete() or Integer::from(incomplete). Incomplete floating-point numbers can be completed using incomplete.complete(53) or Float::with_val(53, incomplete) since the precision has to be specified.
 let mut options = ToSciOptions::default();
    options.set_precision(30);
    println!("{}", Rational::from(3).pow(-1_000_000i64).to_sci_with_options(options));
use malachite_q::Rational;

let e = Rational::from_sci_string("2.718281828459045235360287471352662497757247093699959574966967627724076630353547594571382178525166427").unwrap();
-----------
from decimal import Decimal, getcontext

def gauss_legendre_pi(precision):
    # Set the precision (number of decimal places)
    getcontext().prec = precision + 2  # Add extra digits to avoid rounding errors

    # Initial values
    a = Decimal(1)
    b = Decimal(1) / Decimal(2).sqrt()
    t = Decimal(1) / Decimal(4)
    p = Decimal(1)

    # Iterate until convergence
    for _ in range(precision):
        a_next = (a + b) / 2
        b_next = (a * b).sqrt()
        t_next = t - p * (a - a_next) ** 2
        p_next = 2 * p

        # Update values
        a, b, t, p = a_next, b_next, t_next, p_next

    # Calculate π
    pi_estimate = (a + b) ** 2 / (4 * t)
    return pi_estimate

# Example usage
precision = 100  # Number of decimal places
pi_estimate = gauss_legendre_pi(precision)
print(f"Estimated value of π to {precision} decimal places:\n{pi_estimate}")
----------------
import numpy as np
import matplotlib.pyplot as plt

# Define the interval
x = np.linspace(1, np.e, 100)

# Logarithm function
y_log = np.log(x)

# Quarter-circle approximation
h, k = 1, -1.97625  # Center of the circle
r = 2.97625         # Radius of the circle
y_circle = k + np.sqrt(r**2 - (x - h)**2)

# Plot
plt.plot(x, y_log, label="log(x)")
plt.plot(x, y_circle, label="Quarter-circle approximation")
plt.xlabel("x")
plt.ylabel("y")
plt.legend()
plt.title("Approximating log(x) as a Quarter-Circle")
plt.grid()
plt.show()
--------------
import math
def arcsin_taylor(x, terms=10):
    """Compute arcsin(x) using a Taylor series expansion."""
    result = 0
    for n in range(terms):
        numerator = math.factorial(2 * n)
        denominator = (4**n) * (math.factorial(n)**2) * (2 * n + 1)
        result += (numerator / denominator) * (x ** (2 * n + 1))
    return result

# Example usage
x = 0.5
approx = arcsin_taylor(x, terms=10)
exact = math.asin(x)
print(f"Approximation: {approx}")
print(f"Exact value:   {exact}")
---------
import math

def double_factorial(n):
    """Compute the double factorial of n."""
    if n <= 0:
        return 1
    return n * double_factorial(n - 2)

def arcsin_double_factorial(x, terms=10):
    """Compute arcsin(x) using the double factorial series."""
    result = 0
    for n in range(terms):
        numerator = double_factorial(2 * n - 1)
        denominator = double_factorial(2 * n) * (2 * n + 1)
        result += (numerator / denominator) * (x ** (2 * n + 1))
    return result

# Example usage
x = 0.5
approx = arcsin_double_factorial(x, terms=10)
exact = math.asin(x)
print(f"Approximation: {approx}")
print(f"Exact value:   {exact}")
 */
