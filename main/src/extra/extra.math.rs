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
use min_err_per_step::frax::fast_n_dumb_shortcut_for_cfrac;
#[cfg(feature="meps")]
use min_err_per_step::base::{glob_precision, Pi, _ext_const_E};
#[cfg(feature="meps")]
use min_err_per_step::logarithm::lg;
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
#[cfg(feature="meps")]
use min_err_per_step::nth_root::faav_a;
#[cfg(feature="meps")]
use min_err_per_step::nth_root::faav_b;
#[cfg(feature="meps")]
pub fn try_2rt (x: &rugfloat, err: u64 ) -> rugfloat {
 unsafe {
    let PREC0_ = glob_precision (None);
    let _05 = rugfloat::with_val_64 (PREC0_, 0.5);
    let _1 = rugfloat::with_val_64 (PREC0_, 1);
    let mut start_x: rugfloat = _1.clone();
    faav_a (Some (&mut start_x));
    let tmp: rugfloat = x.clone() / 3;
    let no_less = _05.clone().pow (err - 10);
    faav_a(None).unwrap().as_mut().unwrap().assign (tmp);
    *faav_a(None).unwrap() += (*faav_a(None).unwrap()).clone() >> 2;
    let mut b = _1.clone();
    faav_b (Some (&mut b));
    let mut step: u64 = 101;
    let mut delta: rugfloat = ( (*faav_a(None).unwrap()).clone() - (*faav_b(None).unwrap()).clone() ).abs();
    let mut new_low_delta = _1.clone();
    let mut less_possible = true;
    while new_low_delta > no_less && less_possible {
        *faav_b(None).unwrap() = x.clone () / (*faav_a(None).unwrap()).clone();
	    start_x = (start_x.clone() + b.clone () ) / 2;
        delta = ( (*faav_a(None).unwrap()).clone() - (*faav_b(None).unwrap()).clone() ).abs();
        if delta < new_low_delta {new_low_delta = delta;}
        else { 
          //  dbg! (&delta);
            if step == 0 {
                less_possible = false; 
                step = 101;
                continue;
            } step.dec();
        }
        //step.inc();
    }
 //   dbg! (&b);
    if b < start_x && *x > 0 {return start_x }
    return b
}
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
    #[cfg(not(feature="meps"))]
    let _PREC0 = PREC0;
    #[cfg(feature="meps")]
    let _PREC0 = glob_precision (None);
    let _2 = rugfloat::with_val_64 (_PREC0, 2);
    let _1 = rugfloat::with_val_64 (_PREC0, 1);
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
        sin_x *= __2rt (&cos_x, _PREC0 );//cos_x.sqrt ();
        start_x *= 2;
    }
    dbg! (&start_x);
    return sin_x
}
#[cfg(feature="meps")]
pub fn fast_n_simple_isin (x: &Cu_Complex, err: u64 ) -> Cu_Complex {
use min_err_per_step::complex::traits::Cu_Complex_Pow;
use min_err_per_step::complex::nth_root::isqrt;
//dbg! (&x);
    let _PREC0 = glob_precision (None);
    let _2 = Cu_Complex::init_f64 (2.0, 0.0);;
    let _1 = Cu_Complex::init_f64 (1.0, 0.0);
    let mut start_x: Cu_Complex = x.clone() / _2.pow_u64 (err);
    let mut step: usize = 0;
    let mut sin_x: Cu_Complex = start_x.clone();
    //sin_x = 2 * start_x.clone ();
    let mut cos_x: Cu_Complex = ( _1.clone () - start_x.pow_u64 (2) );
    if start_x.1 < 0 {start_x *= -1;}
    let x_abs = x.abs();
 //   dbg! (&start_x);
    /*sin_x *= cos_x.sqrt ();
    start_x *= 2; */
    while start_x.cmp_jless (&x_abs) {
        cos_x = ( _1.clone () - sin_x.clone().pow_u64 (2) );
        sin_x *= 2;
        sin_x *= isqrt (&cos_x ).unwrap().root0;//cos_x.sqrt ();
        start_x *= 2;
       // dbg!(&sin_x);
    }
  //  dbg! (&start_x);
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
#[cfg(feature = "meps")]
pub fn dbg_gen_backdoor_numero (n_minus_1: &rugfloat, tail: &rugfloat) -> rugfloat {
use IF::banu::faav_102m_m_1;
use IF::banu::faav_102shift;
use min_err_per_step::editing::Full_Prnt_Rugfloat;
    let _102shift = faav_102shift ().to_integer ().unwrap ();
    let _102m_m_1 = faav_102m_m_1 (None).to_integer ().unwrap ();
    dbg! (faav_102m_m_1(None));
    dbg! (&_102m_m_1);
    dbg! (&_102shift);
    let nm1: rugint = n_minus_1.to_integer ().unwrap ();
    let mut cut_tail: rugint = tail.to_integer ().unwrap ();;
    //cut_tail.dbg_prnt(file!(), line!() );
    dbg! (&cut_tail);
    let mut new_n: rugint = nm1 * (_102m_m_1.clone() + 1 );
    //new_n.dbg_prnt(file!(), line!() );
    dbg! (&new_n);
    new_n += _102m_m_1;
    dbg! (&new_n);
    new_n *= _102shift.clone();
    dbg! (&new_n);
    new_n += cut_tail; 
   // dbg! (new_n.to_string_radix(10, Some (glob_precision (None) as usize ) ) );
   dbg! (&new_n);
    return rugfloat::with_val_64 (
        glob_precision (None),
        new_n
    )
}
#[cfg(feature = "meps")]
pub fn dbg_faav_102m_m_1 (m: Option <usize> ) -> rugfloat {
use min_err_per_step::editing::Conv_Strn_2_Rugint;
    static mut init: Lazy < rugfloat > = Lazy::new ( || {
        rugfloat::with_val_64 (
            1,
            1
        ) 
    });
    static _1: Lazy < rugint > = Lazy::new ( || {
        "1".int (10)
    });
    unsafe {
        if let Some ( _m ) = m { 
            let mut _10: rugint = _1.clone() * 10;
            _10 = _10.clone().pow (_m as u32);
            _10 -= 1;
            dbg! (&_10);
            *init =  rugfloat::with_val_64 (
                glob_precision (None),
                _10
            ); 
         //   faav_m (Some (_m) );
         }
        return init.clone()
    }
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
     let _PREC0 = glob_precision (Some (6000));
     #[cfg(not(feature="meps"))]
     let _PREC0 = PREC0;
     #[cfg(feature="meps")]
    { _45deg = Pi();}
    _45deg /= 4;
     let mut rug_sin3_err = __2rt (&_05, _PREC0);
     let mut rug_sin_err = rug_sin3_err.clone();//_45deg.clone().sin();
     let mut rug_cos_err = rug_sin3_err.clone();// _45deg.clone().cos();
     //let mut rug_cos_err =  _45deg.clone().cos();
     let mut rug_cos3_err = rug_sin3_err.clone();//_45deg.clone().cos();
     //let sin_45deg = _45deg.sin();
     /*dbg! ("sqrt(2) for tst");
     __2rt(&_2, 2200);
     dbg! ("end sqrt(2) for tst");*/
     let mut sin_45deg =fast_n_simple_sin ( &_45deg.clone (), _PREC0 as usize);
     dbg! ("mark (-1)");
     #[cfg(feature="meps")]
     //let mut cos_45deg = _45deg.__cos(4200);//fast_n_simple_cos ( &_45deg, 5100);
     let mut cos_45deg = _45deg.__cos();
     #[cfg(not(feature="meps"))]
     let mut cos_45deg = fast_n_simple_cos ( &_45deg, 5100);
     let mut cos3_45deg =fast_n_simple_cos3 ( &_45deg, 2200);
     let mut sin3_45deg =fast_n_simple_sin3 ( &_45deg, 3750);
     #[cfg(feature="meps")]
     use min_err_per_step::base::{ext_const_E, _ext_const_E};
     #[cfg(feature="meps")]
     {
        dbg! ("mark0");
    use min_err_per_step::logarithm::{simple_ln, main_ln };
    use min_err_per_step::mt_logarithm::{
        btree_ln, 
        crawler_ln as crawler_ln_lib, 
        replace_crawler_ln,
        build_crawler_ln, set_crawler_ln_divisors
    };
    use IF::banu::{
        faav_countdown,
        faav_init_tail,
        faav_shift,
        faav_22m_m_1,
        _1st_tst as hella_hello_banu,
        _2nd_tst,
        _try_restore_factors,
        try_banu,
        banu,
        faav_power,
        faav_102m_m_1,
        faav_102shift,
        _gen_backdoor_numero,
        nxt_tail_of_sq_xor,
        faav_m
    };
    use min_err_per_step::editing::{
        conv_str_2_rugfloat,
        ret_extended_float_format,
        exclude_wrong_symbs_from_float_strn
    };
    use min_err_per_step::logarithm::crawler_ln as crawler_ln_lib0;
    use min_err_per_step::logarithm::{main_l2, cook_input_to_ln};
    use CuPs::smart_lags::mutex_lag;
    use CuPs::atomic::glob_delay;
    use min_err_per_step::complex::{
        nth_root::{isqrt, complex_roots, dbg_isqrt},
        traits::Cu_Complex
    };
    use Mademoiselle_Entropia::custom_traits::STRN;
    use Mademoiselle_Entropia::_break;
    use min_err_per_step::complex::traits;
    use min_err_per_step::nth_root::{dbg_2rt, replace_2rt};
    use min_err_per_step::complex::trig::{real_e2x as __tstReal_e2x, dbg_real_e2x, _real_e2x};
    use min_err_per_step::editing::Conv_Strn_2_Rugfloat;
    use min_err_per_step::Rationale::basic::{
        tst as r_tst,
        faav_q_sqrt_shift
    };
    use min_err_per_step::Rationale::traits_4_q_complex::{
        Q_Complex,
        Q_Complex_Pow,
        Q_2_Cu_Complex
    };
    use min_err_per_step::Rationale::trig_q_complex::{
        _real_e2x_4Q,
         real_e2x_4Q
    };
    use min_err_per_step::editing::Conv_Strn_2_Rugint;
    use rug::Rational as rugq;
       /* let x = conv_str_2_rugfloat (
            ".25",
            10
        ).unwrap ();
        dbg! (&x);
        dbg! (__2rt (&x, glob_precision (None)) );
        InterruptMsg ("");
        return (0.0, 0.0); */
        faav_q_sqrt_shift (Some (100) );
        let num = "1".int(10);
        let int_0 = "0".int (10);
        let Q_0: rugq = rugq::from ( (int_0, num.clone()) );
        let Q_1: rugq = rugq::from ( (num.clone(), num ) );
        let exp_q = Q_Complex::init_q (&Q_0, &Q_1);
        let mut tst_mul_assign = exp_q.clone();
        dbg! (&tst_mul_assign);
        tst_mul_assign *= tst_mul_assign.clone();
        let _1_of_Q = Q_Complex::init_u64 (1, 0);
        dbg! (&tst_mul_assign);
        dbg! (exp_q.clone().pow_u64 (2) );
        dbg! (exp_q.clone() * exp_q.clone() * _1_of_Q.clone() );
        dbg! (exp_q.clone() * _1_of_Q );
        dbg! (&exp_q);
        let tst_Q = real_e2x_4Q (&exp_q, 1);
        let mut __tstReal_e2x__ = Cu_Complex::init_f64 (0.0, 1.0);
        let tst_R = __tstReal_e2x (&__tstReal_e2x__, 402);
        dbg! (tst_Q.q_2_cu_complex () );
        dbg! (&tst_R);
        r_tst ();
        _break!("".strn() );
        glob_precision (Some (10_000) );
        faav_countdown (Some (10_000) );    
        faav_shift (Some (1_00));
        faav_102m_m_1 ( Some (2_00) );
        dbg_faav_102m_m_1 (Some (2_00));
        let mut tail = "75974954171".float (10);
        nxt_tail_of_sq_xor  (&mut tail, 3, 3);
        faav_init_tail (Some ( tail.to_string_radix (10, None ) ) );
        let float_tst: rugfloat = "39807508642406493739712550055038@@@
                                   64911990643623425267084063851895!
                                   75946388957261768583316".float (10);
        _break! (float_tst.to_string_radix (10, None) );
        //faav_22m_m_1 (Some (1_000));
        //_break! (faav_22m_m_1 (None).to_string_radix (2, Some (1001)));
        _break! (dbg_gen_backdoor_numero (&float_tst, &tail).to_string_radix (10, None) );
        hella_hello_banu ();  
        //_2nd_tst ();
       // _break! ("stop 2nd tst");
        return (0.0, 0.0);
        let _8: rugfloat = _1.clone() * 8;
        let mut cos_extra = _45deg.cos_extra_prec ();
        let cos_extra_vs__sin = _45deg.__sin () / cos_extra.clone ();
        cos_extra /= _45deg.__cos();
        let __cos_vs__sin  = _45deg.__sin () / _45deg.__cos();
        let power = rugfloat::with_val_64 (PREC0, 0.693147181);
        let ext_const_e = ext_const_E (&power);
        let std_ln = _8.clone().ln();
        dbg! (&std_ln);
       // let _8_log_2: rugfloat = _8.lg (&_2);
       replace_2rt (Some (try_2rt) );
       replace_crawler_ln (Some (crawler_ln_lib) );
       mutex_lag (Some (2500_000_000) );
        let mut div_vec = vec! [283, 47, 2571];
        set_crawler_ln_divisors (Some (div_vec) );
        glob_delay (Some (80_000_000_000) );
       let approx_8 = crawler_ln_lib (&_8, 6000, 100, 65);//_8.pow(&_8_log_2);
        dbg! ("1st run of build crawler");
        let _533 = _1.clone() * 0.07937f64;
        //let _simple_ln = simple_ln (&_533, 100).0;
        //let approx_2 = build_crawler_ln (&_533, 6000, 400, 21);
        //let approx_2 = crawler_ln_lib0 (&_533, 6000, 400, 21);
        //let approx_2 = dbg_divide_n_conquer_2_calc_ln (&_533, 6000, 400, 0);
        let approx_2: rugfloat = main_l2 (&_533, 3500);
        let approx_ln: rugfloat = main_ln (&_533, 3500);
        dbg! ("checked crawler");
        //let tst_cmplx = Cu_Complex::init_f64 (2.0, 23.0); let tst_cmplx = Cu_Complex::init_f64 (31.0, 23.0);
       // glob_precision (Some (8000));
        let tst_cmplx = Cu_Complex::init_f64 (128.0, -27.0);
        
        //dbg! (dbg_2rt (&_533, glob_precision(None) ) );
        let tst_isqrt: complex_roots = isqrt (&tst_cmplx).unwrap();
        let mut __tstReal_e2x__ = Cu_Complex::init_jrugfloat (&approx_2) * -1;//init_f64(0.0, 0.693147181);
        let mut __low_prec_e2x__ = Cu_Complex::init_f64 (0.0, -2.07944154);
        let __dbg_real_e2x = dbg_real_e2x (&__tstReal_e2x__, 3000, fast_n_simple_isin ).unwrap();
        //let x = _1_over_x.clone().pow(-1);
        dbg! (&cos_extra);
        dbg! (&cos_extra_vs__sin);
        dbg! (&__cos_vs__sin);
        dbg! (&ext_const_e);
        dbg! (&approx_2);
        dbg! (&_533);
        dbg! (cook_input_to_ln (&_533 ));
        dbg! (_533.clone() / _2.clone().pow(&approx_2) );
        dbg! (_ext_const_E(&approx_8));
        dbg! (ext_const_E(&std_ln));
        dbg! (ext_const_E(&approx_2));
        dbg! (approx_8 / std_ln);
        //dbg! (__tstReal_e2x (&__low_prec_e2x__, 2050) );
        dbg!(approx_ln / _533.ln() );
        dbg! (_real_e2x (&__low_prec_e2x__, 300) );
        dbg! (&__dbg_real_e2x );//_fast_n_simple_isin3) );
        dbg! (__dbg_real_e2x.radius() );
      //  dbg! (cfrac_e2x (&_1, 100) );
       /* dbg! (&tst_isqrt);
        dbg! (tst_isqrt.root0.clone() * tst_isqrt.root0.clone());
        dbg! (tst_isqrt.root1.clone() * tst_isqrt.root1);*/
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
#[cfg(feature="meps")]
use min_err_per_step::complex::traits::Cu_Complex;
#[cfg(feature="meps")]
pub fn _fast_n_simple_isin3 (x: &Cu_Complex, err: u64 ) -> Cu_Complex {
 use min_err_per_step::complex::trig::gen_prec_for_three;
 use min_err_per_step::complex::traits::Cu_Complex_Pow;
 // dbg! (&x);
    let PREC0_ = glob_precision (None);
    let err: u64 = if PREC0_ < err {
        PREC0_ / gen_prec_for_three ()
     }else {err};
    let _3 = rugfloat::with_val_64 (PREC0_, 3);
    let _1 = rugfloat::with_val_64 (PREC0_, 1);
    let mut start_x: Cu_Complex = x.clone() / &_3.pow (err);
    //dbg! (&start_x);
    let mut step: usize = 0;
    //sin_x = 2 * start_x.clone ();
    let mut sin_3x = start_x.clone();
    if start_x.1 < 0 {start_x *= -1;}
    let x_abs = x.abs();
//    dbg! (&start_x);
    //dbg! (&sin_3x);
    while start_x.cmp_jless ( &x_abs ) {
        sin_3x = 3u64 * sin_3x.clone () - 4* sin_3x.clone ().pow_u64 (3);
        start_x *= 3;
      //  dbg! (&sin_3x);
    }
    //dbg! (&sin_3x);
    return sin_3x
} 
#[cfg(feature="meps")]
pub fn ext_const_E (pow: &rugfloat) -> rugfloat {
    let sign: i8 = if *pow > 0 { 1 } else { -1 };
    let pow = pow.clone() * sign;
    let PREC0_ = glob_precision (None);
  //  let max_terms = PREC0 as usize / 3;
    let (new_coef, canceled_coef) = fast_n_dumb_shortcut_for_cfrac (&pow);//continued_fraction_approximation (&pow, max_terms, PREC0);
    let new_coef = new_coef * sign;
    let ret = re_fast_real_e (&new_coef, &canceled_coef, PREC0_);
    let NaN = rugfloat::with_val_64 (PREC0_, rug::float::Special::Nan);
    if ret == NaN {
        dbg! (&new_coef);
        dbg! (&canceled_coef);
    }
    return ret
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
            crate::faav::real_e (Some( re_fast_real_e(&new_coef, &canceled_coef, prec_) ), prec_ );
        }).join();
    }
    let saved_real_e = crate::faav::real_e(None, 0);
    if saved_real_e > rugfloat::with_val(3, 0.0) {return saved_real_e; }
    const_e.clone()
}
pub fn re_fast_real_e (new_coef: &rugfloat, canceled_coef: &rugfloat, prec_: u64) -> rugfloat {
    let PREC_ = prec_;
    let PREC0_ = prec_;
    let one = rugfloat::with_val_64(PREC0_, 1.0);
    let mut one_div_by = one.clone();
    let rm = Round::Down;
    let exponent: u64 = PREC0_ / 2;
    let mut const_e_base = rugfloat::with_val_64(PREC0_, 2.0);
    //const_e_base.pow_assign_round(exponent, rm);
    const_e_base.pow_assign(exponent); // = 2 ^ m
    //dbg! (&const_e_base);
    let mut big_exp = const_e_base.clone (); // = 2 ^ m
    big_exp *= rugfloat::with_val_64(PREC0_, new_coef); // = new_coef * 2 ^ m
    const_e_base.mul_assign_round(rugfloat::with_val_64(PREC0_ , canceled_coef), rm);
   // dbg! (&const_e_base);
    one_div_by.div_assign_round(&const_e_base, rm);
   // dbg! (&const_e_base);
    const_e_base = one_div_by;
    const_e_base += one.clone();
  /*  dbg! (&big_exp);
    dbg! (&canceled_coef);
    dbg! (&new_coef);*/
    let mut const_e = const_e_base.clone();
    const_e.pow_assign( big_exp );
   /* dbg! (&const_e_base );
    dbg!(&const_e);*/
    if const_e == one {
        let prec_ = (PREC0_ as f64 + PREC0_  as f64 * 0.5) as u64;
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
#[cfg(feature="meps")]
use min_err_per_step::logarithm::simple_ln;
#[cfg(feature="meps")]
pub fn _0crawler_ln (a: &rugfloat, err: u64) -> rugfloat {
    let PREC0_ = glob_precision (None);
    //let _1_over_3 = rugfloat::with_val_64 (PREC0_, 1/3);
    let _1_over_6 = rugfloat::with_val_64 (PREC0_, 1/6);
    let _1_over_24 = rugfloat::with_val_64 (PREC0_, 1/24);
    let mut ret: rugfloat = rugfloat::with_val_64 (PREC0_, 1);
    let mut xn: rugfloat = ret.clone() - 1;//simple_ln (a, err).0;
    let mut e2xn = ext_const_E (&xn);
    for j in 0..err {
        ret = xn.clone() + 2 * (a.clone() - e2xn.clone()) / (a.clone() + e2xn.clone());
        //ret -= 0.5 * (a.clone() - e2xn.clone()).pow(2); 
       // ret -= _1_over_6.clone() * (a.clone() - e2xn.clone()).pow(3); 
       // ret -= _1_over_24.clone() * (a.clone() - e2xn.clone()).pow(4); 
        xn = ret.clone();
    }
    return ret;
}
#[cfg(feature="meps")]
pub fn simple_ln_here (a: &rugfloat, local_prec: u64) -> (rugfloat, rugfloat) {
use min_err_per_step::nth_root::__22mrt;
    let a2x = __22mrt (a, local_prec );
    dbg! ("__22mrt in simple ln here");
    let _1_over_x: rugfloat = 
        rugfloat::with_val_64 (glob_precision (None), 2) << local_prec as usize - 1;
    dbg! ("1 over x");
   // dbg! (&a2x);
    //dbg! (&_1_over_x);
    let  ln_: rugfloat = (a2x - 1) * _1_over_x.clone ();
    dbg! ("exit simple ln here");
    return (ln_, _1_over_x)
}
#[cfg(feature="meps")]
#[no_mangle]
pub fn crawler_ln_lite (a: &rugfloat, err: u64, feeder_cnt: u64, broker: u64) -> rugfloat {
    dbg! ("here");
    let PREC0_ = glob_precision (None);
    dbg! ("glob prec");
    //let _1_over_3 = rugfloat::with_val_64 (PREC0_, 1/3);
    let mut ret: rugfloat = rugfloat::with_val_64 (PREC0_, 1);
    dbg! ("mk ret var");
    let a_ = a.clone();
    dbg! (&a_);
    let mut xn: rugfloat = ret.clone(); //simple_ln_here (&a_, feeder_cnt ).0;
    dbg! ("simple ln");
    let mut e2xn = ext_const_E (&xn);
    dbg! ("ext const e");
    let mut tail: rugfloat = rugfloat::with_val_64 (PREC0_, 0.999);
    let mut count_broker = 0u64;
    let mut dxn = tail.clone();
   // dbg! (&e2xn);
    //let mut dx = xn.clone();
    for j in 0..err {
        dxn = a.clone()/e2xn.clone() - 1;
        ret = xn.clone() + dxn.clone();
       // dbg! (&j);
        //dx = (ret.clone() - xn.clone() ).abs();
        //e2xn *= e2dx_nxt2_1 (&dx);
        if broker == count_broker { 
            e2xn = ext_const_E (&xn);
//            dbg! (&j);
  //          dbg! ("ext const e in for-loop");
            count_broker = 0;
            xn = ret.clone();
            continue;
         }
         (e2xn, tail) = speedup_ln (&a, &tail);
    //     dbg! ("speedup ln");
      //   dbg! (&j);
        count_broker += 1;
        xn = ret.clone();
        //if e2xn == 0 {e2xn = xn.clone(); dbg!("e2xn == 0");}
    }
    dbg! ("exit crawler_ln_lite");
    return ret;
}
#[cfg(feature="meps")]
pub fn crawler_ln (a: &rugfloat, err: u64, feeder_cnt: u64, broker: u64) -> rugfloat {
use min_err_per_step::logarithm::btree_ln;
    let PREC0_ = glob_precision (None);
    //let _1_over_3 = rugfloat::with_val_64 (PREC0_, 1/3);
    let _1_over_6 = rugfloat::with_val_64 (PREC0_, 1/6);
    let _1_over_24 = rugfloat::with_val_64 (PREC0_, 1/24);
    let mut ret: rugfloat = rugfloat::with_val_64 (PREC0_, 1);
    let mut xn: rugfloat = simple_ln (a, feeder_cnt ).0;
    let mut e2xn = ext_const_E (&xn);
    let mut tail: rugfloat = rugfloat::with_val_64 (PREC0_, 0.999);
    let mut count_broker = 0u64;
   // dbg! (&e2xn);
    //let mut dx = xn.clone();
    for j in 0..err {
        ret = xn.clone() + a.clone()/e2xn.clone() - 1;
        ret -= 0.5 * (a.clone() - e2xn.clone()).pow(2); 
        ret -= _1_over_6.clone() * (a.clone() - e2xn.clone()).pow(3); 
        ret -= _1_over_24.clone() * (a.clone() - e2xn.clone()).pow(4); 
        //dx = (ret.clone() - xn.clone() ).abs();
        //e2xn *= e2dx_nxt2_1 (&dx);
        if broker == count_broker { 
            e2xn = ext_const_E (&xn);
            count_broker = 0;
         } else {
            (e2xn, tail) = speedup_ln (&a, &tail);
         }
        count_broker += 1;
        xn = ret.clone();
        //if e2xn == 0 {e2xn = xn.clone(); dbg!("e2xn == 0");}
    }
    return ret;
}
#[cfg(feature="meps")]
pub fn speedup_ln (a: &rugfloat, tail: &rugfloat) -> (rugfloat, rugfloat) {
    let mut a = a.clone() - 1;
    let tail = __2rt (&tail, glob_precision (None) );
    return (a + tail.clone(), tail)
}
 use Mademoiselle_Entropia::custom_traits::helpful_math_ops;
 #[cfg(feature="meps")]
 use min_err_per_step::nth_root::__22mrt;
#[cfg(feature="meps")]
pub fn e2dx_nxt2_1 (dx: &rugfloat) -> rugfloat {
    let mut approx_dx = rugfloat::with_val_64 (
        glob_precision (None),
        0.5
    );
    let mut cnt = 0u64;
    while approx_dx > *dx {
        approx_dx >>= 1;
        cnt += 1;
    }
    cnt.dec();
    let mut e2dx = 1 + dx.clone();
    e2dx = __22mrt (&e2dx, cnt);
    //dbg! (&e2dx);
    return e2dx
}
#[cfg(feature="meps")]
pub fn cfrac_e2x (x: &rugfloat, rounds: i64) -> rugfloat {
    let mut ret = rugfloat::with_val_64 (
        glob_precision (None),
        0
    );
    let mut count = rounds;
    let mut den = 0i64;
    while count > 0 {
        ret = x.clone() / (ret + count);
        count -= 1;
    }
    ret = 1 + ret;
    return ret
}
#[cfg(feature="meps")]
pub fn cfrac_e2x0(x: &rugfloat, rounds: u64) -> rugfloat {
    let precision = glob_precision(None);
    let mut cf = rugfloat::with_val_64(precision, 0.0);
    
    // Work backwards from the deepest level
    for n in (1..=rounds).rev() {
        if n % 2 == 1 {
            // Odd pattern: x / (2n-1 - cf)
            let denom = rugfloat::with_val_64(precision, 2 * n - 1);
            cf = x.clone() / (denom - cf);
        } else {
            // Even pattern: x / (2n + cf)
            let denom = rugfloat::with_val_64(precision, 2 * n);
            cf = x.clone() / (denom + cf);
        }
    }
    
    // Final result: 1 + cf
    rugfloat::with_val_64(precision, 1) + cf
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
