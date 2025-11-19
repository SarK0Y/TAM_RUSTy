use once_cell::sync::Lazy;
use rug::float::Round;
use rug::ops::{AddAssignRound, DivAssignRound, MulAssignRound, PowAssign as rugPowAssign, PowAssignRound, SubAssignRound, Pow as rugpow, CompleteRound };
use rug::{Assign, Integer as rugint, float::Constant as rugconst, Float as rugfloat, ops::SubFrom, Complete};
use rug::float::Constant;
use crate::base::{glob_precision, Pi };
use crate::nth_root::__2rt;
use crate::complex::traits;
use crate::complex::nth_root::isqrt;
use crate::Rationale::nth_root_q_complex::isqrt4Q;
use crate::Rationale::traits_4_q_complex::{Q_Complex, Q_Complex_Pow};
use Mademoiselle_Entropia::minio::InterruptMsg; 
/// [ for testing ]
pub fn fast_n_simple_isin3_4Q (x: &Q_Complex, err: u32 ) -> Q_Complex {
    let _3 = rugint::from (3);
    let mut start_x: Q_Complex = x.clone() / &_3.pow (err);
    dbg! (&start_x);
    let mut step: usize = 0;
    //sin_x = 2 * start_x.clone ();
    let mut sin_3x = Q_Complex::init_u64 (1, 0);
    sin_3x = start_x.clone();
    //dbg! (&sin_3x);
    while start_x.cmp_jless ( x ) {
        sin_3x = 3u64 * sin_3x.clone () - 4* sin_3x.clone ().pow_u64 (3);
//        dbg! (&sin_3x);
        start_x *= 3;
    }
    //dbg! (&sin_3x);
    return sin_3x
} 
pub fn dbg_fast_n_simple_isin3_4Q (x: &Q_Complex, err: u32 ) -> Q_Complex {
    let _3 = rugint::from (3);
    let mut start_x: Q_Complex = x.clone() / &_3.pow (err);
    dbg! (&start_x);
    let mut step: usize = 0;
    //sin_x = 2 * start_x.clone ();
    let mut sin_3x = start_x.clone();
    let mut sin_3x_3 = Q_Complex::init_u64 (1, 0);
    let mut sin_3x_pow3 = sin_3x_3.clone();
    //dbg! (&sin_3x);
    while start_x.cmp_jless ( x ) {
        sin_3x_3 = 3u64 * sin_3x.clone ();
        dbg! (&sin_3x_3);
        sin_3x_pow3 = 4* sin_3x.clone ().pow_u64 (3);
        dbg! (&sin_3x_pow3);
        sin_3x = sin_3x_3.clone() - sin_3x_pow3.clone();
        dbg! (&sin_3x);
        start_x *= 3;
    }
    //dbg! (&sin_3x);
    return sin_3x
} 
/// e^(-xj * j) == cos (-xj) - sin ( xj ) * j ... [set for testing] 
pub fn real_e2x_4Q (x: &Q_Complex, err: u32) -> Q_Complex {
    let sin: Q_Complex = fast_n_simple_isin3_4Q (x, err);
    let mut cos: Q_Complex = Q_Complex::init_u64(1, 0) - sin.pow_u64 (2);
    cos = isqrt4Q (&cos);
    let j = Q_Complex::init_u64 (0, 1);
    return cos - sin * j
}
/// e^(-xj * j) == cos (-xj) + sin ( xj ) * j ... [set for testing] 
pub fn _real_e2x_4Q (x: &Q_Complex, err: u32) -> Q_Complex {
    let sin: Q_Complex = fast_n_simple_isin3_4Q (x, err);
    let mut cos: Q_Complex = Q_Complex::init_u64(1, 0) - sin.pow_u64 (2);
    cos = isqrt4Q (&cos);
    let j = Q_Complex::init_u64 (0, 1);
    return cos + sin * j
}
/*pub fn dbg_real_e2x (x: &Cu_Complex, err: u64, cu_sin: fn (x: &Cu_Complex, err: u64 ) -> Cu_Complex) -> Option <Cu_Complex> {
    let sin: Cu_Complex = cu_sin (x, err);
    let mut cos: Cu_Complex = Cu_Complex::init_f64(1.0, 0.0) - sin.pow_u64 (2);
    let isqrt_roots = isqrt (&cos);
    if let Some (x) = isqrt_roots {
        let j = Cu_Complex::init_f64 (0.0, 1.0);
        cos = x.root0;
        return Some (cos - sin * j )
    } return None    
}*/