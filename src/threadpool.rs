use once_cell::sync::Lazy;
use nix::sys::wait::wait;
use nix::unistd::{fork, ForkResult, getpid, getppid};
use std::{ ffi::CString, env::var, num::NonZero };

use crate::STRN;
pub fn thr_ids (id: usize, mode: crate::enums::threadpool ) {
    static mut ids: Lazy < Vec < usize > > = Lazy::new ( || { Vec::with_capacity (100) } );
}
pub fn new_thr (cmd: &String) {
   match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => {
            println!("Parent process. PID: {}, Child PID: {}", getpid(), child);
//            wait().expect("Failed to wait for child");
        }
        Ok(ForkResult::Child) => {
            println!("Child process. PID: {}, Parent PID: {}", getpid(), getppid());
        }
        Err(err) => eprintln!("Fork failed: {}", err),
    }    
}
pub fn run_kid (cmd: &String) {

}
pub fn form_env () -> CString {
    let PATH = CString::from_vec_with_nul( var ("PATH").unwrap_or( "".strn() ).as_bytes().to_vec() ).unwrap();
    CString::new("").unwrap()
}
/*
use nix::unistd::{execve, fork, ForkResult};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match unsafe { fork()? } {
        ForkResult::Parent { child } => {
            println!("Spawned child with PID: {}", child);
        }
        ForkResult::Child => {
            // Prepare arguments
            let path = CString::new("/bin/ls")?;
            let args = [
                CString::new("ls")?,
                CString::new("-l")?,
                CString::new("/")?,
            ];
            
            // Prepare environment
            let env = [
                CString::new("PATH=/bin:/usr/bin")?
            ];

            // Execute new process
            execve(&path, &args, &env).expect("execve failed");
        }
    }

    Ok(())
}


A Vec<T> is described by 3 values:
    * A pointer to its first element, that can be obtained with .as_mut_ptr()
let v = unsafe { Vec::<T>::from_raw_parts(ptr, length, capacity) };
If you want them to be equals, you can use .shrink_to_fit() on the vector to reduce its capacity as near as its size as possible depending on the allocator.
 print!("{:?}", std::env::vars()); return;
*/
