use once_cell::sync::Lazy;
use rug::float::Round;
use rug::ops::{AddAssignRound, DivAssignRound, MulAssignRound, PowAssign as rugPowAssign, PowAssignRound, SubAssignRound, Pow as rugpow, CompleteRound };
use rug::{Assign, Integer as rugint, float::Constant as rugconst, Float as rugfloat, ops::SubFrom, Complete};
use rug::float::Constant;
use crate::base::{glob_precision, Pi, ext_const_E };
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
    let mut ret = _1.clone ();
    let mut e2x = _1.clone ();
    let mut epsilon: rugfloat = _1 / _2.pow (err);
    let mut direction = rugfloat::with_val_64 (PREC0, 0.5);
    while (y.clone() - e2x.clone() ).abs () > epsilon {
        e2x = ext_const_E (&ret );
        if *y < e2x {ret -= direction.clone();}
        else { ret += direction.clone(); }
        direction >>= 1;
    }
    return ret
}
pub fn filter_input_for_ln (x: &rugfloat, base: Option <&rugfloat>) -> Result <(), ln_err > {
    if *x < 0 { return Err (ln_err::input_cant_be_negative) }
    if let Some ( y ) = base {
        if *y < 0 { return Err (ln_err::base_cant_be_negative)}
    } return Ok (())
}
pub fn normalize_input_for_ln (y: &rugfloat) -> Result <rugfloat, ln_err > {
    let e = ext_const_E (&rugfloat::with_val_64 (glob_precision (None), 1) );
    let mut ret = rugfloat::with_val_64 (glob_precision (None), 1);
    if *y > e { ret = ret.pow (-1) * -1; }
    else {return Err (ln_err::ok)}
    return Ok (ret)
}
pub trait lg {
    fn __ln (&self) -> Self;
    //fn log (&self, base: &rugfloat) -> Self;
}
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
        if let Ok (x) = normalize_input_for_ln (self) {
            return btree_ln (&x, glob_precision (None)).pow (-1)
        }
        return btree_ln (self, glob_precision (None) )
    }
}