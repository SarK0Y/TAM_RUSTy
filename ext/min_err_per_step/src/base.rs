use once_cell::sync::Lazy;
use rug::float::Round;
use rug::ops::{AddAssignRound, DivAssignRound, MulAssignRound, PowAssign as rugPowAssign, PowAssignRound, SubAssignRound, Pow as rugpow};
use rug::{Assign, Integer as rugint, float::Constant as rugconst, Float as rugfloat, ops::SubFrom};
use rug::float::Constant;
use crate::frax::{continued_fraction_approximation, fast_n_dumb_shortcut_for_cfrac };
use Mademoiselle_Entropia::minio::InterruptMsg;
pub enum manage_prec {
    init (u64),
    partial_set (u64),
    get_exp,
    get_big_value,
    get_low_value,
    long_out (rugfloat),
    short_out (u64),
    none 
}
pub fn fast_n_simple_long_Pi (err: u64 ) -> rugfloat {
    let PREC0: u64 = glob_precision (None); 
    let _2 = rugfloat::with_val_64 (PREC0, 2);
    let _1 = rugfloat::with_val_64 (PREC0, 1);
    let err = _1.clone () / _2.clone().pow(err);
    let mut dx: rugfloat = _2.clone();
    dx = dx.sqrt ();
    //let mut x = 0.0f64;
    dx /= 2;
    let mut y: rugfloat = (_1.clone() - dx.clone().pow(2) );
    y = y.sqrt ();
    let mut dy: rugfloat = _1.clone() - y;
    dx = ( dx.clone().pow(2) + dy.clone().pow(2) );
    dx = dx.sqrt ();
    let mut num_of_pts: rugfloat = _1.clone() * 4;
    while dx > err {
        dx = dx / 2;
        y = (_1.clone() - dx.clone().pow (2) );
        y = y.sqrt ();
        dy = _1.clone() - y;
        dx = ( dx.clone().pow (2) + dy.clone().pow (2) );
        dx = dx.sqrt ();
        num_of_pts *= 2;
    } num_of_pts * dx
}
pub fn glob_precision (prec: Option <u64> ) -> u64{
    static mut sav: u64 = 512;
    unsafe {
        if let Some (x) = prec {
            sav = x;
            ctrl_glob_precision (manage_prec::partial_set (x));
        } return sav
    }
}
pub fn ctrl_glob_precision (cmd: manage_prec) -> manage_prec {
    static mut upper: Lazy < rugfloat > = Lazy::new (|| {
        rugfloat::with_val_64 (glob_precision (None), 1)
    });
    static mut bottom: Lazy < rugfloat > = Lazy::new (|| {
        rugfloat::with_val_64 (glob_precision (None), 1)
    });
    unsafe {
        match cmd {
            manage_prec::init (x) => {
                glob_precision (Some (x));
                let shr = glob_precision (None) as usize;
                *upper = rugfloat::with_val_64 (glob_precision (None), 2) << shr;
                *bottom = upper.clone().pow (-1);
                return manage_prec::none
            },
            manage_prec::partial_set ( x ) => {
                let shr = glob_precision (None) as usize;
                *upper = rugfloat::with_val_64 (glob_precision (None), 2) << shr;
                *bottom = upper.clone().pow (-1);
                return manage_prec::none
            },
            manage_prec::get_exp => {return manage_prec::short_out (glob_precision (None))},
            manage_prec::get_big_value => { return manage_prec::long_out (upper.clone() )},
            manage_prec::get_low_value => { return manage_prec::long_out (bottom.clone() )},
            _ => {}
           // manage_prec::
        }
    }
    return manage_prec::none
}
pub fn Pi () -> rugfloat {
    static mut pi: Lazy <rugfloat> = Lazy::new (|| {
        let PREC0 = glob_precision (None);
        rugfloat::with_val_64 (PREC0, 1)
    });
    static mut _1st_run: bool = true;
    static mut prec: u64 = 512;
    unsafe {
        let new_prec = glob_precision (None) - 1;
        if new_prec != prec || _1st_run {
            prec = new_prec;
            *pi = fast_n_simple_long_Pi (prec);
            _1st_run = false;
        } return pi.clone()
    }
}
pub fn re_fast_real_e (new_coef: &rugfloat, canceled_coef: &rugfloat, prec_: u64) -> rugfloat {
    let PREC0_ = prec_;
    let one = rugfloat::with_val_64(PREC0_, 1.0);
    let mut one_div_by = one.clone();
    let rm = Round::Down;
    let exponent: u64 = PREC0_ / 2;
    let mut const_e_base = rugfloat::with_val_64(PREC0_, 2.0);
    //const_e_base.pow_assign_round(exponent, rm);
    const_e_base.pow_assign(exponent); // = 2 ^ m
    //dbg! (&const_e_base);
    let mut big_exp = const_e_base.clone (); // = 2 ^ m
    big_exp *= rugfloat::with_val_64(PREC0_, new_coef.clone() ); // = new_coef * 2 ^ m
    const_e_base.mul_assign_round(rugfloat::with_val_64(PREC0_ , canceled_coef), rm);
    //dbg! (&const_e_base);
    one_div_by.div_assign_round(&const_e_base, rm);
    //dbg! (&const_e_base);
    const_e_base = one_div_by;
    const_e_base += one.clone();
    //dbg! (&big_exp);
    //dbg! (&canceled_coef);
    //dbg! (&new_coef);
    let mut const_e = const_e_base.clone();
    const_e.pow_assign( big_exp);
    //dbg! (&const_e_base );
    //dbg!(&const_e);
    if const_e == one {
        //dbg! ("bad variant");
        let prec_ = (PREC0_ as f64 + PREC0_  as f64 * 0.5) as u64;
            return re_fast_real_e(new_coef, canceled_coef, prec_);
    }
    return const_e//.clone()
}
pub fn ext_const_E (pow: &rugfloat) -> rugfloat {
    let sign: i8 = if *pow > 0 { 1 } else { -1 };
    let pow = pow.clone() * sign;
    let PREC0 = glob_precision (None);
  //  let max_terms = PREC0 as usize / 3;
    let (new_coef, mut canceled_coef) = fast_n_dumb_shortcut_for_cfrac (&pow);//continued_fraction_approximation (&pow, max_terms, PREC0);
    canceled_coef  *= sign;
    return re_fast_real_e (&new_coef, &canceled_coef, PREC0)
}
pub fn _ext_const_E (pow: &rugfloat) -> rugfloat {
    let sign: i8 = if *pow > 0 { 1 } else { -1 };
    let pow = pow.clone() * sign;
    let PREC0 = glob_precision (None);
    let max_terms = PREC0 as usize / 3;
    let (new_coef, canceled_coef) = fast_n_dumb_shortcut_for_cfrac (&pow); //continued_fraction_approximation (&pow, max_terms, PREC0);
    let new_coef = new_coef * sign;
    return re_fast_real_e_tst (&new_coef, &canceled_coef, PREC0)
}
pub fn re_fast_real_e_tst (new_coef: &rugfloat, canceled_coef: &rugfloat, prec_: u64) -> rugfloat {
    let PREC0_ = prec_;
    let one = rugfloat::with_val_64(PREC0_, 1.0);
    let mut one_div_by = one.clone();
    let rm = Round::Nearest;
    let exponent: u64 = PREC0_ / 2;
    let mut const_e_base = rugfloat::with_val_64(PREC0_, 2.0);
    //const_e_base.pow_assign_round(exponent, rm);
    const_e_base.pow_assign(exponent); // = 2 ^ m
    //dbg! (&const_e_base);
    let mut big_exp = const_e_base.clone () / 2; // = 2 ^ m--
    big_exp *= rugfloat::with_val_64(PREC0_, new_coef.clone() ); // = new_coef * 2 ^ m--
    const_e_base.mul_assign_round(rugfloat::with_val_64(PREC0_ , canceled_coef), rm);
    //dbg! (&const_e_base);
    one_div_by.div_assign_round(&const_e_base, rm);
    //dbg! (&const_e_base);
    let mut _1_pl_22negX = one_div_by;
    let mut _1_m_22negativeX = _1_pl_22negX.clone();
    _1_pl_22negX += one.clone();
    _1_m_22negativeX = _1_pl_22negX.clone() - _1_m_22negativeX * 2;
    //dbg! (&big_exp);
    //dbg! (&canceled_coef);
    //dbg! (&new_coef);
    let mut const_e = _1_pl_22negX / _1_m_22negativeX;
    const_e.pow_assign( big_exp);
    //dbg! (&const_e_base );
    //dbg!(&const_e);
    if const_e == one {
        //dbg! ("bad variant");
        let prec_ = (PREC0_ as f64 + PREC0_  as f64 * 0.5) as u64;
            return re_fast_real_e(new_coef, canceled_coef, prec_);
    }
    return const_e//.clone()
}
// http://www.brotherstechnology.com/docs/Closed-Form_Approximations_(MI-1998-12).pdf
// https://www.mpfr.org/algorithms.pdf
//fn