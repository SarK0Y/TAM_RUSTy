use rug::float::Round;
use rug::ops::{AddAssignRound, DivAssignRound, MulAssignRound, PowAssign as rugPowAssign, PowAssignRound, SubAssignRound, Pow as rugpow};
use rug::{Assign, Integer as rugint, float::Constant as rugconst, Float as rugfloat, ops::SubFrom};
use rug::float::Constant;
use crate::base::{glob_precision, Pi };
use Mademoiselle_Entropia::minio::InterruptMsg;
pub fn __2rt (x: &rugfloat, err: u64 ) -> rugfloat {
    let PREC0 = glob_precision (None);
    let _2 = rugfloat::with_val_64 (PREC0, 2);
    let _1 = rugfloat::with_val_64 (PREC0, 1);
    let mut start_x: rugfloat = _1.clone();
    start_x.assign (x >> 3);
    let no_less = _1.clone () / _2.clone().pow (err - 1);
    //dbg! (&no_less);
    let mut b = _1.clone();
   // dbg! (&start_x);
    let mut step: usize = 0;
    //sin_x = 2 * start_x.clone ();
    let mut ds = (start_x.clone() - b.clone() ).abs ();
    while ds > no_less {
        b = x.clone () / start_x.clone ();
	    start_x = (start_x.clone() + b.clone () ) / 2;
        ds = (start_x.clone() - b.clone() ).abs ();
        //dbg! (&ds);
    }
 //   dbg! (&b);
    return b
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
pub fn __22mrt (x: &rugfloat, n: u64) -> (rugfloat, u64 ) {
    let PREC0 = glob_precision (None);
    let mut ret = x.clone ();
    let mut cnt = 1u64;
    while cnt < n {
        ret = __2rt ( &ret, PREC0 );
        cnt *= 2;
    } return (ret, cnt )
}