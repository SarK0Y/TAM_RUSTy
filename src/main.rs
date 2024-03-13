#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused)]
// #![feature(macro_metavar_expr)]
#![allow(unused_variables)]
#![allow(non_upper_case_globals)]
#![allow(while_true)]
#[allow(arithmetic_overflow)]
mod exts;
use exts::*;
use globs18::get_item_from_front_list;
use_all!();

pub(crate) fn split_once(in_string: &str, delim: &str) -> (String, String) {
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
let func_id = 4;
let timestamp = Local::now();
let proper_timestamp = format!("{}", timestamp.format("%Y-%mm-%dd_%H-%M-%S_%f"));
let path_2_cmd = format!("{}/cmd{}.sh", unsafe{ps18::page_struct("", ps18::TMP_DIR_, func_id).str_}, proper_timestamp);
let err_msg = format!("failed create {}", &path_2_cmd);
let mut make_cmd_file = File::create(&path_2_cmd).expect(&err_msg.bold().red());
core18::errMsg_dbg(&path_2_cmd, func_id, -1.0);
let mut cmd = cmd;
if !checkArg("-dbg"){
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
let path_2_list_of_found_files = format!("{}", unsafe{ps18::page_struct("", ps18::FOUND_FILES_, -1).str_});
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
let path_2_list_of_found_files = format!("{}", unsafe{ps18::page_struct("", ps18::FOUND_FILES_, -1).str_});
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
let path_2_list_of_found_files = format!("{}", unsafe{ps18::page_struct("", ps18::FOUND_FILES_, -1).str_});
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
    .stderr(fstderr)
    .output()
    .expect("can't run command in run_cmd_out_sync");
return match from_utf8(&run_command.stdout){
    Ok(utf) => utf,
    _ => "0 0"
}.to_string()
}
fn read_midway_data() -> bool{
    let func_id = func_id18::read_midway_data_;
    let mut added_indx = 0usize;
    loop {
        let stopCode = getStop_code__!();
        let filename = format!("{}/found_files", unsafe{ps18::page_struct("", ps18::TMP_DIR_, -1).str_});
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
    for (indx, line) in reader.lines().enumerate() {
        if indx <= added_indx && added_indx > 0{continue;}
        added_indx = indx;
        let line = line.unwrap();
        let ret = globs18::add_2_front_list(&line, -1); // todo => add_2_front_list
       // let line_dbg = get_item_from_front_list(usize_2_i64(indx), false);
        ps18::set_num_files0(func_id, indx); 
        if dirty!(){println!("line {}", line)}
        if line == stopCode{ps18::fix_num_files(func_id); return true}
    }  if dirty!(){println!("midway ended")}}
    false
}
#[inline(always)]
fn find_files(path: &str, mut in_name: String, path_2_tmp_file: &str) -> bool{
let func_id: i64 = 2;
let mut list_of_found_files: Vec<String> = vec![]; 
let output = format!("{}/found_files", unsafe{ps18::page_struct("", ps18::TMP_DIR_, -1).str_});
if in_name.len() == 0{in_name = core18::put_in_name();}
else{in_name = format!("|{}", form_grep_cmd(&in_name));}
let stopCode: String = unsafe {ps18::page_struct("", ps18::STOP_CODE_,-1).str_};
let mut cmd: String = format!("#!/bin/bash\nfind -L '{path}' -type f{in_name} >> {};echo '{stopCode}' >> {}", output, output);
run_cmd0(cmd);
return true;
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
fn main (){
// stacker::maybe_grow( 8*1024*1024, 32*1024*1024, || {
    // guaranteed to have at least 32K of stack
    initSession();
let out: core18::ret0 = get_arg_in_cmd("-");
let out1: core18::ret0 = get_arg_in_cmd("-тст");
println!("argument from cmd (-tst) {}", String::from_iter(out.s));
println!("argument from cmd (-тст) {}", String::from_iter(out1.s));
unsafe {println!("get stotstp code {}", ps18::page_struct("", 1, 0).str_);}
//unsafe {println!("set stop code {}", page_struct("777", set(1), 0).str_);}
unsafe {println!("get stop code {}", ps18::page_struct("", 1, 0).str_);}
let mut path: String = String::from("~/");
if core18::checkArg("-path"){path = String::from_iter(get_arg_in_cmd("-path").s);}
if core18::checkArg("-path0"){path = String::from_iter(get_arg_in_cmd("-path0").s);}
update18::prime();
let mut Key: String = "tst".blink().to_string();
println!("Key is {}", Key);
ps18::set_prnt("thr main", -1);
//});
return;
}
