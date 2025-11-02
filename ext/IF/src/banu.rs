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
use min_err_per_step::editing::{
    conv_strn_2_rugfloat,
    conv_str_2_rugfloat,
    Conv_Strn_2_Rugfloat,
    Conv_Strn_2_Rugint
};
use min_err_per_step::seqs::bitwise::{
    square_xor,
    road1,
    bloat_upto
};
use Mademoiselle_Entropia::custom_traits::{helpful_math_ops, STRN};
use Mademoiselle_Entropia::minio::InterruptMsg;
use Mademoiselle_Entropia::_break;
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
pub fn faav_countdown (bar: Option <usize> ) -> usize {
    static mut counter: usize = 0;
    unsafe {
        if let Some ( x ) = bar { counter = x; }
        return counter
    }
}
pub fn faav_init_tail (tail: Option <String> ) -> rugfloat {
    static mut init: Lazy < rugfloat > = Lazy::new ( || {
        rugfloat::with_val_64 (
            glob_precision (None),
            0.999
        ) 
    });
    unsafe {
        if let Some ( x ) = tail { 
            *init = conv_strn_2_rugfloat (&x, 10).unwrap_or (
                rugfloat::with_val_64 (
                    glob_precision (None),
                    0.999
                )
            );
         }
        return init.clone()
    }
}
pub fn faav_power (tail: Option <String> ) -> rugfloat {
    static mut init: Lazy < rugfloat > = Lazy::new ( || {
        rugfloat::with_val_64 (
            glob_precision (None),
            0.999
        ) 
    });
    unsafe {
        if let Some ( x ) = tail { 
            *init = conv_strn_2_rugfloat (&x, 10).unwrap_or (
                rugfloat::with_val_64 (
                    glob_precision (None),
                    0.999
                )
            );
         }
        return init.clone()
    }
}
pub fn faav_102m_m_1 (m: Option <usize> ) -> rugfloat {
    static mut init: Lazy < rugfloat > = Lazy::new ( || {
        rugfloat::with_val_64 (
            1,
            1
        ) 
    });
    static _1: Lazy < rugint > = Lazy::new ( || {
        "1".int (10)
    });
    unsafe {
        if let Some ( _m ) = m { 
            let mut _10: rugint = _1.clone() * 10;
            _10 = _10.clone().pow (_m as u32);
            _10 -= 1;
            *init =  rugfloat::with_val_64 (
                glob_precision (None),
                _10
            ); 
            faav_m (Some (_m) );
         }
        return init.clone()
    }
}
pub fn faav_102shift () -> rugfloat {
    static mut init: Lazy < rugfloat > = Lazy::new ( || {
        rugfloat::with_val_64 (
            glob_precision (None),
            10
        ).pow (faav_shift (None) ) 
    });
    unsafe {
        return init.clone()
    }
}

