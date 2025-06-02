use std::ops::Mul;
use std::ops::MulAssign;
use rug::float::Round;
use rug::ops::{AddAssignRound, DivAssignRound, MulAssignRound, PowAssign as rugPowAssign, PowAssignRound, SubAssignRound, Pow as rugpow, CompleteRound };
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
pub fn continued_fraction_approximation(x: rugfloat, max_terms: usize, len_in_bits: u32, ) -> (rugfloat, rugfloat) {
    let mut float_x = x.clone();
    let mut a = float_x.clone().trunc(); 
    let mut numerator = a.clone(); 
    let mut denominator = rugfloat::with_val(len_in_bits, 1.0); 
    let mut prev_numerator = rugfloat::with_val(len_in_bits, 1.0); 
    let mut prev_denominator = rugfloat::with_val(len_in_bits, 0.0);

    float_x -= &a;

    for _ in 0..max_terms {
        if float_x.is_zero() {
            break; 
        }

        // Take the reciprocal
        float_x = rugfloat::with_val(len_in_bits, 1) / float_x;
        a = float_x.clone().trunc();
        let new_numerator = (&a * &numerator + &prev_numerator).complete( len_in_bits.into () );
        let new_denominator = (&a * &denominator + &prev_denominator).complete(len_in_bits.into() );
        prev_numerator = numerator;
        prev_denominator = denominator;
        numerator = new_numerator;
        denominator = new_denominator;
        float_x -= &a; 
    }

    return (numerator, denominator)
}
