use chrono::format::format;
use num_traits::{FloatErrorKind, ToPrimitive};
use once_cell::sync::OnceCell;
use substring::Substring;
use std::{
    fs::File,
    path::Path,
    io::{
        ErrorKind,
        Read,
        Write,
        BufWriter,
        BufReader,
        prelude::*
    },
    os::fd::AsRawFd,
    i64,
    usize,
};
pub const SWTCH_RUN_VIEWER: i64 = 0;
pub const SWTCH_USER_WRITING_PATH: i64 = 1;
use crate::{core18::{errMsg, get_path_from_prnt, update_user_written_path}, ps18::{set_ask_user, get_full_path, get_num_page, get_num_files, page_struct_ret, init_page_struct, child2run}, globs18::{get_item_from_front_list, set_ls_as_front}, func_id18::{viewer_, mk_cmd_file_, where_is_last_pg_}, update18::update_dir_list, complete_path, pg18::form_cmd_line_default, get_prnt, position_of_slash_in_prnt, usize_2_i64, escape_symbs};
pub(crate) unsafe fn check_mode(mode: &mut i64){
    static mut state: i64 = 0;
    if *mode == -1 {*mode = state;}
    state = *mode;
} 
pub(crate) unsafe fn swtch_fn(indx: i64, cmd: String){
    static mut fst_run: bool = true;
    static mut fn_indx: usize = 0;
    static mut fn_: OnceCell<Vec<fn(String) -> bool>> = OnceCell::new();
    if fst_run{
        let fn_vec: Vec<fn(String) -> bool> = Vec::new();
        fn_.set(fn_vec); fst_run = false;
        fn_.get_mut().unwrap().push(run_viewer); // 0
        fn_.get_mut().unwrap().push(user_writing_path); // 1
    }
    if indx < -1{
        let indx = crate::i64_2_usize(crate::set(indx)) - 2;
        let len = fn_.get().expect("Can't unwrap fn_ in swtch_fn").len();
        if indx > len {set_ask_user("indx gets out of fn_ ", -178); return;}
        fn_.get().unwrap()[indx](cmd);
        let mut indx = usize_2_i64(indx);
        check_mode(&mut indx);
        return;
    }
    if indx > -1 {fn_indx = indx.to_usize().unwrap(); return;}
  //  if indx > -1 && !cmd.is_empty(){fn_indx = indx.to_usize().unwrap();}
    let mut mode = indx;
    check_mode(&mut mode);
    fn_.get().unwrap()[fn_indx](cmd);
}
pub(crate) unsafe fn swtch_ps(indx: i64, ps: Option<crate::_page_struct>) -> crate::_page_struct{
    static mut fst_run: bool = true;
    static mut ps_indx: usize = 0;
    static mut ps_: OnceCell<Vec<crate::_page_struct>> = OnceCell::new();
    let dummy = init_page_struct();
    if fst_run{
        let ps_vec: Vec<crate::_page_struct> = Vec::new();
        ps_.set(ps_vec); fst_run = false;
    }
    if ps.is_some(){ps_.get_mut().unwrap().push(ps.unwrap());}
    if indx > -1{ps_indx = indx.to_usize().unwrap(); return dummy;}
    return crate::cpy_page_struct(&mut ps_.get_mut().unwrap()[ps_indx])
}
fn viewer_n_adr(app: String, file: String) -> bool{
    let func_id = crate::func_id18::viewer_;
    if app == "none" {
        crate::core18::errMsg("To run file w/ viewer, You need to type '<indx of viewer> /path/to/file'", func_id);
        return false
    }
    let msg = || -> bool{crate::core18::errMsg("To run file w/ viewer, You need to type '<indx of viewer> <index of file>'", func_id); return false};
    let app_indx = match usize::from_str_radix(app.as_str(), 10){
        Ok(v) => v,
        _ => return msg()
    };
    //let file = escape_symbs(&file);
    let viewer = get_viewer(app_indx, -1, true);
    let cmd = format!("{} {} > /dev/null 2>&1", viewer, file);
    return crate::run_cmd_viewer(cmd)
}
pub(crate) fn run_viewer(cmd: String) -> bool{
    let func_id = crate::func_id18::viewer_;
    let  (app_indx, file_indx) = crate::split_once(&cmd, " ");
    if file_indx.as_str().substring(0, 1) == "/"{return viewer_n_adr(app_indx, file_indx);}
    if app_indx == "none" || file_indx == "none"{
        crate::core18::errMsg("To run file w/ viewer, You need to type '<indx of viewer> <index of file>'", func_id);
        return false
    }
    let msg = || -> bool{crate::core18::errMsg("To run file w/ viewer, You need to type '<indx of viewer> <index of file>'", func_id); return false};
    let app_indx = match usize::from_str_radix(app_indx.as_str(), 10){
        Ok(v) => v,
        _ => return msg()
    };
    let file_indx = match i64::from_str_radix(file_indx.as_str(), 10){
        Ok(v) => v,
        _ => return msg()
    };
    //let file_indx: i64 = crate::globs18::get_proper_indx(file_indx).1;
    let filename = crate::escape_symbs(&get_item_from_front_list(file_indx, true));
    let viewer = get_viewer(app_indx, -1, true);
    let cmd = format!("{} {} > /dev/null 2>&1", viewer, filename);
    return crate::run_cmd_viewer(cmd)
}
pub(crate) fn get_viewer(indx: usize, func_id: i64, thread_safe: bool) -> String{
    let mut func_id_loc = func_id;
    if thread_safe {
        let rnd = get_rnd_u64();
        let mut msk: u64 = !u64::from(1u64 << 63);
        let msg = format!("{:b}", msk);
        if crate::dirty!(){println!("{}", msg.as_str());}
        let mut handle_err = move|| -> i64{let ret = msk & rnd.0; let ret = ret.to_i64().unwrap(); msk = 0; return ret;};
        if !rnd.1{errMsg("/dev/urandom and /dev/random either don't exist or aren't achivable on Your system", func_id); return "none".to_string();}
        func_id_loc = match rnd.0.to_i64(){
            Some(v) => v,
            _ => handle_err()
        };
        if msk == 0{func_id_loc *= -1;}
    }

    let ret = unsafe {share_usize(indx, func_id_loc)};
    if ret.1{return unsafe{crate::page_struct("", crate::VIEWER_, func_id_loc).str_};}
"locked".to_string()
}
pub(crate) fn get_num_of_viewers(func_id: i64) -> i64{return unsafe{crate::page_struct("", crate::NUM_OF_VIEWERS, func_id).int}}
pub(crate) fn add_viewer(val: &str, func_id: i64) -> String{return unsafe{crate::page_struct(val, crate::set(crate::VIEWER_), func_id).str_}}
pub(crate) unsafe fn share_usize(val: usize, func_id: i64) -> (usize, bool){
    static mut owner_id: i64 = i64::MIN;
    static mut actual_val: usize = 0;
    if owner_id == func_id && val == usize::MAX{
        owner_id = i64::MIN;
        return (actual_val, true)
     }
    if owner_id == i64::MIN{owner_id = func_id; actual_val = val; return (val, true);}
    (usize::MAX, false)
}
unsafe fn drop_2_dev_null() -> bool{
    static mut drop: bool = true;
    drop = !drop; return !drop;
}
pub(crate) unsafe fn path_completed(set_state: bool, ret_state: bool) -> bool{
    static mut state: bool = false;
    if ret_state {return state;}
    state = set_state;
    state
}
pub(crate) unsafe fn front_list_indx(val: i64) -> (i64, bool){
    static mut actual_indx: i64 = 0;
    if val == i64::MAX{
        return (actual_indx, true)
     }
    if val > -1 {actual_indx = val; return (val, true);}
    (i64::MAX, false)
}

