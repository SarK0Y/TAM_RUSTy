use once_cell::sync::Lazy;
use rug::float::Round;
use rug::ops::{AddAssignRound, DivAssignRound, MulAssignRound, PowAssign as rugPowAssign, PowAssignRound, SubAssignRound, Pow as rugpow, CompleteRound };
use rug::{Assign, Integer as rugint, float::Constant as rugconst, Float as rugfloat, ops::SubFrom, Complete};
use rug::float::Constant;
use crate::Rationale::basic::__2rt4Q;
use rug::rational::MiniRational;
use rug::Rational as rugq;
use crate::base::{glob_precision, Pi };
use crate::nth_root::__2rt;
use crate::complex::traits;
use crate::complex::traits::Cu_Complex;
use crate::Rationale::traits_4_q_complex::{Q_Complex, Q_Complex_Pow};
use num_complex::Complex as _complex;
use Mademoiselle_Entropia::minio::InterruptMsg;
use num::complex::Complex64;
use std::f64;
pub fn solve_system_for_isqrt4Q(x: &Q_Complex) 
    -> Q_Complex 
{
    let A = &x.0;
    let B = &x.1;
    let discriminant: rugq = B.clone() * B.clone() + 4 * A.clone() * A.clone();
    let sqrt_discriminant = __2rt4Q (&discriminant);
    let a_squared0: rugq = (B.clone() + sqrt_discriminant.clone()) / 2;
    let a0: Q_Complex = if a_squared0 < 0 {
        let tmp: rugq = a_squared0.clone() * -1;
        Q_Complex{ 
            0: rugq::from ( (0, 1) ), 
            1: __2rt4Q (&tmp)
        }
    } else {
        Q_Complex {
            0: __2rt4Q (&a_squared0),
            1: rugq::from ( (0, 1) )
        }
    };  
    let b0: Q_Complex = A.clone() / (2 * a0.clone() );
    let cmp0: rugq = b0.0.clone();
    let cmp1: rugq = b0.1.clone();
    let cmp_a0: rugq = a0.0.clone();
    let cmp_a1: rugq = a0.1.clone();
    let root0 = if cmp0 == 0 {
        Q_Complex {
            0: cmp1 * -1,
            1: cmp_a1
        }
    } else {
        Q_Complex {
            0: cmp_a0,
            1: cmp0
        }
    };
    return root0
}
pub fn isqrt4Q (cmplx: &Q_Complex) -> Q_Complex {
    return solve_system_for_isqrt4Q ( cmplx )
}