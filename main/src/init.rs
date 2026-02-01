use crate::{custom_traits::STRN, key_handlers, run_cmd, run_cmd_out_sync};
use once_cell::sync::Lazy;
use nix::sys::signal::{sigaction, SaFlags, SigAction, SigHandler, SigSet, SIGCHLD};
use nix::sys::wait::waitpid;
use nix::unistd::Pid;
use Mademoiselle_Entropia::_break;
use Mademoiselle_Entropia::minio::InterruptMsg;
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
    if !crate::cmd_keys::extra_info(None) { return }
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
pub fn reload_tam () {
    //crate::set_ask_user ("Let's reload TAM", -851658545);
    let my_pid = unsafe {libc::getpid () };
    let fork_pid = crate::take_list_adr ("fork.pid");
    if !std::path::Path::new (&fork_pid).exists() {
        crate::mk_empty_file (&fork_pid);
    }
    crate::save_file_append_newline_abs_adr_fast (&my_pid.to_string (), &fork_pid);
    crate::mk_empty_file ("reload");
    //panic! ("fork pid {my_pid}");
    std::process::exit (477);
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

