#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused)]
// #![feature(macro_metavar_expr)]
#![allow(unused_variables)]
#![allow(non_upper_case_globals)]
#![allow(while_true)]
#[allow(arithmetic_overflow)]
#[allow(temporary_cstring_as_ptr)]
mod exts;
use exts::*;
use globs18::{get_item_from_front_list, split_once_alt, strn_2_usize, take_list_adr};
use syn::token::Return;
use update18::delay_ms;

use crate::globs18::{get_proper_indx, get_proper_indx_tst};
#[cfg(feature ="mae")]
use Mademoiselle_Entropia::true_rnd::UID_UTF8;
use_all!();

pub(crate) fn split_once(in_string: &str, delim: &str) -> (String, String) {
    if delim.chars().count() > 1{return split_once_alt(&in_string.to_string(), &delim.to_string());}
let mut splitter = in_string.splitn(2, delim);
let first = match splitter.next(){
    Some(val) => val,
    _ => return ("none".to_string(), "none".to_string())
};
let second = match splitter.next(){
    Some(val) => val,
    _ => return (first.to_string(), "none".to_string())
};
return  (first.to_string(), second.to_string());
}
pub(crate) fn split_once_or_ret_null_strs(in_string: &str, delim: &str) -> (String, String) {
    if delim.chars().count() > 1{return split_once_alt(&in_string.to_string(), &delim.to_string());}
let mut splitter = in_string.splitn(2, delim);
let first = match splitter.next(){
    Some(val) => val,
    _ => return ("".to_string(), "".to_string())
};
let second = match splitter.next(){
    Some(val) => val,
    _ => return (first.to_string(), "".to_string())
};
return  (first.to_string(), second.to_string());
}
fn form_grep_cmd(in_name: &String) -> String{
    let mut ret: String = String::new();
    ret.push_str("grep ");
    let split = "go go";
    if core18::check_substr(in_name, "pass==", 0){
     //  let mut in_name = in_name.as_str();
       let in_name = in_name.replace("pass==", "");
       let (opts, name) = split_once(&in_name.as_str(), " ");
       let val4grep = format!("{} '{}'", opts, name);
       ret.push_str(&val4grep);
    }else{let val4grep = format!("'{}'", &in_name); ret.push_str(&val4grep)}
    return ret;
}
fn mk_cmd_file(cmd: String) -> String{
    let mut filter_cmd = String::new();
    if checkArg("-filter-cmd"){
        filter_cmd = String::from_iter(crate::get_arg_in_cmd("-filter-cmd").s);
        let filter = filter_cmd.clone();
        std::thread::spawn(move||{
        popup_msg(&filter);
        });
        let new = cmd.replace(&filter_cmd, "");
        if new != cmd{return "cmd was cleaned".to_string()}
    }
let func_id = 4;
let timestamp = Local::now();
let proper_timestamp = format!("{}", timestamp.format("%Y-%mm-%dd_%H-%M-%S_%f"));
let path_2_cmd = format!("{}/cmd{}.sh", unsafe{ps18::page_struct("", ps18::TMP_DIR_, func_id).str_}, proper_timestamp);
let err_msg = format!("failed create {}", &path_2_cmd);
let mut make_cmd_file = File::create(&path_2_cmd).expect(&err_msg.bold().red());
core18::errMsg_dbg(&path_2_cmd, func_id, -1.0);
let mut cmd = cmd;
if !dbg(false) && !dont_clean_bash(false){
    cmd = format!("{cmd};rm -f {}", path_2_cmd);
}
make_cmd_file.write_all(&cmd.as_bytes());
Command::new("chmod").arg("700").arg(&path_2_cmd).output().expect("");
core18::errMsg_dbg(&cmd, func_id, -1.0);
path_2_cmd.to_string()
}
pub(crate) fn run_cmd_str(cmd: &str) ->bool{return run_cmd_spawn(cmd.to_string());} 
pub fn run_cmd0(cmd: String) -> bool{
let func_id = 5;
let fstdout: String; 
let path_2_cmd = mk_cmd_file(cmd);
let mut stderr_path = "stderr".to_string();
stderr_path = format!("{}stderr", unsafe{ps18::page_struct("", ps18::MAINPATH_, -1).str_});
core18::errMsg_dbg(&stderr_path, func_id, -1.0);
let fstderr = File::create(stderr_path).unwrap();
//let mut fstdout0 = io::BufReader::new(fstdout0);
//errMsg_dbg(&in_name, func_id, -1.0);
let run_command = Command::new("bash").arg("-c").arg(path_2_cmd)//.arg(";echo").arg(stopCode)
//let run_command = Command::new(cmd)
    .stderr(fstderr)
    .output()
    .expect("can't run command in run_cmd");
if run_command.status.success(){
    io::stdout().write_all(&run_command.stdout).unwrap();
    io::stderr().write_all(&run_command.stderr).unwrap();
    return false;
}
true
}
pub fn run_cmd_out(cmd: String) -> String{
let func_id = 5;
let fstdout: String; 
let path_2_cmd = mk_cmd_file(cmd);
let mut stderr_path = "stderr".to_string();
stderr_path = format!("{}stderr", unsafe{ps18::page_struct("", ps18::MAINPATH_, -1).str_});
core18::errMsg_dbg(&stderr_path, func_id, -1.0);
let fstderr = File::create(stderr_path).unwrap();
//let mut fstdout0 = io::BufReader::new(fstdout0);
//errMsg_dbg(&in_name, func_id, -1.0);
let run_command = Command::new("bash").arg("-c").arg(path_2_cmd)//.arg(";echo").arg(stopCode)
//let run_command = Command::new(cmd)
    .stderr(fstderr).stdout(std::process::Stdio::piped())
    .output()
    .expect("can't run command in run_cmd");
if !run_command.status.success(){
    io::stdout().write_all(&run_command.stdout).unwrap();
    io::stderr().write_all(&run_command.stderr).unwrap();
    return match from_utf8(&run_command.stdout){
        Ok(s) => s.to_string(),
        _ => "".to_string()
    };
}
return match from_utf8(&run_command.stdout){
        Ok(s) => s.to_string(),
        _ => "".to_string()
    };
}
pub(crate) fn run_cmd_viewer(cmd: String) -> bool{
let func_id = func_id18::run_cmd_viewer_;
set_ask_user(cmd.as_str(), func_id);
let fstdout: String; 
let path_2_cmd = mk_cmd_file(cmd);
let mut stderr_path = "stderr".to_string();
stderr_path = format!("{}stderr", unsafe{ps18::page_struct("", ps18::MAINPATH_, -1).str_});
core18::errMsg_dbg(&stderr_path, func_id, -1.0);
let fstderr = File::create(stderr_path).unwrap();
let fstdout0 = File::open("/dev/null").unwrap();
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
true
}
pub fn run_cmd(cmd: String) -> bool{
let func_id = 5;
let fstdout: String; 
let path_2_cmd = mk_cmd_file(cmd);
let mut stderr_path = "stderr".to_string();
stderr_path = format!("{}stderr", unsafe{ps18::page_struct("", ps18::MAINPATH_, -1).str_});
let path_2_list_of_found_files = take_list_adr("found_files");
fstdout = String::from(path_2_list_of_found_files); 
core18::errMsg_dbg(&stderr_path, func_id, -1.0);
core18::errMsg_dbg(&fstdout, func_id, -1.0);
let fstderr = File::create(stderr_path).unwrap();
let fstdout0 = File::create(fstdout).unwrap();
globs18::unblock_fd(fstdout0.as_raw_fd());
//let mut fstdout0 = io::BufReader::new(fstdout0);
//errMsg_dbg(&in_name, func_id, -1.0);
let run_command = Command::new("bash").arg("-c").arg(path_2_cmd)//.arg(";echo").arg(stopCode)
//let run_command = Command::new(cmd)
    .stdout(fstdout0)
    .stderr(fstderr)
    .output()
    .expect("can't run command in run_cmd");
if run_command.status.success(){
    io::stdout().write_all(&run_command.stdout).unwrap();
    io::stderr().write_all(&run_command.stderr).unwrap();
    return false;
}
true
}
pub fn run_cmd_spawn(cmd: String) -> bool{
let func_id = 5;
let fstdout: String;
let path_2_cmd = mk_cmd_file(cmd);
let mut stderr_path = "stderr".to_string();
stderr_path = format!("{}stderr", unsafe{ps18::page_struct("", ps18::MAINPATH_, -1).str_});
let path_2_list_of_found_files = format!("{}", unsafe{ps18::page_struct("", ps18::FOUND_FILES_, -1).str_});
fstdout = String::from(path_2_list_of_found_files); 
core18::errMsg_dbg(&stderr_path, func_id, -1.0);
core18::errMsg_dbg(&fstdout, func_id, -1.0);
let fstderr = File::create(stderr_path).unwrap();
let fstdout0 = File::create(fstdout).unwrap();
globs18::unblock_fd(fstdout0.as_raw_fd());
//let mut fstdout0 = io::BufReader::new(fstdout0);
//errMsg_dbg(&in_name, func_id, -1.0);
let run_command = Command::new("bash").arg("-c").arg(path_2_cmd)//.arg(";echo").arg(stopCode)
//let run_command = Command::new(cmd)
    .stdout(fstdout0)
    .stderr(fstderr)
    .spawn()
    .expect("can't run command in run_cmd");
true
}
pub(crate) fn run_cmd_out_sync(cmd: String) -> String{
let func_id = 5;
let fstdout: String; 
let path_2_cmd = mk_cmd_file(cmd);
let mut stderr_path = "stderr".to_string();
stderr_path = format!("{}stderr", unsafe{ps18::page_struct("", ps18::MAINPATH_, -1).str_});
let path_2_list_of_found_files = format!("{}", unsafe{ps18::page_struct("", ps18::FOUND_FILES_, -1).str_});
fstdout = String::from(path_2_list_of_found_files); 
core18::errMsg_dbg(&stderr_path, func_id, -1.0);
core18::errMsg_dbg(&fstdout, func_id, -1.0);
let fstderr = File::create(stderr_path).unwrap();
//let mut fstdout0 = io::BufReader::new(fstdout0);
//errMsg_dbg(&in_name, func_id, -1.0);
let run_command = Command::new("bash").arg("-c").arg(path_2_cmd)//.arg(";echo").arg(stopCode)
//let run_command = Command::new(cmd)
    .stderr(fstderr).stdout(std::process::Stdio::piped())
    .output()
    .expect("can't run command in run_cmd_out_sync");
return match from_utf8(&run_command.stdout){
    Ok(utf) => utf,
    _ => "0 0"
}.to_string()
}
fn read_midway_data() -> bool{
    delay_ms(27);
    if checkArg("-front-lst"){return read_midway_data_not_main0() }
    let func_id = func_id18::read_midway_data_;
    let mut added_indx = 0usize;
    loop {
        let stopCode = getStop_code__!();
        let filename = take_list_adr("found_files").unreel_link_to_depth(1);
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
    for (indx, line) in reader.lines().enumerate() {
        if indx <= added_indx && added_indx > 0{continue;}
        added_indx = indx;
        let line = line.unwrap();
        let ret = globs18::add_2_front_list(&line, -1); // todo => add_2_front_list
       // let line_dbg = get_item_from_front_list(usize_2_i64(indx), false);
        ps18::set_num_files0(func_id, indx); 
        if dirty!(){println!("line {indx} {}", line)}
        if line == stopCode{ps18::fix_num_files(func_id); return true}
    }  }
    if dirty!(){println!("midway ended")}
    false
}
fn read_midway_data_not_main0() -> bool{
    let func_id = func_id18::read_midway_data_;
    let mut added_indx = 0usize;
    loop {
        let stopCode = getStop_code__!();
        let filename = format!("{}/main0", unsafe{ps18::page_struct("", ps18::TMP_DIR_, -1).str_});
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
    for (indx, line) in reader.lines().enumerate() {
        if indx <= added_indx && added_indx > 0{continue;}
        added_indx = indx;
        let line = line.unwrap();
        let ret = globs18::add_2_main0_list(&line ); 
       // let line_dbg = get_item_from_front_list(usize_2_i64(indx), false);
        ps18::set_num_files0(func_id, indx); 
        if dirty!(){println!("line {indx} {}", line)}
        if line == stopCode{ps18::fix_num_files(func_id); return true}
    }  }
    if dirty!(){println!("midway ended")}
    false
}
//#[inline(always)]
fn find_files(path: &str, path_2_tmp_file: &str) -> bool{
    if checkArg("-front-lst"){return find_files_not_main0(path, path_2_tmp_file) }
let func_id: i64 = 2;
let output = format!("{}/found_files", unsafe{ps18::page_struct("", ps18::TMP_DIR_, -1).str_});
/*let mut perms = std::fs::metadata(&output).unwrap().permissions();
perms.set_readonly(true);
std::fs::set_permissions(output, perms); return true; */
let mut in_name = String::new();
let mut list_of_found_files: Vec<String> = vec![]; 
if in_name.len() == 0{in_name = core18::put_in_name();}
else{in_name = format!("|{}", form_grep_cmd(&in_name));}
let stopCode: String = unsafe {ps18::page_struct("", ps18::STOP_CODE_,-1).str_};
let mut cmd: String = format!("#!/bin/bash\nfind -L '{path}' -type f{in_name} >> {};echo '{stopCode}' >> {}", output, output);
run_cmd0(cmd);
return true;
}
fn find_files_not_main0(path: &str, path_2_tmp_file: &str) -> bool{
let func_id: i64 = 2;
let output = format!("{}/main0", unsafe{ps18::page_struct("", ps18::TMP_DIR_, -1).str_});
let mut in_name = String::new();
let mut list_of_found_files: Vec<String> = vec![]; 
if in_name.len() == 0{in_name = core18::put_in_name();}
else{in_name = format!("|{}", form_grep_cmd(&in_name));}
let stopCode: String = unsafe {ps18::page_struct("", ps18::STOP_CODE_,-1).str_};
let mut cmd: String = format!("#!/bin/bash\nfind -L '{path}' -type f{in_name} >> {};echo '{stopCode}' >> {}", output, output);
run_cmd0(cmd);
return true;
}
fn get_arg_in_cmd0(key: &str) -> String{
let mut ret = "".to_string();
let args: Vec<_> = env::args().collect();
//let args_2_str = args.as_slice();
for i in 1..args.len(){
    if /*args_2_str[i]*/args[i] == key { return args[i + 1].clone();}
}
return ret;
}

