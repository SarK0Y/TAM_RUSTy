use std::process::Command;
use std::io::{Write, Read}; use std::io::BufRead; use std::io::prelude::*;
use std::thread::Builder; use std::os::fd::AsRawFd; use std::os::fd::FromRawFd;
use std::os::unix::thread::JoinHandleExt;
use crate::smart_lags::{mamed_mutexes, new_custom_mutex};
use libc::SIGKILL;
use std::panic::catch_unwind;
use termion::raw::IntoRawMode;
use termion::terminal_size;
use substring::Substring;
use once_cell::sync::Lazy;
use crate::custom_traits::{STRN, helpful_math_ops, escaped_chars, STRN_strip};
use crate::prox::{check_alive_proc_by_pid, get_pid_by_name, get_ppid_n_pid_by_name};
use crate::update18::delay_mcs;
//use close_file::Closable;
use std::mem::drop;
use crate::globs18::{bash_unlink, check_strn_in_lst, cmd_decode_mode, cur_win_id, get_item_from_front_list, instance_num, take_list_adr, unblock_fd};
use crate::{checkArg, check_substr, clear_screen, cpy_str, default_term_4_shol_a, dont_scrn_fix, drop_ls_mode, edit_mode_lst, errMsg0, full_path_to_cmd, get_arg_in_cmd, getkey, is_dir, mk_cmd_file_dirty, mk_dummy_lnk, named_mutex, no_view, popup_msg, read_file, read_file_abs_adr, read_prnt, rm_file, run_cmd_out, run_cmd_out_sync, save_file, save_file0, save_file_abs_adr0, save_file_append, save_file_append_newline, set_prnt, split_once, split_once_or_ret_null_strns, tailOFF, term_mv};
#[path = "keycodes.rs"]
mod kcode;
use nix::sys::signal::kill;
use nix::unistd::{ForkResult, Pid};
pub(crate) fn run_term_app_interactive_basic(cmd: String) -> bool{
    let func_id = crate::func_id18::run_cmd_viewer_;
    crate::faav::one_time_sav_prnt ( Some ( read_prnt() ) );
    if let crate::enums::smart_lags::too_small_lag( x ) = crate::smart_lags::fork_lag_mcs_verbose( 70_000 ) { return false; }
    let term_app_screen = take_list_adr("term_app_screen");
    let alt_lnk = split_once( &cmd, " ");
    drop_ls_mode();
    crate::set_ask_user(cmd.as_str(), func_id);
    let mut lc = "ru_RU.UTF-8".to_string();
    if checkArg("-lc"){lc = String::from_iter(get_arg_in_cmd("-lc").s).trim_end_matches('\0').to_string()}
    let (cols, rows) = termion::terminal_size().unwrap();
    let cols = 680; let rows = 700;
    taken_term_msg();
    let adr_of_term_msg = adr_term_msg();
    let pwd = crate::core18::full_escape ( &read_file("env/cd") );
    let cmd = format!("clear;reset;cd {pwd};{cmd} > {term_app_screen}; echo 'free' > {adr_of_term_msg}");
    //let cmd = format!("{cmd} 0 > {fstdin_link} 1 > {fstdout}");
    let path_2_cmd = crate::mk_cmd_file(cmd);
        let mut pid: nix::unistd::Pid; 
    //    ( &format! ("bash -c {path_2_cmd}") ); 
      if let Ok ( res ) = crate::threadpool::new_thr ( &format! ("bash -c {path_2_cmd}") ) {
        match res {
            ForkResult::Parent { child } => {pid = child; },
            _ => { std::process::abort(); return false; }
        }
    } else { std::process::abort(); return false }
 let mut buf: [u8; 128] = [0; 128];
    //let mut read_out0 = crate::BufReader::new(out_out);
   // let mut fstd_in0 = crate::File::create(fstd_in).unwrap();
   let mut op_status = false;
   /*println!("press Enter, Please" );
    let enter: [u8; 1] = [13; 1];
    let mut writeIn_stdin = unsafe {std::fs::File::from_raw_fd(0/*stdin*/)};
    writeIn_stdin.write(&enter); */
   let mut pause_operation = false;
   let mut fst = true;
   let mut key: String = "".strn(); //= getkey().to_lowercase(); 
   let mut proc_id = get_pid_by_name( &alt_lnk.0 );
   let mut count_down = 20;
   while proc_id.is_none (){
    proc_id = get_pid_by_name( &alt_lnk.0 );
    count_down.dec();
    if count_down == 0 {break; }
   }
   if proc_id.is_none () { errMsg0("Sorry, Dear User, no operation was run - Please, hit any key to continue.. Thx."); return false; }
   let proc_id = proc_id.unwrap_or_else (|| { i32::MIN }) ;
   crate::pg18::reset_screen();
   println!("proc id {:?}, proc name {}", proc_id, &alt_lnk.0 );
