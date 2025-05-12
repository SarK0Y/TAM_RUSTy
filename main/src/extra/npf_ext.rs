/*
        TODO: Discrete Logarithm Possible Solution
*/
use std::ops::Mul;
use std::ops::MulAssign;
use rug::float::Round;
use rug::ops::{AddAssignRound, DivAssignRound, MulAssignRound, PowAssign as rugPowAssign, PowAssignRound, SubAssignRound, Pow as rugpow};
use rug::{Complete, Assign, Integer as rugint, float::Constant as rugconst, Float as rugfloat, ops::SubFrom};
use once_cell::sync::Lazy;
use crate::faav;
use crate::STRN;
use crate::{errMsg0, errMsg0 as _msg, globs18::split_once_alt_o_null_strns, ps18::{set_prnt, get_prnt}};
use crate::custom_traits::{helpful_math_ops, STRN_usize, turn_2_i64};
use crate::goto; //::{label, goto};
use crate::npf::{init_form, product_form, vec_product_form, vec_init_form, over_npf, npf_output, Nstr};
pub const PREC: u64 = 8192;
/*pub fn tst (a: &init_form, b: &init_form) -> product_form {
    *a *= *b.clone();
    return *a * *b;
}*/
#[derive(Clone, PartialEq)]
pub struct PQ {
    pub P: init_form,
    pub Q: init_form,
    pub rdx: u32
}
impl PQ {
    pub fn build (P: init_form, Q: init_form, rdx: u32) -> Self { return Self {P, Q, rdx} }
    pub fn new (rdx: u32) -> Self { return Self {P: init_form::mk (rdx.into(), 0), Q: init_form::mk (rdx as u64, 0), rdx} }
}
#[derive(Clone, PartialEq)]
pub struct npf_tail {
    pub _0: u32,
    pub _1: u32,
    pub rdx: u32
}
impl npf_tail {
    pub fn new (rdx: u32) -> Self {return Self {_0: 0, _1: 0, rdx} }
    pub fn next (&self ) -> Option < Self > {
        if self._0 == self._1 && self._1 == self.rdx - 1 { return None }
        let mut ret = self.clone () ;
        if self._0 < self.rdx - 1 {ret._0 += 1; return Some (ret )}
        else {ret._1 += 1; return Some (ret) }
    }
}
#[no_mangle]
pub unsafe fn npf_ext (n: rugint) -> (rugint, rugint, String) {
    let fn_name = "npf ext".strn();
    let mut ret = (rugint::from (0), rugint::from (0), fn_name);
    let mut X = init_form::new();
    let mut Y = init_form::new();
    let mut X_tst: Vec < init_form > = Vec::new();
    let mut Y_tst: Vec < init_form > = Vec::new();
   // let mut x = rugfloat::with_val (3);
   // let mut y = rugfloat::with_val (3);
    let init = init_form::new ();
    let init0 = init_form::mk (2, 0);
    let mut mark_positive_results = Vec:: <usize>::new();
    let mut mark_j = 711_usize;
    let mut finally = vec_product_form { 0: Vec::new() };
    let n_ = format! ("{n}");
   // errMsg0 (n_.as_str());
    while X.num1 () * Y.num1 () < n {
    dbg!(&X); dbg! (&Y);
        X_tst.push ( X.nest ( init0.clone() ) );
        X_tst.push ( X.nest ( init.clone() ) );
        Y_tst.push ( Y.nest ( init0.clone() ) );
        Y_tst.push ( Y.nest ( init.clone() ) );
        finally.0.push (X_tst [0].clone() * Y_tst [0].clone()); // even-even
        finally.0.push (X_tst [1].clone() * Y_tst [1].clone()); // odd-odd
        finally.0.push (X_tst [1].clone() * Y_tst [0].clone()); // odd-even
         let tst = n.clone() % X_tst[0].head.clone();
         dbg!(&tst);
        if X.tail != Y.tail { finally.0.push (X_tst [0].clone() * Y_tst [1].clone() ); /* even-odd */ }
         //dbg!(&finally);
         println! ("&&finally = \n {}", finally);
        for i in 0..finally.0.len() {
            if (n.clone() - finally.0[i].tail.clone() ) % X_tst [0].head.clone() == 0 {mark_positive_results.push ( i ); mark_j = i;};
        }
        if mark_positive_results.len() > 1 {ret = npf_recursion_lock (n.clone (), &mut X, &mut Y ); goto! ("End_npf_ext"); };
        match mark_j {
            0 => {X = X_tst [0].clone(); Y = Y_tst [0].clone()},
            1 => {X = X_tst [1].clone(); Y = Y_tst [1].clone()},
            2 => {X = X_tst [1].clone(); Y = Y_tst [0].clone()},
            3 => {X = X_tst [0].clone(); Y = Y_tst [1].clone()},
            _ => {errMsg0("Strange error."); goto!("End_npf_ext");}
        }
        mark_positive_results.clear();
        X_tst.clear();
        Y_tst.clear ();
        finally.0.clear ();
    }
    label!("End_npf_ext");
    if X.num1 () * Y.num1 () == n {ret.0 = X.num1 (); ret.1 = Y.num1()}
    return ret
}
#[no_mangle]
pub unsafe fn npf_cross_road_ext (n: rugint, X: &mut init_form, Y: &mut init_form) -> (rugint, rugint, String) {
    let fn_name = "cross road ext".strn();
    let mut ret = (rugint::from (0), rugint::from (0), fn_name);
    if !crate::faav::npf_bar (X.log2_head().try_into().unwrap(), None) {println! ("Dead end"); return ret;}
    let mut X_tst = vec_init_form { 0: Vec::new() };
    let mut Y_tst = vec_init_form { 0: Vec::new() };
    let mut mark_positive_results = Vec:: <(init_form, init_form, usize)>::new();
    let mut mark_j = 711_usize;
    let mut finally = Vec:: <product_form>::new ();
    let n_ = format! ("{n}");
    //errMsg0 (n_.as_str());
    while X.num1 () * Y.num1 () < n {
    //dbg!(&X); dbg! (&Y);
        X_tst.0.push ( X.__2x() );
        X_tst.0.push ( X.__2x_plus_1());
        Y_tst.0.push ( Y.__2x() );
        Y_tst.0.push ( Y.__2x_plus_1() );
        finally.push (X_tst.0 [0].clone() * Y_tst.0 [0].clone()); // even-even
        finally.push (X_tst.0 [1].clone() * Y_tst.0 [1].clone()); // odd-odd
        finally.push (X_tst.0 [1].clone() * Y_tst.0 [0].clone()); // odd-even
        finally.push (X_tst.0 [0].clone() * Y_tst.0 [1].clone() ); /* even-odd */
        dbg! (finally.len());
        let mut max_tail_len: usize = 0;
        for i in 0..finally.len() {
            let cur_tail_len: usize = max_tail_match (&finally [i]);
            if (n.clone() - finally[i].tail.clone() ) % X_tst.0 [0].head.clone() == 0 /*|| cur_tail_len > max_tail_len */ {
             //   if cur_tail_len > max_tail_len {max_tail_len = cur_tail_len}
                mark_positive_results.push ( take_pair (i, X.clone(), Y.clone()) ); mark_j = i;
            };
        }
        if mark_positive_results.len() > 0 {dbg! (&mark_positive_results); println!("X_tst {}", X_tst ) }
        if mark_positive_results.is_empty () { println! ("Sorry, no solution found"); goto!("Exit_cross_road_ext");} 
        if mark_positive_results.len() > 1 {
            while let Some(XY) = mark_positive_results.iter().next() {
                if crate::faav::npf_lock (None) {println! ("NPF gets locked"); goto!("Exit_cross_road_ext");}
                let cur_ret = unsafe {check_match_div (&n, &XY.0, &XY.1 ) };
                ret.0 = cur_ret.0;
                ret.1 = cur_ret.1;
                if ret.0 > 1 { goto!("Exit_cross_road_ext");}
                ret = npf_cross_road_lock_ext (n.clone (), &mut XY.0.clone(), &mut XY.1.clone(), 0 );
            }
        }
        match mark_j {
            0 => {*X = X_tst.0 [0].clone(); *Y = Y_tst.0 [0].clone()},
            1 => {*X = X_tst.0 [1].clone(); *Y = Y_tst.0 [1].clone()},
            2 => {*X = X_tst.0 [1].clone(); *Y = Y_tst.0 [0].clone()},
            3 => {*X = X_tst.0 [0].clone(); *Y = Y_tst.0 [1].clone()},
            _ => {errMsg0("Strange error."); return ret}
        }
        mark_positive_results.clear();
        X_tst.0.clear();
        Y_tst.0.clear ();
        finally.clear ();
    }
    label!("Exit_cross_road_ext");
    if X.num1 () * Y.num1 () == n {ret.0 = X.num1 (); ret.1 = Y.num1()}
    println! ("Exit_cross_road");
    return ret
}
#[no_mangle]
pub unsafe fn npf_cross_road_lock_ext (n: rugint, X: &mut init_form, Y: &mut init_form, split: usize) -> (rugint, rugint, String) {
    let fn_name = "cross road lock ext".strn();
    let mut ret = (rugint::from (0), rugint::from (0), fn_name.clone() );
    //dbg! (&X);
    if !crate::faav::npf_bar (X.log2_head().try_into().unwrap(), None) { /* println! ("Dead end {fn_name} {}", X.log2_head() ); */ return ret;}
    let mut X_tst = vec_init_form { 0: Vec::new() };
    let mut Y_tst = vec_init_form { 0: Vec::new() };
    let mut mark_positive_results = Vec:: <(init_form, init_form, usize)>::new();
    let mut mark_j = 711_usize;
    let mut finally = vec_product_form { 0: Vec::new() };
    let n_ = format! ("{n}");
    //errMsg0 (n_.as_str());
    while X.num1 () * Y.num1 () < n {
    //dbg!(&X); dbg! (&Y);
        X_tst.0.push ( X.__2x() );
        X_tst.0.push ( X.__2x_plus_1());
        Y_tst.0.push ( Y.__2x() );
        Y_tst.0.push ( Y.__2x_plus_1() );
        finally.0.push (X_tst.0 [0].clone() * Y_tst.0 [0].clone()); // even-even
        finally.0.push (X_tst.0 [1].clone() * Y_tst.0 [1].clone()); // odd-odd
        finally.0.push (X_tst.0 [1].clone() * Y_tst.0 [0].clone()); // odd-even
        finally.0.push (X_tst.0 [0].clone() * Y_tst.0 [1].clone() ); /* even-odd */
        let mut max_tail_len: usize = 0;
        for i in 0..finally.0.len() {
           // let cur_tail_len: usize = max_tail_match (&finally.0 [i]);
            if (n.clone() - finally.0 [i].tail.clone() ) % X_tst.0 [0].head.clone() == 0 /*|| cur_tail_len > max_tail_len */ {
                //if cur_tail_len > max_tail_len {max_tail_len = cur_tail_len}
                mark_positive_results.push ( take_pair (i, X.clone(), Y.clone()) ); mark_j = i;
            };
        }
       // if mark_positive_results.len() > 0 { dbg! (&mark_positive_results); println!("X_tst {}", X_tst); }
        if mark_positive_results.is_empty () { println! ("Sorry, no solution found"); goto!("Exit_cross_road_lock");} 
        if mark_positive_results.len() > 1 {
            while let Some(XY) = mark_positive_results.pop() {
                //dbg! (&mark_positive_results);
                if crate::faav::npf_lock (None) {println! ("NPF gets locked"); goto!("Exit_cross_road_lock");}
                let cur_ret = unsafe {check_match_div (&n, &XY.0, &XY.1 ) };
                ret.0 = cur_ret.0;
                ret.1 = cur_ret.1;
                if ret.0 > 1 { goto!("Exit_cross_road_lock");}
                ret = npf_cross_road_lock_ext (n.clone (), &mut XY.0.clone(), &mut XY.1.clone(), 0 );
            }
        } else {
                dbg!("one hit");
                let XY = mark_positive_results.pop().unwrap();
                if crate::faav::npf_lock (None) {println! ("NPF gets locked"); goto!("Exit_cross_road_lock");}
                let cur_ret = unsafe {check_match_div (&n, &XY.0, &XY.1 ) };
                ret.0 = cur_ret.0;
                ret.1 = cur_ret.1;
                if ret.0 > 1 { goto!("Exit_cross_road_lock");}
        }
        match mark_j {
            0 => {*X = X_tst.0 [0].clone(); *Y = Y_tst.0 [0].clone()},
            1 => {*X = X_tst.0 [1].clone(); *Y = Y_tst.0 [1].clone()},
            2 => {*X = X_tst.0 [1].clone(); *Y = Y_tst.0 [0].clone()},
            3 => {*X = X_tst.0 [0].clone(); *Y = Y_tst.0 [1].clone()},
            _ => {errMsg0("Strange error."); return ret}
        }
        mark_positive_results.clear();
        X_tst.0.clear();
        Y_tst.0.clear ();
        finally.0.clear ();
    }
    label!("Exit_cross_road_lock");
    if X.num1 () * Y.num1 () == n {ret.0 = X.num1 (); ret.1 = Y.num1()}
   // println! ("Exit_cross_road_lock {}", X.log2_head() );
    return ret
}
pub unsafe fn npf_cross_road_split_ext (n: rugint, X: &mut init_form, Y: &mut init_form, split: usize) -> (rugint, rugint, String) {
    let fn_name = "cross road split".strn();
    let mut ret = (rugint::from (0), rugint::from (0), fn_name.clone() );
    dbg! (&X);
    dbg! (&split);
    if !crate::faav::npf_bar (split, None) {println! ("Dead end {fn_name}"); return ret;}
    let mut X_tst = vec_init_form { 0: Vec::new() };
    let mut Y_tst = vec_init_form { 0: Vec::new() };
    let mut mark_positive_results = Vec:: <(init_form, init_form, usize)>::new();
    let mut mark_j = 711_usize;
    let mut finally = vec_product_form { 0: Vec::new() };
    let n_ = format! ("{n}");
    //errMsg0 (n_.as_str());
    while X.num1 () * Y.num1 () < n {
    //dbg!(&X); dbg! (&Y);
        X_tst.0.push ( X.__2x() );
        X_tst.0.push ( X.__2x_plus_1());
        Y_tst.0.push ( Y.__2x() );
        Y_tst.0.push ( Y.__2x_plus_1() );
        finally.0.push (X_tst.0 [0].clone() * Y_tst.0 [0].clone()); // even-even
        finally.0.push (X_tst.0 [1].clone() * Y_tst.0 [1].clone()); // odd-odd
        finally.0.push (X_tst.0 [1].clone() * Y_tst.0 [0].clone()); // odd-even
        finally.0.push (X_tst.0 [0].clone() * Y_tst.0 [1].clone() ); /* even-odd */
        dbg! (finally.0.len());
        let mut max_tail_len: usize = 0;
        for i in 0..finally.0.len() {
            let cur_tail_len: usize = max_tail_match (&finally.0 [i]);
            if (n.clone() - finally.0 [i].tail.clone() ) % X_tst.0 [0].head.clone() == 0 || cur_tail_len > max_tail_len {
                if cur_tail_len > max_tail_len {max_tail_len = cur_tail_len}
                mark_positive_results.push ( take_pair (i, X.clone(), Y.clone()) ); mark_j = i;
            };
        }
        if mark_positive_results.len() > 0 {dbg! (&mark_positive_results); println!("X_tst {}", X_tst); }
        if mark_positive_results.is_empty () { println! ("Sorry, no solution found"); goto!("Exit_cross_road_split_ext");} 
        if mark_positive_results.len() > 1 {
            while let Some(XY) = mark_positive_results.pop() {
                //dbg! (&mark_positive_results);
                if crate::faav::npf_lock (None) {println! ("NPF gets locked"); goto!("Exit_cross_road_split_ext");}
                let cur_ret = unsafe {check_match_div (&n, &XY.0, &XY.1 ) };
                ret.0 = cur_ret.0;
                ret.1 = cur_ret.1;
                if ret.0 > 1 { goto!("Exit_cross_road_split_ext");}
                ret = npf_cross_road_split_ext (n.clone (), &mut XY.0.clone(), &mut XY.1.clone(), split + 1 );
            }
        }
        match mark_j {
            0 => {*X = X_tst.0 [0].clone(); *Y = Y_tst.0 [0].clone()},
            1 => {*X = X_tst.0 [1].clone(); *Y = Y_tst.0 [1].clone()},
            2 => {*X = X_tst.0 [1].clone(); *Y = Y_tst.0 [0].clone()},
            3 => {*X = X_tst.0 [0].clone(); *Y = Y_tst.0 [1].clone()},
            _ => {errMsg0("Strange error."); return ret}
        }
        mark_positive_results.clear();
        X_tst.0.clear();
        Y_tst.0.clear ();
        finally.0.clear ();
    }
    label!("Exit_cross_road_split_ext");
    if X.num1 () * Y.num1 () == n {ret.0 = X.num1 (); ret.1 = Y.num1()}
    println! ("Exit_cross_road_split_ext");
    return ret
}
pub fn take_pair (mark_j: usize, X: init_form, Y: init_form) -> (init_form, init_form, usize) {
    let mut ret = (X, Y, mark_j);
    //dbg! (&ret);
    match mark_j {
                0 => {ret.0 = ret.0.__2x(); ret.1 = ret.1.__2x();}, //{X = X_tst [0].clone(); Y = Y_tst [0].clone()},
                1 => {ret.0 = ret.0.__2x_plus_1(); ret.1 = ret.1.__2x_plus_1();}, //{X = X_tst [1].clone(); Y = Y_tst [1].clone()},
                2 => {ret.0 = ret.0.__2x_plus_1(); ret.1 = ret.1.__2x();}, //{X = X_tst [1].clone(); Y = Y_tst [0].clone()},
                3 => {ret.0 = ret.0.__2x(); ret.1 = ret.1.__2x_plus_1();}, //{X = X_tst [0].clone(); Y = Y_tst [1].clone()},
                _ => {errMsg0("Strange error."); return ret}
            } /*dbg! (&ret);*/ return ret
}
pub fn npf_recursion (n: rugint, X: &mut init_form, Y: &mut init_form) -> npf_output {
let ret0: npf_output = unsafe { npf_cross_road_ext (n.clone(), X, Y ) };
if crate::faav::npf_lock (None) {println! ("End npf_recursion"); return ret0;}
let mut x_ = X.clone(); let mut y_ = Y.clone();
    let mut thr = std::thread::spawn ( move || {
        let ret: npf_output = unsafe {
                npf_cross_road_ext (n.clone(), &mut x_, &mut y_ )
        };
        over_npf (Some (ret.clone () ));
    });
    thr.join(); return over_npf (None).expect ("over_npf failed");
}
pub fn npf_recursion_lock (n: rugint, X: &mut init_form, Y: &mut init_form) -> npf_output {
let ret0: npf_output = unsafe { if !crate::faav::npf_split( None ) {
                npf_cross_road_lock_ext (n.clone(), X, Y, 0 )
            }  else { npf_cross_road_split_ext (n.clone(), X, Y, 0 ) }
        };
if crate::faav::npf_lock (None) {println! ("End npf_recursion_lock"); return ret0;}
let mut x_ = X.clone(); let mut y_ = Y.clone();
    let mut thr = std::thread::spawn ( move || {
        println! ("Run npf_recursion_lock");
        let ret: npf_output = unsafe { if !crate::faav::npf_split( None ) {
                npf_cross_road_lock_ext (n.clone(), &mut x_, &mut y_, 0 )
            }  else { npf_cross_road_split_ext (n.clone(), &mut x_, &mut y_, 0 ) }
        };
        over_npf (Some (ret.clone () ));
    });
    thr.join(); return over_npf (None).expect ("over_npf failed");
}
pub fn max_tail_match (pf: &product_form) -> usize {
    let tail_str = pf.tail.to_string_radix (2);
    if Nstr (None).unwrap ().contains (&tail_str) { return tail_str.len () } return 0
}
pub fn set_bar (cmd: &String) {
use crate::custom_traits::STRN_usize;
    let max_id = cmd.replace ("npf bar", "").trim_start().trim_end().strn().usize0();
    crate::faav::npf_bar (0, Some (max_id) );
}
pub fn check_match_div (n: &rugint, X: &init_form, Y: &init_form) -> (rugint, rugint) {
    let __1 = rugint::from (1);
    let mut res= 700usize;
    let mut div = __1.clone();
    loop {
        div = check_div (&n, X.tail.clone() ); 
        if div > 1 && div < *n { res = 0; break }
        div = check_div (&n, X.num1().clone() ); 
        if div > 1 && div < *n { res = 1; break }
        div = check_div (&n, Y.tail.clone() ); 
        if div > 1 && div < *n { res = 2; break }
        div = check_div (&n, Y.num1().clone() ); 
        if div > 1 && div < *n { res = 3; break } break;
    }
    if div > 1 && div < *n { dbg! (&div ); crate::faav::npf_lock (Some (true)); }
    match res {
        0 => {return (div.clone(), X.tail.clone() );},
        1 => {return (div.clone(), X.num1().clone() )},
        2 => {return (div.clone(), Y.tail.clone() )},
        3 => {return (div.clone(), Y.num1().clone() )},
        _ => {return (__1.clone(), __1) }
    }
}
pub fn check_div (n: &rugint, div: rugint ) -> rugint {
    let __1 = rugint::from (1);
    if n.clone() % div.clone() == 0 { return n / div } return __1.clone()
}
pub fn show_npf_split_mode () {
    let status = faav::npf_split( None );
    let msg = format! ("npf split state {status}");
    _msg(&msg);
}
pub fn Set_NPF () {
    crate::faav::npf_lock (Some (false) );
    let prnt = get_prnt (1001876412);
    let (_, num) = split_once_alt_o_null_strns (&prnt, &" ".strn());
    if num == "" {errMsg0 ("proper command: npf <Your number>"); return}
    let num = num.replace(",", "");
    let num = rugint::parse (num);
    if num.is_err() {errMsg0 ("Set proper number, Please"); return}
    let mut num = num.unwrap().complete ();
    let num_str = format! ("npf {num}");
    set_prnt (&num_str, 479541533);
    let ret = unsafe { npf_ext (num) };
    let ret = if ret.0 > 1 {format! ("Q = {}, P = {}, id = {}", ret.0, ret.1, ret.2)} else 
                {format! ("Sorry, Dear User, no solution found Q: {}, P: {} - You can try deeper search {{press Ins}}{{npf bar <Number of Upper Bit>}} ", ret.0, ret.1)};
    errMsg0 (ret.as_str());
}
//fn
/*
use std::ops::MulAssign;

#[derive(Debug, Clone, Copy)]
struct Scalar(f64);

impl MulAssign<f64> for Scalar {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
    }
}

fn main() {
    let mut s = Scalar(5.0);
    s *= 2.0;
    println!("{:?}", s); // Scalar(10.0)
}

*/
