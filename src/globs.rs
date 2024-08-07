use chrono::format; use std::io::BufRead;
use num_traits::ToPrimitive;
use crate::update18::upd_screen_or_not;
use crate::{add_cmd_in_history, cached_ln_of_found_files, manage_lst, name_of_front_list, read_tail, run_cmd_out_sync, save_file_append_newline};
use crate::custom_traits::{STRN, helpful_math_ops, fs_tools};
use crate::{exts::globs_uses, run_cmd0, ps18::{shift_cursor_of_prnt, get_prnt, set_ask_user}, 
swtch::{local_indx, front_list_indx, check_mode, SWTCH_USER_WRITING_PATH, SWTCH_RUN_VIEWER, swtch_fn, set_user_written_path_from_prnt,
     set_user_written_path_from_strn, user_wrote_path}, 
     core18::calc_num_files_up2_cur_pg, func_id18, ln_of_found_files, read_prnt, get_path_from_strn, repeat_char, set_prnt, rm_file, file_prnt,
      get_mainpath, run_cmd_str, get_tmp_dir, read_file, mark_front_lst, split_once, fix_num_files, i64_2_usize, cpy_str, set_front_list, read_front_list, 
      save_file, TMP_DIR_, where_is_last_pg, run_cmd_out, tailOFF, get_path_from_prnt, from_ls_2_front, set_num_files, clean_cache, drop_ls_mode, popup_msg,
       set_full_path, update18::background_fixing, save_file_append_abs_adr, checkArg, get_arg_in_cmd, shm_tam_dir, read_file_abs_adr, u64_from_strn, 
       save_file_abs_adr0, errMsg0};