let abort = std::thread::spawn(move|| {
    let mut count_out = 0;
   while key != "k" {
    if !fst { key = getkey().to_lowercase() };
    fst = false;
    if read_term_msg() == "free" {op_status = true; break;}
       if key == "p"{
        unsafe {
            if !pause_operation{kill (Pid::from_raw (proc_id), nix::sys::signal::SIGSTOP ); 
                println!("Operation paused."); popup_msg("pause"); pause_operation = true; continue;}
            else{kill ( Pid::from_raw (proc_id),  nix::sys::signal::SIGCONT ); popup_msg("continue"); pause_operation = false;}
        }
       }
       if "k" == key { break; }
       //if stop_op { break; }
       let update_screen = read_file_abs_adr( &term_app_screen );
       println!("{update_screen}\npress k or K to abort operation\nHit P or p to pause."); count_out += 1;
       if count_out > 20 { return;}
   }
  if !op_status{println!("Operation aborted")}; 
if get_pid_by_name( &alt_lnk.0 ).is_some () {
    unsafe{
        kill ( Pid::from_raw (proc_id), nix::sys::signal::SIGABRT ); kill ( Pid::from_raw (proc_id), nix::sys::signal::SIGKILL );
    }
}
}); abort.join().unwrap ();
   // crate::cmd_keys::drop_ext_modes ( Some (true) );
println!("Dear User, Please, hit any key to continue.. Thanks.");
getkey();
crate::smart_lags::fork_lag_mcs_verbose(10);
true
}
pub(crate) fn run_term_app_interactive_basic_4_group(cmd: &String, groupID: &String) -> bool{
    let func_id = crate::func_id18::run_cmd_viewer_;
    crate::faav::one_time_sav_prnt ( Some ( read_prnt() ) );
   // if let crate::enums::smart_lags::too_small_lag( x ) = crate::smart_lags::fork_lag_mcs_verbose( 70_000 ) { return false; }
   
    let term_app_screen = take_list_adr("term_app_screen");
    drop_ls_mode();
    crate::set_ask_user(cmd.as_str(), func_id);
    let mut lc = "ru_RU.UTF-8".to_string();
    if checkArg("-lc"){lc = String::from_iter(get_arg_in_cmd("-lc").s).trim_end_matches('\0').to_string()}
    let (cols, rows) = termion::terminal_size().unwrap();
    let cols = 680; let rows = 700;
    taken_term_msg();
    let adr_of_term_msg = adr_term_msg();
    let pwd = crate::core18::full_escape ( &read_file("env/cd") );
    let (procName, _ ) = split_once(&cmd, " ");
    let procName = crate::read_tail(&procName, "/");
    //let cmd = format!("clear;reset;cd {pwd};{cmd}&pkill -stop {procName}; echo 'taken' > {adr_of_term_msg}");
    //let cmd = format!("{cmd} 0 > {fstdin_link} 1 > {fstdout}");
    //let path_2_cmd = crate::mk_cmd_file(cmd);
        let mut pid_kid: nix::unistd::Pid; 
    //    ( &format! ("bash -c {path_2_cmd}") ); 
    //  if let Ok ( res ) = crate::threadpool::new_thr ( &format! ("bash -c {path_2_cmd}") ) {
      if let Ok ( res ) = crate::threadpool::new_thr_no_bash ( &cmd  ) {
        match res {
            ForkResult::Parent { child } => {pid_kid = child; },
            _ => { std::process::abort(); return false; }
        }
    } else { std::process::abort(); return false }
 let mut buf: [u8; 128] = [0; 128];
    //let mut read_out0 = crate::BufReader::new(out_out);
   // let mut fstd_in0 = crate::File::create(fstd_in).unwrap();
   let mut op_status = false;
   /*println!("press Enter, Please" );
    let enter: [u8; 1] = [13; 1];
    let mut writeIn_stdin = unsafe {std::fs::File::from_raw_fd(0/*stdin*/)};
    writeIn_stdin.write(&enter); */
   let mut pause_operation = false;
   let mut fst = true;
   let mut key: String = "".strn(); //= getkey().to_lowercase(); 
   let mut proc_id= i32::MIN;
   let mut ppid = i32::MIN;
   let mut count_down = 7;
   
  // if ppid == i32::MIN { errMsg0("Sorry, Dear User, no operation was run - Please, hit any key to continue.. Thx."); return false; }
  // crate::pg18::reset_screen();
  proc_id = pid_kid.into ();
   let groupID_cpy = groupID.strn();
   let groupID_cpy1 = groupID.strn();
   println!("proc id {}, proc name {}", proc_id, groupID );
   kill ( Pid::from_raw (proc_id),  nix::sys::signal::SIGCONT );
