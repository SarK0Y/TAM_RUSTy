use once_cell::sync::Lazy;
use nix::sys::wait::wait;
use nix::unistd::{fork, ForkResult, getpid, getppid, execve};
use std::{ ffi::CString, env::var, num::NonZero };

use crate::{breaks, helpful_math_ops, split_once, STRN};
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
    let c_str = |arg: &String| -> CString {CString::new( format! ( "{arg}\0" ).as_str()  ).unwrap() };
    let empty_c_str = || -> CString {CString::new( format! ( "\0" ).as_str()  ).unwrap() };
    let empty =  empty_c_str ();
    let mut env: [CString; 1024];
    let mut args: [ CString; 1024];
    for i in 0..1024 { env [ i ] = empty.clone() }

    let (app_name, _ ) = split_once( cmd, " ");
    let mut cnt = 0usize;
    loop {
        let (arg, cmd ) = split_once( cmd, " ");
        if arg == "none" { break }
        args[ cnt ] = c_str (&arg); cnt.inc();
    }
    execve ( &c_str ( &app_name), &args, form_env (&mut env) );
}
pub fn form_env <'a > (env_str: &'a mut [CString] ) -> &'a [CString] {
//    let mut env_vec: Vec < String > = Vec::new();
    let mut count = 0usize;
    let key = format! ("PATH={}\0",  var ("PATH").unwrap_or( "".strn() ) );
    env_str[ count ] =  CString::new (key.as_str() ).unwrap_or( CString::new("").unwrap() );
    count.inc();
        env_str
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
let PATH = CString::from_vec_with_nul( var ("PATH").unwrap_or( "".strn() ).as_bytes().to_vec() ).unwrap();

*/
