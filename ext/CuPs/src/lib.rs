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
#[path = "stop.flood.rs"]
pub mod smart_lags;
#[path = "enums.rs"]
pub mod enums;
use crate::enums::*;
#[path = "atomic.rs"]
pub mod atomic;
use crate::atomic::*;
#[path = "some.extra.funx.rs"]
pub mod some_extra_funx;