#[macro_use]
#[path = "exts.rs"]
mod exts;
use exts::*;
//use gag::RedirectError;

use crate::{swtch::{user_wrote_path, user_wrote_path_prnt, read_user_written_path, path_completed}, update18::{update_dir_list, fix_screen}, shift_cursor_of_prnt, run_cmd_str, split_once, run_cmd_out, cached_ln_of_found_files, run_cmd_out_sync};

use self::ps21::{set_ask_user, get_prnt, set_prnt, get_mainpath, get_tmp_dir};
core_use!();
pub(crate) fn bkp_tmp_dir() -> String{
    static mut bkp: OnceCell<String> = OnceCell::new();
  if crate::C!(bkp.get()) == None{
    let fst: String = unsafe{crate::page_struct("", crate::TMP_DIR_, -75811245).str_};
    let ret = fst.clone();
    crate::C!(bkp.set(fst));
    return ret;
  }
  crate::C!(bkp.get().expect("bkp_tmp_dir failed").to_string())
}
pub(crate) fn up_front_list(){
    let list = read_front_list();
    let tmp_dir = get_tmp_dir(-1595741);
    if tmp_dir == ""{return;}
    let found_files = format!("{tmp_dir}/found_files");
    let active_list = format!("{tmp_dir}/{}", list);
    let cmd = format!("#up_front_list\nln -sf {active_list} {found_files}");
    run_cmd_str(&cmd);
}
pub(crate) fn set_front_list(list: &str){
    let tmp_dir = get_tmp_dir(-155741);
    if tmp_dir == ""{return;}
    let found_files = format!("{tmp_dir}/found_files");
    let active_list = format!("{tmp_dir}/{}", list);
    let cmd = format!("#set_front_list\nln -sf {active_list} {found_files}");
    run_cmd_out_sync(cmd);
    mark_front_lst(list);
    crate::wait_4_empty_cache();
    if list == "merge"{fix_screen();}
}
pub(crate) fn mark_front_lst(name: &str){
    if name != "ls"{save_file(name.to_string(), "front_list".to_string());}
    else {save_file(name.to_string(), "ls.mode".to_string());}
}
pub(crate) fn initSession() -> bool{
    let func_id = 1;
    let run_command = Command::new("bash").arg("-c").arg("cd ~;pwd")
    .output()
    .expect("something wrong with cd command");
errMsg_dbg(from_utf8(&run_command.stderr).unwrap(), func_id, -1.0);  
if !run_command.status.success(){
    io::stdout().write_all(&run_command.stdout).unwrap();
    io::stderr().write_all(&run_command.stderr).unwrap();
    return false;
}
let mut proper_output: Vec<u8> = run_command.stdout;
let last: usize = proper_output.len() - 1;
let _ =proper_output.pop();
errMsg_dbg(from_utf8(&proper_output).unwrap(), func_id, -1.0);  
let timestamp = Local::now();
let proper_timestamp = format!("{}", timestamp.format("%Y-%mm-%dd_%H-%M-%S_%f"));
let mainpath: String = format!("{}/.TAM_SESSIONS/{proper_timestamp}/", from_utf8(&proper_output).unwrap().to_string());
//let mainpath = escape_symbs(&mainpath);
errMsg_dbg(&mainpath, func_id, -1.0);
let run_shell = Command::new("mkdir").arg("-p").arg(&mainpath).output();
if checkArg("-dbg"){println!("shell out = {:?}", run_shell)};
if !Path::new(&mainpath).exists(){
   let res = fs::create_dir(&mainpath);
   println!("res = {:?}", res);
}
if !Path::new(&mainpath).exists(){
    println!("{mainpath} cannot be made");
    return false;
}
crate::C!(crate::page_struct(&mainpath, set(ps21::MAINPATH_), func_id));
let mut path_2_shm = "";
while true{
    if Path::new("/dev/shm").exists(){path_2_shm = "/dev/shm"; break;}
    if Path::new("/run/shm").exists(){path_2_shm = "/run/shm"; break;}
    if Path::new("/sys/shm").exists(){path_2_shm = "/sys/shm"; break;}
    if Path::new("/proc/shm").exists(){path_2_shm = "/proc/shm"; break;}
    if Path::new("/opt/shm").exists(){path_2_shm = "/opt/shm"; break;}
    if Path::new("/var/shm").exists(){path_2_shm = "/var/shm"; break;}
    panic!("no way for shared memory: device /dev/shm and its analogs don't exist or maybe Your system ain't common Linux");
}
let path_2_found_files_list = format!("{}/TAM_{}", path_2_shm, proper_timestamp);
let err_msg = format!("{} wasn't created", path_2_found_files_list);
let run_shell1 = Command::new("mkdir").arg("-p").arg(&path_2_found_files_list).output().expect(&err_msg.bold().red());
if checkArg("-dbg"){println!("shell out = {:?}", run_shell1)};
let err_msg = format!("{} permission denied", path_2_found_files_list);
let run_shell2 = Command::new("chmod").arg("700").arg(&path_2_found_files_list).output().expect(&err_msg.bold().red());
if checkArg("-dbg"){println!("shell out = {:?}", run_shell2)};
crate::C!(crate::page_struct(&path_2_found_files_list, set(crate::TMP_DIR_), func_id));
let tmp_dir = path_2_found_files_list;
bkp_tmp_dir();
let path_2_found_files_list_dot = format!("{}/TAM_{}/.", path_2_shm, proper_timestamp);
let err_msg = format!("{} permission denied", path_2_found_files_list_dot);
let run_shell3 = Command::new("chmod").arg("700").arg(&path_2_found_files_list_dot).output().expect(&err_msg.bold().red());
if checkArg("-dbg"){println!("shell out = {:?}", run_shell3)};
let path_2_found_files_list = format!("{}/TAM_{}/found_files", path_2_shm, proper_timestamp);
let err_msg = format!("{} can't be created", path_2_found_files_list);
let run_shell4 = Command::new("touch").arg("-f").arg(&path_2_found_files_list).output().expect(&err_msg.bold().red());
if checkArg("-dbg"){println!("shell out = {:?}", run_shell4)};
unsafe{crate::page_struct(&path_2_found_files_list, set(crate::FOUND_FILES_), func_id);
       crate::page_struct("empty", set(crate::KONSOLE_TITLE_), func_id);}
    crate::globs18::set_main0_as_front();
    crate::set_prnt("", func_id);
    unsafe {crate::swtch::form_list_of_viewers(false);}
    crate::save_file("".to_string(), "main0.pg".to_string());
    mark_front_lst("main0");
    let mk_cache_dir = format!("mkdir -p {tmp_dir}/cache");
    crate::run_cmd0(mk_cache_dir);
    return true;
}
pub(crate) fn errMsg_dbg(msg: &str, val_func_id: i64, delay: f64) {
    if !checkArg("-dbg") {return}
    if delay == -1.0{
        let msg = format!("{} said: {}", crate::func_id18::get_func_name(val_func_id), msg);
        set_ask_user(&msg.bold().red(), val_func_id);}
}
pub(crate) fn errMsg(msg: &str, val_func_id: i64) {
        let msg = format!("{} said: {}", crate::func_id18::get_func_name(val_func_id), msg);
        set_ask_user(&msg.bold().red(), val_func_id);
}
pub(crate) fn checkArg(key: &str) -> bool{
    let len_of_cmd_line = env::args().len();
    let args: Vec<String> = env::args().collect();
    let i: i64 = 0;
    for i in 0..len_of_cmd_line{
        if args[i] == key.to_string(){
            return true;
        }
    }
    return false;
}
pub(crate) fn set(item: i64) -> i64{
    return item * -1;
}
pub(crate) fn this_item_takes_global_val(id: i64) -> i64 {
    return set(id);
}
pub(crate) fn set_proper_num_pg(num_pg: i64){
    let front_list = format!("{}/{}", crate::get_tmp_dir(-371033), "front_list");
    let listName_dot_pg = format!("{}.pg", read_front_list());
    save_file(num_pg.to_string(), listName_dot_pg);
}
pub(crate) fn rgx_from_file(rgx: String, src: &str, out: &str){
    let prime_path = format!("{}", get_tmp_dir(84411254));
    if prime_path == ""{set_ask_user("Sorry, No access to tmp directory.. Sorry", -5611115); return;}
    let (opts, rgx) = split_once(&rgx, " ");
    let src = format!("{prime_path}/{}", escape_symbs(&src.to_string()));
    let out = format!("{prime_path}/{}", escape_symbs(&out.to_string()));
    let cmd = format!("grep {opts} '{rgx}' {src} > {out}");
    run_cmd_str(cmd.as_str());
}
pub(crate) fn rgx_from_prnt(rgx: String, out: &str){
    rgx_from_file(rgx, "prnt", out)
}
pub(crate) fn read_rgx_from_prnt(rgx: String, out: &str) -> String{
    rgx_from_prnt(rgx, out);
    read_file(out)
}
pub(crate) fn drop_ls_mode(){
    save_file("".to_string(), "ls.mode".to_string());
    up_front_list()
}
pub(crate) fn read_front_list() -> String{
    let mut active_lst = read_file("ls.mode");
    if active_lst == "" {active_lst = read_file("front_list");}
    active_lst
}
pub(crate) fn read_front_list_but_ls() -> String{
    let active_lst = read_file("front_list");
    active_lst
}
pub(crate) fn read_proper_num_pg() -> i64{
    let front_list = format!("{}/{}", crate::get_tmp_dir(-371011), "front_list");
    let num_pg = format!("{}.pg", read_front_list());
    let num_pg = read_file(&num_pg);
    match i64::from_str_radix(num_pg.as_str(), 10){
        Ok(num) => return num,
        _ => return 0
    }
}
pub(crate) struct ret0 {
   pub s: [char; 512],
   pub res: bool
}
pub(crate) fn escape_symbs(str0: &String) -> String{
    let  strr = str0.as_str();
    let strr = strr.replace("-", r"\-");
    let strr = strr.replace(" ", r"\ ");
    let strr = strr.replace("$", r"\$");
    let strr = strr.replace("'", r"\'");
    let strr = strr.replace("`", r"\`");
    let strr = strr.replace("(", r"\(");
    let strr = strr.replace("}", r"\}");
    let strr = strr.replace("{", r"\{");
    let strr = strr.replace(")", r"\)");
    let strr = strr.replace("]", r"\]");
    let strr = strr.replace("[", r"\[");
    let strr = strr.replace("&", r"\&");
    return strr.to_string();
}
pub(crate) fn key_f12(func_id: i64){
    unsafe {crate::shift_cursor_of_prnt(0, func_id)};
    crate::set_prnt("", func_id);
    rm_user_written_path(func_id)
}
pub(crate) fn check_substr(orig: &String, probe: &str, start_from: usize) -> bool{
    let func_id = 3;
    let probe: String = String::from(probe.to_string());
    let substr: &str= &orig.as_str();
    let substr = substr.substring(start_from, probe.len()).to_string();
    errMsg_dbg(&substr, func_id, -1.0);
     if probe.ne(&substr){
        return false;
     }
     return true;
}

