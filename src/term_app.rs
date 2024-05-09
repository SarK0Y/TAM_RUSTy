use std::process::Command;
use std::io::{Write, Read};
use std::thread::Builder;
use std::os::fd::AsRawFd;
use std::io::BufRead;
use std::io::prelude::*;
use libc::SIGKILL;
use termion::terminal_size;
use substring::Substring;
use once_cell::sync::Lazy;
//use close_file::Closable;
use std::mem::drop;
use crate::globs18::{unblock_fd, take_list_adr, get_item_from_front_list};
use crate::{run_cmd_out, popup_msg, getkey, cpy_str, save_file, save_file_append, tailOFF, is_dir, split_once, read_prnt, set_prnt, read_file, rm_file, checkArg, get_arg_in_cmd, term_mv, save_file0, dont_scrn_fix, run_cmd_out_sync};
#[path = "keycodes.rs"]
mod kcode;
pub(crate) fn run_term_app(cmd: String) -> bool{
let func_id = crate::func_id18::run_cmd_viewer_;
crate::set_ask_user(cmd.as_str(), func_id);
let mut lc = "ru_RU.UTF-8".to_string();
if checkArg("-lc"){lc = String::from_iter(get_arg_in_cmd("-lc").s).trim_end_matches('\0').to_string()}
let (cols, rows) = termion::terminal_size().unwrap();
let cols = 680; let rows = 700;
let fstdout: String; 
let mut stderr_path = "stderr".to_string();
stderr_path = format!("{}stderr_term_app", unsafe{crate::ps18::page_struct("", crate::ps18::MAINPATH_, -1).str_});
crate::core18::errMsg_dbg(&stderr_path, func_id, -1.0);
let fstderr = crate::File::create(stderr_path).unwrap();
//unblock_fd(fstdin0.as_raw_fd());
//let mut fstdout0 = io::BufReader::new(fstdout0);
//errMsg_dbg(&in_name, func_id, -1.0);
taken_term_msg();
let adr_of_term_msg = adr_term_msg();
let cmd = format!("clear;reset;{cmd} 2>&1; echo 'free' > {adr_of_term_msg}");
//let cmd = format!("{cmd} 0 > {fstdin_link} 1 > {fstdout}");
let path_2_cmd = crate::mk_cmd_file(cmd);
let (mut out_out, mut out_in) = os_pipe::pipe().unwrap();
let (mut in_out, mut in_in) = os_pipe::pipe().unwrap();
let mut run_command = Command::new("bash").arg("-c").arg(path_2_cmd)//.arg(";echo").arg(stopCode)
//let run_command = Command::new(cmd)
    .env("LC_ALL", &lc)
    .env("LANG", lc)
    .stderr(fstderr)
//    .stdout(out_in)//(std::process::Stdio::piped())
  //  .stdin(in_out)//(std::process::Stdio::piped())
    .spawn()
    .expect("can't run command in run_term_app");

let abort = std::thread::spawn(move|| {
    let mut buf: [u8; 128] = [0; 128];
    //let mut read_out0 = crate::BufReader::new(out_out);
   // let mut fstd_in0 = crate::File::create(fstd_in).unwrap();
   let mut op_status = false;
   while getkey().to_lowercase() != "k" {
    if read_term_msg() == "free" {op_status = true; break;}
       println!("press k or K")
   }
  if !op_status{println!("Operation aborted")};
run_command.kill();
//unsafe{libc::kill(g, SIGKILL)}
kill_proc_w_pid(&get_pid_by_dummy(&ending("")), "-9")
});
while read_term_msg() != "free" {
    std::thread::sleep(std::time::Duration::from_millis(150));
}
println!("Dear User, Please, hit any key to continue.. Thanks.");
getkey();
true
}
pub(crate) fn run_term_app1(cmd: String) -> bool{
let func_id = crate::func_id18::run_cmd_viewer_;
let mut lc = "ru_RU.UTF-8".to_string();
if checkArg("-lc"){lc = String::from_iter(get_arg_in_cmd("-lc").s).trim_end_matches('\0').to_string()}
crate::set_ask_user(cmd.as_str(), func_id);
let fstdout: String; 
let mut stderr_path = "stderr".to_string();
stderr_path = format!("{}stderr_term_app", unsafe{crate::ps18::page_struct("", crate::ps18::MAINPATH_, -1).str_});
crate::core18::errMsg_dbg(&stderr_path, func_id, -1.0);
let fstderr = crate::File::create(stderr_path).unwrap();
//unblock_fd(fstdin0.as_raw_fd());
//let mut fstdout0 = io::BufReader::new(fstdout0);
//errMsg_dbg(&in_name, func_id, -1.0);
let cmd = format!("{cmd} 2>&1");
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
true
}
pub(crate) fn tui_or_not(cmd: String, fname: &mut String) -> bool{
    if check_known_cmd(&cmd, "nano /"){return true;}
    if check_known_cmd(&cmd, "vim /"){return true;}
    if check_known_cmd(&cmd, "vi /"){return true;}
    if check_known_cmd(&cmd, "mc "){
        if !is_dir(fname){
            //*fname = crate::Path::new(&fname).parent().unwrap().to_str().unwrap().to_string();
            tailOFF(fname, "/");
        }
        return true;}
    false
}
pub(crate) fn check_known_cmd(cmd:&String, name: &str) -> bool{
    let cmd0 = cmd.replace(name, "");
    if cmd0.len() < cmd.len(){return true} 
    false
}
pub(crate) fn term(cmd: &String){
    if read_term_msg() == "stop"{return;}
    else {taken_term_msg()}
     let (cmd, subcmd) = split_once(&cmd, ":>:");
    //let (_, cmd) = split_once(&cmd, " ");
    let cmd = cmd.trim_start().to_string();
    if cmd.substring(0, 7) == "term mv"{crate::term_mv(&cmd); return;}
    if cmd.substring(0, 7) == "term cp"{crate::term_cp(&cmd); return;}
    let state = dont_scrn_fix(false).0; if state {dont_scrn_fix(true);}
    run_term_app(cmd.replace("term", "").trim_start().trim_end().to_string());
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
pub(crate) fn get_pid_by_dummy(ending: &str) -> String{
    let dummy = mk_dummy_file();
    let cmd = format!("ps -eo pid,args|grep '{dummy}'|grep -Eio '[0-9]+\\s+{ending}'|grep -Eo '[0-9]+'");
#[cfg(feature="in_dbg")]
{ crate::report(&cmd, "pid of dummy"); println!("pid of dummy {}", cmd); }
    run_cmd_out_sync(cmd)
}
pub(crate) fn kill_proc_w_pid(pid: &String, sig: &str){
    run_cmd_out_sync(format!("kill {sig} {pid}"));
}
pub(crate) fn ending(sav: &str) -> String{
    static mut save: Lazy<String> = Lazy::new(||{String::new()});
    if sav != ""{unsafe {save.clear(); save.push_str(sav);}}
    unsafe{let ret: String = save.to_string(); ret}
}
//fn