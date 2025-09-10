use rug::float::Round;
use rug::ops::{AddAssignRound, DivAssignRound, MulAssignRound, PowAssign as rugPowAssign, PowAssignRound, SubAssignRound, Pow as rugpow};
use rug::{Assign, Integer as rugint, float::Constant as rugconst, Float as rugfloat, ops::SubFrom};
use rug::float::Constant;
use crate::base::{glob_precision, Pi };
use Mademoiselle_Entropia::minio::InterruptMsg;
pub fn __2rt (x: &rugfloat, err: usize ) -> rugfloat {
    let PREC0 = glob_precision (None);
    let _2 = rugfloat::with_val_64 (PREC0, 2);
    let _1 = rugfloat::with_val_64 (PREC0, 1);
    let mut start_x: rugfloat = _1.clone();
    start_x.assign (x >> 3);
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