fn end_termios(termios: &Termios){
  let res = match tcsetattr(0, TCSANOW, &termios){
        Err(e) => {format!("{}", e)},
        Ok(len) => {format!("{:?}", len)}
    };
    io::stdout().flush().unwrap();
}

#[inline(always)]
pub(crate) fn custom_cmd_4_find_files(custom_cmd: String) -> bool{
let func_id: i64 = 2;
let mut list_of_found_files: Vec<String> = vec![]; 
let output = format!("{}/found_files", unsafe{crate::ps18::page_struct("", crate::ps18::TMP_DIR_, -1).str_});
let stopCode: String = unsafe {crate::ps18::page_struct("", crate::ps18::STOP_CODE_,-1).str_};
let mut cmd: String =  format!("#!/bin/bash\n{} > {};echo '{stopCode}' >> {}", custom_cmd, output, output);
crate::run_cmd0(cmd);
return true;
}
#[inline(always)]
pub(crate) fn find_files_ls(custom_cmd: String) -> bool{
let func_id: i64 = 2;
let mut list_of_found_files: Vec<String> = vec![]; 
let output = format!("{}/ls", unsafe{crate::ps18::page_struct("", crate::ps18::TMP_DIR_, -1).str_});
let stopCode: String = unsafe {crate::ps18::page_struct("", crate::ps18::STOP_CODE_,-1).str_};
let mut cmd: String =  format!("#!/bin/bash\n{} > {};echo '{stopCode}' >> {}", custom_cmd, output, output);
crate::run_cmd0(cmd);
return true;
}
pub(crate) fn getkey() -> String{
let mut Key: String ="".to_string();
let xccnt = unsafe{exec_cmd_cnt(false)};
 let mut stdin = io::stdin();
    let stdin_fd = 0;
    let mut stdout = io::stdout(); 
    let mut stdin_buf: [u8; 6] =[0;6];
    let termios = Termios::from_fd(stdin_fd).unwrap();
    let mut new_termios = termios.clone();
    stdout.lock().flush().unwrap(); 
    new_termios.c_lflag &= !(ICANON | ECHO); 
    let enter = ||
{
    let enter: [u8; 1] = [13; 1];
    let mut writeIn_stdin = unsafe {File::from_raw_fd(0/*stdin*/)};
    //writeIn_stdin.write(&enter);
    //println!("gotta enter");
};
//loop {
    let res = match tcsetattr(stdin_fd, TCSANOW, &new_termios){
        Err(e) => {format!("{}", e)},
        Ok(len) => {format!("kkkkkkkkkkk {:#?}", len)}
    };
    let red_stdin = stdin.read(&mut stdin_buf);
    //stdout.lock().flush().unwrap();
    end_termios(&termios);
    if crate::dirty!() {println!("len of red {:?}", red_stdin.unwrap());}
    let str0 = match str::from_utf8(&stdin_buf){
        Ok(s) => s,
        _ => ""
    };
    let msg = format!("getch {} {:?}", str0, stdin_buf);
    if stdin_buf != [0; 6]{
        let mut i = 0;
        loop{
            let ch = match str0.chars().nth(i){
                Some(ch) => ch,
                _ => return Key
            };
        if ch == '\0' {return Key;}
        Key.push(ch);
        i += 1;}}
//}
Key
}
pub(crate) fn cpy_str(in_str: &String) -> String{
    in_str.to_string()
}
pub(crate) fn complete_path(dir: &str, opts: &str, no_grep: bool){
    let proper_dir = crate::escape_symbs(&dir.to_string());
    update_dir_list(&proper_dir, opts, no_grep);
    let not_full_path = get_path_from_prnt();//read_user_written_path();
    let num_of_ln_in_dir_lst = ln_of_found_files(usize::MAX).1;
    let mut get_prnt_dbg: fn (i64) -> String = get_prnt;
    let mut prnt = String::from("");
    //for i in 0..100{
        prnt.push_str(read_prnt().as_str());
      //  if prnt != ""{break;}
    //}
    let mut prnt = "".to_string();
    if prnt.len() == 0{set_ask_user("prnt is empty", -5);}
    if num_of_ln_in_dir_lst == 1{
        let mut full_path = ln_of_found_files(0).0;
        let is_dir = Path::new(&full_path).is_dir();
        if is_dir{full_path.push('/');}
        prnt = prnt.replace(&not_full_path, &full_path);
        let msg = format!("prnt: {}", prnt);
        //popup_msg(msg.as_str());
        set_prnt(&prnt, -47);
        let prnt = read_prnt();
        set_ask_user(&prnt, -5);
        rewrite_user_written_path(&full_path);
        //unsafe{crate::swtch::path_completed(true, false);}
        let proper_dir = crate::escape_symbs(&full_path.to_string());
        update_dir_list(&proper_dir, opts, no_grep);
    }
}
pub(crate) fn update_user_written_path(e: std::io::Error) -> File{
    println!("{:?}", e);
    let user_written_path = user_wrote_path_prnt();
    let err_msg = format!("update_user_written_path() can't create {}", user_written_path);
    rm_file(&user_written_path);
    File::options().create_new(true).write(true).read(true).open(user_written_path).expect(&err_msg)
}
pub(crate) fn rewrite_user_written_path(new_path: &String) {
    let user_written_path = user_wrote_path();
    let err_msg = format!("update_user_written_path() can't create {}", user_written_path);
    rm_file(&user_written_path);
    let mut write_path = File::options().create_new(true).write(true).read(true).open(user_written_path).expect(&err_msg);
    write_path.write_all(new_path.as_bytes());
}
pub(crate) fn rm_user_written_path(func_id: i64) {
    let user_written_path = user_wrote_path_prnt();
    let err_msg = format!("update_user_written_path() can't delete {}", user_written_path);
    rm_file(&user_written_path);
    let existed = Path::new(&user_written_path).exists();
   // if existed{set_ask_user(&err_msg, func_id);}
    let user_written_path = user_wrote_path();
    let err_msg = format!("update_user_written_path() can't delete {}", user_written_path);
    rm_file(&user_written_path);
    let existed = Path::new(&user_written_path).exists();
 //   if existed{set_ask_user(&err_msg, func_id);}
}
pub(crate) fn rm_file(file: &String) -> bool{
    let err_msg = format!("rm_file can't remove {}", file);
    let rm_cmd = Command::new("rm").arg("-f").arg(file)
    .output()
    .expect(&err_msg);
    true
}
pub(crate) fn read_midway_data_4_ls() -> bool{
   // return true;
    let func_id = crate::func_id18::read_midway_data_;
    let mut added_indx = 0usize;
    loop {
        let stopCode = getStop_code__!();
        let filename = format!("{}/found_files", unsafe{crate::ps18::page_struct("", crate::ps18::TMP_DIR_, -1).str_});
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
    for (indx, line) in reader.lines().enumerate() {
        if indx <= added_indx && added_indx > 0{continue;}
        added_indx = indx;
        let line = line.unwrap();
        let ret = crate::globs18::add_2_front_list(&line, -1); 
        //let line_dbg = get_item_from_front_list(usize_2_i64(indx), false); 
        if dirty!(){println!("line {}", line)}
        crate::ps18::set_num_files0(func_id, indx);
        if line == stopCode{crate::ps18::fix_num_files(func_id); return true}
    }  if dirty!(){println!("midway ended")}}
    false
}
//#[io_cached]
pub(crate) fn ln_of_found_files(get_indx: usize) -> (String, usize){
    let ls_mode = read_front_list();
    if ls_mode != "ls"{
    if !checkArg("-no-cache") {
        if get_indx < usize::MAX{return cached_ln_of_found_files(get_indx);}}
    }
     let stopCode = getStop_code__!();
        let filename = format!("{}/found_files", unsafe{crate::ps18::page_struct("", crate::ps18::TMP_DIR_, -1).str_});
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut len = 0usize;
    for (indx, line) in reader.lines().enumerate() {
        if indx == get_indx{return (line.unwrap(), indx);}
        len = indx;
    }
    return ("no str gotten".to_string(), len);
}
pub(crate) fn size_of_found_files() -> u64{
     let stopCode = getStop_code__!();
        let filename = format!("{}/found_files", unsafe{crate::ps18::page_struct("", crate::ps18::TMP_DIR_, -1).str_});
        match fs::metadata(filename){
            Ok(op) => op,
            _ => return 0u64
        }.len()
}
pub(crate) fn get_path_from_strn(strn: String) -> String{
    let len: usize = strn.chars().count();
    let mut ret = String::new();
    let mut yes_path = false;
    for i in 0..len{
        let char0 =strn.chars().nth(i).unwrap();
        if char0 == '/'{yes_path = true;}
        if yes_path{ret.push(char0);}
    }
    ret
}
pub(crate) fn get_path_from_prnt() -> String{
    let got_path = read_prnt();
    let len: usize = got_path.chars().count();
    let mut ret = String::new();
    let mut yes_path = false;
    for i in 0..len{
        let char0 = got_path.chars().nth(i).unwrap();
        if char0 == '/'{yes_path = true;}
        if yes_path{ret.push(char0);}
    }
    ret
}
pub(crate) fn save_file0(content: String, fname: String) -> bool{
    let fname = format!("{}/{}", crate::get_tmp_dir(-157), fname);
    let cmd = format!("echo '{}' > {}", content, fname);
    run_cmd_str(cmd.as_str());
    true
}
pub(crate) fn save_file(content: String, fname: String) -> bool{
    let fname = format!("{}/{}", crate::get_tmp_dir(-157), fname);
    let cmd = format!("touch -f {fname}");
    //run_cmd_str(cmd.as_str());
    let anew_file = || -> File{rm_file(&fname); return File::options().create_new(true).write(true).open(&fname).expect(&fname)};
    let mut file: File = match File::options().create(false).read(true).truncate(true).write(true).open(&fname){
        Ok(f) => f,
        _ => anew_file()
    };
    file.write(content.as_bytes()).expect("save_file failed");
    true
}
pub(crate) fn rewrite_file_abs_adr(content: String, fname: String) -> bool{
    let anew_file = || -> File{rm_file(&fname); return File::options().create_new(true).write(true).open(&fname).expect(&fname)};
    let mut file: File = match File::options().create(false).read(true).truncate(true).write(true).open(&fname){
        Ok(f) => f,
        _ => anew_file()
    };
    file.write(content.as_bytes()).expect("rewrite_file_abs_adr failed");
    true
}
pub(crate) fn save_file_abs_adr(content: String, fname: String) -> bool{
    let anew_file = || -> File{rm_file(&fname); return File::options().create_new(true).write(true).open(&fname).expect(&fname)};
    let existing_file = || -> File{
        let timestamp = Local::now();
        let fname = format!("{}", timestamp.format("%Y-%mm-%dd_%H-%M-%S_%f")); return File::options().create_new(true).write(true).open(&fname).expect(&fname)};
    let mut file: File = match File::options().create(false).read(true).append(true).write(true).open(&fname){
        Ok(f) => f,
        Err(e) => match e.kind(){
            std::io::ErrorKind::NotFound => anew_file(),
            std::io::ErrorKind::AlreadyExists => File::options().read(true).append(true).write(true).open(&fname).unwrap(),
            _ => existing_file()
        }
    };
    file.write(content.as_bytes()).expect("save_file failed");
    true
}
pub(crate) fn save_file_append(content: String, fname: String) -> bool{
    let fname = format!("{}/{}", crate::get_tmp_dir(-157), fname);
    let cmd = format!("touch -f {fname}");
    //run_cmd_str(cmd.as_str());
    let anew_file = || -> File{rm_file(&fname); return File::options().create_new(true).write(true).open(&fname).expect(&fname)};
    let existing_file = || -> File{
        let timestamp = Local::now();
        let fname = format!("{}", timestamp.format("%Y-%mm-%dd_%H-%M-%S_%f")); return File::options().create_new(true).write(true).open(&fname).expect(&fname)};
    let mut file: File = match File::options().create(false).read(true).append(true).write(true).open(&fname){
        Ok(f) => f,
        Err(e) => match e.kind(){
            std::io::ErrorKind::NotFound => anew_file(),
            std::io::ErrorKind::AlreadyExists => File::options().read(true).append(true).write(true).open(&fname).unwrap(),
            _ => existing_file()
        }
    };
    file.write(content.as_bytes()).expect("save_file failed");
    true
}
pub(crate) fn save_file_append_abs_adr(content: String, fname: String) -> bool{
    let cmd = format!("touch -f {fname}");
    //run_cmd_str(cmd.as_str());
    let anew_file = || -> File{return match File::options().create_new(true).write(true).open(&fname){
        Ok(f) => f,
        Err(e) => match e.kind(){
            std::io::ErrorKind::AlreadyExists => File::options().read(true).append(true).write(true).open(&fname).unwrap(),
            _ => File::options().create_new(true).write(true).open(&fname).unwrap()
    }}};
    let existing_file = || -> File{
        let timestamp = Local::now();
        let fname = format!("{}", timestamp.format("%Y-%mm-%dd_%H-%M-%S_%f")); return File::options().create_new(true).write(true).open(&fname).expect(&fname)};
    let mut file: File = match File::options().create(false).read(true).append(true).write(true).open(&fname){
        Ok(f) => f,
        Err(e) => match e.kind(){
            std::io::ErrorKind::NotFound => anew_file(),
            std::io::ErrorKind::AlreadyExists => File::options().read(true).append(true).write(true).open(&fname).unwrap(),
            _ => existing_file()
        }
    };
    file.write(content.as_bytes()).expect("save_file failed");
    true
}
pub(crate) fn read_file(fname: &str) -> String{
    let fname = format!("{}/{}", crate::get_tmp_dir(-157), fname);
    //let err = format!("failed to read {}", fname);
    let mut file: File = match File::open(&fname){
        Ok(f) => f,
        Err(e) => return "".to_string()//format!("{:?}", e)
    };
    let mut ret = String::new();
    file.read_to_string(&mut ret);
    ret
}
pub(crate) fn read_prnt() -> String{read_file("prnt")}
pub(crate) fn file_prnt(content: String){
    save_file(cpy_str(&content), "prnt".to_string());
    let path = get_path_from_strn(content);
    save_file(path, "user_wrote_path".to_string());}
