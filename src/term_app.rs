use std::process::Command;
use std::io::{Write, Read};
use std::thread::Builder;
use std::os::fd::AsRawFd;
use std::io::BufRead;
use std::io::prelude::*;
use termion::terminal_size;
//use close_file::Closable;
use std::mem::drop;
use crate::globs18::unblock_fd;
use crate::{run_cmd_out, popup_msg, getkey, cpy_str, save_file, save_file_append, tailOFF, is_dir, split_once};
#[path = "keycodes.rs"]
mod kcode;
pub(crate) fn run_term_app(cmd: String) -> bool{
let func_id = crate::func_id18::run_cmd_viewer_;
crate::set_ask_user(cmd.as_str(), func_id);
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
let cmd = format!("clear;reset;{cmd}");
//let cmd = format!("{cmd} 0 > {fstdin_link} 1 > {fstdout}");
let path_2_cmd = crate::mk_cmd_file(cmd);
let (mut out_out, mut out_in) = os_pipe::pipe().unwrap();
let (mut in_out, mut in_in) = os_pipe::pipe().unwrap();
let mut run_command = Command::new("bash").arg("-c").arg(path_2_cmd)//.arg(";echo").arg(stopCode)
//let run_command = Command::new(cmd)
    .env("LC_ALL", "ru_RU.UTF-8")
    .env("LANG", "ru_RU.UTF-8")
    .stderr(fstderr)
//    .stdout(out_in)//(std::process::Stdio::piped())
  //  .stdin(in_out)//(std::process::Stdio::piped())
    .spawn()
    .expect("can't run command in run_term_app");

 std::thread::spawn(move|| {
    let mut buf: [u8; 128] = [0; 128];
    //let mut read_out0 = crate::BufReader::new(out_out);
   // let mut fstd_in0 = crate::File::create(fstd_in).unwrap();
run_command.wait();
save_file_append("\nexit rw_std".to_string(), "logs".to_string());
}).join();
true
}
pub(crate) fn run_term_app1(cmd: String) -> bool{
let func_id = crate::func_id18::run_cmd_viewer_;
crate::set_ask_user(cmd.as_str(), func_id);
let fstdout: String; 
let mut stderr_path = "stderr".to_string();
stderr_path = format!("{}stderr_term_app", unsafe{crate::ps18::page_struct("", crate::ps18::MAINPATH_, -1).str_});
crate::core18::errMsg_dbg(&stderr_path, func_id, -1.0);
let fstderr = crate::File::create(stderr_path).unwrap();
//unblock_fd(fstdin0.as_raw_fd());
//let mut fstdout0 = io::BufReader::new(fstdout0);
//errMsg_dbg(&in_name, func_id, -1.0);
let cmd = format!("{cmd}");
//let cmd = format!("{cmd} 0 > {fstdin_link} 1 > {fstdout}");
let path_2_cmd = crate::mk_cmd_file(cmd);
let (mut out_out, mut out_in) = os_pipe::pipe().unwrap();
let (mut in_out, mut in_in) = os_pipe::pipe().unwrap();
let mut run_command = Command::new("bash").arg("-c").arg(path_2_cmd)//.arg(";echo").arg(stopCode)
//let run_command = Command::new(cmd)
    .env("LC_ALL", "ru_RU.UTF-8")
    .env("LANG", "ru_RU.UTF-8")
    .stderr(fstderr)
//    .stdout(out_in)//(std::process::Stdio::piped())
  //  .stdin(in_out)//(std::process::Stdio::piped())
    .spawn()
    .expect("can't run command in run_term_app1");

 std::thread::spawn(move|| {
run_command.wait();
save_file_append("\nexit rw_std".to_string(), "logs".to_string());
}).join();
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
     let (cmd, subcmd) = split_once(&cmd, ":>:");
    let (_, cmd) = split_once(&cmd, " ");
    let cmd = cmd.trim_start().to_string();
    run_term_app(cmd);
}