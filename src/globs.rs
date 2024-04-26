use chrono::format;
use num_traits::ToPrimitive;
use crate::{exts::globs_uses, run_cmd0, ps18::{shift_cursor_of_prnt, get_prnt, set_ask_user}, swtch::{local_indx, front_list_indx, check_mode, SWTCH_USER_WRITING_PATH, SWTCH_RUN_VIEWER, swtch_fn, set_user_written_path_from_prnt, set_user_written_path_from_strn, user_wrote_path}, core18::calc_num_files_up2_cur_pg, func_id18, ln_of_found_files, read_prnt, get_path_from_strn, repeat_char, set_prnt, rm_file, file_prnt, get_mainpath, run_cmd_str, get_tmp_dir, read_file, mark_front_lst, split_once, fix_num_files, i64_2_usize, cpy_str, set_front_list, read_front_list, save_file, TMP_DIR_, where_is_last_pg, run_cmd_out, tailOFF, get_path_from_prnt, from_ls_2_front, set_num_files, clean_cache, drop_ls_mode, popup_msg, set_full_path, update18::background_fixing, save_file_append_abs_adr};
self::globs_uses!();
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
    if check_symb_in_strn(&data, "|"){return sieve_list0(data)}
    clean_cache("filter");
    let data = data.replace("sieve ", "");
    let (mut opts, mut data) = split_once(&data, " ");
    if opts == "none".to_string() || data == "none".to_string(){
        set_ask_user("example: sieve -Ei some\\shere", 5977871);
    }
    if opts == "none"{return}
    if data == "none"{
        data = opts;
        opts = "-Ei".to_string()}
    let found_files_path = format!("{}/found_files", get_tmp_dir(18441));
    let filter_file_path_tmp = format!("{}/filter.tmp", get_tmp_dir(18441));
    let filter_file_path = format!("{}/filter", get_tmp_dir(18441));
    let cmd = format!("echo '' > {}", filter_file_path_tmp);
    run_cmd_str(cmd.as_str());
    let cmd = format!("grep {} {} {} > {}", opts, data, found_files_path, filter_file_path_tmp);
    run_cmd_str(cmd.as_str());
    if match std::fs::metadata(&filter_file_path_tmp){
        Ok(g) => g,
        _=> return sieve_list0(data)
    }.len() == 0{sieve_list0(data); return}
    let cmd = format!("mv {} {}", filter_file_path_tmp, filter_file_path);
    run_cmd_str(cmd.as_str());
    let cmd = format!("#filter as front\nln -sf {} {}", filter_file_path, found_files_path);
    run_cmd_str(cmd.as_str());
    mark_front_lst("filter");
    let dbg = crate::fix_num_files0(5977871);
    let dbg1 = dbg;
    set_full_path(&data, -19784542001);
}
pub(crate) fn sieve_list0(data: String){
    clean_cache("filter");
    let data = data.replace("sieve ", "");
    let (mut opts, mut data) = split_once(&data, " ");
    if opts == "none".to_string() || data == "none".to_string(){
        set_ask_user("example: sieve -Ei some\\shere", 5977871);
    }
    if opts == "none"{return}
    if data == "none"{
        data = opts;
        opts = "-Ei".to_string()}
    let found_files_path = format!("{}/found_files", get_tmp_dir(18441));
    let filter_file_path_tmp = format!("{}/filter.tmp", get_tmp_dir(18441));
    let filter_file_path = format!("{}/filter", get_tmp_dir(18441));
    let cmd = format!("echo '' > {}", filter_file_path_tmp);
    run_cmd_str(cmd.as_str());
    let cmd = format!("grep {} '{}' {} > {}", opts, data, found_files_path, filter_file_path_tmp);
    run_cmd_str(cmd.as_str());
    let cmd = format!("mv {} {}", filter_file_path_tmp, filter_file_path);
    run_cmd_str(cmd.as_str());
    let cmd = format!("#filter as front\nln -sf {} {}", filter_file_path, found_files_path);
    run_cmd_str(cmd.as_str());
    mark_front_lst("filter");
    let dbg = crate::fix_num_files0(5977871);
    let dbg1 = dbg;
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
    F1_key();
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
pub(crate) fn F1_key() -> String{
    let mut prnt: String = read_prnt();
   set_main0_as_front();
   crate::ps18::fix_num_files(-13971);
   clean_cache("main0");
format!("go2 {}", read_file("main0.pg"))
}
pub(crate) fn F3_key() -> String{
    let mut prnt: String = read_prnt();
    let orig_path = get_path_from_strn(crate::cpy_str(&prnt));
    if orig_path.len() == 0 {if tailOFF(&mut prnt, " "){
        crate::set_prnt(&prnt, -1);
    return prnt    
    }}
    crate::C_!(set_ls_as_front(); front_list_indx(crate::globs18::LS_););
       let ls_mode = take_list_adr("ls.mode");
    let mut ret_2_Front = || ->String{prnt = prnt.replace("/", ""); set_prnt(&prnt, -2317712); crate::C!(swtch_fn(0, "".to_string()));from_ls_2_front(ls_mode); 
    "".to_string()};
    let mut path = format!("{}/", match Path::new(&orig_path).parent(){
        Some(path) => path,
        _ => return ret_2_Front()
    }.to_str().unwrap());
    path = path.replace("//", "/");
    prnt = prnt.replace(&orig_path, &path);
    set_prnt(&prnt, -1405);
    /*let user_wrote_path = user_wrote_path();
    rm_file(&user_wrote_path);*/
    set_user_written_path_from_strn(path.to_string());
    prnt
}
pub(crate) fn Ins_key() -> String{
    let mut prnt: String = read_prnt();
    let path = get_path_from_strn(crate::cpy_str(&prnt));
    let mut file_indx = String::new();
    let spaces = repeat_char(63, " ");
    println!(" \rPlease, enter indx of dir/file name to autocomplete: {}", spaces);
    io::stdin().read_line(&mut file_indx).expect("Ins_key failed to read console");
    let file_indx = file_indx.as_str().substring(0, file_indx.len() -1);
    let mut err_msg = "".to_string();
    let mut handle_err =|e: std::num::ParseIntError| -> i64 {err_msg = format!("{:?}", e); -1i64};
    let file_indx = match i64::from_str_radix(&file_indx, 10){
        Ok(int) => int,
        Err(e) => handle_err(e)
    };
    if file_indx == -1i64{set_ask_user(&err_msg, -1); return "none done".to_string();}
    let mut file = get_item_from_front_list(file_indx, true);
    let is_dir = crate::Path::new(&file).is_dir();
    if is_dir {file.push('/');}
    prnt = prnt.replace(&path, &file);
    crate::set_prnt(&prnt, -1);
    prnt
}
pub(crate) fn Enter(){
    let mut prnt = get_prnt(-881454);
    let (term, _) = split_once(&prnt, " ");
    if term == "term"{
        prnt = format!("{prnt}:>:no_upd_scrn");
        //set_prnt(&prnt, -881454);
    }
    let mut mode = 0i64;
    crate::C!(check_mode(&mut mode));
    if mode == SWTCH_USER_WRITING_PATH{mode = SWTCH_RUN_VIEWER}
    crate::C!(crate::swtch::swtch_fn(mode, "".to_string()));
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
     if len > 0 {len -= 1;}
    let mut indx = unsafe {shift_cursor_of_prnt(2, -2).shift};
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
     len -= 1; 
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
pub fn ins_last_char_to_string1_from_string1_ptr(indx: usize, origString: &mut String) {
    let mut len = origString.chars().count(); if len == 0 || len == 1 || indx == len -1 {return}
     let mut ret = String::new();
     len -= 1; 
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
    for i in 0..1000{
        list_id = unsafe {front_list_indx(i64::MAX)};
        if list_id.1{break;}
    }
    if !list_id.1{set_ask_user("Can't access to Front list", -1); return "!!no¡".to_string()}
    let mut front_list = read_front_list();
    front_list.push_str(".len");
    //if front_list != "main0.len"{return len_of_list_wc(&front_list);}
    let num = read_file(&front_list);
    if num == ""{return "0".to_string()}
    return num;
}
pub fn len_of_list_wc(name: &str) -> String{
    let mut list_adr = take_list_adr(&name);
    let cmd = format!("wc -l {list_adr}");
    let len_list = crate::run_cmd_out_sync(cmd);
    list_adr.push_str(".len");
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
    let front_list_adr = take_list_adr(&front_list);
    let cmd = format!("wc -l {front_list_adr}");
    let len_front_list = crate::run_cmd_out_sync(cmd);
    front_list.push_str(".len");
    let (len_front_list, _) = split_once(&len_front_list, " ");
    crate::save_file(cpy_str(&len_front_list), front_list);
    return len_front_list;
}
pub(crate) fn get_proper_indx(indx: i64, fixed_indx: bool) -> (usize, i64){
    let last_pg = where_is_last_pg();
    let mut indx = indx;
    if indx < 0{
        let mut indx = indx * -1;
        if last_pg < indx{
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
        let ret = crate::cpy_str(&MAIN0[indx]);
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
        if FRONT.len() > indx && list == MAIN0_{ret = cpy_str(&FRONT[indx])}
        else{ret = ln_of_found_files(indx).0;}
        return ret.to_string()}//return FRONT.get().unwrap()[indx].to_string()}
    if op_code == LEN{return ln_of_found_files(usize::MAX).1.to_string()}//return FRONT.get().unwrap().len().to_string()}
}
"wrong".to_string()
}
pub(crate) fn take_list_adr(name: &str) -> String{
    return format!("{}/{name}", get_tmp_dir(6774154783));
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
            //println!("{}", maybe);
        } else {
            if found {ret.1.push(i); continue;}
            if maybe == *delim {ret.1.push(i); found = true; continue;}
            count_delim_chars = 0;
            ret.0.push_str(maybe.as_str());
            ret.0.push(i);
            maybe = String::new();
        }
    }
    ret
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
//fn