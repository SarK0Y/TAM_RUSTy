use once_cell::sync::Lazy;
use rug::float::Round;
use rug::ops::{AddAssignRound, DivAssignRound, MulAssignRound, 
    PowAssign as rugPowAssign, PowAssignRound, SubAssignRound, Pow as rugpow, CompleteRound };
use rug::{Assign, Integer as rugint, float::Constant as rugconst, Float as rugfloat, ops::SubFrom, Complete};
use rug::float::Constant;
use crate::base::{glob_precision, Pi };
use crate::nth_root::__2rt;
use rug::rational::MiniRational;
use rug::Rational as rugq;
use num_complex::Complex as _complex;
use std::ops::{Add, Div, DivAssign, Mul, MulAssign};
use Mademoiselle_Entropia::minio::InterruptMsg;
#[derive(PartialEq, Debug, Clone)]
pub struct Q_Complex (
    pub rugq,
    pub rugq
);
impl Q_Complex {
    /*pub fn init_rugfloat (_0: &rugfloat, _1: &rugfloat) -> Self {
        return __init_rugfloat (_0, _1 )
    }*/
    pub fn _z (&self) -> Self {
        return __z (self)
    }
    pub fn cmp_less (&self, other: &Q_Complex) -> bool {
        return __simple_cmp_less (self, other)
    }
    pub fn cmp_jless (&self, other: &Q_Complex) -> bool {
        return __simple_cmp_jless (self, other)
    }
    pub fn abs (&self) -> Self {
        return __abs (self)
    }
    pub fn radius (&self) -> rugfloat {
        return __radius (self)
    } 
}
pub fn __radius (z: &Q_Complex) -> rugfloat {
    let prec = glob_precision (None);
    return __2rt (&z.radius2 (), prec )
}
pub fn __abs (z: &Q_Complex) -> Q_Complex {
    return Q_Complex {
        0: z.0.clone().abs(),
        1: z.1.clone().abs()
    }
}
/* pub fn __init_rugfloat (_0: &rugfloat, _1: &rugfloat ) -> Q_Complex {
        return Q_Complex {
            0: _0.clone(),
            1: _1.clone()
        }
}*/
pub fn __init_jrugq (j: &rugq ) -> Q_Complex {
        return Q_Complex {
            0: rugq::from ( (0, 1) ),
            1: j.clone()
        }
}
pub fn __simple_cmp_less (left: &Q_Complex, right: &Q_Complex) -> bool {
    if left.0 < right.0 && left.1 < right.1 { return true }
    return false
}
pub fn __simple_cmp_jless (left: &Q_Complex, right: &Q_Complex) -> bool {
    if left.1 < right.1 { return true }
    return false
}
pub fn __z (cmplx: &Q_Complex) -> Q_Complex {
    return Q_Complex (
        cmplx.0.clone(),
        cmplx.1.clone() * -1
    )
}
pub trait basic_fnx_4_complex {
    fn radius2 (&self) -> rugq;
}
impl basic_fnx_4_complex for Q_Complex {
    fn radius2 (&self) -> rugq {
        return self.0.clone().pow(2) + self.1.clone().pow(2)
    }
}
impl Add for Q_Complex {
    type Output = Q_Complex;
    fn add (self, other: Q_Complex ) -> Q_Complex {
        return Q_Complex {
            0: self.0.clone() + other.0.clone(),
            1: self.1.clone() + other.1.clone()
        }
    }
}
impl std::ops::Sub for Q_Complex {
    type Output = Q_Complex;
    fn sub (self, other: Q_Complex ) -> Q_Complex {
        return Q_Complex {
            0: self.0.clone() - other.0.clone(),
            1: self.1.clone() - other.1.clone()
        }
    }
}
impl Div for Q_Complex {
    type Output = Q_Complex;
    fn div (self, other: Q_Complex ) -> Q_Complex {
        let r2 = other.radius2();
        let mut ret = self.clone() * other._z(); 
        return Q_Complex {
            0: ret.0 / r2.clone(),
            1: ret.1 / r2
        }
    }
}
impl Div <&rugq> for Q_Complex {
    type Output = Q_Complex;
    fn div (self, other: &rugq ) -> Q_Complex {
        return Q_Complex {
            0: self.0 / other.clone(),
            1: self.1 / other.clone()
        }
    }
}
impl Div <Q_Complex> for rugq {
    type Output = Q_Complex;
    fn div (self, other: Q_Complex ) -> Q_Complex {
        let r2 = other.radius2();
        let mut ret = Q_Complex { 
            0: self.clone(),
            1: rugq::from ( (0, 1) ) } * other._z(); 
        return Q_Complex {
            0: ret.0 / r2.clone(),
            1: ret.1 / r2
        }
    }
}
impl Div <&Q_Complex> for rugq {
    type Output = Q_Complex;
    fn div (self, other: &Q_Complex ) -> Q_Complex {
        let r2 = other.radius2();
        let mut ret = Q_Complex { 
            0: self.clone(),
            1: rugq::from ( (0, 1) ) } * other._z(); 
        return Q_Complex {
            0: ret.0 / r2.clone(),
            1: ret.1 / r2
        }
    }
}
impl Mul for Q_Complex {
    type Output = Q_Complex;
    fn mul (self, other: Q_Complex ) -> Q_Complex {
        return Q_Complex {
            0: self.0.clone() * other.0.clone() - self.1.clone() * other.1.clone(),
            1: self.1.clone() * other.0.clone() + self.0.clone() * other.1.clone()
        }
    }
}
impl <T> Mul <T> for Q_Complex where 
    T: Copy,
    rugfloat: Mul <T,  Output = rugfloat> {
    type Output = Q_Complex;
    fn mul (self, other: T ) -> Q_Complex {
        return Q_Complex {
            0: self.0.clone() * other,
            1: self.1.clone() * other
        }
    }
}
macro_rules! impl_mul_for_Q_Complex {
    ($($t:ty),*) => {
        $(
            impl std::ops::Mul<Q_Complex> for $t {
                type Output = Q_Complex;
                fn mul(self, other: Q_Complex) -> Q_Complex {
                    Q_Complex(
                        other.0.clone() * self,
                        other.1.clone() * self,
                    )
                }
            }
        )*
    };
}

// Usage
impl_mul_for_Q_Complex!(u64, f64, i32, u32, f32, i64);

impl MulAssign for Q_Complex {
    fn mul_assign (&mut self, other: Q_Complex ) {
        self.0 = self.0.clone() * other.0.clone() - self.1.clone() * other.1.clone();
        self.1 = self.1.clone() * other.0.clone() + self.0.clone() * other.1.clone();
    }
}
impl <T>MulAssign <T> for Q_Complex where 
    rugfloat: MulAssign<T>,
    T: Copy {
    fn mul_assign (&mut self, other: T ) {
        self.0 *= other;
        self.1 *= other;
    }
}
pub trait Q_Complex_Pow {
    fn pow_u64 (&self, exp: u64) -> Self;
}
impl Q_Complex_Pow for Q_Complex {
    fn pow_u64 (&self, exp: u64) -> Self {
       // dbg!(&exp);
        let mut exp = exp;
        let mut ret = Q_Complex::init_f64 (1.0, 0.0);
        let mut sq = self.clone();
        while exp > 0 {
 //           dbg!(&exp);
            if exp & 1 == 1 {
                ret *= sq.clone();
   //             dbg! (&ret);
            } sq *= sq.clone();
     //       dbg! (&sq);
            exp /= 2;
        } ret
    }
}