fn get_arg_in_cmd(key: &str) -> core18::ret0{
let mut s: [char; 512] = ['\0'; 512];
let mut ret: core18::ret0 = core18::ret0{s, res: false};
let args: Vec<_> = env::args().collect();
let args_2_str = args.as_slice();
for i in 1..args.len(){
    if args_2_str[i] == key {
        let arr: Vec<char> = (args[i + 1]).chars().collect();
        if arr.len() > 512 {
            for i in 0..511{
               ret.s[i] = arr[i];
            }
        } else{
            for i in 0..arr.len(){
               ret.s[i] = arr[i];
            }
        }
        ret.res = true;
        return ret;
}
}
ret.res = false;
return ret;
}
fn self_dive(nm: String){// just sidekick to crrash tst :)
    std::thread::spawn(||{
        let nm = nm;
    self_dive(nm);
    });
    for i in 0..1_000_000 {
        self_dive("dive, baby".to_string());
    }
    return
}
fn main (){
    /*#[cfg(any(feature="in_dbg", feature="dbg0"))]
    panic!("kkkkkkkkkkkkkkkkkkkkmmmmmmmmmmmmmmmm............");*/
    // let mut x = 0u64;
 //   loop{ x += 1;}
 /*#[cfg(feature ="mae")] let tst = UID_UTF8(15);
 #[cfg(feature ="mae")]
 println!("{} {}", tst, tst.chars().count() ); return;*/
    use ctrlc;
    ctrlc::CtrlC::set_handler(||{SYS()});
    if checkArg("-mk-dummy-file"){
        if !cfg!(feature="mae"){println!("Dear User, enable feature mae", );}
        let name = String::from_iter(get_arg_in_cmd("-mk-dummy-file").s).trim().strn();
        let len = String::from_iter(get_arg_in_cmd("-len").s).trim().trim_matches('\0').strn();
        let len = strn_2_usize(len.strn());
        let content = String::from_iter(get_arg_in_cmd("-content").s).trim().strn();
 #[cfg(feature="mae")] mk_dummy_filo(&name, content.as_str(), len.unwrap_or(1));
 SYS();
    }
    if cfg!(feature="in_dbg"){println!("feature in_dbg been activated"); getkey();};
   //initSession();
   if checkArg("-ver") || checkArg("-version") || checkArg("--version"){initSession(); info(); SYS();}
   if checkArg("-rilocan"){initSession(); rilocan(); return;}
   let key = "-encrypt-copy".strn();
   if checkArg(&key){
    if !cfg!(feature="mae"){println!("Dear User, enable feature mae", ); SYS()}
    initSession();
    let name = String::from_iter(get_arg_in_cmd(&key).s).trim().trim_matches('\0').strn();
    #[cfg(feature="mae")] encrypt_n_keep_orig_file(&name);
    SYS();
   }
   let key = "-decrypt-copy".strn();
   if checkArg(&key){
    if !cfg!(feature="mae"){println!("Dear User, enable feature mae", );}
    initSession();
    let name = String::from_iter(get_arg_in_cmd(&key).s).trim().trim_matches('\0').strn();
    #[cfg(feature="mae")] decrypt_copy(&name);
    SYS();
   }
   let key = "-count-ch".strn();
#[cfg(feature="mae")] 
   if checkArg(&key){
    initSession();
     use Mademoiselle_Entropia::help_funcs;
    let ch = String::from_iter(get_arg_in_cmd(&key).s).trim().trim_matches('\0').strn();
    let name = String::from_iter(get_arg_in_cmd("-file").s).trim().trim_matches('\0').strn();
     let mut file =  match help_funcs::get_file(&name.strn()){Ok(f) => f, _ => return};
    file.count_chars(&ch);
    SYS();
   }
   /*/
    if checkArg("-dbg") || checkArg("-dbg1") || checkArg("-dbg2"){popup_msg("starting");}
let mut path: String = String::from("~/");
if core18::checkArg("-path"){path = String::from_iter(get_arg_in_cmd("-path").s);}
if core18::checkArg("-path0"){path = String::from_iter(get_arg_in_cmd("-path0").s);}*/
update18::prime();
let mut Key: String = "tst".blink().to_string();
println!("Key is {}", Key);
//});
return;
}
