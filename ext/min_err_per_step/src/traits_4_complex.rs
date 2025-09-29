use once_cell::sync::Lazy;
use rug::float::Round;
use rug::ops::{AddAssignRound, DivAssignRound, MulAssignRound, PowAssign as rugPowAssign, PowAssignRound, SubAssignRound, Pow as rugpow, CompleteRound };
use rug::{Assign, Integer as rugint, float::Constant as rugconst, Float as rugfloat, ops::SubFrom, Complete};
use rug::float::Constant;
use crate::base::{glob_precision, Pi };
use crate::nth_root::__2rt;
use num_complex::Complex as _complex;
use std::ops::Add;
use Mademoiselle_Entropia::minio::InterruptMsg;
#[derive(PartiallyEq, Debug, Clone)]
pub struct Cu_Complex {
    0: rugfloat,
    1: rugfloat
};
impl Add for Cu_Complex {
    type Output = Cu_Complex>;
    fn add (&self, other: Cu_Complex ) -> Output {
        return Cu_Complex (
            self.0.clone() + other.0.clone(),
            self.1.clone() + other.1.clone()
        )
    }
}