use rug::float::Round;
use rug::ops::{
    AddAssignRound, DivAssignRound,
    MulAssignRound, PowAssign as rugPowAssign,
    PowAssignRound, SubAssignRound,
    Pow as rugpow, CompleteRound,
};
use rug::{
    Assign, Integer as rugint,
    float::Constant as rugconst,
    Float as rugfloat, ops::SubFrom,
    Complete
};
use rug::rational::MiniRational;
use rug::Rational as rugq;
use rug::float::Constant;
use std::convert::TryFrom;
use substring::Substring;
use crate::base::{glob_precision, Pi };
use crate::nth_root::__2rt;
use Mademoiselle_Entropia::custom_traits::STRN;
use Mademoiselle_Entropia::minio::InterruptMsg;
pub struct Rationale {
    num: rugint,
    den: rugint
}
pub fn tst () {
    let a = rugq::from ( (1, 7) );
    let b = rugq::from ( (1, 3) );
    let a_pl_b = a+b;
    dbg!(&a_pl_b);
}
pub fn __2rt4Q (x: &rugq) -> rugq {
    let num: rugfloat = rugfloat::with_val_64 (
        glob_precision (None),
        x.numer ()
    );
    let den: rugfloat = rugfloat::with_val_64 (
        glob_precision (None),
        x.denom ()
    );
    let mut new_den = __2rt (
        &den,
        glob_precision (None)
    );
    let mut new_num = __2rt (
        &num,
        glob_precision (None)
    );
    let _10 = rugfloat::with_val_64 (
        glob_precision (None),
        10
    );
    let shift = _10.pow (faav_q_sqrt_shift (None));
    new_den *= shift.clone();
    new_num *= shift;
    let new_den = new_den.to_integer ().unwrap ();
    let new_num = new_num.to_integer ().unwrap ();
    return rugq::from ((new_num, new_den) );
}
pub fn faav_q_sqrt_shift ( new_shift: Option <usize>) -> usize {
    static mut shift: usize = 0;
    unsafe {
        if let Some ( x ) = new_shift { shift = x; }
        return shift
    }
}