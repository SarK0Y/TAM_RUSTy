use once_cell::sync::Lazy;
use rug::float::Round;
use rug::ops::{AddAssignRound, DivAssignRound, MulAssignRound, PowAssign as rugPowAssign, PowAssignRound, SubAssignRound, Pow as rugpow, CompleteRound };
use rug::{Assign, Integer as rugint, float::Constant as rugconst, Float as rugfloat, ops::SubFrom, Complete};
use rug::float::Constant;
use crate::base::{glob_precision, Pi };
use crate::nth_root::__2rt;
use crate::complex::traits;
use crate::complex::traits::Cu_Complex;
use num_complex::Complex as _complex;
use Mademoiselle_Entropia::minio::InterruptMsg;
use num::complex::Complex64;
use std::f64;
fn solve_system_complex(A: Complex64, B: Complex64) -> Vec<(Complex64, Complex64)> {
    let mut solutions = Vec::new();
    
    // Calculate discriminant: B² + 4A²
    let discriminant = B * B + Complex64::new(4.0, 0.0) * A * A;
    
    // Calculate both roots for a²
    let sqrt_discriminant = discriminant.sqrt();
    
    let a_squared1 = (B + sqrt_discriminant) / Complex64::new(2.0, 0.0);
    let a_squared2 = (B - sqrt_discriminant) / Complex64::new(2.0, 0.0);
    
    // For each a², calculate both square roots to get a
    let a_values1 = complex_sqrt(a_squared1);
    let a_values2 = complex_sqrt(a_squared2);
    
    // Generate all solutions
    for &a in &a_values1 {
        if a != Complex64::new(0.0, 0.0) {
            let b = A / a;
            solutions.push((a, b));
        }
    }
    
    for &a in &a_values2 {
        if a != Complex64::new(0.0, 0.0) {
            let b = A / a;
            solutions.push((a, b));
        }
    }
    
    // Handle special case when A = 0
    if A == Complex64::new(0.0, 0.0) {
        // When A = 0, ab = 0, so either a=0 or b=0
        // From a² - b² = B:
        // If a=0: -b² = B => b = ±√(-B)
        // If b=0: a² = B => a = ±√B
        
        let sqrt_b = complex_sqrt(B);
        let sqrt_neg_b = complex_sqrt(-B);
        
        for &a in &sqrt_b {
            solutions.push((a, Complex64::new(0.0, 0.0)));
        }
        
        for &b in &sqrt_neg_b {
            solutions.push((Complex64::new(0.0, 0.0), b));
        }
    }
    
    // Remove duplicates
    solutions.dedup_by(|a, b| {
        (a.0 - b.0).norm() < 1e-10 && (a.1 - b.1).norm() < 1e-10
    });
    
    solutions
}

fn complex_sqrt(z: Complex64) -> Vec<Complex64> {
    let r = z.norm().sqrt();
    let theta = z.arg() / 2.0;
    
    let root1 = Complex64::from_polar(r, theta);
    let root2 = Complex64::from_polar(r, theta + f64::consts::PI);
    
    vec![root1, root2]
}

