#![allow(static_mut_refs)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(non_upper_case_globals)]
//mod goto;
//pub use crate::goto::{label, goto};
#[path = "dev_lex.rs"]
pub mod lex;
pub mod edit_funx;
pub mod strns;
pub mod faav;