let abort = std::thread::spawn(move|| {
    let mut count_out = 0;
   loop {
    if !fst { key = getkey().to_lowercase() };
    fst = false;
    if read_term_msg() == "free" {op_status = true; break;}
    if !check_alive_proc_by_pid( proc_id ) { crate::faav::proc_exited( Some ( true ) ); break; }
       if key == "p"{
        unsafe {
            if !pause_operation{kill (Pid::from_raw (proc_id), nix::sys::signal::SIGSTOP ); 
                println!("Operation paused."); popup_msg("pause"); pause_operation = true; continue;}
            else{kill ( Pid::from_raw (proc_id),  nix::sys::signal::SIGCONT ); popup_msg("continue"); pause_operation = false;}
        }
       }
    if "k" == key { 
        match std::fs::remove_file (&groupID_cpy) {Ok (fs) => fs, _ => {} }; 
        match std::fs::remove_dir_all (&groupID_cpy) {Ok (fs) => fs, _ => { bash_unlink( &groupID_cpy); } };
        kill ( Pid::from_raw (proc_id), nix::sys::signal::SIGABRT ); kill ( Pid::from_raw (proc_id), nix::sys::signal::SIGKILL );
        /*while let Some(proc_id) = get_pid_by_name( &groupID_cpy1.clone() ) {
            unsafe{
                kill ( Pid::from_raw (proc_id), nix::sys::signal::SIGABRT ); kill ( Pid::from_raw (proc_id), nix::sys::signal::SIGKILL );
            } 
        }*/

    crate::faav::kill_prox_chain( Some ( true ) ); break; }
       //if stop_op { break; }
       let update_screen = read_file_abs_adr( &term_app_screen );
       println!("proc id {proc_id} {update_screen}\npress k or K to abort operation\nHit P or p to pause."); count_out += 1;
      // if count_out > 20 { return;}
   }
}); //abort.join().unwrap ();
let proc_id1 = proc_id;
let check_alive_thr = std::thread::spawn ( move || {
    use crate::smart_lags::mamed_mutexes;
    dbg!("tst");
    while check_alive_proc_by_pid( proc_id1 ) {
        delay_mcs( 7711 );
    } 
    crate::atomic_op::stdin_write(Some ("k") );
   // crate::atomic_op::stdin_write:: < &str> (None ); 
}); check_alive_thr.join().unwrap ();
   // crate::cmd_keys::drop_ext_modes ( Some (true) );
