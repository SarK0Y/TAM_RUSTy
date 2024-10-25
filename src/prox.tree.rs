use nix::sys::signal; use nix::sys::signal::kill as kl; use nix::unistd::Pid;
use crate::threadpool::{mk_tree_of_prox, sig_2_tree_of_prox}; use crate::threadpool::{tree_of_prox, prox};
use crate::custom_traits::{STRN, turn_2_i64};
use procfs::process::all_processes;
use sysinfo::System; 
pub fn short_name_4_nix_sig (name: &str) -> Option< signal::Signal > {
    match name.trim_end().to_lowercase().as_str() {
        "-stop" => return Some( signal::SIGSTOP ),
        "-cont" => return Some( signal::SIGCONT ),
        "-abort" => return Some (signal::SIGABRT ),
        "-kill" => return Some (signal::SIGKILL ),
        _ => return None
    }
}
pub fn send_prox_sig (pid: i32, sig: Option < signal::Signal >) {
    if let Some (x ) = sig {
        let mut tree : *mut tree_of_prox = **mk_tree_of_prox(pid);
        unsafe { sig_2_tree_of_prox( &mut (*tree), x); } return;
    }
    kl( Pid::from_raw( pid ), None);
}
pub fn sig_2_proc_n_its_kids (cmd: &String) {
    let cmd = cmd.replace ("sig 2 proc ", "").trim_start_matches (" ").strn();
    let (pid, sig) = crate::split_once( &cmd, " ");
    let pid: i32 = pid.i640() as i32;
    send_prox_sig(pid, short_name_4_nix_sig (&sig ));
}
pub fn get_pid_by_name ( name: &String ) -> Option < i32 > {
        let mut system = System::new_all();
    system.refresh_all();

    for (pid, process) in system.processes() {
        let process_cmd = process.cmd().join ( &std::ffi::OsString::from  ( " ") ).to_str ().unwrap_or("").to_string (); 
       // println!("{}", process_cmd );
        if process_cmd.find (name ).is_some() {
            return Some( pid.as_u32 () as i32 ); 
        }
    }
    None
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