pub(crate) fn position_of_slash_in_prnt() -> usize{
    let got_path = read_prnt();
    let ret = usize::MAX;
    let mut i = 0usize;
    loop{
        let char0 = match got_path.chars().nth(i){
            Some(ch) => ch,
            _ => return ret
        };
        if char0 == '/'{return i;}
        i += 1;
    }
}
pub(crate) fn i64_2_usize(v: i64) -> usize{
    let mut ret = 0usize;
    let unit = 1i64;
    let shl = 1usize;
    let mut v = v;
    let i64_len: usize = size_of::<i64>() * 8;
    for i in 0..i64_len{
        if v == 0{break;}
        if v & unit == 1{ret += (shl << i);}       
        v = v >> 1;
    }
    ret
}
pub(crate) fn usize_2_i64(v: usize) -> i64{
    let mut ret = 0i64;
    let unit = 1usize;
    let shl = 1i64;
    let mut v = v;
    let usize_len: usize = size_of::<usize>() * 8;
    for i in 0..usize_len{
        if v == 0{break;}
        if v & unit == 1{ret += (shl << i);}
        v = v >> 1;
    }
    ret
}
pub(crate) fn getch() -> char{
let mut ch: char ='\0';
 let mut stdin = io::stdin();
    let stdin_fd = 0;
    let mut stdout = io::stdout();
    let mut stdin_buf: [u8; 6] =[0;6];
    let termios = Termios::from_fd(stdin_fd).unwrap();
    let mut new_termios = termios.clone();
    stdout.lock().flush().unwrap(); 
    new_termios.c_lflag &= !(ICANON | ECHO); 
    let enter = ||
{
    let enter: [u8; 1] = [13; 1];
    let mut writeIn_stdin = unsafe {File::from_raw_fd(0/*stdin*/)};
    writeIn_stdin.write(&enter);
    println!("gotta enter");
};
loop {
    let res = match tcsetattr(stdin_fd, TCSANOW, &new_termios){
        Err(e) => {format!("{}", e)},
        Ok(len) => {format!("kkkkkkkkkkk {:#?}", len)}
    };
    let red_stdin = stdin.read(&mut stdin_buf);
    stdout.lock().flush().unwrap();
    end_termios(&termios);
    if crate::dirty!() {println!("len of red {:?}", red_stdin.unwrap());}
    let str0 = match str::from_utf8(&stdin_buf){
        Ok(s) => s,
        _ => ""
    };
    let msg = format!("getch {} {:?}", str0, stdin_buf);
    achtung(&msg);
    if stdin_buf != [0; 6]{return str0.chars().nth(0).unwrap()}
}
ch
}
pub(crate) fn popup_msg(msg: &str){
    if crate::checkArg("-no-popup"){return;}
    let msg = format!("notify-send '{}'", msg);
    crate::run_cmd_str(&msg);
}
pub(crate) fn achtung(msg: &str){
    if !crate::checkArg("-dbg") || !crate::checkArg("-use-achtung"){return;}
    let msg = format!("notify-send '{}'", msg);
    crate::run_cmd_str(&msg);
}
pub(crate) fn calc_num_files_up2_cur_pg() -> i64{
    let func_id= crate::func_id18::calc_num_files_up2_cur_pg_;
    let ps = unsafe {crate::swtch::swtch_ps(-1, None)};
     let mut num_page; if ps.num_page != i64::MAX{num_page = ps.num_page;}else{num_page = crate::get_num_page(func_id);}
    let mut num_cols; if ps.num_cols != i64::MAX{num_cols = ps.num_cols;}else{num_cols = crate::get_num_cols(func_id);}
    let mut num_rows; if ps.num_rows != i64::MAX{num_rows = ps.num_rows;}else{num_rows = crate::get_num_rows(func_id);}
    if ps.col_width != i64::MAX{crate::set_col_width(ps.col_width, func_id);}
    let num_items_on_pages = num_cols * num_rows; let stopCode: String = crate::getStop_code__!();
    let counted_files = num_page * num_cols * num_rows;
    return counted_files;
}
pub(crate)
fn check_substring(orig: String, probe: String, start_from: usize) -> bool{
    let substr: &str= &orig.as_str();
    let substr = substr.substring(start_from, probe.len() - 1).to_string();
     if probe.ne(&substr){
        return false;
     }
     return true;
}
pub(crate) fn put_in_name() -> String{
    let mut ret: String = String::new();
    let len_of_cmd_line = env::args().len() -1;
    let args: Vec<String> = env::args().collect();
    let i: i64 = 0;
    for i in 0..len_of_cmd_line{
        if args[i] == "-in_name".to_string() || args[i] == "-in-name".to_string(){
            let cmd = format!("|{}", crate::form_grep_cmd(&args[i+1]));
            ret.push_str(&cmd);
        }
    }
    return ret;
}
pub(crate) fn ins_newlines(len_of_line: usize, origString: &mut String) {
    let num_of_loops = origString.len() / len_of_line;
    for i in 1..num_of_loops{
        let indx = i * len_of_line;
        origString.push('\n');
        ins_last_char_to_string1_from_string1_ptr(indx, origString);
    }
}
pub(crate) fn raw_ren_file(src: String, dst: String){
    let cmd = format!("mv {src} {dst}");
    run_cmd_str(cmd.as_str());
}
pub(crate) fn mkdir(name: String){
    let name = escape_symbs(&name);
    let cmd = format!("mkdir -p {name}");
    run_cmd_str(cmd.as_str());
}
pub(crate) fn raw_mkdir(name: String){
    let out = format!("{}/raw_mkdir.out", get_tmp_dir(4512154749974)); 
    let cmd = format!("mkdir -p {name} > {out} 2>&1");
    run_cmd_str(cmd.as_str());
}
pub(crate) fn path_exists(path: String) -> bool{
    let timestamp = Local::now();
let proper_timestamp = format!("{}", timestamp.format("%Y-%mm-%dd_%H-%M-%S_%f"));
let out: String = format!("{}/path_exists{}.out", get_tmp_dir(10008745), proper_timestamp);
let cmd = format!("find -L {path}");
let is_path = run_cmd_out(cmd);
let path = path.len();//chars().count();
//clear_screen();
//println!("{is_path}");
let is_path = is_path.len() + 1;//chars().count();
//println!("{path} and {is_path}");
//getkey();
if is_path == path{return true}
false
}
pub(crate) fn link_list_2_front(name: &str){
    let front = take_list_adr("found_files");
    let list = take_list_adr(name);
    let cmd = format!("ln -sf {list} {front}");
    run_cmd_str(cmd.as_str());
}
pub(crate) fn from_ls_2_front(ls_mode: String){
    let front = read_front_list_but_ls();
    //let ls_mode = take_list_adr("ls.mode");
    rm_file(&ls_mode);
    link_list_2_front(front.as_str());
    C!(crate::swtch::swtch_fn(0, "".to_string()));
}
pub(crate) fn tailOFF(strn: &mut String, delim: &str) -> bool{
    let len = strn.chars().count();
    let mut ret = String::new();
    let empty = String::new();
    let delim = delim.to_string().chars().nth(0).unwrap();
    for i in 0..len{
        let ch = match strn.chars().nth(i){
            Some(ch) => ch,
            _ => empty.chars().nth(0).unwrap()
        };
        if ch == delim && i < len - 1{ret = "".to_string(); continue;}
        ret.push(ch);
    }
    if ret.len() == 0{return false}
    //*strn = strn.replace(&ret, "").trim_end_matches(delim).to_string();
    let ret_delim = format!("{ret}{delim}sss");
    strn.push_str("sss");
    ret.push_str("sss");
    *strn = strn.replace(&ret_delim, "");
    *strn = strn.replace(&ret, ""); 
    true
}
pub(crate) fn read_tail(strn: &String, delim: &str) -> String{
    let len = strn.chars().count();
    let mut ret = String::new();
    let empty = String::new();
    let delim = delim.to_string().chars().nth(0).unwrap();
    for i in 0..len{
        let ch = match strn.chars().nth(i){
            Some(ch) => ch,
            _ => empty.chars().nth(0).unwrap()
        };
        if ch == delim && i < len - 1{ret = "".to_string(); continue;}
        ret.push(ch);
    }
    ret  
}
pub(crate) fn is_dir(path: &String) -> bool{
    let is_dir = path.chars().count() - 1;
    let is_dir = path.chars().nth(is_dir);
    if is_dir.expect("is_dir failed").to_string() != "/"{return false;}
    true
}