save_file_abs_adr0("free".strn(), adr_of_term_msg);
if crate::faav::proc_exited( None ) { return true }
//if op_status{println!("Operation aborted"); return false; }
if crate::faav::kill_prox_chain( None ) {println!("Operation/Chain killed"); return false; }
crate::smart_lags::fork_lag_mcs_verbose(10);
true
}
pub fn run_proc_by_proc (cmd: &String, groupID: &String){
    let mut groupID = groupID.strn();
    let mut prox_lst: Vec < String > = Vec::new ();
    let mut cmd = cmd.strn();
    let mut proc = "".strn();
    (proc, cmd) = crate::term_app::add_interactive_mode_to_cmd0( &cmd );
    if proc != "" {prox_lst.push (proc.clone () );}
    loop {
        (proc, cmd) = crate::term_app::add_interactive_mode_to_cmd( &cmd );
        if proc != "" {prox_lst.push (proc.clone () );}
        if cmd == "" { break; }
    }
    for proc in prox_lst {
        let ret = run_term_app_interactive_basic_4_group( &proc, &groupID);
       // dbg! (&ret);
        if !ret { break; }
      
    }
    println!("Dear User, Please, hit any key to continue.. Thanks.");
getkey();
}
pub(crate) fn run_term_app_ren(cmd: String) -> bool{
let func_id = crate::func_id18::run_cmd_viewer_;
drop_ls_mode();
crate::set_ask_user(cmd.as_str(), func_id);
let mut lc = "ru_RU.UTF-8".to_string();
if checkArg("-lc"){lc = String::from_iter(get_arg_in_cmd("-lc").s).trim_end_matches('\0').to_string()}
let (cols, rows) = termion::terminal_size().unwrap();
let cols = 680; let rows = 700;
let fstdout: String; 
let mut stderr_path = "stderr".to_string();
stderr_path = format!("{}stderr_term_app", unsafe{crate::ps18::page_struct("", crate::ps18::MAINPATH_, -1).str_});
crate::core18::errMsg_dbg(&stderr_path, func_id, -1.0);
#[cfg(not(feature = "mae" ))] let fstderr = crate::File::create(stderr_path).unwrap();
#[cfg(feature = "mae")] crate::mae::mk_empty_fil0(&stderr_path);
#[cfg(feature = "mae")] use Mademoiselle_Entropia::help_funcs::get_file;
#[cfg(feature = "mae")] let fstderr = match get_file(&stderr_path){Ok(f) => f, Err(e) => {crate::errMsg0(&format!("{e:?}") ); return false}};

//unblock_fd(fstdin0.as_raw_fd());
//let mut fstdout0 = io::BufReader::new(fstdout0);
//errMsg_dbg(&in_name, func_id, -1.0);
taken_term_msg();
let adr_of_term_msg = adr_term_msg();
let pwd = read_file("env/cd");
let cmd = format!("clear;reset;cd {pwd};{cmd} 2>&1; echo 'free' > {adr_of_term_msg}");
//let cmd = format!("{cmd} 0 > {fstdin_link} 1 > {fstdout}");
let path_2_cmd = crate::mk_cmd_file(cmd);
let (mut out_out, mut out_in) = os_pipe::pipe().unwrap();
let (mut in_out, mut in_in) = os_pipe::pipe().unwrap();
let err_msg = format!("run_term_app_ren cmd path {path_2_cmd}, fstderr {fstderr:#?}");
let mut run_command = Command::new("bash").arg("-c").arg(path_2_cmd)//.arg(";echo").arg(stopCode)
//let run_command = Command::new(cmd)
    .env("LC_ALL", &lc)
    .env("LANG", lc)
   // .current_dir(pwd)
   // .stderr(fstderr)
//    .stdout(out_in)//(std::process::Stdio::piped())
  //  .stdin(in_out)//(std::process::Stdio::piped())
    .spawn()
    .expect(&err_msg);

let abort = std::thread::spawn(move|| {
    let mut buf: [u8; 128] = [0; 128];
    //let mut read_out0 = crate::BufReader::new(out_out);
   // let mut fstd_in0 = crate::File::create(fstd_in).unwrap();
   let mut op_status = false;
   println!("press Enter");
    let enter: [u8; 1] = [13; 1];
    //let mut writeIn_stdin = unsafe {std::fs::File::from_raw_fd(0/*stdin*/)};
   // writeIn_stdin.write(&enter);
   let mut pause_operation = false;
   let mut fst = true;
   let mut key: String = getkey().to_lowercase(); 
   while key != "k" {
    if !fst{key = getkey().to_lowercase();}
    fst = false;
    if read_term_msg() == "free" {op_status = true; break;}
       if key == "p"{
        if !pause_operation{kill_proc_w_pid0(&get_pid_by_dummy(&ending("")), "-STOP"); println!("Operation paused."); popup_msg("pause"); pause_operation = true; continue;}
        else{kill_proc_w_pid0(&get_pid_by_dummy(&ending("")), "-CONT"); popup_msg("continue"); pause_operation = false;}
       }
       println!("press k or K to abort operation\nHit P or p to pause.");
   }
  if !op_status{println!("Operation aborted")};
run_command.kill();
//unsafe{libc::kill(g, SIGKILL)}
kill_proc_w_pid0(&get_pid_by_dummy(&ending("")), "-9")
}); abort.join();
println!("Dear User, Please, hit any key to continue.. Thanks.");
getkey();
true
}
pub(crate) fn run_term_app1(cmd: String) -> bool{
let func_id = crate::func_id18::run_cmd_viewer_;
drop_ls_mode();
    if crate::term_app::run_new_win_bool( None) { crate::term_app::new0__(&cmd); return true}
let mut lc = "ru_RU.UTF-8".to_string();
if checkArg("-lc"){lc = String::from_iter(get_arg_in_cmd("-lc").s).trim_end_matches('\0').to_string()}
crate::set_ask_user(cmd.as_str(), func_id);
{dont_scrn_fix(true); no_view(true, true);}
let fstdout: String; 
let mut stderr_path = "stderr".to_string();
stderr_path = format!("{}stderr_term_app", unsafe{crate::ps18::page_struct("", crate::ps18::MAINPATH_, -1).str_});
crate::core18::errMsg_dbg(&stderr_path, func_id, -1.0);
let fstderr = crate::File::create(stderr_path).unwrap();
//unblock_fd(fstdin0.as_raw_fd());
//let mut fstdout0 = io::BufReader::new(fstdout0);
//errMsg_dbg(&in_name, func_id, -1.0);
let pwd = read_file("env/cd");
let cmd = format!("cd {pwd};{cmd} 2>&1");
//let cmd = format!("{cmd} 0 > {fstdin_link} 1 > {fstdout}");
let path_2_cmd = crate::mk_cmd_file(cmd);
let (mut out_out, mut out_in) = os_pipe::pipe().unwrap();
let (mut in_out, mut in_in) = os_pipe::pipe().unwrap();
let mut run_command = Command::new("bash").arg("-c").arg(path_2_cmd)//.arg(";echo").arg(stopCode)
//let run_command = Command::new(cmd)
    .env("LC_ALL", &lc) //"ru_RU.UTF-8")
    .env("LANG", lc)
    .stderr(fstderr)
//    .stdout(out_in)//(std::process::Stdio::piped())
  //  .stdin(in_out)//(std::process::Stdio::piped())
    .spawn()
    .expect("can't run command in run_term_app1");

 std::thread::spawn(move|| {
run_command.wait();
//save_file_append("\nexit rw_std".to_string(), "logs".to_string());
}).join();
println!("Dear User, Please, hit any key to continue.. Thanks.");
getkey();
{dont_scrn_fix(true); no_view(true, false);}
true
}
pub(crate) fn run_term_app(cmd: String) -> bool{
let func_id = crate::func_id18::run_cmd_viewer_;
let mut lc = "ru_RU.UTF-8".to_string();
drop_ls_mode();
    if crate::term_app::run_new_win_bool( None) { crate::term_app::run_new_win_bool( Some( false ) ); crate::term_app::new0__(&cmd); return true; }
if checkArg("-lc"){lc = String::from_iter(get_arg_in_cmd("-lc").s).trim_end_matches('\0').to_string()}
crate::set_ask_user(cmd.as_str(), func_id);
{dont_scrn_fix(true); no_view(true, true);}
let fstdout: String; 
let mut stderr_path = "stderr".to_string();
stderr_path = format!("{}stderr_term_app", unsafe{crate::ps18::page_struct("", crate::ps18::MAINPATH_, -1).str_});
crate::core18::errMsg_dbg(&stderr_path, func_id, -1.0);
let fstderr = crate::File::create(stderr_path).unwrap();
//unblock_fd(fstdin0.as_raw_fd());
//let mut fstdout0 = io::BufReader::new(fstdout0);
//errMsg_dbg(&in_name, func_id, -1.0);
let pwd = read_file("env/cd");
let cmd = format!("clear;reset;cd {pwd};{cmd} 2>&1");
//let cmd = format!("{cmd} 0 > {fstdin_link} 1 > {fstdout}");
let path_2_cmd = crate::mk_cmd_file(cmd);
let (mut out_out, mut out_in) = os_pipe::pipe().unwrap();
let (mut in_out, mut in_in) = os_pipe::pipe().unwrap();
let mut run_command = Command::new("bash").arg("-c").arg(path_2_cmd)//.arg(";echo").arg(stopCode)
//let run_command = Command::new(cmd)
    .env("LC_ALL", &lc) //"ru_RU.UTF-8")
    .env("LANG", lc)
    .stderr(fstderr)
//    .stdout(out_in)//(std::process::Stdio::piped())
  //  .stdin(in_out)//(std::process::Stdio::piped())
    .spawn()
    .expect("can't run command in run_term_app1");

 std::thread::spawn(move|| {
run_command.wait();
//save_file_append("\nexit rw_std".to_string(), "logs".to_string());
}).join();
println!("Dear User, Please, hit any key to continue.. Thanks.");
getkey();
{dont_scrn_fix(true); no_view(true, false);}
true
}
pub(crate) fn tui_or_not(cmd: String, fname: &mut String) -> bool{
    if check_known_cmd(&cmd, "nano"){return true;}
    if check_known_cmd(&cmd, "vim"){return true;}
    if check_known_cmd(&cmd, "nvim"){return true;}
    if check_known_cmd(&cmd, "nvim.app"){return true;}
    if check_known_cmd(&cmd, "vi"){return true;}
    if check_known_cmd(&cmd, "mc "){
        if !is_dir(fname){
            //*fname = crate::Path::new(&fname).parent().unwrap().to_str().unwrap().to_string();
            tailOFF(fname, "/");
        }
        return true;}
    false
}
pub fn run_new_win_bool (toggle: Option< bool >) -> bool {
    static mut state: bool = false;
    unsafe {
        if let Some (x) = toggle {
            state = x;
        } state
    }
}
pub(crate) fn check_known_cmd(cmd:&String, name: &str) -> bool{
    let cmd0 = cmd.trim_start_matches( name );
    if cmd0.len() < cmd.len(){return true} 
    false
}
pub(crate) fn term(cmd: &String){
    let mut cmd = cmd.trim_start().strn();
    if edit_mode_lst(None) {return; }
    if read_term_msg() == "stop"{return;}
    else {taken_term_msg()}
    if crate::term_app::run_new_win_bool( None) { crate::term_app::new0__(&cmd); }
    if cmd.substring(0, 2) == ">_"{cmd = cmd.replace(">_", "term") }
    let mut subcmd = "".to_string();
     if crate::globs18::check_substrn(&cmd, ":>:"){(cmd, subcmd) = split_once(&cmd, ":>:");}
    //let (_, cmd) = split_once(&cmd, " ");
    let cmd0 = "term rsync".strn();
    if cmd.substring(0, cmd0.len() ) == cmd0 {dbg! ("tst"); crate::term_rsync(&cmd ); return;}
    if cmd.substring(0, 7) == "term mv"{crate::term_mv(&cmd); return;}
    if cmd.substring(0, 7) == "term cp"{crate::term_cp(&cmd); return;}
    if cmd.substring(0, 7) == "term rm"{crate::term_rm(&cmd); return;}
    if default_term_4_shol_a(&cmd){return}
    let state = dont_scrn_fix(false).0; if state {dont_scrn_fix(true);}
    let (_, cmd) = crate::split_once_or_ret_null_strns(&cmd, " "); 
    run_term_app(cmd.trim_start().trim_end().strn());
}
pub fn id_of_child_win () -> usize {
    static mut id: usize = 0;
    unsafe {
        let ret = id; id.inc(); return ret;
    }
}
pub fn run_cmd_in_extra_interactive_mode (cmd: &String) {
    crate::term_app::run_proc_by_proc(&cmd, &"".strn()); //*/
    
}
pub fn add_interactive_mode_to_cmd0 (cmd: &String) -> (String, String ) {
    let mut cmd = cmd.strn();
    if cmd.substring (0, 5) == "iterm" { cmd = cmd.substring (5, cmd.len() ).strn () ;}
    if cmd.substring (0, 3) == "i>_" {cmd = cmd.substring (3, cmd.len() ).strn () ; }
    let cmd = cmd.trim_start().strn ();
    let ( mut chunk_of_op, nxt_ops ) = split_once_or_ret_null_strns( &cmd, ";"); 
    let (cmd, _) = split_once_or_ret_null_strns( &chunk_of_op, " ");
    //if cmd == "" { return (mode_cmd, cmd); }
    //crate::mk_dummy_lnk_( &cmd)
    chunk_of_op = format!("{} {}", full_path_to_cmd( &cmd ), chunk_of_op ); 
    ( chunk_of_op, nxt_ops )
}
pub fn add_interactive_mode_to_cmd (cmd: &String) -> (String, String ) {
    let cmd = cmd.trim_start().trim_end ().strn ();
    let ( mut chunk_of_op, nxt_ops ) = split_once_or_ret_null_strns( &cmd, ";"); 
    let (cmd, _) = split_once_or_ret_null_strns( &chunk_of_op, " ");
    //if cmd == "" { return (mode_cmd, cmd); }
    //crate::mk_dummy_lnk_( &cmd)
    chunk_of_op = format!("{} {}", full_path_to_cmd( &cmd ), chunk_of_op ); 
    ( chunk_of_op, nxt_ops )
}
pub(crate) fn new0__ (cmd: &String){
    let mut cmd = cmd.trim_start().strn();
    if edit_mode_lst(None) {return; }
    if read_term_msg() == "stop"{return;}
    else {taken_term_msg()}
    let mut cmd = cmd.trim_start_matches("new ").strn();
    let mut subcmd = "".to_string();
    if crate::globs18::check_substrn(&cmd, ":>:"){(cmd, subcmd) = split_once(&cmd, ":>:");}
    //if cmd.substring(0, 7) == "term mv"{crate::term_mv(&cmd); return;}
    //if cmd.substring(0, 7) == "term cp"{crate::term_cp(&cmd); return;}
    //if cmd.substring(0, 7) == "term rm"{crate::term_rm(&cmd); return;}
    if default_term_4_shol_a(&cmd){return}
    let state = dont_scrn_fix(false).0; if state {dont_scrn_fix(true);}
    let (app_name, _ ) = split_once( &cmd, " " );
    let prefix = format! ( "kid.{}.{}{}.{}", id_of_child_win (), crate::globs18::id_suffix(), cur_win_id ( None ), app_name );
    let prnt_prefix_2_title = crate::mk_cmd_file_dirty( format!(r"echo -e '\033]30;{prefix}\007'"  ) );
    let cmd = format!( "{} '{prnt_prefix_2_title};{cmd}'", konsole ( None ) );
    let path_2_cmd = mk_cmd_file_dirty( format! ("{cmd}" ) );
    let cmd = format!("/bin/bash -c {path_2_cmd}",  );
   // run_term_app(cmd.trim_start().trim_end().strn());
   crate::save_file0(cmd.clone(), "fn_new0__".strn() );
   crate::threadpool::new_thr(&cmd ); return;
   }
