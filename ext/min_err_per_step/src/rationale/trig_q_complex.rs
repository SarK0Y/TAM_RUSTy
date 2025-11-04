use once_cell::sync::Lazy;
use rug::float::Round;
use rug::ops::{AddAssignRound, DivAssignRound, MulAssignRound, PowAssign as rugPowAssign, PowAssignRound, SubAssignRound, Pow as rugpow, CompleteRound };
use rug::{Assign, Integer as rugint, float::Constant as rugconst, Float as rugfloat, ops::SubFrom, Complete};
use rug::float::Constant;
use crate::base::{glob_precision, Pi };
use crate::nth_root::__2rt;
use crate::complex::traits;
use crate::complex::nth_root::isqrt;
use crate::Rationale::traits_4_q_complex::{Q_Complex, Q_Complex_Pow};
use Mademoiselle_Entropia::minio::InterruptMsg; 
/// [ for testing ]
pub fn fast_n_simple_isin3 (x: &Q_Complex, err: u64 ) -> Q_Complex {
    let _3 = rugint::from (3);
    let mut start_x: Q_Complex = x.clone() / &_3.pow (err);
    //dbg! (&start_x);
    let mut step: usize = 0;
    //sin_x = 2 * start_x.clone ();
    let mut sin_3x = Q_Complex::init_u64 (1, 0);
    sin_3x = start_x.clone();
    //dbg! (&sin_3x);
    while start_x.cmp_jless ( x ) {
        sin_3x = 3u64 * sin_3x.clone () - 4* sin_3x.clone ().pow_u64 (3);
        start_x *= 3;
    }
    //dbg! (&sin_3x);
    return sin_3x
} 
pub fn gen_prec_for_three () -> u64 {
    let prec: f64 = glob_precision (None) as f64 / 1.6;
    return prec as u64
}
/// e^(-xj * j) == cos (-xj) - sin ( xj ) * j ... [set for testing] 
pub fn real_e2x (x: &Q_Complex, err: u64) -> Option <Q_Complex> {
    let sin: Q_Complex = fast_n_simple_isin3 (x, err);
    let mut cos: Q_Complex = Q_Complex::init_u64(1, 0) - sin.pow_u64 (2);
    let isqrt_roots = isqrt (&cos);
    if let Some (x) = isqrt_roots {
        let j = Q_Complex::init_u64 (0, 1);
        cos = x.root0;
        return Some (cos - sin * j )
    } return None    
}
/// e^(-xj * j) == cos (-xj) + sin ( xj ) * j ... [set for testing] 
pub fn _real_e2x (x: &Q_Complex, err: u64) -> Option <Q_Complex> {
    let sin: Q_Complex = fast_n_simple_isin3 (x, err);
    let mut cos: Q_Complex = Q_Complex::init_u64(1, 0) - sin.pow_u64 (2);
    let isqrt_roots = isqrt (&cos);
    if let Some (x) = isqrt_roots {
        let j = Q_Complex::init_u64 (0, 1);
        cos = x.root0;
        return Some (cos + sin * j )
    } return None    
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