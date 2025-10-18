use once_cell::sync::Lazy;
use rug::float::Round;
use rug::ops::{
    AddAssignRound,
    DivAssignRound,
    MulAssignRound,
    PowAssign as rugPowAssign,
    PowAssignRound,
    SubAssignRound,
    Pow as rugpow,
    CompleteRound,
 };
use rug::{Assign, Integer as rugint, float::Constant as rugconst, Float as rugfloat, ops::SubFrom, Complete};
use rug::float::Constant;
use std::error::Error;
use min_err_per_step::nth_root::__2rt;
use min_err_per_step::base::glob_precision;
use Mademoiselle_Entropia::custom_traits::helpful_math_ops;
use Mademoiselle_Entropia::minio::InterruptMsg;
#[derive(Clone, Debug)]
pub enum manage_primes {
    get (usize),
    lst_size,
    out_usize (usize),
    out_u64 (u64),
    null
}
pub fn faav_primes (cmd: &manage_primes) -> manage_primes {
    static mut lst: Lazy < Vec <u64> > = Lazy::new (|| {
        vec![2, 3, 5, 7, 11, 13,
             17, 19, 23, 29, 31,
             37, 41, 43, 47, 53,
             59, 61, 67, 71, 73,
             79, 83, 89, 97, 101, 103]
    });
    unsafe {
        match *cmd {
            manage_primes::get ( x ) => { return manage_primes::out_u64 ( lst [x] ) },
            manage_primes::lst_size => {return manage_primes::out_usize (lst.len() )},
            _ => { return manage_primes::null }
        }
    }

}
pub fn faav_shift (shift: Option <usize> ) -> usize {
    static mut sh: usize = 0;
    unsafe {
        if let Some ( x ) = shift { sh = x; }
        return sh.clone()
    }
}
#[derive(Clone, Debug)]
pub struct banu {
    pub n: rugfloat,
    pub tail: rugfloat
}
pub fn gen_backdoor_numero (n: &rugfloat, tail: &rugfloat) -> banu {
    let mut new_tail: rugfloat = __2rt (tail, glob_precision (None) )
        << faav_shift (None);
    let save_new_tail = new_tail.clone();
    new_tail = new_tail.floor();
    let mut new_n: rugfloat = n.clone() << faav_shift ( None );
    new_n += new_tail; 
    return banu {
        n: new_n,
        tail: save_new_tail
    }
}
