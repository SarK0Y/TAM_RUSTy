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
pub fn gor (n: &rugint) -> rugint {

    todo!()
}
pub fn B_base_num_sys (num: &rugint, rdx: &rugint) -> Vec < rugint > {
    let mut residue = rugint::from (1u64);
    let mut conv: Vec <rugint> = Vec::new();
    let mut rdx = rdx.clone();
    let mut num = num.clone();
    while num > 0 {
        residue = num.clone() % rdx.clone();
        conv.push ( residue.clone() );
        num -= residue;
        num /= rdx.clone();
    } return conv
}