pub(crate) fn new2__ (cmd: &String){
    let mut cmd = cmd.trim_start().strn();
    if edit_mode_lst(None) {return; }
    if read_term_msg() == "stop"{return;}
    else {taken_term_msg()}
    let mut cmd = cmd.trim_start_matches("new ").strn();
    let mut subcmd = "".to_string();
    if crate::globs18::check_substrn(&cmd, ":>:"){(cmd, subcmd) = split_once(&cmd, ":>:");}
    //if cmd.substring(0, 7) == "term mv"{crate::term_mv(&cmd); return;}
    //if cmd.substring(0, 7) == "term cp"{crate::term_cp(&cmd); return;}
    //if cmd.substring(0, 7) == "term rm"{crate::term_rm(&cmd); return;}
    if default_term_4_shol_a(&cmd){return}
    let state = dont_scrn_fix(false).0; if state {dont_scrn_fix(true);}
    let (app_name, _ ) = split_once( &cmd, " " );
    let prefix = format! ( "kid.{}.{}{}.{}", id_of_child_win (), crate::globs18::id_suffix(), cur_win_id ( None ), app_name );
    let prnt_prefix_2_title = crate::mk_cmd_file_dirty( format!(r"echo -e '\033]30;{prefix}\007'"  ) );
    let cmd = format!( "{} '{prnt_prefix_2_title};{cmd}'&", konsole ( None ) );
    let path_2_cmd = mk_cmd_file_dirty( format! ("{cmd}" ) );
    let cmd = format!("{path_2_cmd}");
    println!( "{cmd}" );
   // run_term_app(cmd.trim_start().trim_end().strn());
    let fstdout: String;  let func_id = -617506194i64;
    let mut stderr_path = "stderr".to_string();
    stderr_path = format!("{}stderr", unsafe{crate::ps18::page_struct("", crate::ps18::MAINPATH_, -1).str_});
    crate::core18::errMsg_dbg(&stderr_path, func_id, -1.0);
    let fstderr = crate::File::create(stderr_path).unwrap();
    let fstdout0 = crate::File::open("/dev/null").unwrap();
    //let mut fstdout0 = io::BufReader::new(fstdout0);
    //errMsg_dbg(&in_name, func_id, -1.0);
    crate::threadpool::new_thr(&cmd ); return;
    let run_command = Command::new( "bash" ).arg( "-c" ).arg(cmd)//.arg(";echo").arg(stopCode)
    //let run_command = Command::new(cmd)
        .stderr(fstderr)
        .stdout(fstdout0)
        .spawn()
        .expect("can't run command in run_cmd_viewer");
    /*if run_command.status.success(){
        io::stdout().write_all(&run_command.stdout).unwrap();
        io::stderr().write_all(&run_command.stderr).unwrap();
        return false;
    }*/
    return
}

