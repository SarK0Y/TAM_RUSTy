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
    let int_1 = rugint::from (1);
    let mut ret = square_xor ( x ) * x.clone () * coef.clone ();
    let ret_sb = ret.to_integer()
                 .unwrap_or(int_1.clone () )
                .significant_bits ();
    let x_sb = x.to_integer()
                .unwrap_or(int_1)
                .significant_bits ();
    ret >>= (ret_sb - x_sb);
    return ret.floor ()
}
pub enum bloater <'a>{
    float (&'a rugfloat),
    int (&'a rugint)
}
pub fn bloat (x0: bloater, coef: &rugfloat) -> rugint {
    match x0 {
        bloater::float (x) => {
            return ( square_xor ( &x ) * x.clone () * coef.clone () )
                .to_integer ()
                .unwrap ();
        },
        bloater::int (x) => {
            let x = rugfloat::with_val_64 (glob_precision (None), x);
            return ( square_xor ( &x ) * x.clone () * coef.clone () )
                .to_integer ()
                .unwrap ();
            
        }
    }

}
pub fn bloat_upto (x: &rugfloat, coef: &rugfloat, upto: u32) -> rugfloat {
    let mut bloat_it: rugint = bloat (bloater::float (x), coef);
    while bloat_it.significant_bits () < upto {
        bloat_it = bloat (bloater::int (&bloat_it), coef);
    }
    bloat_it >>= (bloat_it.significant_bits () - upto );
    return rugfloat::with_val_64 (glob_precision (None), bloat_it)
}