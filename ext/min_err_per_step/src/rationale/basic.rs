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
use rug::Rational;
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
    let a = Rational::from ( (1, 7) );
    let b = Rational::from ( (1, 3) );
    let a_pl_b = a+b;
    dbg!(&a_pl_b);
}