pub(crate) unsafe fn local_indx(set_new_state: bool) -> bool{
    static mut actual_state: bool = false;
    if !set_new_state{
        return actual_state
     }
    actual_state = !actual_state;
    return actual_state
}
pub(crate) fn get_rnd_u64() -> (u64, bool){
    let mut get_rnd_device: File = match File::open("/dev/urandom"){
        Ok(File) => File,
        Err(e) => match e.kind(){
           ErrorKind::NotFound => match File::open("/dev/random"){
            Ok(File) => File,
            _ => return (0, false)
        }
        _ => return (0, false)}
    };
    let mut buf: [u8; 8] = [0; 8]; get_rnd_device.read(&mut buf);
    let mut rnd_u64: u64 = 0;
    for i in 0..buf.len(){
        let shl = u64::from(*buf.get(i).unwrap());
        rnd_u64 += u64::from(shl << i*8);
    }
    return (rnd_u64, true)
}
pub(crate) unsafe fn form_list_of_viewers(drop_1st_run: bool){
static mut fst_run: bool = true;
if drop_1st_run {fst_run = true;}
if !fst_run {return;}
fst_run = false;
let args: Vec<_> = crate::env::args().collect();
let arg = args.as_slice();
for i in 1..args.len(){
    if arg[i] == "-view_w" || arg[i] == "-view-w" {
        let viewer: String = (args[i + 1]).chars().collect();
        add_viewer(&viewer, -1);
    }
}
}
pub(crate) fn print_viewers(){
let num_of_viewers = get_num_of_viewers(-1).to_usize().unwrap();
    for i in 0..num_of_viewers{
        print!("|||{}: {}", i, get_viewer(i, -1, true));
    }
    println!("")
}
pub fn print_pg_info(){
    let num_page = get_num_page(-1);
    let num_files = get_num_files(-1);
    let last_pg = crate::where_is_last_pg();
    let info = format!("Number of files/pages {}/{} p. {}", num_files, last_pg, num_page);
    println!("{}", info);
}
pub(crate) fn user_wrote_path() -> String{
    return Path::new(&unsafe {format!("{}/user_wrote_path", unsafe{crate::ps18::page_struct("", crate::ps18::TMP_DIR_, -1).str_})}).to_str().unwrap().to_string()
}
pub(crate) fn user_wrote_path_prnt() -> String{
    return Path::new(&unsafe {format!("{}/user_wrote_path_prnt", unsafe{crate::ps18::page_struct("", crate::ps18::TMP_DIR_, -1).str_})}).to_str().unwrap().to_string()
}
pub(crate) fn set_user_written_path_from_strn(strn: String) -> bool{
    let save_path = user_wrote_path();
    let save_path1 = user_wrote_path();
    let strn = crate::get_path_from_strn(strn);
   // set_ask_user(&save_path, -1); //dbg here
    let mut file_2_write_path = match File::options().create(true).open(save_path){
        Ok(p) => p,
        Err(e) => update_user_written_path(e)
    }; //.expect("user_wrote_path failed ");
    //let mut writer = BufWriter::new(file_2_write_path)
    file_2_write_path.write_all(strn.as_bytes()).expect("user_wrote_path failed write in");
    crate::globs18::unblock_fd(file_2_write_path.as_raw_fd());
    let written_path = read_user_written_path();
    update_dir_list(&written_path, "-maxdepth 1", false);
    true
}
pub(crate) fn set_user_written_path_from_prnt() -> String{
    set_ls_as_front();
    let save_path = user_wrote_path();
    let save_path1 = user_wrote_path();
    let path_from_prnt = get_path_from_prnt();
    //set_ask_user(&save_path, -1); //dbg here
    let mut file_2_write_path = match File::options().create_new(true).open(save_path){
        Ok(p) => p,
        Err(e) => update_user_written_path(e)
    }; //.expect("user_wrote_path failed ");
    //let mut writer = BufWriter::new(file_2_write_path);
    let key = format!("{}", path_from_prnt);
    file_2_write_path.write_all(path_from_prnt.as_bytes()).expect("user_wrote_path failed write in");
    crate::globs18::unblock_fd(file_2_write_path.as_raw_fd());
    let written_path = read_user_written_path();
    update_dir_list(&written_path, "-maxdepth 1", false);
    written_path
}

