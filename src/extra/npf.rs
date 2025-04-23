use std::ops::Mul;
use std::ops::MulAssign;
use rug::float::Round;
use rug::ops::{AddAssignRound, DivAssignRound, MulAssignRound, PowAssign as rugPowAssign, PowAssignRound, SubAssignRound, Pow as rugpow};
use rug::{Complete, Assign, Integer as rugint, float::Constant as rugconst, Float as rugfloat, ops::SubFrom};
use crate::STRN;
use crate::{errMsg0, globs18::split_once_alt_o_null_strns, ps18::{set_prnt, get_prnt}};
pub const PREC: u64 = 8192;
#[derive(Debug, Clone, PartialEq)]
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
}
impl MulAssign for init_form {
    fn mul_assign(&mut self, rhs: init_form) {
        self.head = self.head.clone() * rhs.head;
        self.tail = self.head.clone() * rhs.tail + self.tail.clone()
    }
}
#[derive(Debug, Clone, PartialEq)]
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
pub fn npf (n: rugint) -> (rugint, rugint) {
    let mut ret = (rugint::from (0), rugint::from (0));
    let mut X = init_form::mk (4, 3);
    let mut Y = init_form::mk (4, 3);
    let mut X_tst: Vec < init_form > = Vec::new();
    let mut Y_tst: Vec < init_form > = Vec::new();
   // let mut x = rugfloat::with_val (3);
   // let mut y = rugfloat::with_val (3);
    let init = init_form::new ();
    let init0 = init_form::mk (2, 0);
    let mut count_positive_results = 0_usize;
    let mut j = 0_usize;
    let mut mark_j = j;
    let mut finally = Vec:: <product_form>::new ();
    while X.num1 () * Y.num1 () < n {
        X_tst.push ( X.nest ( init0.clone() ) );
        X_tst.push ( X.nest ( init.clone() ) );
        Y_tst.push ( Y.nest ( init0.clone() ) );
        Y_tst.push ( Y.nest ( init.clone() ) );
        finally.push (X_tst [0].clone() * Y_tst [0].clone()); // even-even
        finally.push (X_tst [1].clone() * Y_tst [1].clone()); // odd-odd
        finally.push (X_tst [1].clone() * Y_tst [0].clone()); // odd-even
        if X.tail != Y.tail { finally.push (X_tst [0].clone() * Y_tst [1].clone() ); /* even-odd */ }
        else { finally.push ( product_form::new() )}
        for i in 0..finally.len() {
            if (n.clone() - finally[i].tail.clone() ) % X_tst [0].head.clone() == 0 {count_positive_results += 1; mark_j = j;};
            j += 1;
        }
        if count_positive_results > 1 {errMsg0("Simple NPF failed."); ret.0 = X.num1 (); ret.1 = Y.num1(); return ret};
        match mark_j {
            0 => {X = X_tst [0].clone(); Y = Y_tst [0].clone()},
            1 => {X = X_tst [1].clone(); Y = Y_tst [1].clone()},
            2 => {X = X_tst [1].clone(); Y = Y_tst [0].clone()},
            3 => {X = X_tst [0].clone(); Y = Y_tst [1].clone()},
            _ => {}
        }
        j = 0;
        count_positive_results = 0;
        X_tst.clear();
        Y_tst.clear ();
        finally.clear ();
    }
    if X.num1 () * Y.num1 () == n {ret.0 = X.num1 (); ret.1 = Y.num1()}
    return ret
}
pub fn Set_NPF () {
    let prnt = get_prnt (1001876412);
    let (_, num) = split_once_alt_o_null_strns (&prnt, &" ".strn());
    if num == "" {errMsg0 ("proper command: npf <Your number>"); return}
    let num = rugint::parse (num);
    if num.is_err() {errMsg0 ("Set proper number, Please"); return}
    let num = num.unwrap().complete ();
    let ret = npf (num);
    let ret = format! ("Q = {}, P = {}", ret.0, ret.1);
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
