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