pub fn faav_22m_m_1 (m: Option <usize> ) -> rugfloat {
    static mut init: Lazy < rugfloat > = Lazy::new ( || {
        rugfloat::with_val_64 (
            1,
            1
        ) 
    });
    static _1: Lazy < rugfloat > = Lazy::new ( || {
        rugfloat::with_val_64 (
            glob_precision (None),
            1
        ) 
    });
    unsafe {
        if let Some ( _m ) = m { 
            *init = _1.clone() << _m; 
            *init -= 1;
            faav_m (Some (_m) );
         }
        return init.clone()
    }
}
pub fn faav_m (m: Option <usize> ) -> usize {
    static mut sav_m: usize = 0;
    unsafe {
        if let Some ( x ) = m { sav_m = x; }
        return sav_m
    }
}
pub fn nxt_tail_of_sq_xor (tail: &mut rugfloat, rounds: usize, pow: isize) {
    static mut _1st_run: bool = true;
    unsafe {
        if _1st_run {
            tail.pow_assign (pow );
            _1st_run = false;
        }
    }
    let _1: rugfloat = "1".float(10);
    *tail = bloat_upto (&tail, &_1, faav_shift (None) as u32 );
    for _ in 1..rounds {
        *tail = road1 (&tail, &_1);
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
pub fn _gen_backdoor_numero (n_minus_1: &rugfloat, tail: &rugfloat) -> rugfloat {
    let mut cut_tail: rugfloat = tail.clone();
    dbg! (&cut_tail);
    let mut new_n: rugfloat = n_minus_1.clone() * (faav_102m_m_1 (None) + 1 );
    new_n += faav_102m_m_1 (None);
    new_n *= faav_102shift ();
    new_n += cut_tail; 
    dbg! (new_n.to_string_radix(10, Some (glob_precision (None) as usize ) ) );
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
    if *tst > 1 {
        dbg! (tst);
        return None }
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
pub fn _try_restore_factors (lst: &Vec <(u64, u64)>) -> banu {
    let mut _1 = rugfloat::with_val_64 (glob_precision (None), 1);
    let mut maybe_P = _1.clone ();
    let mut maybe_Q = _1.clone ();

    for p in lst {
        for _ in 0..p.1 {
            if maybe_P > maybe_Q { maybe_Q *= p.0; }
            else {maybe_P *= p.0}
        }
    }
    return banu {
        maybe_P,
        maybe_Q
    }
}
pub fn _gcd (_0: &rugfloat, _1: &rugfloat) -> rugfloat {
    let mut ret = rugfloat::with_val (10, 23);
    let mut ret_ = rugfloat::with_val (10, 23);
    if _1 > _0 {
            ret = _1.clone ();
            ret_ = _0.clone();
    } else {
        ret = _0.clone();
        ret_ = _1.clone();
    }
    while ret_ > 0 {
        ret %= ret_.clone();
        ret_ %= ret.clone();
    }
    if ret == 0 { ret +=1; }
    return ret
}
pub fn _2nd_tst () {
    let N = conv_str_2_rugfloat (
        "1881988129206079638386972394616504398071635633794173827007633564
        22988859715234665485319060606504743045317388011303396716199692321
        205734031879550656996221305168759307650257059",
    10
    ).unwrap();
    dbg! (&N);
    let mut tail: rugfloat = faav_init_tail (None);
    let mut counter = faav_countdown (None);
    let mut bnN = rugfloat::with_val_64 (1, 0);
    while counter > 0 {
        bnN = _gen_backdoor_numero (&N, &mut tail);
        tail = __2rt (&tail, glob_precision (None) );
        dbg! (&tail);
        let gcd = _gcd (&bnN, &N);
        if gcd > 1 && gcd < N {
            _break! (gcd.to_string_radix (10, None));
        }
        counter.dec();
    }
}
pub fn _1st_tst () {
    let P = conv_strn_2_rugfloat (
        &"3980750864240649373971255005503864911990643623425267084063851895759463889572
          61768583317".strn(),
          10
    ).unwrap();
    let Q = conv_strn_2_rugfloat (
        &"4727721461074353025362230719730482246329146953020971164598521711305207112563
          63590397527".strn(),
        10
    ).unwrap();
    dbg! (&P);
    dbg! (&Q);
    let _1 = rugfloat::with_val_64 (1, 1);
    let Pm1: rugfloat = P -1;
    let Qm1: rugfloat = Q -1; 
    let mut tail = faav_init_tail (None);
    let mut counter = faav_countdown (None);
    let mut bnP = _1.clone();
    let mut bnQ = _1.clone();
    let mut saveP = _1.clone();
    let mut saveQ = _1;
    let mut got_P: Option <Vec <(u64, u64) > > = None;
    let mut got_Q: Option <Vec <(u64, u64) > > = None;
    loop {
        if got_P.is_none() {
            bnP = _gen_backdoor_numero (&Pm1, &tail );
           // dbg! (bnP.to_string_radix(2, None) );
            saveP = bnP.clone();
            got_P = try_banu (&mut bnP);
        }
        if got_Q.is_none() {
            bnQ = _gen_backdoor_numero (&Qm1, &tail );
            saveQ = bnQ.clone();
            got_Q = try_banu (&mut bnQ);
        }
        if got_P.is_some() && got_Q.is_some() ||
        counter == 0 {
            break;
        }
        counter.dec();
        nxt_tail_of_sq_xor (&mut tail, 3, 3);
        dbg! (&tail);
    }
    if got_P.is_none() {
        InterruptMsg ("P failed.");
    }
    if got_Q.is_none () {
        InterruptMsg ("Q failed");
    }
    if got_P.is_none () && got_Q.is_none () {
        dbg! (&counter);
        InterruptMsg ("");
        return
    }
    let mut bnN = saveP.clone() * saveQ.clone();
    let got_N = try_banu (&mut bnN).unwrap();
    let finale: banu = _try_restore_factors (&got_N);
    if finale.maybe_P != saveP && finale.maybe_Q != saveQ {
        InterruptMsg ("finale failed");
    } else {
        InterruptMsg ("Yeah, pal - we got it :)")
    }
    dbg! (&finale);
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