use nix::sys::signal; use nix::sys::signal::kill as kl; use nix::unistd::Pid;
use crate::threadpool::mk_tree_of_prox; use crate::threadpool::{tree_of_prox, prox};
use crate::custom_traits::{STRN, turn_2_i64, helpful_math_ops}; 
use procfs::process::all_processes;
use sysinfo::System; 
use std::ptr;
use crate::threadpool::branch_state;
use crate::enums::calc_kids;
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
        unsafe { sig_2_tree_of_prox( &mut (*tree), x); } 
        del_prox_tree ( &mut tree ); return;
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
pub fn sig_2_tree_of_prox (tree: &mut  tree_of_prox, sig: nix::sys::signal::Signal ){
    
    let mut branch: *mut tree_of_prox = tree;
    unsafe {
        let mut ret = sig_2_branch_of_prox( &mut *branch, sig, calc_kids::set_direction);
        let mut prev = ret.clone ();
        loop {
         branch = ret.0;   
         if branch == ptr::null_mut () { break; }
         ret = sig_2_branch_of_prox( &mut *branch, sig, calc_kids::default_way );
         if ret == prev { break; }
         if ret.0 == ptr::null_mut () { break; }
         prev = ret.clone ();
       //  dbg! (&prev); dbg! (&ret);
        // dbg! (&(*prev.0).cursor); dbg! (&(*ret.0).cursor);
        }
    }
}
pub fn sig_2_branch_of_prox (tree: &mut  tree_of_prox, sig: nix::sys::signal::Signal, mode: calc_kids ) -> (*mut tree_of_prox, branch_state){
    use nix::sys::signal::kill as kl;
    use nix::unistd::Pid;
    unsafe {
        if mode == calc_kids::set_direction { count_kids_properly(tree, mode.clone () ); }
        kl ( Pid::from_raw( (*tree).ppid ), sig );
        let root_len = (*(*tree).kids).len();
        let direction_to_count = if (*tree).up != ptr::null_mut() {(*(*tree).up).direction_to_count} else {(*tree).direction_to_count };
        (*tree).direction_to_count = direction_to_count;
        if (*(*tree).proxid_of_kid).len() == 0 /*|| (*tree).direction_to_count != direction_to_count */{ return ( (*tree).up, branch_state::jump_up ); }
        let pids: &Vec <i32> = &(*(*tree).proxid_of_kid);
        //dbg!( &(*(*tree).proxid_of_kid) );
            for pid in pids {
                if let Ok (x) = kl ( Pid::from_raw(*pid ), sig ) {}
            } let cur = count_kids_properly(tree, mode);
            if (*(*tree).kids).len() == 0 { return ( (*tree).up, branch_state::jump_up ); }
            let mut ret = ptr::null_mut ();
            if (*(*tree).kids).len () > cur { ret = (*(*tree).kids)[ cur ] }
            else { return ( (*tree).up, branch_state::jump_up ) }
            ( ret, branch_state::down )    
        }
}
pub fn count_kids_properly (tree: &mut  tree_of_prox, mode: calc_kids) -> usize {
    unsafe {
        if mode == calc_kids::set_direction {
            let mut len = (*(*tree).proxid_of_kid).len();
            if len <= (*tree).cursor { (*tree).cursor = len.dec(); (*tree).direction_to_count = true; }
            else { (*tree).direction_to_count = false }
        }
        let ret = (*tree).cursor;
        if (*tree).direction_to_count { (*tree).cursor.dec(); }
        else { (*tree).cursor.inc(); }
        ret
    }
}
pub fn del_prox_tree ( tree: &mut *mut tree_of_prox) {
    unsafe {
        if *tree == std::ptr::null_mut () { return }
        if (*(**tree).kids).len() == 0 { std::ptr::drop_in_place ( *tree ); }
        let mut tmp: *mut tree_of_prox;
        while *tree != std::ptr::null_mut () && (*(**tree).kids).len() > 0 {
            tmp = *tree;
            *tree = (*(**tree).kids).pop().unwrap();
            if *tree == std::ptr::null_mut () { *tree = tmp; }
            if (*(**tree).kids).len () == 0{
                if (**tree).up != std::ptr::null_mut (){ *tree = (**tree).up; } else { *tree = std::ptr::null_mut (); } 
                 std::ptr::drop_in_place ( (*tmp).proxid_of_kid );
                 std::ptr::drop_in_place ( (*tmp).kids );
                 std::ptr::drop_in_place ( tmp );
            }
        } 
    }
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
