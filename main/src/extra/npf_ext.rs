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
use crate::npf::{init_form, product_form, vec_product_form, vec_init_form, over_npf, npf_output, Nstr, check_div};
pub const PREC: u64 = 8192;
/*pub fn tst (a: &init_form, b: &init_form) -> product_form {
    *a *= *b.clone();
    return *a * *b;
}*/
#[derive(Clone, PartialEq, Debug)]
pub struct PQ {
    pub P: init_form,
    pub Q: init_form,
    pub nxt: iter_tail,
    pub rdx: u32
}
impl PQ {
    pub fn build (P: init_form, Q: init_form, nxt: iter_tail, rdx: u32) -> Self { return Self {P, Q, nxt, rdx} }
    pub fn new (rdx: u32) -> Self { return Self {P: init_form::mk (rdx.into(), 0), Q: init_form::mk (rdx as u64, 0), nxt: iter_tail::new(rdx), rdx} }
    pub fn after (&self) -> Option < Self > {
        let nxt_tail = self.nxt.next();
        let nxt_tail = if nxt_tail.is_none () { return None } else { nxt_tail.unwrap () };
        let rdx = self.rdx as u64;
        let P = init_form::mk (rdx, nxt_tail._1 as u64 );
        let P = self.P.nest (P);
        let Q = init_form::mk (rdx, nxt_tail._0 as u64);
        let Q = self.Q.nest (Q);
        return Some (Self {P, Q, nxt: nxt_tail, rdx: self.rdx} )
    }
    pub fn _1st_iter (&self) -> Self {
        let cur_tail = self.nxt.clone();
        let rdx = self.rdx as u64;
        let P = init_form::mk (rdx, cur_tail._1 as u64 );
        let P = self.P.nest (P);
        let Q = init_form::mk (rdx, cur_tail._0 as u64);
        let Q = self.Q.nest (Q);
        return Self {P, Q, nxt: cur_tail, rdx: self.rdx}
    }
    pub fn Q (&self) -> init_form {return self.Q.clone () }
    pub fn P (&self) -> init_form {return self.P.clone () }
    pub fn product (&self) -> product_form { return self.P() * self.Q() }
    pub fn _a(&self) -> Self { return self.clone() }
}
#[derive(Clone, PartialEq, Debug)]
pub struct iter_tail {
    pub _0: u32,
    pub _1: u32,
    pub rdx: u32
}
impl iter_tail {
    pub fn new (rdx: u32) -> Self {return Self {_0: 0, _1: 0, rdx} }
    pub fn next (&self ) -> Option < Self > {
        if self._0 == self._1 && self._1 == self.rdx - 1 { return None }
        let mut ret = self.clone () ;
        if self._0 < self.rdx - 1 {ret._0 += 1; return Some (ret )}
        else {ret._1 += 1; ret._0 = 0; return Some (ret) }
    }
    pub fn reset (&mut self) { self._1 = 0; self._0 = 0; }
    pub fn null (&mut self) -> Self { self._1 = 0; self._0 = 0; return self.clone () }
}
#[derive(Clone, PartialEq, Debug)]
pub struct vec_PQ ( pub Vec < PQ > );
#[no_mangle]
pub unsafe fn npf_ext (n: rugint, rdx: u32) -> (rugint, rugint, String) {
    let fn_name = "npf ext".strn();
    let mut ret = (rugint::from (0), rugint::from (0), fn_name);
    let mut pq: PQ = PQ::new (rdx);
    let mut mark_j = 711_usize;
    let mut finally = vec_PQ { 0: Vec::<PQ>::new() };
    let mut mark_positive_results = Vec:: <usize>::new();
    let n_ = format! ("{n}");
   // errMsg0 (n_.as_str());
    while pq.Q.num1 () * pq.P.num1 () < n {
        pq.nxt = pq.nxt.null();
        finally.0.push ( pq._1st_iter() );
        while let Some (next) = pq.after() { finally.0.push (next ) }
        let a = &finally.0 [0].Q;
         //dbg!(&finally);
         println! ("&&finally = \n {:?}", finally);
        for i in 0..finally.0.len() {
            let product = finally.0[i].product ();
            if (n.clone() - product.tail() ) % a.head() == 0 {mark_positive_results.push ( i ); mark_j = i;};
        }
        if mark_positive_results.len() > 1 {ret = npf_recursion_lock (n.clone (), pq._a() ); goto! ("End_npf_ext"); };
        pq = finally.0 [mark_j]._a();
        mark_positive_results.clear();
        finally.0.clear ();
    }
    label!("End_npf_ext");
    let X = pq.P();
    let Y = pq.Q();
    if X.num1 () * Y.num1 () == n {ret.0 = X.num1 (); ret.1 = Y.num1()}
    return ret
}
pub unsafe fn npf_low_bush (n: rugint, mut pq: PQ, split: usize) -> (rugint, rugint, String) {
    let fn_name = "npf low bush".strn();
    let mut ret = (rugint::from (0), rugint::from (0), fn_name);
    let mut mark_j = 711_usize;
    let mut finally = vec_PQ { 0: Vec::<PQ>::new() };
    let mut mark_positive_results = Vec:: <usize>::new();
    let n_ = format! ("{n}");
   // errMsg0 (n_.as_str());
    while pq.Q.num1 () * pq.P.num1 () < n {
        pq.nxt = pq.nxt.null();
        finally.0.push ( pq._1st_iter() );
        while let Some (next) = pq.after() { finally.0.push (next ) }
        let a = &finally.0 [0].Q;
         //dbg!(&finally);
         println! ("&&finally = \n {:?}", finally);
        for i in 0..finally.0.len() {
            let product = finally.0[i].product ();
            if (n.clone() - product.tail() ) % a.head() == 0 {mark_positive_results.push ( i ); mark_j = i;};
        }
        if mark_positive_results.len() > 1 {ret = npf_low_bush (n.clone (), pq._a(), 0 ); goto! ("End_npf_low_bush"); };
        pq = finally.0 [mark_j]._a();
        mark_positive_results.clear();
        finally.0.clear ();
    }
    label!("End_npf_low_bush");
    let X = pq.P();
    let Y = pq.Q();
    if X.num1 () * Y.num1 () == n {ret.0 = X.num1 (); ret.1 = Y.num1()}
    return ret
}
pub unsafe fn npf_long_bush (n: rugint, mut pq: PQ, split: usize) -> (rugint, rugint, String) {
    let fn_name = "npf long bush".strn();
    let mut ret = (rugint::from (0), rugint::from (0), fn_name);
    let mut mark_j = 711_usize;
    let mut finally = vec_PQ { 0: Vec::<PQ>::new() };
    let mut mark_positive_results = Vec:: <usize>::new();
    let n_ = format! ("{n}");
   // errMsg0 (n_.as_str());
    while pq.Q.num1 () * pq.P.num1 () < n {
        pq.nxt = pq.nxt.null();
        finally.0.push ( pq._1st_iter() );
        while let Some (next) = pq.after() { finally.0.push (next ) }
        let a = &finally.0 [0].Q;
         //dbg!(&finally);
         println! ("&&finally = \n {:?}", finally);
        for i in 0..finally.0.len() {
            let product = finally.0[i].product ();
            if (n.clone() - product.tail() ) % a.head() == 0 {mark_positive_results.push ( i ); mark_j = i;};
        }
        if mark_positive_results.len() > 1 {ret = npf_long_bush (n.clone (), pq._a(), split + 1 ); goto! ("End_npf_long_bush"); };
        pq = finally.0 [mark_j]._a();
        mark_positive_results.clear();
        finally.0.clear ();
    }
    label!("End_npf_long_bush");
    let X = pq.P();
    let Y = pq.Q();
    if X.num1 () * Y.num1 () == n {ret.0 = X.num1 (); ret.1 = Y.num1()}
    return ret
}
pub fn npf_recursion_lock (n: rugint, pq: PQ ) -> npf_output {
let ret0: npf_output = unsafe { if !crate::faav::npf_split( None ) {
                npf_low_bush (n.clone(), pq._a(), 0 )
            }  else { npf_long_bush (n.clone(), pq._a(), 0 ) }
        };
if crate::faav::npf_lock (None) {println! ("End npf_recursion_lock"); return ret0;}
let mut pq_ =pq.clone();
    let mut thr = std::thread::spawn ( move || {
        println! ("Run npf_recursion_lock");
        let ret: npf_output = unsafe { if !crate::faav::npf_split( None ) {
                npf_low_bush (n.clone(), pq_, 0 )
            }  else { npf_low_bush (n.clone(), pq_, 0 ) }
        };
        over_npf (Some (ret.clone () ));
    });
    thr.join(); return over_npf (None).expect ("over_npf failed");
}
pub fn max_tail_match (pf: &product_form) -> usize {
    let tail_str = pf.tail.to_string_radix (2);
    if Nstr (None).unwrap ().contains (&tail_str) { return tail_str.len () } return 0
}
pub fn __check_match_div (n: &rugint, pq: &PQ) -> (rugint, rugint) {
    let X = &pq.Q;
    let Y = &pq.P;
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
