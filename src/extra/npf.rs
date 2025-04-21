use std::ops::Mul;
use std::ops::MulAssign;
use rug::float::Round;
use rug::ops::{AddAssignRound, DivAssignRound, MulAssignRound, PowAssign as rugPowAssign, PowAssignRound, SubAssignRound, Pow as rugpow};
use rug::{Assign, Integer as rugint, float::Constant as rugconst, Float as rugfloat, ops::SubFrom};
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
    let mut X = init_form::mk (4, 3);
    let mut Y = init_form::mk (4, 3);
    let mut X_prev: Vec < init_form > = Vec::new();
    let mut Y_prev: Vec < init_form > = Vec::new();
    let mut x = rugfloat::with_val (3);
    let mut y = rugfloat::with_val (3);
    let init = init_form::new ();
    let mut finally = product_form::new ();
    return (rugfloat::with_val (0), rugfloat::with_val (0))
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