// Real-only version for comparison
fn solve_system_real(A: f64, B: f64) -> Option<(f64, f64)> {
    let discriminant = B * B + 4.0 * A * A;
    
    if discriminant < 0.0 {
        return None;
    }
    
    let sqrt_discriminant = discriminant.sqrt();
    let a_squared = (B + sqrt_discriminant) / 2.0;
    
    if a_squared < 0.0 {
        return None;
    }
    
    let a = a_squared.sqrt();
    
    if a == 0.0 {
        if A == 0.0 {
            return Some((0.0, 0.0));
        } else {
            return None;
        }
    }
    
    let b = A / a;
    Some((a, b))
}
#[derive(PartialEq, Debug, Clone)]
pub struct complex_roots {
    pub root0: Cu_Complex,
    pub root1: Cu_Complex
}
pub fn sqrt_2 () -> rugfloat {
    static mut _const: Lazy <rugfloat> = Lazy::new(|| {
        let PREC0_ = glob_precision (None);
        rugfloat::with_val_64 (PREC0_, 1)    
    });
    static mut _1st: bool = true;
    unsafe {
        if _1st {
            let PREC0_ = glob_precision (None);
            *_const = __2rt(&rugfloat::with_val_64 (PREC0_, 2), PREC0_);
            _1st = false;
        } return _const.clone ()
    }
}
pub fn sqrt_05 () -> rugfloat {
    static mut _const: Lazy <rugfloat> = Lazy::new(|| {
        let PREC0_ = glob_precision (None);
        rugfloat::with_val_64 (PREC0_, 1)    
    });
    static mut _1st: bool = true;
    unsafe {
        if _1st {
            let PREC0_ = glob_precision (None);
            *_const = __2rt(&rugfloat::with_val_64 (PREC0_, 0.5), PREC0_);
            _1st = false;
        } return _const.clone ()
    }
}
pub fn solve_system_for_isqrt(A: &rugfloat, B: &rugfloat) 
    -> Option<complex_roots> 
{
    if *A == rug::float::Special::Nan ||
       *B == rug::float::Special::Nan ||
       *A == rug::float::Special::Infinity ||
       *B == rug::float::Special::Infinity ||
       *A == rug::float::Special::NegInfinity ||
       *B == rug::float::Special::NegInfinity  {
        return None;
    }
    let PREC0_ = glob_precision (None);
    let a = rugfloat::with_val_64 (PREC0_, 1);
    let sqrt_i: Cu_Complex = Cu_Complex {0: sqrt_05(), 1: sqrt_05() };
    //let mut complex_a = _complex::new (a, a);
    let discriminant: rugfloat = B.clone() * B.clone() + 4 * A.clone() * A.clone();
    let sqrt_discriminant = __2rt (&discriminant, PREC0_);
    let a_squared0: rugfloat = (B.clone() + sqrt_discriminant.clone()) / 2.0;
    let a0: Cu_Complex = if a_squared0 < 0 {
        let tmp: rugfloat = a_squared0.clone() * -1;
        Cu_Complex{ 
            0: rugfloat::with_val_64 (PREC0_, 0), 
            1: __2rt (&tmp, PREC0_)
        }
    } else {
        Cu_Complex {
            0: __2rt (&a_squared0, PREC0_),
            1:  rugfloat::with_val_64 (PREC0_, 0)
        }
    };  
    let b0: Cu_Complex = A.clone() / a0.clone();
    let cmp0: rugfloat = b0.0.clone();
    let cmp1: rugfloat = b0.1.clone();
    let cmp_a0: rugfloat = a0.0.clone();
    let cmp_a1: rugfloat = a0.1.clone();
    let root0 = if cmp0 == 0 {
        Cu_Complex {
            0: cmp1 * -1,
            1: cmp_a1
        }
    } else {
           Cu_Complex {
            0: cmp_a0,
            1: cmp0
        }
    };
    let ret = complex_roots {
        root0: root0.clone(),
        root1: root0 * -1,
    };
    return Some(ret)
}
pub fn isqrt (cmplx: &Cu_Complex) -> Option <complex_roots> {
    let A = cmplx.1.clone() / 2;
    return solve_system_for_isqrt (&A, &cmplx.0)
}
pub fn dbg_solve_system_for_isqrt(A: &rugfloat, B: &rugfloat) 
    -> Option<complex_roots> 
{
    if *A == rug::float::Special::Nan ||
       *B == rug::float::Special::Nan ||
       *A == rug::float::Special::Infinity ||
       *B == rug::float::Special::Infinity ||
       *A == rug::float::Special::NegInfinity ||
       *B == rug::float::Special::NegInfinity  {
        return None;
    }
    let PREC0_ = glob_precision (None);
    let a = rugfloat::with_val_64 (PREC0_, 1);
    let sqrt_i: Cu_Complex = Cu_Complex {0: sqrt_05(), 1: sqrt_05() };
    //let mut complex_a = _complex::new (a, a);
    let discriminant: rugfloat = B.clone() * B.clone() + 4 * A.clone() * A.clone();
    let sqrt_discriminant = __2rt (&discriminant, PREC0_);
    dbg! (&sqrt_discriminant);
    let a_squared0: rugfloat = (B.clone() + sqrt_discriminant.clone()) / 2.0;
    let a_squared1: rugfloat = (B.clone() - sqrt_discriminant.clone()) / 2.0;
    dbg! (&a_squared1);
    let a0: Cu_Complex = if a_squared0 < 0 {
        let tmp: rugfloat = a_squared0.clone() * -1;
        Cu_Complex{ 
            0: rugfloat::with_val_64 (PREC0_, 0), 
            1: __2rt (&tmp, PREC0_)
        }
    } else {
        Cu_Complex {
            0: __2rt (&a_squared0, PREC0_),
            1:  rugfloat::with_val_64 (PREC0_, 0)
        }
    };  
        
    let a1: Cu_Complex = if a_squared1 < 0 {
        let tmp: rugfloat = a_squared1.clone() * -1;
        Cu_Complex{ 
            0: rugfloat::with_val_64 (PREC0_, 0), 
            1: __2rt (&tmp, PREC0_)
        }
    } else {
        Cu_Complex {
            0: __2rt (&a_squared1, PREC0_),
            1:  rugfloat::with_val_64 (PREC0_, 0)
        }
    };      
    let b0: Cu_Complex = A.clone() / a0.clone();
    let b1: Cu_Complex = A.clone() / a1.clone();
    let cmp0 = b0.0.clone();
    let cmp1 = b0.1.clone();
    let cmp_a0 = a0.0.clone();
    let cmp_a1 = a0.1.clone();
    let root0 = if cmp0 == 0 {
        Cu_Complex {
            0: cmp1 * -1,
            1: cmp_a1
        }
    } else {
           Cu_Complex {
            0: cmp_a0,
            1: cmp0
        }
    };
    let cmp0 = b1.0.clone();
    let cmp1 = b1.1.clone();
    let cmp_a0 = a1.0.clone();
    let cmp_a1 = a1.1.clone();
    let root1 = if cmp0 == 0 {
        Cu_Complex {
            0: cmp1 * -1,
            1: cmp_a1
        }
    } else {
           Cu_Complex {
            0: cmp_a0,
            1: cmp0
        }
    };
    let ret = complex_roots {
        root0,
        root1,
    };
    return Some(ret)
}
pub fn dbg_isqrt (cmplx: &Cu_Complex) -> Option <complex_roots> {
    let A = cmplx.1.clone() / 2;
    return dbg_solve_system_for_isqrt (&A, &cmplx.0)
}
