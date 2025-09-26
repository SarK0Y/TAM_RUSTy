use once_cell::sync::Lazy;
use rug::float::Round;
use rug::ops::{AddAssignRound, DivAssignRound, MulAssignRound, PowAssign as rugPowAssign, PowAssignRound, SubAssignRound, Pow as rugpow, CompleteRound };
use rug::{Assign, Integer as rugint, float::Constant as rugconst, Float as rugfloat, ops::SubFrom, Complete};
use rug::float::Constant;
use crate::base::{glob_precision, ctrl_glob_precision, manage_prec, Pi, ext_const_E };
use crate::nth_root::{__22mrt, __2rt};
use std::error::Error;
use Mademoiselle_Entropia::custom_traits::helpful_math_ops;
use Mademoiselle_Entropia::minio::InterruptMsg;
pub enum ln_err {
    input_cant_be_negative,
    base_cant_be_negative,
    ok
}
pub fn btree_ln (y: &rugfloat, err: u64) -> rugfloat {
    let PREC0 = glob_precision (None);
    let _2 = rugfloat::with_val_64 (PREC0, 2);
    let _1 = rugfloat::with_val_64 (PREC0, 1);
    let mut ret = init_ln (y);
    let mut e2x = _1.clone ();
    let mut epsilon: rugfloat = _1 / _2.pow (err);
    dbg! (&epsilon);
    let mut direction: rugfloat = ret.clone() >> 1;
    while direction > epsilon {
        e2x = ext_const_E (&ret );
        if *y < e2x {ret -= direction.clone();}
        else { ret += direction.clone(); }
        direction >>= 1;
     //   dbg! (&direction);
    }
    dbg! ("btree_ln");
    dbg! (&ret);
    return ret
}
pub fn init_ln (x: &rugfloat) -> rugfloat {
    let mut _22n = rugfloat::with_val_64 ( glob_precision (None), 2);
    if *x > 1 {
        while _22n < *x {
            _22n <<= 1;
        } return _22n
    }
    if *x < 1 {
        while _22n > *x {
            _22n >>= 1
        }
    } return _22n * -1
}
pub fn filter_input_for_ln (x: &rugfloat, base: Option <&rugfloat>) -> Result <(), ln_err > {
    if *x < 0 { return Err (ln_err::input_cant_be_negative) }
    if let Some ( y ) = base {
        if *y < 0 { return Err (ln_err::base_cant_be_negative)}
    } return Ok (())
}
pub fn normalize_input_for_ln (y: &rugfloat) -> Result <rugfloat, ln_err > {
    let e = ext_const_E (&rugfloat::with_val_64 (glob_precision (None), 1) );
    dbg! (&e);
    let mut ret = rugfloat::with_val_64 (glob_precision (None), 1);
    if *y > e { ret = ret.pow (-1); }
    else {return Err (ln_err::ok)}
    return Ok (ret)
}
pub trait lg {
    fn __ln (&self) -> Self;
    fn lg (&self, base: &rugfloat) -> Self;
}
/// needs testing
impl lg for rugfloat {
    fn __ln (&self) -> Self {
        match filter_input_for_ln (&self, None) {
            Err (ln_err::input_cant_be_negative) => {
                InterruptMsg ("Please, don't use negative value.");
                return self.clone()
            },
            Err(ln_err::base_cant_be_negative) => {
                InterruptMsg ("Please, don't use negative base.");
                return self.clone()
            },
            _ => {}
        }
        /*if let Ok (x) = normalize_input_for_ln (self) {
            return btree_ln (&x, glob_precision (None) - 3 ).pow (-1)
        } */
        return btree_ln (self, glob_precision (None) - 3 )
    }
    fn lg (&self, base: &rugfloat) -> Self {
        match filter_input_for_ln (&self, Some (base) ) {
            Err (ln_err::input_cant_be_negative) => {
                InterruptMsg ("Please, don't use negative value.");
                return self.clone()
            },
            Err(ln_err::base_cant_be_negative) => {
                InterruptMsg ("Please, don't use negative base.");
                return self.clone()
            },
            _ => {}
        }
        let mut value = rugfloat::with_val_64 (glob_precision (None), 1);
        let mut base_ = rugfloat::with_val_64 (glob_precision (None), 1);
        value = btree_ln (self, glob_precision (None) - 3);
        base_ = btree_ln (self, glob_precision (None) );
        /*if let Ok (x) = normalize_input_for_ln (self) {
            value = btree_ln (&x, glob_precision (None) - 3).pow (-1);
        } else { value = btree_ln (self, glob_precision (None) - 3); }*/
        let base_less_than_1: bool = if *base < 0 {
            true
        } else { false }; 
     /*   if let Ok (x) = normalize_input_for_ln (base) {
            base_ = btree_ln (&x, glob_precision (None)).pow (-1);
        } else { base_ = btree_ln (self, glob_precision (None) ); }*/
    let mut ret: rugfloat = value / base_;
    if base_less_than_1 { ret *= -1;}
    return ret
    }   
}
/// Good to be feeder.
pub fn simple_ln (a: &rugfloat, local_prec: u64) -> (rugfloat, rugfloat) {
    let a2x = __22mrt (a, local_prec );
    let _1_over_x: rugfloat = 
        rugfloat::with_val_64 (glob_precision (None), 2) << local_prec as usize - 1;
   // dbg! (&a2x);
    //dbg! (&_1_over_x);
    let  ln_: rugfloat = (a2x - 1) * _1_over_x.clone ();
    return (ln_, _1_over_x)
}
/// How to establish Numerical Gravity: https://alg0z.blogspot.com/2025/09/fake-calculations-to-feed-numerical.html
pub fn crawler_ln (a: &rugfloat, err: u64, feeder_cnt: u64, broker: u64) -> rugfloat {
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
pub fn speedup_ln (a: &rugfloat, tail: &rugfloat) -> (rugfloat, rugfloat) {
    let mut a = a.clone() - 1;
    let tail = __2rt (&tail, glob_precision (None) );
    return (a + tail.clone(), tail)
}
//fn