self::globs_uses!();
use once_cell::unsync::Lazy as UnsyncLazy;
pub const MAIN0_: i64 =  1;
pub const FRONT_: i64 =  2;
pub const FILTERED_: i64 =  3;
pub const MERGE_: i64 =  4;
pub const LS_: i64 =  5;
pub const ADD: i64 =  6;
pub const INS: i64 =  7;
pub const DEL: i64 =  8;
pub const GET: i64 =  9;
pub const LEN: i64 =  10;
pub const SET_FRONT_LIST: i64 =  11;
pub fn rm_char_from_string(indx: usize, origString: &String) -> String{
    let len = origString.chars().count(); let mut ret = String::new();
    for i in 0..len{
        if i != indx{
         let char1: char = match origString.chars().nth(i){
            Some(ch) => ch,
            _ => {return ret}
        };
        ret.push(char1);
        }
    }
    ret
}
pub(crate) fn check_strn_in_lst(list: &str, str1: &str) -> bool{
    let func_name = "check_strn_in_lst";
    let adr_of_lst = take_list_adr_env(list);
    let mut file = match std::fs::File::options().read(true).open(adr_of_lst){
        Ok(f) => f, Err(e) => {println!("{func_name} failed {e:?}"); return false}
    };
    let mut read_file = crate::BufReader::new(file);
    for line in read_file.lines(){
        let line0 = line.unwrap_or("".strn() ).trim_end().strn();
        if line0 == str1{return true}
    }
    false
}
pub(crate) fn exclude_strn_from_list(strn: String, list: &str){
    let list_tmp = format!("{}/{}.tmp", get_tmp_dir(18551), list);
    let list = format!("{}/{}", get_tmp_dir(18551), list);
    let cmd = format!("grep -Ev '{}' {} > {}", strn, list, list_tmp);
    run_cmd_str(cmd.as_str());
    let cmd = format!("mv {} {}", list_tmp, list);
    run_cmd_str(cmd.as_str());
}
pub(crate) fn check_symb_in_strn(strn: &String, symb: &str) -> bool{
    let symb = symb.to_string();
    let len = strn.chars().count();
    for ch in strn.chars(){
        if Some(ch) == symb.chars().nth(0){return true}
    }
    false
}
pub(crate) fn sieve_list(data: String){
    //clean_cache("filter");
    //clean_cache("filter_history");
    //crate::clean_fast_cache(Some(true) );
    if check_symb_in_strn(&data, "|"){return sieve_list0(data)}
    let data0 = data.replace("sieve ", "");
    let (mut opts, mut data0) = split_once(&data0, " ");
    if opts == "none".strn() || data0 == "none".strn(){
        set_ask_user("example: sieve -Ei some\\shere", 5977871);
    }
    let mut history_mode = "".strn();
    if check_substrn(&name_of_front_list("", false), "history"){history_mode = "_history".strn();}
    if opts == "none"{return}
    if data0 == "none"{
        data0 = opts;
        opts = "-Ei".to_string()}
    let found_files_path = format!("{}/found_files", get_tmp_dir(18441));
    let filter_file_path_tmp = format!("{}/filter{history_mode}.tmp", get_tmp_dir(18441));
    let filter_file_path = format!("{}/filter{history_mode}", get_tmp_dir(18441));
   #[cfg(feature = "mae")] {let filter_name = format!("filter{history_mode}" ); crate::cache::set_uid_cache(&filter_name);}
    let cmd = format!("echo '' > {}", filter_file_path_tmp);
    run_cmd_str(cmd.as_str());
    let cmd = format!("grep {} {} {} > {}", opts, crate::full_escape(&data0), found_files_path.clone().unreel_link_to_file0(), filter_file_path_tmp);
    run_cmd_str(cmd.as_str());
    if match std::fs::metadata(&filter_file_path_tmp){
        Ok(g) => g,
        _=> return sieve_list0(data)
    }.len() == 0{sieve_list0(data); return}
    let cmd = format!("mv {} {}", filter_file_path_tmp, filter_file_path);
    run_cmd_str(cmd.as_str());
    let cmd = format!("#filter as front\nln -sf {} {}", filter_file_path, found_files_path);
    run_cmd_str(cmd.as_str());
    crate::set_front_list2(&format!("filter{history_mode}"), 2 );
    let dbg = crate::fix_num_files0(5977871);
    let dbg1 = dbg;
    set_full_path(&data, -19784542001);
    add_cmd_in_history(&format!("sieve {data}") );
}
pub(crate) fn sieve_list0(data: String){
    let data = data.replace("sieve ", "");
    let (mut opts, mut data) = split_once(&data, " ");
    if opts == "none".to_string() || data == "none".to_string(){
        set_ask_user("example: sieve -Ei some\\shere", 5977871);
    }
    let mut history_mode = "".strn();
    if check_substrn(&name_of_front_list("", false), "history"){history_mode = "_history".strn();}
    if opts == "none"{return}
    if data == "none"{
        data = opts;
        opts = "-Ei".to_string()}
    let found_files_path = format!("{}/found_files", get_tmp_dir(18441));
    let filter_file_path_tmp = format!("{}/filter{history_mode}.tmp", get_tmp_dir(18441));
    let filter_file_path = format!("{}/filter{history_mode}", get_tmp_dir(18441));
    #[cfg(feature = "mae")] {let filter_name = format!("filter{history_mode}" ); crate::cache::set_uid_cache(&filter_name);}
    let cmd = format!("echo '' > {}", filter_file_path_tmp);
    run_cmd_str(cmd.as_str());
    let cmd = format!("grep {} '{}' {} > {}", opts, data, found_files_path.clone().unreel_link_to_file(), filter_file_path_tmp);
    run_cmd_str(cmd.as_str());
    if match std::fs::metadata(&filter_file_path_tmp){
        Ok(g) => g,
        _=> return errMsg0("Sorry, sieve failed")
    }.len() == 0{errMsg0("Sorry, sieve failed"); return}
    let cmd = format!("mv {} {}", filter_file_path_tmp, filter_file_path);
    run_cmd_str(cmd.as_str());
    let cmd = format!("#filter as front\nln -sf {} {}", filter_file_path, found_files_path);
    run_cmd_str(cmd.as_str());
    crate::set_front_list2(&format!("filter{history_mode}"), 2 );
    let dbg = crate::fix_num_files0(5977871);
    let dbg1 = dbg;

    add_cmd_in_history(&format!("sieve {data}") );
}
pub(crate) fn merge(data: String){
    drop_ls_mode();
    let data = data.replace("mrg ", "");
    set_ask_user("example: 'mrg' to add entire front list to merge or 'mrg /path/to/file' or 'mrg <file's index>'", 5977871);
    let found_files_path = format!("{}/found_files", get_tmp_dir(1911441));
    let filter_file_path_tmp = format!("{}/merge.tmp", get_tmp_dir(1911441));
    let filter_file_path = format!("{}/merge", get_tmp_dir(1911441));
    let indx = match i64::from_str_radix(&data, 10){
        Ok(i) => i,
        _ => i64::MIN
    };
   let mut path = get_path_from_strn(data);
    if indx > i64::MIN{
        let fname = /*crate::escape_apostrophe(&*/get_item_from_front_list(indx, true);//);
        //let cmd = format!("echo '{}' >> {}", fname, filter_file_path);
        crate::save_file_append_newline_abs_adr(fname.clone(), filter_file_path);
        if path == ""{path = fname}
        let info = format!("Merge: {path}");
        set_full_path(&info, 97400148467);    
        //run_cmd_str(cmd.as_str());   
        return;
    }
    if path.len() > 0 {
        let cmd = format!("echo '{}' >> {}", path, filter_file_path);
        run_cmd_str(cmd.as_str());
        let info = format!("Merge: {path}");
        set_full_path(&info, 97400148467);    
        return;
    }
    let cmd = format!("cat {} >> {}", found_files_path, filter_file_path);
    run_cmd_str(cmd.as_str());
    //let cmd = format!("mv {} {}", filter_file_path_tmp, filter_file_path);
    //run_cmd_str(cmd.as_str());
  //  let dbg = crate::fix_num_files0(64977871);
    //let dbg1 = dbg;
}
pub(crate) fn clear_merge(){
    let filter_file_path = format!("{}/merge", get_tmp_dir(1911471));
    rm_file(&filter_file_path);
    clean_cache("merge");
    crate::key_handlers::F1_key();
}
pub(crate) fn show_ls(){
    unsafe{set_ls_as_front(); front_list_indx(crate::globs18::LS_);}
    crate::ps18::fix_num_files(-13972);
}
pub(crate) fn get_num_pg_4_main0() -> i64{
    let num_pg = crate::read_file("main0.pg");
    match i64::from_str_radix(&num_pg, 10){
        Ok(num) => return num,
        _ => return 0
    };
}
pub(crate) fn set_valid_list_as_front(){
    let tmp_dir = get_tmp_dir(-6015);
    let active_lst = format!("{}/{}", &tmp_dir, read_file("front_list"));
    let front_list_link = format!("{}/found_files", &tmp_dir);
    let cmd = format!("#valid list as front\nln -sf {} {}", active_lst, front_list_link);
    run_cmd_str(&cmd);
}
pub(crate) fn set_valid_list_as_front0(){
    let front_list = take_list_adr("found_files").unreel_link_to_depth(1);
    let front_list = read_tail(&front_list, "/");
    set_front_list(&front_list);
    upd_screen_or_not((0, front_list) );
}
pub fn unblock_fd(fd: RawFd) -> io::Result<()> {
    let flags = unsafe { fcntl(fd, F_GETFL, 0) };
    if flags < 0 {return Err(io::Error::last_os_error());}
    let flags = flags | O_NONBLOCK;
    let res = unsafe { fcntl(fd, F_SETFL, flags) };
    if res != 0 {return Err(io::Error::last_os_error());}
    Ok(())
}
pub fn bksp() -> String{
       let mut len = get_prnt(-3).chars().count(); if len == 0 {return len.to_string();}
     let mut ret = String::new();
     let prnt = get_prnt(-3);
     len.dec();
    let mut indx = unsafe {shift_cursor_of_prnt(2, None, -2).shift};
    if indx <= len {indx = len - indx;}
    ret = rm_char_from_string(indx, &prnt);
    if len == 0{save_file("".to_string(), "prnt".to_string());}
        ////println!("ret {}", ret);
 //   let ls_mode = take_list_adr("ls.mode");
    let is_path = get_path_from_prnt();
   // if is_path == ""{from_ls_2_front(ls_mode)}
    ret
}
pub fn ins_last_char_to_string1_from_string1(indx: usize, origString: String) -> String{
    let mut len = origString.chars().count(); if len == 0 || len == 1 || indx == len -1 {return origString.to_string();}
     let mut ret = String::new();
     len.dec(); 
    let char0: char =match origString.chars().nth(len){
        Some(ch) => ch,
        _ => {/*println!("kkkkkkkkk");*/ return origString.to_string();}
    };
    if crate::dirty!(){
    let msg = format!("'ins_last_char_to_string1_from_string1 indx {} orig{} char0 {} orig len {}'", indx, origString, char0, len);
    //run_cmd0(&msg);
    println!("{}", &msg);
    }
    for i in 0..len {
        let char1: char = origString.chars().nth(i).unwrap();
        if i == indx{ret.push(char0);}
        ret.push(char1);
       // println!("{}", char1);
    }
    ////println!("ret {}", ret);
    ret
}
pub fn ins_patch_to_string(indx: usize, origString: String, patch: &String) -> String{
    let mut len = origString.chars().count(); 
    //if len == 0 || len == 1 || indx == len -1 {return origString.to_string();}
     let mut ret = String::new();    
    if crate::dirty!(){
    let msg = format!("'ins_patch_to_string indx {} orig{} char0 {} orig len {}'", indx, origString, patch, len);
    //run_cmd0(&msg);
    println!("{}", &msg);
    }
    ret  = format!("{}{}{}", origString.substring(0, indx), patch, origString.substring(indx, len));
    ret
}
pub fn ins_last_char_to_string1_from_string1_ptr(indx: usize, origString: &mut String) {
    let mut len = origString.chars().count(); if len == 0 || len == 1 || indx == len -1 {return}
     let mut ret = String::new();
     len.dec(); 
    let char0: char =match origString.chars().nth(len){
        Some(ch) => ch,
        _ => {/*println!("kkkkkkkkk");*/ return;}
    };
    if crate::dirty!(){
    let msg = format!("'ins_last_char_to_string1_from_string1 indx {} orig{} char0 {} orig len {}'", indx, origString, char0, len);
    //run_cmd0(&msg);
    println!("{}", &msg);
    }
    for i in 0..len {
        let char1: char = origString.chars().nth(i).unwrap();
        if i == indx{ret.push(char0);}
        ret.push(char1);
       // println!("{}", char1);
    }
    origString.clear(); origString.push_str(ret.as_str());
    ////println!("ret {}", ret);
}
pub(crate) fn print_type_of<T>(_: &T) {
println!("{}", std::any::type_name::<T>())
}
pub fn eq_str(str1: &str, str2: &str) -> i64{
let str1_len = str1.len();
let str2_len = str2.len();
if str1_len == 0 || str2_len == 0{return i64::MIN}
let mut result: i64 = 0;
let mut i: usize = 0;
while i < str1_len && i < str2_len {
    if str1.chars().nth(i) == None || str1.chars().nth(i) == None{break;}
    if crate::dirty!(){ println!("eq_str char1 {:?} to char2 {:?} i {}", str1.chars().nth(i), str2.chars().nth(i), i);}
    let a = str1.chars().nth(i).unwrap();
    let b = str2.chars().nth(i).unwrap();
    if a < b {
        result = -1;
        break;
    } 
    if a > b {
        result = 1;
        break;
    }
    i += 1;
}
result
}
pub fn eq_ansi_str(str1: &str, str2: &str) -> i64{
let mut ansi_str1 = str1.bytes(); //ANSIString::from(str1);
let mut ansi_str2 = str2.bytes(); //ANSIString::from(str2);
let str1_len = str1.len();
let str2_len = str2.len();
if str1_len == 0 || str2_len == 0{return i64::MIN}
if str1_len != str2_len{return i64::MAX}
// Loop over the strings and compare each character
let mut result: i64 = 0;
let mut i: usize = 0;
while i < str1_len && i < str2_len {
    let char_of_str1 = ansi_str1.next();
    let char_of_str2 = ansi_str2.next();
    if char_of_str1 == None || char_of_str2 == None{println!("char is None"); break;}
    if crate::dirty!(){ println!("eq_ansi_str char1 {:?} to char2 {:?} i {}", str1.chars().nth(i), str2.chars().nth(i), i);}
    let a = char_of_str1.unwrap();
    let b = char_of_str2.unwrap();
    if a < b {
        result = -1;
        break;
    } 
    if a > b {
        result = 1;
        break;
    }
    i += 1;
}
result
}
pub fn add_2_front_list(val: &str, func_id: i64) -> String{
    let mut list_id: (i64, bool) = (1i64, false);
    for i in 0..1000_000{
        list_id = unsafe {front_list_indx(i64::MAX)};
        if list_id.1{break;}
    }
    if !list_id.1{set_ask_user("Can't access to Front list", func_id); return "!!no¡".to_string()}
    return unsafe{lists(val, list_id.0, 0, ADD)}
}
pub fn add_2_main0_list(val: &str) -> String{
   return unsafe{lists(val, MAIN0_, 0, ADD)}
   // save_file_append_newline(val.strn(), "main0".strn());
   // val.strn()
}
pub fn len_of_main0_list() -> String{
    let fst_var = unsafe{lists("", MAIN0_, 0, LEN)};
    if fst_var != "0"|| fst_var != ""{return fst_var}
    return len_of_list_wc("main0");
}
pub fn raw_len_of_front_list() -> String{
      let mut list_id: (i64, bool) = (1i64, false);
    for i in 0..1000{
        list_id = unsafe {front_list_indx(i64::MAX)};
        if list_id.1{break;}
    }
    if !list_id.1{set_ask_user("Can't access to Front list", -1); return "!!no¡".to_string()}
    let front_list = read_front_list();
    if front_list == "main0"{return len_of_main0_list()}
    return unsafe{lists("", list_id.0, 0, LEN)}
}
pub fn len_of_front_list() -> String{
      let mut list_id: (i64, bool) = (1i64, false);
   /* for i in 0..1000{
        list_id = unsafe {front_list_indx(i64::MAX)};
        if list_id.1{break;}
    }*/
  //  if !list_id.1{set_ask_user("Can't access to Front list", -1); return "!!no¡".to_string()}
    let name_front_list = read_front_list();
    let mut front_list = take_list_adr_len(&name_front_list);
    //if front_list != "main0.len"{return len_of_list_wc(&front_list);}
    let num = read_file_abs_adr(&front_list);
    if num == ""{return len_of_list_wc(&name_front_list)}
    return num;
}
pub fn len_of_list_wc(name: &str) -> String{
    let mut list_adr = take_list_adr_env(&name);
    let cmd = format!("wc -l {list_adr}");
    let len_list = crate::run_cmd_out_sync(cmd);
    list_adr = take_list_adr_len(&name);
    let (len_list, _) = split_once(&len_list, " ");
    crate::rewrite_file_abs_adr(cpy_str(&len_list), list_adr.to_string());
    return len_list;
}
pub fn len_of_front_list_wc() -> String{
      let mut list_id: (i64, bool) = (1i64, false);
    for i in 0..1000{
        list_id = unsafe {front_list_indx(i64::MAX)};
        if list_id.1{break;}
    }
    if !list_id.1{set_ask_user("Can't access to Front list", -1); return "!!no¡".to_string()}
    let mut front_list = read_front_list();
    let front_list_adr = take_list_adr_env(&front_list);
    let cmd = format!("wc -l {front_list_adr}");
    let len_front_list = crate::run_cmd_out_sync(cmd);
    front_list = take_list_adr_len(&front_list);
    let (len_front_list, _) = split_once(&len_front_list, " ");
    crate::save_file_abs_adr0(cpy_str(&len_front_list), front_list);
    return len_front_list;
}
pub(crate) fn get_proper_indx(indx: i64, fixed_indx: bool) -> (usize, i64){
    let last_pg = where_is_last_pg();
    let mut indx = indx;
    if indx < 0{
        let mut indx = indx * -1;
        if last_pg < indx && last_pg > 0{
            indx = last_pg - (indx/last_pg) * last_pg;
            return (i64_2_usize(indx), indx); 
        }
        let indx = last_pg - indx + 1;
        return (i64_2_usize(indx), indx);
    }
    let mut fix_inputed_indx = indx;
    if !unsafe {local_indx(false)} && fixed_indx == true {fix_inputed_indx += calc_num_files_up2_cur_pg(); indx = fix_inputed_indx;}
    let mut proper_indx: i64 = 0;
    let mut len: i64 = 0;
    if indx > 0{proper_indx = indx;}
    len = match i64::from_str_radix(len_of_front_list().as_str(), 10){
        Ok(i64_) => i64_,
        _ => 0
    };
    if len == 0{return (0usize, 0i64)}
    return (i64_2_usize(proper_indx), proper_indx);
}
pub(crate) fn get_proper_indx_tst(indx: i64, fixed_indx: bool) -> (usize, i64){
    //let last_pg = where_is_last_pg();
    /*if indx < 0{
        let mut indx = indx * -1;
        if last_pg < indx{
            indx = last_pg - (indx/last_pg) * last_pg;
            return (i64_2_usize(indx), indx); 
        }
        let indx = last_pg - indx + 1;
        return (i64_2_usize(indx), indx);
    }*/
    let mut fix_inputed_indx = indx;
    //if !unsafe {local_indx(false)} && fixed_indx {fix_inputed_indx += calc_num_files_up2_cur_pg();}
    let indx = fix_inputed_indx;
    let mut proper_indx: i64 = 0;
    let mut len: i64 = 0;
    if indx > 0{proper_indx = indx;}
    println!("{}", proper_indx.to_string());
    println!("{}", indx.to_string());
 
    return (i64_2_usize(proper_indx), proper_indx);
}
pub(crate) fn get_item_from_front_list(indx: i64, fixed_indx: bool) -> String{
    let proper_indx = get_proper_indx(indx, fixed_indx);
    if proper_indx.0 == usize::MAX{return "front list is empty".to_string()}
    //if !list_id.1{set_ask_user("Can't access to Front list", -1); return "!!no¡".to_string()}
    return crate::C!(lists("", FRONT_, proper_indx.0, GET))
}
pub fn set_main0_as_front(){crate::drop_ls_mode(); mark_front_lst("main0"); unsafe{lists("", MAIN0_, 0, SET_FRONT_LIST);}}
pub fn set_ls_as_front() -> String{
      let mut list_id: (i64, bool) = (1i64, false);
  //  for i in 0..1000{
        list_id = crate::C!(front_list_indx(LS_));
    //    if list_id.1{break;}
    //}
    //if !list_id.1{set_ask_user("Can't access to Front list", -1); return "!!no¡".to_string()}
    let tmp_dir = get_tmp_dir(-66774125);
    let ls_path = format!("{}/{}", &tmp_dir, "ls");
    let found_files_path = format!("{}/{}", &tmp_dir, "found_files");
    let cmd = format!("touch -f {}", &ls_path);
    run_cmd_str(&cmd);
    let cmd = format!("#sets ls as front\nln -sf {} {}", ls_path, found_files_path);
    run_cmd_str(&cmd);
    mark_front_lst("ls");
    let ret = || -> String{crate::C!(lists("", LS_, 0, SET_FRONT_LIST)); return "ok".to_string()};
    //background_fixing();
    ret()
}
pub unsafe fn lists(val: &str, list: i64, indx: usize, op_code: i64) -> String{
static mut MAIN0: Vec<String> = Vec::new();
let mut FRONT: &[String] = MAIN0.as_mut_slice();
//tst.to_vec().push("".to_string());
//static mut LS: RwLock<Lazy<Vec<String>>> = RwLock::new(Lazy::new(||{vec!["".to_string()]})); // OnceCell<Vec<String>> = OnceCell::new();
/*if Some(LS.get()) != None{
    let mut ls_vec: Vec<String> = Vec::new();
    LS.set(ls_vec);
}*/
let mut list = list;
let front_list = read_front_list();
//set_front_list(&front_list);
if front_list == "main0"{list = MAIN0_;}
if front_list == "ls"{list = LS_;}
if list == MAIN0_ {
    if op_code == GET{
        if MAIN0.len() <= indx{return "".to_string();}
        let mut ret = crate::cpy_str(&MAIN0[indx]);
        ret = crate::rec_from_patch(&ret).unwrap_or(ret);
        return ret.to_string()
    }
    if op_code == ADD{
        MAIN0.push(val.to_string());
       return "ok".to_string()
    }
    if op_code == LEN{return MAIN0.len().to_string()}
    if op_code == SET_FRONT_LIST {
       FRONT = MAIN0.as_mut_slice();
       let main_path = get_tmp_dir(-13374);
       let main0_path = format!("{}/{}", &main_path, "main0");
       let found_files_path = format!("{}/{}", &main_path, "found_files");
       if !Path::new(&main0_path).exists(){
        let cmd = format!("touch -f {}", &main0_path);
        run_cmd_str(&cmd);
       }
       let cmd = format!("#sets main0 as front\nln -sf {} {}", main0_path, found_files_path);
       run_cmd_str(&cmd);
       return "main0 gets set as front".to_string();
    }
}
if list == LS_ {
    if op_code == GET{
        let ret = ln_of_found_files(indx).0;
        return ret.to_string()
    }
    if op_code == ADD{
        /*LS.write().expect("Can't write into LS").push(val.to_string());
        let mut len = LS.read().unwrap().len() - 1;
        if len > 2{len -= 2;}
        set_ask_user(& LS.read().unwrap()[len], -1);*/
       return "dummy ls ADD".to_string()
    }
    if op_code == LEN{return ln_of_found_files(usize::MAX).1.to_string()}
    if op_code == SET_FRONT_LIST {
       
       return "ls gets set as front".to_string();
    }
}
if list == FRONT_ {
    if op_code == GET{
        let mut ret = String::new();
        if FRONT.len() > indx && list == MAIN0_{
        ret = cpy_str(&FRONT[indx]);
        ret = crate::rec_from_patch(&ret).unwrap_or(ret);
    }
        else{ret = cached_ln_of_found_files(indx).0;}
        return ret.to_string()}//return FRONT.get().unwrap()[indx].to_string()}
    if op_code == LEN{return ln_of_found_files(usize::MAX).1.to_string()}//return FRONT.get().unwrap().len().to_string()}
}
"wrong".to_string()
}
pub(crate) fn take_list_adr(name: &str) -> String{
    return format!("{}/{name}", crate::bkp_tmp_dir(None, false));
}
pub(crate) fn renew_lists(new_item: String){
    let front_lst = take_list_adr(&read_front_list());
    let main0 = take_list_adr("main0");
    let cmd = format!("echo {new_item} >> {}", front_lst);
    run_cmd_str(cmd.as_str());
    let size = std::fs::metadata(&main0).unwrap().len() - 4;
    let cmd = format!("truncate -s {size} {main0}");
    run_cmd_str(cmd.as_str());
    let stop = crate::getStop_code__!();
    let cmd = format!("echo {new_item} >> {main0}");
    run_cmd_str(cmd.as_str());
    let cmd = format!("echo {stop} >> {main0}");
    run_cmd_str(cmd.as_str());
    add_2_main0_list(&new_item);
}
pub(crate) fn split_once_alt(strn: &String, delim: &String) -> (String, String){
    let mut maybe = String::new();
    let mut found = false;
    let delim_len = delim.chars().count();
    let strn_len = strn.chars().count();
    let mut count_delim_chars = 0usize;
    let mut ret = (String::new(), String::new());
    for i in strn.chars(){
        if count_delim_chars < delim_len && Some(i) == delim.chars().nth(count_delim_chars) && !found{
            maybe.push(i);
            count_delim_chars += 1;
            if maybe == *delim {found = true;}
        } else {
            if found {ret.1.push(i); continue;}
           // if maybe == *delim {ret.1.push(i); found = true; continue;}
            count_delim_chars = 0;
            ret.0.push_str(maybe.as_str());
            ret.0.push(i);
            maybe = String::new();
        }
    }
    if !found {return ("none".to_string(), "none".to_string())}
    ret
}
pub(crate) fn check_substrn(strn: &String, delim: &str) -> bool{
    let mut maybe = String::new();
    let delim_len = delim.chars().count();
    let strn_len = strn.chars().count();
    let mut count_delim_chars = 0usize;
    for i in strn.chars(){
        if count_delim_chars < delim_len && Some(i) == delim.chars().nth(count_delim_chars){
            maybe.push(i);
            count_delim_chars += 1;
            //println!("{}", maybe);
        } else {
            if maybe == *delim {return true}
            count_delim_chars = 0;
            maybe = String::new();
            if count_delim_chars < delim_len && Some(i) == delim.chars().nth(count_delim_chars){
            maybe.push(i);
            count_delim_chars += 1;
            }
        }
    }
    if maybe == *delim {return true}
    false
}
pub(crate) fn check_substrn01(strn: &String, delim: &str) -> bool{
    let mut maybe = String::new();
    let delim_len = delim.chars().count();
    let strn_len = strn.chars().count();
    let mut count_delim_chars = 0usize;
    for i in strn.chars(){
        if count_delim_chars < delim_len && Some(i) == delim.chars().nth(count_delim_chars){
            maybe.push(i);
            count_delim_chars += 1;
            //println!("{}", maybe);
        } else {
            if maybe == *delim {return true}
            count_delim_chars = 0;
            maybe = String::new();
            if count_delim_chars < delim_len && Some(i) == delim.chars().nth(count_delim_chars){
            maybe.push(i);
            count_delim_chars += 1;
            }
        }
    }
    if maybe == *delim {return true}
    false
}

