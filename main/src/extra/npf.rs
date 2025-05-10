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
pub const PREC: u64 = 8192;
type npf_output = (rugint, rugint, String);
#[derive(Clone, PartialEq)]
pub struct vec_product_form ( Vec <product_form> );
pub struct vec_init_form ( Vec <init_form> );
//type vec_init_form = std::vec::Vec <init_form>;
pub fn over_npf (ret: Option < npf_output>) -> Option <npf_output> {
    static mut state: Lazy < Option <npf_output > > = Lazy::new (|| {None});
    unsafe {
        if ret.is_some() { *state = ret} state.clone()
    }
}
pub fn max_head (head: Option < rugint >) -> Option < rugint > {
    static mut val: Lazy <rugint > = Lazy::new (|| {rugint::from (0) });
    let __0 = rugint::from (0);
    unsafe {
        if let Some (x) = head {
            if x < 0 { *val = __0; return None }
            if x > *val { *val = x.clone(); return Some (x) }
        } return Some (val.clone());
    }
}
pub fn Nstr (ret: Option < String >) -> Option <String> {
    static mut state: Lazy < Option < String > > = Lazy::new (|| {None});
    unsafe {
        if ret.is_some() { *state = ret} state.clone()
    }
}
#[derive(Clone, PartialEq)]
pub struct init_form {
    pub head: rugint,
    pub tail: rugint
}
impl init_form {
    fn new () -> init_form {
        return Self {
            head: rugint::from ( 2 ),
            tail: rugint::from (1),
        }
    }
    fn mk (head: u64, tail: u64) -> init_form {
        return Self {
            head: rugint::from ( head ),
            tail: rugint::from ( tail ),
        }
    }
    fn nest (&self, rhs: init_form) -> init_form {
        return Self {
            head: self.head.clone() * rhs.head,
            tail: self.head.clone() * rhs.tail + self.tail.clone()
        }
    }
    fn num1 (&self) -> rugint {
        return self.head.clone() + self.tail.clone()
    } 
    fn num (&self, x: rugint) -> rugint {
        return self.head.clone() * x + self.tail.clone()
    } 
    fn __2x_plus_1 (&mut self) -> init_form {
        return self.nest ( init_form::new() )
    }
    fn __2x (&mut self) -> init_form {
        return self.nest ( init_form::mk (2, 0) )
    }
    fn log2_head (&self) -> u32 {
        return self.head.find_one (0).unwrap ()
    }
}
impl MulAssign for init_form {
    fn mul_assign(&mut self, rhs: init_form) {
        self.head = self.head.clone() * rhs.head;
        self.tail = self.head.clone() * rhs.tail + self.tail.clone()
    }
}
#[derive(Clone, PartialEq)]
pub struct product_form {
    pub xy: rugint,
    pub y: rugint,
    pub x: rugint,
    pub tail: rugint
}
impl product_form {
    fn new () -> Self {
        return Self {
            xy: rugint::from (0),
            y: rugint::from(0),
            x: rugint::from(0),
            tail: rugint::from(0),
        }
    }
}
impl Mul for init_form {
    type Output = product_form;
    fn mul(self, rhs: init_form) -> product_form {
        let mut ret: product_form = product_form::new();
        ret.xy = self.head.clone() * rhs.head.clone();
        ret.x = self.head.clone() * rhs.tail.clone();
        ret.y = rhs.head * self.tail.clone();
        ret.tail = self.tail.clone() * rhs.tail;
        return ret;
    }
}
/*pub fn tst (a: &init_form, b: &init_form) -> product_form {
    *a *= *b.clone();
    return *a * *b;
}*/
#[no_mangle]
pub unsafe fn npf (n: rugint) -> (rugint, rugint, String) {
    let fn_name = "std".strn();
    Nstr (Some (n.to_string_radix(2) ));
    let mut ret = (rugint::from (0), rugint::from (0), fn_name);
    let mut X = init_form::mk (4, 3);
    let mut Y = init_form::mk (4, 3);
    let mut X_tst: Vec < init_form > = Vec::new();
    let mut Y_tst: Vec < init_form > = Vec::new();
   // let mut x = rugfloat::with_val (3);
   // let mut y = rugfloat::with_val (3);
    let init = init_form::new ();
    let init0 = init_form::mk (2, 0);
    let mut count_positive_results = 0_usize;
    let mut mark_j = 711_usize;
    let mut finally = vec_product_form { 0: Vec::new () };
    let n_ = format! ("{n}");
    //errMsg0 (n_.as_str());
    while X.num1 () * Y.num1 () < n {
    dbg!(&X); dbg! (&Y);
        X_tst.push ( X.nest ( init0.clone() ) );
        X_tst.push ( X.nest ( init.clone() ) );
        Y_tst.push ( Y.nest ( init0.clone() ) );
        Y_tst.push ( Y.nest ( init.clone() ) );
        finally.0.push (X_tst [0].clone() * Y_tst [0].clone()); // even-even
        finally.0.push (X_tst [1].clone() * Y_tst [1].clone()); // odd-odd
        finally.0.push (X_tst [1].clone() * Y_tst [0].clone()); // odd-even
        //dbg!(&finally);
        println! ("&&finally = \n {}", finally);
         let tst = n.clone() % X_tst[0].head.clone();
         dbg!(&tst);
        if X.tail != Y.tail { finally.0.push (X_tst [0].clone() * Y_tst [1].clone() ); /* even-odd */ }
        else { finally.0.push ( product_form::new() )}
        for i in 0..finally.0.len() {
            if (n.clone() - finally.0 [i].tail.clone() ) % X_tst [0].head.clone() == 0 {count_positive_results += 1; mark_j = i;};
        }
        if count_positive_results > 1 {errMsg0("Simple NPF failed."); ret.0 = X.num1 (); ret.1 = Y.num1(); goto! ("End_npf");};
        match mark_j {
            0 => {X = X_tst [0].clone(); Y = Y_tst [0].clone()},
            1 => {X = X_tst [1].clone(); Y = Y_tst [1].clone()},
            2 => {X = X_tst [1].clone(); Y = Y_tst [0].clone()},
            3 => {X = X_tst [0].clone(); Y = Y_tst [1].clone()},
            _ => {return npf_orig( n );}
        }
        count_positive_results = 0;
        X_tst.clear();
        Y_tst.clear ();
        finally.0.clear ();
    }
    label! ("End_npf");
    if X.num1 () * Y.num1 () == n {ret.0 = X.num1 (); ret.1 = Y.num1()}
    if ret.0 == 0 || ret.1 == 0 {return npf_orig (n);}
    return npf_orig (n);
}
#[no_mangle]
pub unsafe fn npf_orig (n: rugint) -> (rugint, rugint, String) {
    let fn_name = "orig".strn();
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
        if mark_positive_results.len() > 1 {ret = npf_recursion_lock (n.clone (), &mut X, &mut Y ); goto! ("End_orig"); };
        match mark_j {
            0 => {X = X_tst [0].clone(); Y = Y_tst [0].clone()},
            1 => {X = X_tst [1].clone(); Y = Y_tst [1].clone()},
            2 => {X = X_tst [1].clone(); Y = Y_tst [0].clone()},
            3 => {X = X_tst [0].clone(); Y = Y_tst [1].clone()},
            _ => {errMsg0("Strange error."); goto!("End_orig");}
        }
        mark_positive_results.clear();
        X_tst.clear();
        Y_tst.clear ();
        finally.0.clear ();
    }
    label!("End_orig");
    if X.num1 () * Y.num1 () == n {ret.0 = X.num1 (); ret.1 = Y.num1()}
    return ret
}
#[no_mangle]
pub unsafe fn npf_cross_road (n: rugint, X: &mut init_form, Y: &mut init_form) -> (rugint, rugint, String) {
    let fn_name = "cross road".strn();
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
        if mark_positive_results.is_empty () { println! ("Sorry, no solution found"); goto!("Exit_cross_road");} 
        if mark_positive_results.len() > 1 {
            while let Some(XY) = mark_positive_results.iter().next() {
                if crate::faav::npf_lock (None) {println! ("NPF gets locked"); goto!("Exit_cross_road");}
                let cur_ret = unsafe {check_match_div (&n, &XY.0, &XY.1 ) };
                ret.0 = cur_ret.0;
                ret.1 = cur_ret.1;
                if ret.0 > 1 { goto!("Exit_cross_road");}
                ret = npf_cross_road_lock (n.clone (), &mut XY.0.clone(), &mut XY.1.clone(), 0 );
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
    label!("Exit_cross_road");
    if X.num1 () * Y.num1 () == n {ret.0 = X.num1 (); ret.1 = Y.num1()}
    println! ("Exit_cross_road");
    return ret
}
#[no_mangle]
pub unsafe fn npf_cross_road_lock (n: rugint, X: &mut init_form, Y: &mut init_form, split: usize) -> (rugint, rugint, String) {
    let fn_name = "cross road lock".strn();
    let mut ret = (rugint::from (0), rugint::from (0), fn_name.clone() );
    dbg! (&X);
    if !crate::faav::npf_bar (X.log2_head().try_into().unwrap(), None) {println! ("Dead end {fn_name}"); return ret;}
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
        if mark_positive_results.is_empty () { println! ("Sorry, no solution found"); goto!("Exit_cross_road_lock");} 
        if mark_positive_results.len() > 1 {
            while let Some(XY) = mark_positive_results.pop() {
                //dbg! (&mark_positive_results);
                if crate::faav::npf_lock (None) {println! ("NPF gets locked"); goto!("Exit_cross_road_lock");}
                let cur_ret = unsafe {check_match_div (&n, &XY.0, &XY.1 ) };
                ret.0 = cur_ret.0;
                ret.1 = cur_ret.1;
                if ret.0 > 1 { goto!("Exit_cross_road_lock");}
                ret = npf_cross_road_lock (n.clone (), &mut XY.0.clone(), &mut XY.1.clone(), 0 );
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
    label!("Exit_cross_road_lock");
    if X.num1 () * Y.num1 () == n {ret.0 = X.num1 (); ret.1 = Y.num1()}
    println! ("Exit_cross_road_lock");
    return ret
}
pub unsafe fn npf_cross_road_split (n: rugint, X: &mut init_form, Y: &mut init_form, split: usize) -> (rugint, rugint, String) {
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
        if mark_positive_results.is_empty () { println! ("Sorry, no solution found"); goto!("Exit_cross_road_split");} 
        if mark_positive_results.len() > 1 {
            while let Some(XY) = mark_positive_results.pop() {
                //dbg! (&mark_positive_results);
                if crate::faav::npf_lock (None) {println! ("NPF gets locked"); goto!("Exit_cross_road_split");}
                let cur_ret = unsafe {check_match_div (&n, &XY.0, &XY.1 ) };
                ret.0 = cur_ret.0;
                ret.1 = cur_ret.1;
                if ret.0 > 1 { goto!("Exit_cross_road_split");}
                ret = npf_cross_road_split (n.clone (), &mut XY.0.clone(), &mut XY.1.clone(), split + 1 );
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
    label!("Exit_cross_road_split");
    if X.num1 () * Y.num1 () == n {ret.0 = X.num1 (); ret.1 = Y.num1()}
    println! ("Exit_cross_road_split");
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
let ret0: npf_output = unsafe { npf_cross_road (n.clone(), X, Y ) };
if crate::faav::npf_lock (None) {println! ("End npf_recursion"); return ret0;}
let mut x_ = X.clone(); let mut y_ = Y.clone();
    let mut thr = std::thread::spawn ( move || {
        let ret: npf_output = unsafe {
                npf_cross_road (n.clone(), &mut x_, &mut y_ )
        };
        over_npf (Some (ret.clone () ));
    });
    thr.join(); return over_npf (None).expect ("over_npf failed");
}
pub fn npf_recursion_lock (n: rugint, X: &mut init_form, Y: &mut init_form) -> npf_output {
let ret0: npf_output = unsafe { if !crate::faav::npf_split( None ) {
                npf_cross_road_lock (n.clone(), X, Y, 0 )
            }  else { npf_cross_road_split (n.clone(), X, Y, 0 ) }
        };
if crate::faav::npf_lock (None) {println! ("End npf_recursion_lock"); return ret0;}
let mut x_ = X.clone(); let mut y_ = Y.clone();
    let mut thr = std::thread::spawn ( move || {
        println! ("Run npf_recursion_lock");
        let ret: npf_output = unsafe { if !crate::faav::npf_split( None ) {
                npf_cross_road_lock (n.clone(), &mut x_, &mut y_, 0 )
            }  else { npf_cross_road_split (n.clone(), &mut x_, &mut y_, 0 ) }
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
pub fn en_npf_split () {
    crate::faav::npf_split( Some (true) );
    show_npf_split_mode();
}
pub fn no_npf_split () {
    crate::faav::npf_split( Some (false) );
    show_npf_split_mode();
}
pub fn show_npf_sq_mode () {
    let status = faav::npf_sq( None );
    let msg = format! ("npf sq state {status}");
    _msg(&msg);
}
pub fn en_npf_sq () {
    let prnt = get_prnt (1501876412);
    let (_, num) = split_once_alt_o_null_strns (&prnt, &" ".strn());
    if num == "" {errMsg0 ("proper command: npf <Your number>"); return}
    let num = num.replace(",", "");
    let num = rugint::parse (num);
    if num.is_err() {errMsg0 ("Set proper number, Please"); return}
    let mut num = num.unwrap().complete ();
    num *= num.clone();
    let num_str = format! ("npf {num}");
    set_prnt (&num_str, 479541533);
}
pub fn no_npf_sq () {
    crate::faav::npf_sq( Some (false) );
    show_npf_sq_mode();
}
pub fn __rdx (cmd: &String) {
    let num = cmd.replace ("rdx ", "").trim_end().trim_start().strn();
    let (rdx, num) = split_once_alt_o_null_strns (&num, &" ".strn());
    let rdx = rdx.trim_end().trim_start().strn().i640() as i32;
    let num = num.replace(",", "");
    let num = rugint::parse (num);
    if num.is_err() {errMsg0 ("Set proper number, Please"); return}
    let num = num.unwrap().complete ();
    let num = num.to_string_radix (rdx);
    errMsg0(&num );
}
pub fn __num_2 (cmd: &String) {
    let pow = cmd.replace ("num 2", "").trim_end().trim_start().strn().usize0() as u32;
    errMsg0(&pow.to_string() );
    num_2(pow - 1);
}
pub fn num_2(pow: u32) {
    let prnt = get_prnt (1504876412);
    let (_, num) = split_once_alt_o_null_strns (&prnt, &" ".strn());
    if num == "" {errMsg0 ("proper command: npf <Your number>"); return}
    let num = num.replace(",", "");
    let num = rugint::parse (num);
    if num.is_err() {errMsg0 ("Set proper number, Please"); return}
    let mut num = num.unwrap().complete ();
    num *= num.clone().pow( pow );
    let num_str = format! ("npf {num}");
    set_prnt (&num_str, 419541533);
}
pub fn __num_x (cmd: &String) {
    let num = cmd.replace ("num x", "").trim_end().trim_start().strn();
    let num = num.replace(",", "");
    let __1 = rugint::parse ("1" ).unwrap();
    let num = rugint::parse (num).unwrap_or(__1).complete();
    num_x( num );
}
pub fn num_x( x: rugint) {
    let prnt = get_prnt (1504876412);
    let (_, num) = split_once_alt_o_null_strns (&prnt, &" ".strn());
    if num == "" {errMsg0 ("proper command: npf <Your number>"); return}
    let num = num.replace(",", "");
    let num = rugint::parse (num);
    if num.is_err() {errMsg0 ("Set proper number, Please"); return}
    let mut num = num.unwrap().complete ();
    num *= x;
    let num_str = format! ("npf {num}");
    set_prnt (&num_str, 419541533);
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
    let ret = unsafe { npf (num) };
    let ret = if ret.0 > 1 {format! ("Q = {}, P = {}, id = {}", ret.0, ret.1, ret.2)} else 
                {format! ("Sorry, Dear User, no solution found - You can try deeper search {{press Ins}}{{npf bar <Number of Upper Bit>}} ")};
    errMsg0 (ret.as_str());
}
impl std::fmt::Display for product_form {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let tail_bin = self.tail.to_string_radix(2);
        return write!(f, "product_form (xy: {}, x: {}\n y: {}\n tail dec: {}\n tail bin: {})", self.xy, self.x, self.y, self.tail, tail_bin);
    }
}
impl std::fmt::Display for vec_product_form {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let cvec = self.0.clone();
        for j in cvec{
            println! ("{}", j);
        }
        return write!(f, "");
    }
}
impl std::fmt::Display for init_form {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let head_exp = self.head.find_one(0).unwrap();
        let tail_bin = self.tail.to_string_radix(2);
        return write!(f, "init_form (head: {} = 2e{} \n tail dec: {}\n tail bin: {})", self.head, head_exp, self.tail, tail_bin);
    }
}
impl std::fmt::Display for vec_init_form {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let cvec = self.0.clone();
        for j in cvec{
            println! ("{}", j);
        }
        return write!(f, "");
    }
}
impl std::fmt::Debug for product_form {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let tail_bin = self.tail.to_string_radix(2);
        return write!(f, "product_form (xy: {}, x: {}\n y: {}\n tail dec: {}\n tail bin: {})", self.xy, self.x, self.y, self.tail, tail_bin);
    }
}
impl std::fmt::Debug for vec_product_form {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let cvec = self.0.clone();
        for j in cvec{
            println! ("{}", j);
        }
        return write!(f, "");
    }
}
impl std::fmt::Debug for init_form {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let head_exp = self.head.find_one(0).unwrap();
        let tail_bin = self.tail.to_string_radix(2);
        return write!(f, "init_form (head: {} = 2e{} \n tail dec: {}\n tail bin: {})", self.head, head_exp, self.tail, tail_bin);
    }
}
impl std::fmt::Debug for vec_init_form {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let cvec = self.0.clone();
        for j in cvec{
            println! ("{}", j);
        }
        return write!(f, "");
    }
}
#[no_mangle]
pub unsafe fn __check_match_div (n: &rugint, X: &init_form, Y: &init_form) -> (rugint, rugint) {
    let __1 = rugint::from (1);
    let mut res= 700usize;
    let div = check_div (&n, X.tail.clone() ); 
    if div > 1 { res = 0; goto!("ok_done")}
    let div = check_div (&n, X.num1().clone() ); 
    if div > 1 { res = 1; goto!("ok_done") }
    let div = check_div (&n, Y.tail.clone() ); 
    if div > 1 { res = 2; goto!("ok_done") }
    let div = check_div (&n, Y.num1().clone() ); 
    if div > 1 { res = 3; goto!("ok_done") }
    //goto! ("no_divisor_found");
    return (__1.clone(), __1);
    println! ("no print");
    label! ("ok_done");
    crate::faav::npf_lock (Some (true));
    match res {
        0 => {return (div.clone(), X.tail.clone() );},
        1 => {return (div.clone(), X.num1().clone() )},
        2 => {return (div.clone(), Y.tail.clone() )},
        3 => {return (div.clone(), Y.num1().clone() )},
        _ => {return (__1.clone(), __1) }
    }
    label! ("no_divisor_found");
    return (__1.clone(), __1);
    goto! ("no_divisor_found");
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
