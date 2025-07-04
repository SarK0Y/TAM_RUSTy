//build.rs for goto1717

use std::env;
use std::fs;
use std::path::Path;

fn main() {
#[cfg(feature ="tst")]
    let src = "./src/tst_goto.rs";
#[cfg(feature ="stable")]
    let src = "./src/stable_goto.rs";
    let dst = "./src/lib.rs";
#[cfg(any(feature ="stable", feature = "tst") )]
    match fs::copy(src, dst) {
        Ok(bytes_copied) => {
            println!("Successfully copied {} bytes from '{}' to '{}'.", bytes_copied, src, dst);
        }
        Err(e) => {
            eprintln!("Error copying file: {}", e);
        }
    }
}
