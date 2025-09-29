#![allow(static_mut_refs)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
//mod goto;
//pub use crate::goto::{label, goto};
#[path = "base.rs"]
pub mod base;
#[path = "trig.rs"]
pub mod trig;
#[path = "nth_root.rs"]
pub mod nth_root;
#[path = "frax.rs"]
pub mod frax;
#[path = "logarithm.rs"]
pub mod logarithm;
#[path = "complex.rs"]
pub mod complex;
