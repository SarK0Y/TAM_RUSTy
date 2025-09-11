use rug::float::Round;
use rug::ops::{AddAssignRound, DivAssignRound, MulAssignRound, PowAssign as rugPowAssign, PowAssignRound, SubAssignRound, Pow as rugpow};
use rug::{Assign, Integer as rugint, float::Constant as rugconst, Float as rugfloat, ops::SubFrom};
use rug::float::Constant;
use crate::base::{glob_precision, Pi };
use crate::nth_root::__2rt;
use Mademoiselle_Entropia::minio::InterruptMsg;
pub fn fast_n_simple_sin3 (x: &rugfloat, err: u64 ) -> rugfloat {
    let PREC0 = glob_precision (None);
    let _3 = rugfloat::with_val_64 (PREC0, 3);
    let _1 = rugfloat::with_val_64 (PREC0, 1);
    let mut start_x: rugfloat = x / _3.pow (err);
    dbg! (&start_x);
    let mut step: usize = 0;
    //sin_x = 2 * start_x.clone ();
    let mut sin_3x: rugfloat = _1.clone ();
    sin_3x = start_x.clone();
    dbg! (&sin_3x);
    while start_x < *x {
        sin_3x = 3 * sin_3x.clone () - 4* sin_3x.clone ().pow (3);
        start_x *= 3;
    }
    dbg! (&sin_3x);
    return sin_3x
} 
pub fn gen_prec_for_three () -> u64 {
    let prec: f64 = glob_precision (None) as f64 / 1.6;
    return prec as u64
}
pub trait Trig {
    fn __sin (&self) -> Self;
    fn __cos (&self) -> Self;
    fn sign_cos (&self) -> i8;
}
impl Trig for rugfloat {
    fn __sin (&self) -> Self {
        return fast_n_simple_sin3 (self, glob_precision (None) )
    }
    fn __cos (&self) -> Self {
        let PREC0 = glob_precision (None);
        let _1 = rugfloat::with_val_64 (PREC0, 1);
        let sign = Trig::sign_cos (self);
        let mut cos =  fast_n_simple_sin3 (self, gen_prec_for_three () );
        cos = _1 - cos.clone() * cos.clone();
        cos = __2rt ( &cos, glob_precision (None) );
        return sign * cos
    }
    fn sign_cos (&self) -> i8 {
        let pi = Pi ();
        let _2pi = pi.clone() * 2;
        let angle = self.clone() % _2pi;
        let mut quadrant = pi.clone () / 2;
        if angle <= quadrant {return  1 }
        quadrant = pi.clone();
        if angle <= quadrant {return -1 }
        quadrant = 1.5 * pi.clone ();
        if angle <= quadrant {return -1 }
        quadrant = 2 * pi;
        if angle <= quadrant {return  1 }
        return 1
    } 

}
pub trait Trig_w_local_prec {
    fn __sin (&self, prec: u64) -> Self;
    fn __cos (&self, prec: u64) -> Self;
    fn sign_cos (&self) -> i8;
}
impl Trig_w_local_prec for rugfloat {
    fn __sin (&self, prec: u64) -> Self {
        return fast_n_simple_sin3 (self, prec )
    }
    fn __cos (&self, PREC0: u64) -> Self {
        let _1 = rugfloat::with_val_64 (PREC0, 1);
        let sign = Trig_w_local_prec::sign_cos (self);
        let mut cos =  fast_n_simple_sin3 (self, gen_prec_for_three () );
        cos = _1 - cos.clone() * cos.clone();
        cos = __2rt ( &cos, glob_precision (None) );
        dbg! (&cos);
        return sign * cos
    }
    fn sign_cos (&self) -> i8 {
        let pi = Pi ();
        let _2pi = pi.clone() * 2;
        let angle = self.clone() % _2pi;
        let mut quadrant = pi.clone () / 2;
        if angle <= quadrant {return  1 }
        quadrant = pi.clone();
        if angle <= quadrant {return -1 }
        quadrant = 1.5 * pi.clone ();
        if angle <= quadrant {return -1 }
        quadrant = 2 * pi;
        if angle <= quadrant {return  1 }
        return 1
    } 
}