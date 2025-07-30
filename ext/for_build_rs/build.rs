//build.rs for rst_lex

use std::env;
use std::fs;
use std::path::Path;
//extern crate rst_lex;
//use crate::lex::_cleanup;
//use rst_lex::faav::cleanup_dbg_attr;
include! ("./src/dev_lex.rs");
pub fn cleanup () {
    let mut attr = cleanup_dbg_attr::new();
    let src = "./src/dev_lex.rs";
    let dst = "./src/lex.rs";
    match fs::copy(src, dst) {
        Ok(bytes_copied) => {
            println!("Successfully copied {} bytes from '{}' to '{}'.", bytes_copied, src, dst);
        }
        Err(e) => {
            eprintln!("Error copying file: {}", e);
        }
    }
}
fn main() {

}
