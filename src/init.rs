use crate::{custom_traits::STRN, key_handlers, run_cmd, run_cmd_out_sync};
use once_cell::sync::Lazy;
use nix::sys::signal::{sigaction, SaFlags, SigAction, SigHandler, SigSet, SIGCHLD};
use nix::sys::wait::waitpid;
use nix::unistd::Pid;
pub fn user_home_dir() -> String {
    static mut home: Lazy<String> = Lazy::new(|| "".strn());
    static mut fst: bool = true;
    unsafe {
        if fst {
            let cmd = format!("cd ~/;pwd");
            *home = run_cmd_out_sync(cmd);
            fst = false;
        } format!("{}/", home.trim_end())
    }
}
extern "C" fn handle_sigchld(_: libc::c_int) {
    match waitpid(Pid::from_raw(-1), None) {
        Ok(status) => println!("Child exited with status {:?}", status),
        Err(err) => eprintln!("waitpid() failed: {}", err),
    }
}
extern "C" fn handle_sigchld_null(_: libc::c_int) {}
pub fn set_sig_chld_hook (){
     let sig_action = SigAction::new(
        SigHandler::Handler(handle_sigchld),
        SaFlags::SA_RESTART,
        SigSet::empty(),
    );
    
    unsafe {
        sigaction(SIGCHLD, &sig_action); //?;
    }
}
pub fn unset_sig_chld_hook (){
     let sig_action = SigAction::new(
        SigHandler::Handler(handle_sigchld_null),
        SaFlags::SA_RESTART,
        SigSet::empty(),
    );
    
    unsafe {
        sigaction(SIGCHLD, &sig_action); //?;
    }
}
//fn
/*
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sig_action = SigAction::new(
        SigHandler::Handler(handle_sigchld),
        SaFlags::SA_RESTART,
        SigSet::empty(),
    );
    
    unsafe {
        sigaction(SIGCHLD, &sig_action)?;
    }

    // Rest of your parent process code...

    Ok(())
}
*/

