use once_cell::sync::Lazy;
use nix::sys::wait::wait;
use nix::unistd::{fork, ForkResult, getpid, getppid};
pub fn thr_ids (id: usize, mode: crate::enums::threadpool ) {
    static mut ids: Lazy < Vec < usize > > = Lazy::new ( || { Vec::with_capacity (100) } );
}
pub fn new_thr (cmd: &String) {
   match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => {
            println!("Parent process. PID: {}, Child PID: {}", getpid(), child);
            wait().expect("Failed to wait for child");
        }
        Ok(ForkResult::Child) => {
            println!("Child process. PID: {}, Parent PID: {}", getpid(), getppid());
        }
        Err(err) => eprintln!("Fork failed: {}", err),
    }    
}
