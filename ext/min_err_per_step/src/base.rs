use rug::float::Round;
use rug::ops::{AddAssignRound, DivAssignRound, MulAssignRound, PowAssign as rugPowAssign, PowAssignRound, SubAssignRound, Pow as rugpow};
use rug::{Assign, Integer as rugint, float::Constant as rugconst, Float as rugfloat, ops::SubFrom};
use rug::float::Constant;
use Mademoiselle_Entropia::minio::InterruptMsg;
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
