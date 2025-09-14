use rug::float::Round;
use rug::ops::{AddAssignRound, DivAssignRound, MulAssignRound, PowAssign as rugPowAssign, PowAssignRound, SubAssignRound, Pow as rugpow, CompleteRound };
use rug::{Assign, Integer as rugint, float::Constant as rugconst, Float as rugfloat, ops::SubFrom, Complete};
use rug::float::Constant;
use crate::base::{glob_precision, Pi };
use Mademoiselle_Entropia::minio::InterruptMsg;
pub fn continued_fraction_approximation(x: &rugfloat, max_terms: usize, len_in_bits: u64 ) -> (rugfloat, rugfloat) {
    let mut float_x = x.clone();
    let mut a = float_x.clone().trunc(); 
    let mut numerator = a.clone(); 
    let mut denominator = rugfloat::with_val_64(len_in_bits, 1.0); 
    let mut prev_numerator = rugfloat::with_val_64(len_in_bits, 1.0); 
    let mut prev_denominator = rugfloat::with_val_64(len_in_bits, 0.0);
    let eps: rugfloat = rugfloat::with_val_64(len_in_bits + 1, 2.0);
    let eps: rugfloat = eps.pow ( -1.0 * len_in_bits as f64 );
    let mut stop_x: rugfloat = eps.clone();
    float_x -= &a;

    for _ in 0..max_terms {
        if float_x.is_zero() {
            break; 
        }
        stop_x = numerator.clone() / denominator.clone();
        if (stop_x - x ).abs() < eps {break}
        // Take the reciprocal
        float_x = rugfloat::with_val_64(len_in_bits, 1) / float_x;
        a = float_x.clone().trunc();
        let new_numerator = (&a * &numerator + &prev_numerator).complete( len_in_bits as u32);
        let new_denominator = (&a * &denominator + &prev_denominator).complete(len_in_bits as u32);
        prev_numerator = numerator;
        prev_denominator = denominator;
        numerator = new_numerator;
        denominator = new_denominator;
        float_x -= &a; 
    }
    let res = numerator.clone() / denominator.clone();
    dbg! (&res);
    return (numerator, denominator)
}