pub(crate) fn decode_sub_cmd(cmd: &String, sub_cmd: &str) -> (String, bool){
    let sub_cmd0 = sub_cmd.to_string();
    let mut full_sub_cmd = String::new(); let mut val = String::new();
    if crate::globs18::check_substrn(&cmd, sub_cmd){
        let (_, sub_cmd) = split_once_alt(&cmd, &sub_cmd.to_string());
        let (sub_cmd_val, _) = split_once_alt( &sub_cmd, &"::".to_string());
        if sub_cmd_val == "none"{errMsg0("Example of sub-command: >>>lst::name_of_list::<<<"); return (cmd.to_string(), false);}
        full_sub_cmd = format!("{sub_cmd0}{sub_cmd_val}::"); val = sub_cmd_val;
    }
    match sub_cmd{
        "lst::" => {
            let lst_adr = take_list_adr_env(&val);
            if full_sub_cmd ==""{return (cmd.to_string(), false)}
            return (cmd.replace(&full_sub_cmd, &lst_adr), true);
        }
        _ => return (cmd.to_string(), false)
    }
}
pub(crate) fn decode_sub_cmds(cmd: &String) -> String{
    let mut ret0 = cmd.to_string();
    loop {
        let ret = decode_sub_cmd(&ret0, "lst::");
        if ret.1{ ret0 = ret.0; continue; }
        {break;}
    }
#[cfg(feature="in_dbg")] {save_file(ret0.clone(), "decoded_prnt".to_string()); crate::report(&ret0, "decoded_prnt");}
#[cfg(feature = "mae")] crate::cache::upd_uids_for_lsts();
    ret0    
}
pub(crate) fn take_list_adr_env(name: &str) -> String{
    match name {
        "main0" => return take_list_adr("main0"),
        "found_files" => return take_list_adr("found_files"),
        "filter" => return take_list_adr("filter"),
        "filter_history" => return take_list_adr("filter_history"),
        "cd" => return take_list_adr("cd"),
        "ls" => return take_list_adr("ls"),
        "merge" => return take_list_adr("merge"),
        "lst" => return take_list_adr("lst"),
        "history" => return take_list_adr("history"),
        "mae" => return take_list_adr("mae"),
        "decrypted" => return take_list_adr("decrypted"),
        "copied" => return take_list_adr("copied"),
        "none" => return "/not_existed_at_All".strn(),
        _ => return take_list_adr(&crate::full_escape(&format!("env/lst/{name}"))),
    }
}
pub(crate) fn take_list_adr_len(name: &str) -> String{
    match name {
        "main0" => return take_list_adr("main0.len"),
        "filter" => return take_list_adr("filter.len"),
        "filter_history" => return take_list_adr("filter_history.len"),
        "cd" => return take_list_adr("cd.len"),
        "ls" => return take_list_adr("ls.len"),
        "merge" => return take_list_adr("merge.len"),
        "lst" => return take_list_adr("lst.len"),
        "history" => return take_list_adr("history.len"),
        "mae" => return take_list_adr("mae.len"),
        "decrypted" => return take_list_adr("decrypted.len"),
        "copied" => return take_list_adr("copied.len"),
        _ => return take_list_adr(&crate::full_escape(&format!("env/lst_opts/{name}.len"))),
    }
}
pub(crate) fn take_list_adr_pg(name: &str) -> String{
    match name {
        "main0" => return take_list_adr("main0.pg"),
        "filter" => return take_list_adr("filter.pg"),
        "filter_history" => return take_list_adr("filter_history.pg"),
        "cd" => return take_list_adr("cd.pg"),
        "ls" => return take_list_adr("ls.pg"),
        "merge" => return take_list_adr("merge.pg"),
        "lst" => return take_list_adr("lst.pg"),
        "history" => return take_list_adr("history.pg"),
        "mae" => return take_list_adr("mae.pg"),
        "decrypted" => return take_list_adr("decrypted.pg"),
        "copied" => return take_list_adr("copied.pg"),
        _ => return take_list_adr(&crate::full_escape(&format!("env/lst_opts/{name}.pg"))),
    }
}
pub(crate) fn drop_key(Key: &mut String, ext: &mut Option<&mut crate::__ext_msgs::_ext_msgs>) -> String{
    Key.clear();
    return crate::hotKeys(Key, ext);
}
pub(crate) fn strn_2_u64(strn: String) -> Option<u64>{
    match u64::from_str_radix(&strn, 10){
        Ok(num) => Some(num),
        _ => None
    }
}
pub(crate) fn strn_2_usize(strn: String) -> Option<usize>{
    match usize::from_str_radix(&strn, 10){
        Ok(num) => Some(num),
        _ => None
    }
}
pub(crate) fn seg_size() -> usize{
    static mut fst_run: bool = false;
    static mut seg_size: usize = 150;
    if !unsafe {fst_run}{
        unsafe {fst_run = true;}
        if crate::checkArg("-cache-seg-size"){
            let seg_size_new = String::from_iter(crate::get_arg_in_cmd("-cache-seg-size").s).trim_end_matches('\0').to_string();
            let ret = strn_2_usize(seg_size_new);
            if ret != None{unsafe {seg_size = ret.unwrap();}}
        }
    }
    unsafe{seg_size}
}
pub(crate) fn check_char_in_strn(strn: &String, is_there_ch: char) -> String{
    for ch in strn.chars(){
        if ch == is_there_ch{return String::from(is_there_ch);}
    }
    "no".to_string()
}
pub(crate) fn instance_num() -> u64{
    let path_2_id_suffix = format!("{}/{}", shm_tam_dir(None), crate::full_escape( &id_suffix() ) ); let num = crate::read_file_abs_adr0(&path_2_id_suffix); 
    let num = u64_from_strn(&num).0; save_file_abs_adr0((num + 1).to_string(), path_2_id_suffix); num
}
pub(crate) fn id_suffix() -> String{
    if checkArg("-window-mark"){
        return String::from_iter(get_arg_in_cmd("-window-mark").s).trim_end_matches('\0').to_string() 
    }
    return format!("{}TR{}", crate::getStop_code__!(), crate::getStop_code__!())
}
pub(crate) fn gen_win_title() -> String{
    let win_num = instance_num(); let win_class_id = id_suffix(); return format!("{win_num}{win_class_id}");
}
pub(crate) fn check_patch_mark(strn: &String) -> bool{
    let strn_len = strn.chars().count();
    let mut strn = strn;
    let patch_mark_len = "::patch".to_string().chars().count();
    if strn_len > patch_mark_len && strn.substring(strn_len - patch_mark_len, strn_len) == "::patch"{return true} false
}
pub(crate) fn split_strn_by_not_escaped_spaces(strn: &String) -> Vec<String>{
    let mut vec: Vec<String> = Vec::new(); let strn = strn.trim_start_matches(' ').trim_end_matches(' ').to_string();
    let len = strn.chars().count();
    let mut slice = "".to_string();
    for v in 0..len{
        let ch = strn.chars().nth(v);
        if ch == Some(' ') && v > 0 && strn.chars().nth(v - 1) != Some('\\'){vec.push(slice.clone()); slice.clear(); continue;}
        slice.push(ch.unwrap());
    }
    vec
}
pub(crate) fn enum_not_escaped_spaces_in_strn(strn: &String) -> Vec<usize>{
    let mut vec: Vec<usize> = Vec::new(); let strn = strn.trim_start_matches(' ').trim_end_matches(' ').to_string();
    let len = strn.chars().count();
    for v in 0..len{
        let ch = strn.chars().nth(v);
        if ch == Some(' ') && v > 0 && strn.chars().nth(v - 1) != Some('\\'){ vec.push(v.clone()); }
    }
    vec
}
pub(crate) fn enum_not_escaped_spaces_in_strn_down_to(strn: &String, bar: usize) -> Vec<usize>{
    let mut vec: Vec<usize> = Vec::new(); let strn = strn.trim_start_matches(' ').trim_end_matches(' ').to_string();
    let len = strn.chars().count();
    for v in 0..len{
        let ch = strn.chars().nth(v);
        if ch == Some(' ') && v > 0 && strn.chars().nth(v - 1) != Some('\\') && v > bar{ vec.push(v.clone()); }
    }
    vec
}
pub(crate) fn enum_not_escaped_spaces_in_strn_up_to(strn: &String, bar: usize) -> Vec<usize>{
    let mut vec: Vec<usize> = Vec::new(); let strn = strn.trim_start_matches(' ').trim_end_matches(' ').to_string();
    for v in 0..bar{
        let ch = strn.chars().nth(v);
        if ch == Some(' ') && v > 0 && strn.chars().nth(v - 1) != Some('\\'){ vec.push(v.clone()); }
    }
    vec
}
pub(crate) fn path_to_shm(path: Option<&String>) -> &'static String{
    static mut shm_adr: Lazy< String > = Lazy::new(||{String::new()});    
    static mut fst_run: bool = true;
    if unsafe {fst_run} {
        if path.is_some(){
            unsafe{*shm_adr = path.unwrap().strn(); fst_run = false}
            return Box::leak( Box::new("".strn() ) )
        }
    }
    return unsafe {Box::leak(Box::new(shm_adr.clone() ) ) }
}
pub(crate) fn Users_home_dir() -> String{
    let home = run_cmd_out_sync("cd ~;pwd".strn());
    if home.substring(0, 1) != "/"{errMsg0("Can't get User's home direcrory"); return "".strn()}
    home
}
pub(crate) fn load_bash_history(){
    let home_dir = Users_home_dir().trim_end().strn();
    let orig_bash = format!("{home_dir}/.bash_history");
    let cpy_to = format!("{home_dir}/bash_history_cpy");
    let bkp_bash = format!("cat {orig_bash} > {cpy_to}");
    run_cmd_out_sync(bkp_bash);
    let cmd_lst = format!("lst {home_dir}/bash_history_cpy");
    set_front_list("bash_history_cpy");
    manage_lst(&cmd_lst);
}
pub(crate) fn load_fish_history(){
    let home_dir = Users_home_dir().trim_end().strn();
    let orig_fish = format!("{home_dir}/.local/share/fish/fish_history");
    let cpy_to = format!("{home_dir}/.local/share/fish/fish_history_cpy");
    let bkp_fish = format!("cat {orig_fish}|grep -Ei '\\s*-\\s*cmd:'|sed 's/\\s*-\\s*cmd:/term /g'|uniq > {cpy_to}");
    run_cmd_out_sync(bkp_fish);
    let cmd_lst = format!("lst {home_dir}/.local/share/fish/fish_history_cpy");
    set_front_list("fish_history_cpy");
    manage_lst(&cmd_lst);
}
//fn