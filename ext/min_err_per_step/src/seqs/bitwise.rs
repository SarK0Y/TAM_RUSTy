use rug::float::Round;
use rug::ops::{AddAssignRound, DivAssignRound, MulAssignRound, PowAssign as rugPowAssign, PowAssignRound, SubAssignRound, Pow as rugpow, CompleteRound };
use rug::{Assign, Integer as rugint, float::Constant as rugconst, Float as rugfloat, ops::SubFrom, Complete};
use rug::float::Constant;
use crate::base::{glob_precision, Pi };
use Mademoiselle_Entropia::minio::InterruptMsg;
use Mademoiselle_Entropia::_break;
pub fn square_xor (x: &rugfloat) -> rugfloat {
    let mut ri: rugint = x.to_integer().unwrap();
    let sb = ri.significant_bits();
    ri.pow_assign (2);
    let low_mask: rugint = (rugint::from (1) << sb ) - 1;
    //let high_mask: rugint = low_mask.clone() << sb;
    let low_ri = ri.clone () & low_mask;
    ri >>= sb;
    ri ^= low_ri;
    return rugfloat::with_val (x.prec(), ri)
}
pub fn road1 (x: &rugfloat, coef: &rugfloat) -> rugfloat {
    return square_xor ( x ) * x.clone () * coef.clone ()
}