pub(crate) fn new1__ (cmd: &String){
    let mut cmd = cmd.trim_start().strn();
    if edit_mode_lst(None) {return; }
    if read_term_msg() == "stop"{return;}
    else {taken_term_msg()}
    let mut cmd = cmd.trim_start_matches("new ").strn();
    let mut subcmd = "".to_string();
    if crate::globs18::check_substrn(&cmd, ":>:"){(cmd, subcmd) = split_once(&cmd, ":>:");}
    //if cmd.substring(0, 7) == "term mv"{crate::term_mv(&cmd); return;}
    //if cmd.substring(0, 7) == "term cp"{crate::term_cp(&cmd); return;}
    //if cmd.substring(0, 7) == "term rm"{crate::term_rm(&cmd); return;}
    if default_term_4_shol_a(&cmd){return}
    let state = dont_scrn_fix(false).0; if state {dont_scrn_fix(true);}
    let cmd = format! ("{} {cmd}", konsole( None ) );
    let cmd = cmd.replace_unesc_ch(";", &format! ("& {} ", konsole(None ) ) );
    let cmd = format!("{cmd}&");
    println!( "{cmd}" );
   // run_term_app(cmd.trim_start().trim_end().strn());
    let fstdout: String;  let func_id = -617506194i64;
    let path_2_cmd = crate::mk_cmd_file_dirty(cmd);
    let mut stderr_path = "stderr".to_string();
    stderr_path = format!("{}stderr", unsafe{crate::ps18::page_struct("", crate::ps18::MAINPATH_, -1).str_});
    crate::core18::errMsg_dbg(&stderr_path, func_id, -1.0);
    let fstderr = crate::File::create(stderr_path).unwrap();
    let fstdout0 = crate::File::open("/dev/null").unwrap();
    //let mut fstdout0 = io::BufReader::new(fstdout0);
    //errMsg_dbg(&in_name, func_id, -1.0);
    let run_command = Command::new("bash").arg("-c").arg(path_2_cmd)//.arg(";echo").arg(stopCode)
    //let run_command = Command::new(cmd)
        .stderr(fstderr)
        .stdout(fstdout0)
        .spawn()
        .expect("can't run command in run_cmd_viewer");
    /*if run_command.status.success(){
        io::stdout().write_all(&run_command.stdout).unwrap();
        io::stderr().write_all(&run_command.stderr).unwrap();
        return false;
    }*/
    return
}
pub fn konsole (cmd: Option< String >) -> String {
    static mut term: Lazy< String > = Lazy::new( || {"konsole --hold -e bash -c".strn() });
    unsafe {
        if let Some( x ) = cmd { *term = x} term.strn()
    }
}
pub(crate) fn process_tag(key: String){
    let valid: String = match key.as_str(){
        "#" => key.as_str(),
        "0" => key.as_str(),
        "1" => key.as_str(),
        "2" => key.as_str(),
        "3" => key.as_str(),
        "4" => key.as_str(),
        "5" => key.as_str(),
        "6" => key.as_str(),
        "7" => key.as_str(),
        "8" => key.as_str(),
        "9" => key.as_str(),
        _ => return validate_tag(key)
   }.to_string();
    save_file_append(valid, "tag".to_string());
}
pub(crate) fn validate_tag(key: String){
    let mut prnt = read_prnt();
    let mut tag = read_file("tag");
    let tag0 = tag.clone();
    tag = tag.replace("##", "");
    let tag = match i64::from_str_radix(&tag, 10){
        Ok(i) => i,
        _ => i64::MIN
    };
    if tag == i64::MIN{
        prnt = prnt.replace("sl:", "");
        set_prnt(&prnt, -48721112507);
        let tag = take_list_adr("tag");
        rm_file(&tag);
        return;
    }
    let tag = get_item_from_front_list(tag, true);
    prnt = prnt.replace(&tag0, &tag);
    prnt = prnt.replace("sl:", "");
    set_prnt(&prnt, -48721112507);
    let tag = take_list_adr("tag");
    rm_file(&tag);
}
pub(crate) fn shol_on(){
    let tag = take_list_adr("tag");
    rm_file(&tag);
    let prnt = read_prnt();
    let prnt = format!("sl:{prnt}");
    set_prnt(&prnt, 59841774);
}
pub(crate) fn stop_term_msg(){
    save_file0("stop".to_string(), "msgs/term/state".to_string());
}
pub(crate) fn free_term_msg(){
    save_file0("free".to_string(), "msgs/term/state".to_string());
}
pub(crate) fn taken_term_msg(){
    save_file0("taken".to_string(), "msgs/term/state".to_string());
}
pub(crate) fn read_term_msg() -> String{
    read_file("msgs/term/state")
}
pub(crate) fn adr_term_msg() -> String{
    take_list_adr("msgs/term/state")
}
pub(crate) fn mk_dummy_file() -> String{
    save_file0("".to_string(), "msgs/term/dummy_file_4_id".to_string());
    take_list_adr("msgs/term/dummy_file_4_id")
}
pub(crate) fn mk_empty_file(name: &String){
    match std::fs::remove_file(name){Ok (f) => {}, _ => {} }; 
    crate::save_file_abs_adr("".strn(), name.strn());
}
pub(crate) fn get_pid_by_dummy(ending: &str) -> String{
    let dummy = mk_dummy_file();
    let cmd = format!("ps -eo pid,args|grep -Ei 'tam.*dummy'|grep -Eio '[0-9]+\\s+{ending}'|grep -Eo '[0-9]+'");
#[cfg(feature="in_dbg")]
{ crate::report(&cmd, "pid of dummy"); println!("pid of dummy {}", cmd); }
    run_cmd_out_sync(cmd)
}
pub(crate) fn ending(sav: &str) -> String{
    static mut save: Lazy<String> = Lazy::new(||{String::new()});
    if sav != ""{unsafe {save.clear(); save.push_str(sav);}}
    unsafe{let ret: String = save.to_string(); ret}
}
pub(crate) fn kill_proc_w_pid0(pid: &String, sig: &str){
    run_cmd_out_sync(format!("kill {sig} {pid}"));
}
//fn
/*
use nix::sys::signal::kill;
use nix::unistd::Pid;

if let Err(Errno::ESRCH) = kill(Pid::from_raw(99999), None) {
    println!("Process does not exist");
}
*/
