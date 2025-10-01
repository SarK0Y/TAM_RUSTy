use once_cell::sync::Lazy;
use rug::float::Round;
use rug::ops::{AddAssignRound, DivAssignRound, MulAssignRound, PowAssign as rugPowAssign, PowAssignRound, SubAssignRound, Pow as rugpow};
use rug::{Assign, Integer as rugint, float::Constant as rugconst, Float as rugfloat, ops::SubFrom};
use rug::float::Constant;
use crate::base::{glob_precision, ctrl_glob_precision, Pi };
use Mademoiselle_Entropia::custom_traits::helpful_math_ops;
use Mademoiselle_Entropia::minio::InterruptMsg;
fn faav_a (pointer: Option <*mut rugfloat>) -> Option <*mut rugfloat> {
    static mut state: Lazy < Option <*mut rugfloat> > = Lazy::new (|| {None});
    unsafe {
        if pointer.is_some() { *state = pointer} state.clone()
    }
}
fn faav_b (pointer: Option <*mut rugfloat>) -> Option <*mut rugfloat> {
    static mut state: Lazy < Option <*mut rugfloat> > = Lazy::new (|| {None});
    unsafe {
        if pointer.is_some() { *state = pointer} state.clone()
    }
}
pub fn __2rt (x: &rugfloat, err: u64 ) -> rugfloat {
 unsafe {
    let PREC0 = glob_precision (None);
    let _05 = rugfloat::with_val_64 (PREC0, 0.5);
    let _1 = rugfloat::with_val_64 (PREC0, 1);
    let mut start_x: rugfloat = _1.clone();
    faav_a (Some (&mut start_x));
    let tmp: rugfloat = x.clone() / 3;
    let no_less = _05.clone().pow (err - 10);
    faav_a(None).unwrap().as_mut().unwrap().assign (tmp);
    *faav_a(None).unwrap() += (*faav_a(None).unwrap()).clone() >> 2;
    let mut b = _1.clone();
    faav_b (Some (&mut b));
    let mut step: u64 = 0;
    while ( (*faav_a(None).unwrap()).clone() - (*faav_b(None).unwrap()).clone())
        .abs() > no_less {
        *faav_b(None).unwrap() = x.clone () / (*faav_a(None).unwrap()).clone();
	    start_x = (start_x.clone() + b.clone () ) / 2;
        //step.inc();
    }
 //   dbg! (&b);
    if b < start_x && *x > 0 {return start_x }
    return b
}
} 
pub fn dbg_2rt (x: &rugfloat, err: u64 ) -> rugfloat {
 unsafe {
    let PREC0 = glob_precision (None);
    let _05 = rugfloat::with_val_64 (PREC0, 0.5);
    let _1 = rugfloat::with_val_64 (PREC0, 1);
    let mut start_x: rugfloat = _1.clone();
    faav_a (Some (&mut start_x));
    let tmp: rugfloat = x.clone() / 3;
    let no_less = _05.clone().pow (err - 10);
    faav_a(None).unwrap().as_mut().unwrap().assign (tmp);
    *faav_a(None).unwrap() += (*faav_a(None).unwrap()).clone() >> 2;
    let mut b = _1.clone();
    faav_b (Some (&mut b));
    let mut step: u64 = 0;
    while ( (*faav_a(None).unwrap()).clone() - (*faav_b(None).unwrap()).clone())
        .abs() > no_less {
        *faav_b(None).unwrap() = x.clone () / (*faav_a(None).unwrap()).clone();
	    start_x = (start_x.clone() + b.clone () ) / 2;
        dbg! (&start_x);
        dbg! (&b);
        //step.inc();
    }
 //   dbg! (&b);
    if b < start_x && *x > 0 {return start_x }
    return b
}
} 
pub fn nthrt (x: &rugfloat, pow: &rugfloat, err: u64) -> rugfloat {
    let PREC0 = glob_precision (None);
    let mut ret = rugfloat::with_val_64 (PREC0, 1);
    let pows = get_pows (pow, err);
    let mut step = 0u64;
    let mut approx = rugfloat::with_val_64 (PREC0, x);
    for j in pows {
        while step < j {
            approx = __2rt (&approx, err);
            step += 1;
        }
        ret *= approx.clone();  
    }
    return ret
}
pub fn get_pows (pow: &rugfloat, err: u64) -> Vec <u64>{
    let PREC0 = glob_precision (None);
    let mut step = 0u64;
    let mut ret = Vec::<u64>::new();
    let _2 = rugfloat::with_val_64 (PREC0, 2);
    let _1 = rugfloat::with_val_64 (PREC0, 1);
    let mut approx = rugfloat::with_val_64 (PREC0, 0);
    let mut try0 = approx.clone();
    let mut closer = approx.clone();
    while (pow.clone () - approx.clone() ) > err{
        closer = _1.clone () / (_2.clone() << step as usize );
        try0 = closer + approx.clone();
        if try0 < *pow { approx = try0; ret.push (step) }
        step += 1;
    }
    return ret
}
/// t = 2^(2^m) [root] where t < n
pub fn __22mrt (x: &rugfloat, n: u64) -> rugfloat {
    let PREC0 = glob_precision (None);
    let mut ret = x.clone ();
    for t in 0..n {
        ret = __2rt ( &ret, PREC0 );
 //       dbg! (&t);
   //     dbg! (&ret);
    } return ret
}