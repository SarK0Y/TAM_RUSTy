use once_cell::sync::Lazy;
use rug::float::Round;
use rug::ops::{AddAssignRound, DivAssignRound, MulAssignRound, PowAssign as rugPowAssign, PowAssignRound, SubAssignRound, Pow as rugpow, CompleteRound };
use rug::{Assign, Integer as rugint, float::Constant as rugconst, Float as rugfloat, ops::SubFrom, Complete};
use rug::float::Constant;
use crate::base::{glob_precision, ctrl_glob_precision, manage_prec, Pi, ext_const_E };
use crate::nth_root::__22mrt;
use std::error::Error;
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
/// pending for tst
pub fn simple_ln (a: &rugfloat, local_prec: u64) -> (rugfloat, rugfloat) {
    let a2x = __22mrt (a, local_prec );
    let _1_over_x: rugfloat = 
        rugfloat::with_val_64 (glob_precision (None), 2) << local_prec as usize - 1;
   // dbg! (&a2x);
    //dbg! (&_1_over_x);
    let  ln_: rugfloat = (a2x - 1) * _1_over_x.clone ();
    return (ln_, _1_over_x)
}
//fn
//x_{n+1} = x_n - f/f' - (f''f²)/(2f'³) - (f'''f³)/(6f'⁴) - (f''''f⁴)/(24f'⁵) + O(f⁵)