use std::ops::Mul;
use std::ops::MulAssign;
use rug::float::Round;
use rug::ops::{AddAssignRound, DivAssignRound, MulAssignRound, PowAssign as rugPowAssign, PowAssignRound, SubAssignRound, Pow as rugpow};
use rug::{Assign, Integer as rugint, float::Constant as rugconst, Float as rugfloat, ops::SubFrom};
use crate::errMsg0;
#[derive(Debug, Clone, PartialEq)]
pub struct init_form {
    pub head: rugfloat,
    pub tail: rugfloat
}
impl init_form {
    fn new () -> init_form {
        return Self {
            head: rugfloat::with_val (2),
            tail: rugfloat::with_val (1),
        }
    }
    fn mk (head: u64, tail: u64) -> init_form {
        return Self {
            head: rugfloat::with_val_u64 (head),
            tail: rugfloat::with_val_u64 (tail),
        }
    }
    fn nest (&self, rhs: init_form) -> init_form {
        return Self {
            head: self.head * rhs.head,
            tail: self.head * rhs.tail + self.tail
        }
    }
    fn num1 (&self) -> rugfloat {
        return self.head + self.tail
    } 
    fn num (&self, x: rugfloat) -> rugfloat {
        return self.head * x + self.tail
    } 
}
impl MulAssign for init_form {
    fn mul_assign(&mut self, rhs: init_form) -> init_form {
        return Self {
            head: self.head * rhs.head,
            tail: self.head * rhs.tail + self.tail
        }
    }
}
pub struct product_form {
    pub xy: rugfloat,
    pub y: rugfloat,
    pub x: rugfloat,
    pub tail: rugfloat
}
impl product_form {
    fn new () -> Self {
        return Self {
            xy: rugfloat::with_val (0),
            y: rugfloat::with_val(0),
            x: rugfloat::with_val(0),
            tail: rugfloat::with_val(0),
        }
    }
}
impl Mul for init_form {
    fn mul(&self, rhs: init_form) -> product_form {
        let mut ret: product_form = product_form::new();
        ret.xy = self.head * rhs.head;
        ret.x = self.x * rhs.tail;
        ret.y = rhs.x * self.tail;
        ret.tail = self.tail * rhs.tail;
        return ret;
    }
}
pub fn tst (a: &init_form, b: &init_form) -> product_form {
    a *= b;
    return a * b;
}
pub fn npf (n: rugfloat) -> (rugfloat, rugfloat) {
    let mut ret = (rugfloat::with_val (0), rugfloat::with_val (0));
    let mut X = init_form::mk (4, 3);
    let mut Y = init_form::mk (4, 3);
    let mut X_tst: Vec < init_form > = Vec::new();
    let mut Y_tst: Vec < init_form > = Vec::new();
    let mut x = rugfloat::with_val (3);
    let mut y = rugfloat::with_val (3);
    let init = init_form::new ();
    let init0 = init_form::mk (2, 0);
    let mut count_positive_results = 0_usize;
    let mut j = 0_usize;
    let mut mark_j = j;
    let mut finally = Vec:: <product_form>::new ();
    while X.num1 () * Y.num1 () < n {
        X_tst.push ( X.nest ( init0 ) );
        X_tst.push ( X.nest ( init ) );
        Y_tst.push ( Y.nest ( init0 ) );
        Y_tst.push ( Y.nest ( init ) );
        finally.push (X_tst [0] * Y_tst [0]); // even-even
        finally.push (X_tst [1] * Y_tst [1]); // odd-odd
        finally.push (X_tst [1] * Y_tst [0]); // odd-even
        if X.tail != Y.tail { finally.push (X_tst [0] * Y_tst [1]); /* even-odd */ }
        else { finally.push ( product_form::new() )}
        for fin in finally {
            if (n - fin.tail) % X_tst [0].head == 0 {count_positive_results += 1; mark_j = j;};
            j += 1;
        }
        if count_positive_results > 1 {errMsg0("Simple NPF failed."); ret.0 = X.num1 (); ret.1 = Y.num1(); return ret};
        match mark_j {
            0 => {X = X_tst [0]; Y = Y_tst [0]},
            1 => {X = X_tst [1]; Y = Y_tst [1]},
            2 => {X = X_tst [1]; Y = Y_tst [0]},
            3 => {X = X_tst [0]; Y = Y_tst [1]},
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