pub(crate) fn user_writing_path(key: String) -> bool{
    let cur_cur_pos = get_prnt(-19).chars().count() - unsafe {crate::shift_cursor_of_prnt(0, -19).shift};
    if position_of_slash_in_prnt() >= cur_cur_pos {unsafe {swtch_fn(-2, crate::cpy_str(&key))} return false;}
    let mut save_path = user_wrote_path();
    let mut save_path1 = user_wrote_path();
   // set_ask_user(&save_path, -1); //dbg here
    let key = key.replace("//", "/");
    let path_exist = Path::new(&read_user_written_path()).exists();
    if key.chars().count() > 1 {save_path1 = "/dev/null".to_string(); save_path = "/dev/null".to_string();} 
    else if path_exist && key != "/" && crate::ln_of_found_files(usize::MAX).1 < 2usize {/*if unsafe {drop_2_dev_null()}*/{save_path1 = "/dev/null".to_string(); save_path = "/dev/null".to_string();}}
    let dbg_prnt = get_prnt(-5);
    set_ask_user(&dbg_prnt, -1);
    let mut file_2_write_path = match File::options().create_new(true).append(true).open(save_path){
        Ok(p) => p,
        Err(e) => match e.kind(){
          ErrorKind::AlreadyExists => match File::options().append(true).write(true).open(save_path1){
            Ok(f) => f,
            _ => update_user_written_path(e)
          }
        _ => return false
        }
    }; //.expect("user_wrote_path failed ");
    //let mut writer = BufWriter::new(file_2_write_path);
    let key = format!("{}", key);
    file_2_write_path.write_all(key.as_bytes()).expect("user_wrote_path failed write in");
    crate::globs18::unblock_fd(file_2_write_path.as_raw_fd());
    let mut written_path = read_user_written_path();
    let written_path_from_prnt = get_path_from_prnt();
    if written_path_from_prnt.chars().count() > written_path.chars().count(){written_path = written_path_from_prnt;}
    complete_path(&written_path, "-maxdepth 1", false);
    form_cmd_line_default();
    true
}
pub(crate) fn read_user_written_path() -> String{
    let save_path = user_wrote_path();
    let mut file_2_read_path = match File::open(save_path){
        Ok(f) => f,
        Err(e) => return "".to_string()
    };
    let mut reader = BufReader::new(file_2_read_path);
    let mut ret = String::new();
    reader.read_to_string(&mut ret).expect("read_user_written_path failed write in");
    ret
}