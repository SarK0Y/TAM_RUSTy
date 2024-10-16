use nix::sys::signal; use nix::sys::signal::kill as kl; use nix::unistd::Pid;
use crate::threadpool::sig_2_tree_of_prox; use crate::threadpool::{tree_of_prox, prox};
use crate::custom_traits::{STRN, turn_2_i64};
pub fn short_name_4_nix_sig (name: &str) -> Option< signal::Signal > {
    match name.to_lowercase().as_str() {
        "-stop" => return Some( signal::SIGSTOP ),
        "-cont" => return Some( signal::SIGCONT ),
        "-abort" => return Some (signal::SIGABRT ),
        "-kill" => return Some (signal::SIGKILL ),
        _ => return None
    }
}
pub fn send_prox_sig (pid: i32, sig: Option < signal::Signal >) {
    if let Some (x ) = sig {
        let mut tree : *mut tree_of_prox = *tree_of_prox::new ( pid );
        unsafe { sig_2_tree_of_prox( &mut (*tree), x); } return;
    }
    kl( Pid::from_raw( pid ), None);
}
pub fn sig_2_proc_n_its_kids (cmd: &String) {
    let cmd = cmd.replace ("sig 2 proc ", "").trim_start_matches (" ").strn();
    let (pid, sig) = crate::split_once( &cmd, " ");
    let pid: i32 = (pid.i640() & i32::MIN as i64) as i32;
    send_prox_sig(pid, short_name_4_nix_sig (&sig ));
}
//fn
/* use std::fs;

fn read_proc_status(pid: i32) -> std::io::Result<String> {
    fs::read_to_string(format!("/proc/{}/status", pid))
}

let pid = 1234; // Replace with actual PID
match read_proc_status(pid) {
    Ok(status) => println!("Process {} status:\n{}", pid, status),
    Err(e) => println!("Failed to read process status: {}", e),
}
use sysinfo::{ProcessExt, System, SystemExt};

let mut sys = System::new_all();
sys.refresh_all();

for (pid, process) in sys.processes() {
    println!("{}: {} status: {:?}", pid, process.name(), process.status());
}
use signal_hook::{iterator::Signals, SIGINT};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut signals = Signals::new(&[SIGINT])?;
    for sig in signals.forever() {
        println!("Received signal {:?}", sig);
    }
    Ok(())
}
*/
