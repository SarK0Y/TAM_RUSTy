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
#[cfg(any(feature ="dev_hell_n_fun", feature = "cleanup"))]
#[cfg(not(feature ="stable"))]
#[path = "dev_lex.rs"]
pub mod lex;
#[cfg(feature ="stable")]
#[cfg(not(feature ="dev_hell_n_fun"))]
#[cfg(not(feature ="cleanup"))]
#[path = "stable_lex.rs"]
pub mod lex;
#[cfg(any(feature ="dev_hell_n_fun", feature = "stable", feature = "cleanup" ) )]
pub mod edit_funx;
#[cfg(any(feature ="dev_hell_n_fun", feature = "stable", feature = "cleanup" ) )]
pub mod strns;
#[cfg(any(feature ="dev_hell_n_fun", feature = "stable", feature = "cleanup") )]
pub mod faav;
    
