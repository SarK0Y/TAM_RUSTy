use once_cell::sync::Lazy;
use rug::float::Round;
use rug::ops::{AddAssignRound, DivAssignRound, MulAssignRound, PowAssign as rugPowAssign, PowAssignRound, SubAssignRound, Pow as rugpow, CompleteRound };
use rug::{Assign, Integer as rugint, float::Constant as rugconst, Float as rugfloat, ops::SubFrom, Complete};
use rug::float::Constant;
use crate::base::{glob_precision, Pi };
use crate::nth_root::__2rt;
use num_complex::Complex as _complex;
use std::ops::{Add, Div, DivAssign, Mul};
use Mademoiselle_Entropia::minio::InterruptMsg;
#[derive(PartialEq, Debug, Clone)]
pub struct Cu_Complex (
    pub rugfloat,
    pub rugfloat
);
impl Cu_Complex {
   pub fn init_f64 (_0: f64, _1: f64) -> Self {
        let PREC0_ = glob_precision (None);
        return Self {
            0: rugfloat::with_val_64 (PREC0_, _0),
            1: rugfloat::with_val_64 (PREC0_, _1),
        }
    }
    pub fn _z (&self) -> Self {
        return __z (self)
    }
}
pub fn __z (cmplx: &Cu_Complex) -> Cu_Complex {
    return Cu_Complex (
        cmplx.0.clone(),
        cmplx.1.clone() * -1
    )
}
pub trait basic_fnx_4_complex {
    fn radius2 (&self) -> rugfloat;
}
impl basic_fnx_4_complex for Cu_Complex {
    fn radius2 (&self) -> rugfloat {
        return self.0.clone().pow(2) + self.1.clone().pow(2)
    }
}
impl Add for Cu_Complex {
    type Output = Cu_Complex;
    fn add (self, other: Cu_Complex ) -> Cu_Complex {
        return Cu_Complex {
            0: self.0.clone() + other.0.clone(),
            1: self.1.clone() + other.1.clone()
        }
    }
}
impl Div for Cu_Complex {
    type Output = Cu_Complex;
    fn div (self, other: Cu_Complex ) -> Cu_Complex {
        let r2 = other.radius2();
        let mut ret = self.clone() * other; 
        return Cu_Complex {
            0: ret.0 / r2.clone(),
            1: ret.1 / r2
        }
    }
}
impl Div <Cu_Complex> for rugfloat {
    type Output = Cu_Complex;
    fn div (self, other: Cu_Complex ) -> Cu_Complex {
        let r2 = other.radius2();
        let mut ret = Cu_Complex { 
            0: self.clone(),
            1: rugfloat::with_val (2, 0) } * other._z(); 
        return Cu_Complex {
            0: ret.0 / r2.clone(),
            1: ret.1 / r2
        }
    }
}
impl Div <&Cu_Complex> for rugfloat {
    type Output = Cu_Complex;
    fn div (self, other: &Cu_Complex ) -> Cu_Complex {
        let r2 = other.radius2();
        let mut ret = Cu_Complex { 
            0: self.clone(),
            1: rugfloat::with_val (2, 0) } * other._z(); 
        return Cu_Complex {
            0: ret.0 / r2.clone(),
            1: ret.1 / r2
        }
    }
}
impl Mul for Cu_Complex {
    type Output = Cu_Complex;
    fn mul (self, other: Cu_Complex ) -> Cu_Complex {
        return Cu_Complex {
            0: self.0.clone() * other.0.clone() - self.1.clone() * other.1.clone(),
            1: self.1.clone() * other.0.clone() + self.0.clone() * other.1.clone()
        }
    }
}
type T = i64;
impl Mul<T> for Cu_Complex {
    type Output = Cu_Complex;
    fn mul (self, other: T ) -> Cu_Complex {
        return Cu_Complex {
            0: self.0.clone() * other,
            1: self.1.clone() * other
        }
    }
}