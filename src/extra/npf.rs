use std::ops::Mul;
use rug::float::Round;
use rug::ops::{AddAssignRound, DivAssignRound, MulAssignRound, PowAssign as rugPowAssign, PowAssignRound, SubAssignRound, Pow as rugpow};
use rug::{Assign, Integer as rugint, float::Constant as rugconst, Float as rugfloat, ops::SubFrom};
#[derive(Debug, Clone, PartialEq)]
pub struct init_form {
    pub head: rugfloat,
    pub tail: rugfloat
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
    return a * b;
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