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
        vec![997, 991, 983, 977, 971, 967, 953, 947, 941, 937, 929, 919, 911, 907, 887, 883, 881,
             877, 863, 859, 857, 853, 839, 829, 827, 823, 821, 811, 809, 797, 787, 773, 769, 761,
             757, 751, 743, 739, 733, 727, 719, 709, 701, 691, 683, 677, 673, 661, 659, 653, 647,
             643, 641, 631, 619, 617, 613, 607, 601, 599, 593, 587, 577, 571, 569, 563, 557, 547,
             541, 523, 521, 509, 503, 499, 491, 487, 479, 467, 463, 461, 457, 449, 443, 439, 433,
             431, 421, 419, 409, 401, 397, 389, 383, 379, 373, 367, 359, 353, 349, 347, 337, 331,
             317, 313, 311, 307, 293, 283, 281, 277, 271, 269, 263, 257, 251, 241, 239, 233, 229,
             227, 223, 211, 199, 197, 193, 191, 181, 179, 173, 167, 163, 157, 151, 149, 139, 137,
             131, 127, 113, 109, 107, 103, 101, 97, 89, 83, 79, 73, 71, 67, 61, 59, 53, 47, 43, 41,
             37, 31, 29, 23, 19, 17, 13, 11, 7, 5, 3, 2]
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
    pub maybe_P: rugfloat,
    pub maybe_Q: rugfloat
}
pub fn gen_backdoor_numero (n_minus_1: &rugfloat, tail: &mut rugfloat) -> rugfloat {
    *tail = __2rt (tail, glob_precision (None) ) << faav_shift (None);
    let cut_tail = tail.clone().floor();
    let mut new_n: rugfloat = n_minus_1.clone() << faav_shift ( None );
    new_n += cut_tail; 
    return new_n
}
pub fn try_banu (tst: &mut rugfloat) -> Option <Vec < (u64, u64 ) > > {
    static mut prime_lst_len: Lazy <usize > =  Lazy::new (|| {
        match faav_primes (&manage_primes::lst_size) {
            manage_primes::out_usize (x) => {x},
        _ => { 0 }
    } });
unsafe {
    let mut count_factors: Vec < (u64, u64) > = Vec::with_capacity ( *prime_lst_len);
    for i in 0..*prime_lst_len {
        let rec: u64 =  match faav_primes (&manage_primes::get (i) ) {
            manage_primes::out_u64 (x) => {x},
           _ => { return None }
        };
       if tst.clone() % rec == 0 {
            count_factors.push ( (rec, 1) );
            *tst /= rec;
            while tst.clone() % rec == 0 {
                *tst /= rec;
                let len = count_factors.len();
                count_factors [len - 1].1 += 1;
            }
        }
    }
    if *tst > 1 { return None }
    return Some (count_factors )
}
}
pub fn try_restore_factors (lst: Vec <u64>) -> banu {
    let mut _1 = rugfloat::with_val_64 (glob_precision (None), 1);
    let mut maybe_P = _1.clone ();
    let mut maybe_Q = _1.clone ();
    for p in lst {
        if maybe_P > maybe_Q { maybe_Q *= p; }
        else {maybe_P *= p}
    }
    return banu {
        maybe_P,
        maybe_Q
    }
}

//fn
/*
    RSA-576 = 1881988129206079638386972394616504398071635633794173827007633564
    22988859715234665485319060606504743045317388011303396716199692321205734031879550656996221305168759307650257059

    RSA-576 = 3980750864240649373971255005503864911990643623425267084063851895759463889572
          61768583317
        × 4727721461074353025362230719730482246329146953020971164598521711305207112563
          63590397527
https://en.wikipedia.org/wiki/RSA_Factoring_Challenge
*/