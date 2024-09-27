use std::process::Command;
use std::io::{Write, Read}; use std::io::BufRead; use std::io::prelude::*;
use std::thread::Builder; use std::os::fd::AsRawFd; use std::os::fd::FromRawFd;
use libc::SIGKILL;
use termion::terminal_size;
use substring::Substring;
use once_cell::sync::Lazy;
use crate::custom_traits::{STRN, helpful_math_ops, escaped_chars};
//use close_file::Closable;
use std::mem::drop;
use crate::globs18::{check_strn_in_lst, cur_win_id, get_item_from_front_list, instance_num, take_list_adr, unblock_fd};
use crate::{checkArg, check_substr, cpy_str, default_term_4_shol_a, dont_scrn_fix, drop_ls_mode, edit_mode_lst, get_arg_in_cmd, getkey, is_dir, mk_cmd_file_dirty, no_view, popup_msg, read_file, read_prnt, rm_file, run_cmd_out, run_cmd_out_sync, save_file, save_file0, save_file_abs_adr0, save_file_append, save_file_append_newline, set_prnt, split_once, tailOFF, term_mv};
#[path = "keycodes.rs"]
mod kcode;
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
    if cmd.substring(0, 7) == "term mv"{crate::term_mv(&cmd); return;}
    if cmd.substring(0, 7) == "term cp"{crate::term_cp(&cmd); return;}
    if cmd.substring(0, 7) == "term rm"{crate::term_rm(&cmd); return;}
    if default_term_4_shol_a(&cmd){return}
    let state = dont_scrn_fix(false).0; if state {dont_scrn_fix(true);}
    let (_, cmd) = split_once(&cmd, " ");
    run_term_app(cmd.trim_start().trim_end().strn());
}
pub fn id_of_child_win () -> usize {
    static mut id: usize = 0;
    unsafe {
        let ret = id; id.inc(); return ret;
    }
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
