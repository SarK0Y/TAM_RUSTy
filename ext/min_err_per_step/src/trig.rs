use rug::float::Round;
use rug::ops::{AddAssignRound, DivAssignRound, MulAssignRound, PowAssign as rugPowAssign, PowAssignRound, SubAssignRound, Pow as rugpow};
use rug::{Assign, Integer as rugint, float::Constant as rugconst, Float as rugfloat, ops::SubFrom};
use rug::float::Constant;
use crate::base::{glob_precision, Pi };
use Mademoiselle_Entropia::minio::InterruptMsg;
pub fn fast_n_simple_sin3 (x: &rugfloat, err: u64 ) -> rugfloat {
    let PREC0 = glob_precision (None);
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
pub trait Trig {
    fn __sin (&self) -> Self;
}
impl Trig for rugfloat {
    fn __sin (&self) -> Self {
        return fast_n_simple_sin3 (self, glob_precision (None) )
    }
}