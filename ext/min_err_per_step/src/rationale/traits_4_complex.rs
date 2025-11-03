use once_cell::sync::Lazy;
use rug::float::Round;
use rug::ops::{AddAssignRound, DivAssignRound, MulAssignRound, 
    PowAssign as rugPowAssign, PowAssignRound, SubAssignRound, Pow as rugpow, CompleteRound };
use rug::{Assign, Integer as rugint, float::Constant as rugconst, Float as rugfloat, ops::SubFrom, Complete};
use rug::float::Constant;
use crate::base::{glob_precision, Pi };
use crate::nth_root::__2rt;
use num_complex::Complex as _complex;
use std::ops::{Add, Div, DivAssign, Mul, MulAssign};
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
    pub fn init_rugfloat (_0: &rugfloat, _1: &rugfloat) -> Self {
        return __init_rugfloat (_0, _1 )
    }
    pub fn init_jrugfloat ( j: &rugfloat) -> Self {
        return __init_jrugfloat ( j )
    }
    pub fn _z (&self) -> Self {
        return __z (self)
    }
    pub fn cmp_less (&self, other: &Cu_Complex) -> bool {
        return __simple_cmp_less (self, other)
    }
    pub fn cmp_jless (&self, other: &Cu_Complex) -> bool {
        return __simple_cmp_jless (self, other)
    }
    pub fn abs (&self) -> Self {
        return __abs (self)
    }
    pub fn radius (&self) -> rugfloat {
        return __radius (self)
    } 
}
pub fn __radius (z: &Cu_Complex) -> rugfloat {
    let prec = glob_precision (None);
    return __2rt (&z.radius2 (), prec )
}
pub fn __abs (z: &Cu_Complex) -> Cu_Complex {
    return Cu_Complex {
        0: z.0.clone().abs(),
        1: z.1.clone().abs()
    }
}
pub fn __init_rugfloat (_0: &rugfloat, _1: &rugfloat ) -> Cu_Complex {
        return Cu_Complex {
            0: _0.clone(),
            1: _1.clone()
        }
}
pub fn __init_jrugfloat (j: &rugfloat ) -> Cu_Complex {
        let prec = j.prec_64();
        return Cu_Complex {
            0: rugfloat::with_val_64 (prec, 0),
            1: j.clone()
        }
}
pub fn __simple_cmp_less (left: &Cu_Complex, right: &Cu_Complex) -> bool {
    if left.0 < right.0 && left.1 < right.1 { return true }
    return false
}
pub fn __simple_cmp_jless (left: &Cu_Complex, right: &Cu_Complex) -> bool {
    if left.1 < right.1 { return true }
    return false
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
impl std::ops::Sub for Cu_Complex {
    type Output = Cu_Complex;
    fn sub (self, other: Cu_Complex ) -> Cu_Complex {
        return Cu_Complex {
            0: self.0.clone() - other.0.clone(),
            1: self.1.clone() - other.1.clone()
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
impl Div <&rugfloat> for Cu_Complex {
    type Output = Cu_Complex;
    fn div (self, other: &rugfloat ) -> Cu_Complex {
        return Cu_Complex {
            0: self.0 / other.clone(),
            1: self.1 / other.clone()
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
impl <T> Mul <T> for Cu_Complex where 
    T: Copy,
    rugfloat: Mul <T,  Output = rugfloat> {
    type Output = Cu_Complex;
    fn mul (self, other: T ) -> Cu_Complex {
        return Cu_Complex {
            0: self.0.clone() * other,
            1: self.1.clone() * other
        }
    }
}
macro_rules! impl_mul_for_cu_complex {
    ($($t:ty),*) => {
        $(
            impl std::ops::Mul<Cu_Complex> for $t {
                type Output = Cu_Complex;
                fn mul(self, other: Cu_Complex) -> Cu_Complex {
                    Cu_Complex(
                        other.0.clone() * self,
                        other.1.clone() * self,
                    )
                }
            }
        )*
    };
}

// Usage
impl_mul_for_cu_complex!(u64, f64, i32, u32, f32, i64);

impl MulAssign for Cu_Complex {
    fn mul_assign (&mut self, other: Cu_Complex ) {
        self.0 = self.0.clone() * other.0.clone() - self.1.clone() * other.1.clone();
        self.1 = self.1.clone() * other.0.clone() + self.0.clone() * other.1.clone();
    }
}
impl <T>MulAssign <T> for Cu_Complex where 
    rugfloat: MulAssign<T>,
    T: Copy {
    fn mul_assign (&mut self, other: T ) {
        self.0 *= other;
        self.1 *= other;
    }
}
pub trait Cu_Complex_Pow {
    fn pow_u64 (&self, exp: u64) -> Self;
}
impl Cu_Complex_Pow for Cu_Complex {
    fn pow_u64 (&self, exp: u64) -> Self {
       // dbg!(&exp);
        let mut exp = exp;
        let mut ret = Cu_Complex::init_f64 (1.0, 0.0);
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