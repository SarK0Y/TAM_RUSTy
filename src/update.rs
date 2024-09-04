use once_cell::sync::Lazy;
use ps0::{fix_num_files, get_mainpath};

use crate::{basic, bkp_main_path, checkArg, clean_cache, clear_patch, clear_screen, complete_path, custom_traits::{fs_tools, STRN}, dont_scrn_fix, drop_ls_mode, errMsg0, exts::update_uses, from_ls_2_front, get_path_from_prnt, globs18::{check_substrn, path_to_shm, set_main0_as_front, strn_2_u64, MAIN0_}, init::user_home_dir, mk_dummy_file, mk_empty_file, name_of_front_list, popup_msg, read_file, read_file_abs_adr, read_front_list, read_midway_data, read_prnt, rm_file, save_file, set_front_list, set_prnt, split_once, swtch::{front_list_indx, swtch_fn, SWTCH_USER_WRITING_PATH}, swtch_ls, tailOFF, KonsoleTitle, ManageLists}; 
use self::{func_id17::{find_files, read_midway_data_}, globs17::{set_ls_as_front, take_list_adr, len_of_front_list_wc, len_of_main0_list, gen_win_title}, ps0::set_num_files};
update_uses!();
use std::{borrow::Borrow, time::Instant};
pub(crate) fn main_update(){
    let func_id = crate::func_id18::main_update;
    let mut no_path =true;
    let mut path: String ="".to_string();
    let mut in_name: String = "".to_string();
    if  crate::checkArg("-find_files") ||  crate::checkArg("-find-files"){
        if  crate::checkArg("-path"){path = get_arg_in_cmd("-path").s.iter().collect(); no_path = false;}
        if  crate::checkArg("-path0"){path = get_arg_in_cmd("-path0").s.iter().collect(); no_path = false;}
        if no_path {panic!("No path was provided: set flag '-path' or '-path0");}
        KonsoleTitle(&gen_win_title());
        if  crate::checkArg("-rows"){let val: i64 = i64::from_str_radix(&crate::__get_arg_in_cmd("-rows"), 10).expect(
            "set number of rows as an integer: '-rows 9'"
        ); crate::set_num_rows(val, func_id);}
        if  crate::checkArg("-cols"){let val: i64 = i64::from_str_radix(&crate::__get_arg_in_cmd("-cols"), 10).expect(
            "set number of columns as an integer: '-cols 3'"
        ); ps0::set_num_cols(val, func_id);}
        let thr_midway = thread::Builder::new().stack_size(2 * 1024 * 1024).name("read_midway".to_string());
        let thr_find_files = thread::Builder::new().stack_size(2 * 1024 * 1024).name("find_files".to_string());
        
        let orig_lst = take_list_adr("found_files").unreel_link_to_depth(1);
        //logs(&orig_lst, "see");
        std::fs::remove_file(&orig_lst);
        upd_screen_or_not((-1, "".strn() ) );
        if !checkArg("-slow-load"){
            thr_find_files.spawn(move||{
                println!("spawn find files");
                crate::find_files(path.as_str(), "");
                if crate::dirty!(){println!("exit find files")};
            });
            thr_midway.spawn(||{
                println!("spawn midway data");
                crate::read_midway_data();
                if crate::dirty!(){println!("exit midway data");}
            });
    }else{
            thr_find_files.spawn(move||{
                println!("spawn find files");
                crate::find_files(path.as_str(), "");
                if crate::dirty!(){println!("exit find files")};
            }).unwrap().join();
            thr_midway.spawn(||{
                println!("spawn midway data");
                crate::read_midway_data();
                if crate::dirty!(){println!("exit midway data");}
            });
    }
    }
clear_patch();
}
pub(crate) fn delay_ms(sleep: u64){
    std::thread::sleep(std::time::Duration::from_millis(sleep));
}
pub(crate) fn delay_mcs(sleep: u64){
    std::thread::sleep(std::time::Duration::from_micros(sleep));
}
pub(crate) fn delay_ns(sleep: u64){
    std::thread::sleep(std::time::Duration::from_nanos(sleep));
}
pub(crate) fn delay_secs(sleep: u64){
    std::thread::sleep(std::time::Duration::from_secs(sleep));
}
pub(crate) fn prime(){
    crate::initSession();
    let key = "-front-lst";
    main_update();
    if checkArg(key){
        let cmd = crate::__get_arg_in_cmd(key);
        crate::front_lst(&cmd);
    }else{
        C!(front_list_indx(MAIN0_));
        C!(set_main0_as_front());
        set_front_list("main0");
    }
    let mut base = crate::basic::new();
println!("len of main0 list {}", globs17::len_of_main0_list());
    let builder = thread::Builder::new().stack_size(8 * 1024 * 1024).name("manage_page".to_string());
let handler = builder.spawn(move || {
let mut ps__: crate::_page_struct = crate::init_page_struct();
ps__.num_cols = i64::MAX; ps__.num_page = i64::MAX; ps__.num_rows = i64::MAX;
C_!(crate::swtch::swtch_ps(0, Some(ps__)););
if checkArg("-no-ext"){crate::manage_pages(&mut None);}
else{base.manage_pages()}
println!("stop manage_page");
}).unwrap();
//background_fixing_count(2);
delay_ms(37);
    handler.join().unwrap();
    println!("len of main0 list {}", globs17::len_of_main0_list());
}
pub(crate) fn update_dir_list(dir: &str, opts: &str, no_grep: bool){
    if !swtch_ls(false, false){drop_ls_mode(); return}
    let mut head = String::new();
    let mut tail = String::new();
    let os_str = OsStr::new("");
    let mut dir_len = dir.chars().count();
    if dir_len ==0 {return;}
    if dir.chars().nth(dir_len -1).expect("update_dir_list failed to get file name") != '/'{
    head = match Path::new(dir).file_name(){
        Some(p) => p,
        _ => os_str
    }.to_str().unwrap().to_string();
    tail = match Path::new(dir).parent(){
        Some(p) => p,
        _ => Path::new("/")
    }.to_str().unwrap().to_string();
    
} else {
    head = "".to_string();
    tail = dir.substring(0, dir_len).to_string();
}
    let mut cmd = format!("find -L {} {}|grep -Ei '{}'", tail, opts, head);
    if no_grep{cmd = format!("find -L {}/{}", tail, head);}
    crate::find_files_ls_no_stop_code(cmd);
    fix_num_files(5197521);
    clear_screen();
    name_of_front_list("ls", true);
    delay_ms(12);
    crate::mk_cnt();
    //background_fixing_count_n_delay(2, 40);
}
pub fn prev_key <T: STRN + ToString  + Borrow <str> + std::cmp::PartialEq<T> > (prev: T) -> String {
    static mut key: Lazy <String> = Lazy::new(|| {String::new()});
    unsafe {
        if prev.strn() != "" {*key = prev.strn() } key.strn()
    }
}
pub(crate) fn lets_write_path(key: String){
    if !swtch_ls(false, false) {return;}
    //C_!(set_ls_as_front(); front_list_indx(crate::globs18::LS_););
    let anchor = (-1i64, "ls".strn());
    upd_screen_or_not(anchor);
    set_front_list("ls");
    let mode: i64 = crate::swtch::SWTCH_USER_WRITING_PATH;
    if mode < 0{return;}
    name_of_front_list("ls", true);
    let mut key0: String;
    if prev_key("") == "~" {
     let mut prnt = read_prnt();
     key0 = crate::init::user_home_dir(); prnt = prnt.replace("~", &key0);
     set_prnt(&prnt, 1198001452 );
} else {key0 = key}
     C!(swtch_fn(mode, "".strn() )); C!(swtch_fn( -1, key0)); 

}
pub(crate) fn background_fixing(){ 
    if checkArg("-no-shadow-fix"){return;}       
    let (prnt, subcmd) = split_once(&read_prnt(), ":>:");
    if subcmd == "no_upd_scrn"{
        set_prnt(&prnt, -164547841);
        return;
    }
 let builder = thread::Builder::new().stack_size(2 * 1024 * 1024).name("background_fixing".to_string());
 builder.spawn(|| {
    let mut bkgrnd: fn() = _background_fixing;
     if checkArg("-dbg"){bkgrnd = dbg_background_fixing;}
    bkgrnd();
    fix_screen();
});
}
pub(crate) fn background_fixing_count(num: usize){        
    if checkArg("-no-shadow-fix"){return;}
    let (prnt, subcmd) = split_once(&read_prnt(), ":>:");
    if subcmd == "no_upd_scrn"{
        set_prnt(&prnt, -164547841);
        return;
    }
 let builder = thread::Builder::new().stack_size(2 * 1024 * 1024).name("background_fixing".to_string());
 builder.spawn(move|| {
    let mut bkgrnd: fn() = _background_fixing;
     if checkArg("-dbg"){bkgrnd = dbg_background_fixing;}
     bkgrnd();
    fix_screen_count(num);
});
}
pub(crate) fn background_fixing_count_n_delay(num: usize, delay: u64){        
    if checkArg("-no-shadow-fix"){return;}
    let (prnt, subcmd) = split_once(&read_prnt(), ":>:");
    if subcmd == "no_upd_scrn"{
        set_prnt(&prnt, -164547841);
        return;
    }
 let builder = thread::Builder::new().stack_size(2 * 1024 * 1024).name("background_fixing".to_string());
 builder.spawn(move|| {
    let mut bkgrnd: fn() = _background_fixing;
     if checkArg("-dbg"){bkgrnd = dbg_background_fixing;}
     bkgrnd();
    fix_screen_count_n_delay(num, delay);
});
}

fn _background_fixing(){
    let func_id = crate::func_id18::background_fixing_;
    //return;
    let mut check_main0_len = len_of_main0_list();
let mut drop_ls = true;
let ls_mode = take_list_adr("ls.mode");
     let main0_len = len_of_main0_list();
     if check_main0_len != main0_len && check_main0_len != "0"{
        check_main0_len = main0_len;
        save_file(crate::cpy_str(&check_main0_len), "main0.len".to_string());
        //else{drop_ls = !drop_ls}
     }
     let front_list_len = crate::read_front_list();
     let front_list_len = match i64::from_str_radix(&crate::globs18::len_of_list_wc(&front_list_len), 10){
        Ok(i) => i,
        _ => 0
  };
     crate::C!(crate::page_struct_int(front_list_len, crate::set(crate::NUM_FILES_), func_id));//}
}
fn dbg_background_fixing(){
    let func_id = crate::func_id18::background_fixing_;
    //return;
    let mut check_main0_len = len_of_main0_list();
let mut drop_ls = true;
let ls_mode = take_list_adr("ls.mode");
//loop {
  //   thread::sleep(Duration::from_millis(5000));
     let main0_len = len_of_main0_list();
     if check_main0_len != main0_len && check_main0_len != "0"{
        check_main0_len = main0_len;
        save_file(crate::cpy_str(&check_main0_len), "main0.len".to_string());
        //else{drop_ls = !drop_ls}
     }
     let front_list_len = crate::read_front_list();
     let front_list_len = match i64::from_str_radix(&crate::globs18::len_of_list_wc(&front_list_len), 10){
        Ok(i) => i,
        _ => 0
  };
     crate::C!(crate::page_struct_int(front_list_len, crate::set(crate::NUM_FILES_), func_id));//}
     let check_ls_mode = read_front_list();
     if check_ls_mode != "ls"{from_ls_2_front(ls_mode.clone());}
     save_file(check_ls_mode, "dbg_ls.mode".to_string());
}

pub(crate) fn fix_screen(){
    return;
    if checkArg("-no-shadow-fix"){return;}
    if dont_scrn_fix(false).0{dont_scrn_fix(dont_scrn_fix(false).1);return;} // if user did set flag - drop it after use
    std::thread::spawn(||{
        for i in 0..1{
            clear_screen();
            let mut ps: crate::_page_struct = unsafe {crate::swtch::swtch_ps(-1, None)};
            let mut data = "".to_string();
            let num_pg = crate::get_num_page(-5555555121);
            let num_pgs = crate::where_is_last_pg();
            crate::swtch::print_viewers();
            crate::swtch::print_pg_info();
            let mut base = basic::new();
            if num_pg < num_pgs || num_pgs ==0 {base.cache_active = false; base.build_page_(&mut ps);}
            println!("{}", crate::get_prnt(-1));
            crate::pg18::form_cmd_newline_default();
           std::thread::sleep(std::time::Duration::from_millis(1115));        
        }
    });
}
pub(crate) fn fix_screen_count(num: usize){
    return;
    if checkArg("-no-shadow-fix"){return;}
    if dont_scrn_fix(false).0{dont_scrn_fix(dont_scrn_fix(false).1);return;}
    std::thread::spawn(move||{
        for i in 0..num{
            clear_screen();
            let mut ps: crate::_page_struct = unsafe {crate::swtch::swtch_ps(-1, None)};
            let mut data = "".to_string();
            let num_pg = crate::get_num_page(-5555555121);
            let num_pgs = crate::where_is_last_pg();
            crate::swtch::print_viewers();
            crate::swtch::print_pg_info();
            let mut base = basic::new();
            if num_pg < num_pgs || num_pgs ==0 {base.cache_active = false; base.build_page_(&mut ps);}
            println!("{}", crate::get_prnt(-1));
            crate::pg18::form_cmd_newline_default();
           std::thread::sleep(std::time::Duration::from_millis(615));        
        }
    });
}
pub(crate) fn fix_screen_count_n_delay(num: usize, delay: u64){
    return;
    if checkArg("-no-shadow-fix"){return;}
    if dont_scrn_fix(false).0{dont_scrn_fix(dont_scrn_fix(false).1);return;}
    std::thread::spawn(move||{
        for i in 0..num{
            clear_screen();
            let mut ps: crate::_page_struct = unsafe {crate::swtch::swtch_ps(-1, None)};
            let mut data = "".to_string();
            let num_pg = crate::get_num_page(-5555555121);
            let num_pgs = crate::where_is_last_pg();
            crate::swtch::print_viewers();
            crate::swtch::print_pg_info();
            let mut base = basic::new();
            if num_pg < num_pgs || num_pgs ==0 {base.cache_active = false; base.build_page_(&mut ps);}
            println!("{}", crate::get_prnt(-1));
            crate::pg18::form_cmd_newline_default();
           delay_ms(delay);        
        }
    });
}
pub(crate) fn alive_session(){
    let main_path_alive = format!("{}/alive", crate::core18::bkp_main_path(None, false) );
    let shm_alive = take_list_adr("alive");
    mk_empty_file(&shm_alive);
    std::os::unix::fs::symlink(shm_alive, main_path_alive);
spawn(||{
    loop {
        let timestamp = std::time::SystemTime::now();
        let secs: u64 = match timestamp.duration_since(std::time::UNIX_EPOCH){Ok(dur) => dur, _ => return}.as_secs();
        save_file(secs.strn(), "alive".strn());
        std::thread::sleep(std::time::Duration::from_secs(15));        
    }
});
} 
pub(crate) fn clean_dead_tams(){
    let limit = 240u64;
    let nowtime = std::time::SystemTime::now();
    let nowtime: u64 = match nowtime.duration_since(std::time::UNIX_EPOCH){Ok(dur) => dur, _ => return}.as_secs();
    let shm_adr = path_to_shm(None);
    let find_tams = format!("du {shm_adr}|grep -Eo 'TAM_[0-9]{{4}}.*_[0-9]*/'|uniq");
    let run_cmd = match Command::new("bash").arg("-c").arg(&find_tams).output(){
        Ok(cmd) => cmd, _ => return errMsg0("Failed to get list of dead tams")
    };
    let build_stdout = String::from_utf8_lossy(&run_cmd.stdout);
    for ln in build_stdout.lines(){
        let ln0 = ln;
        let read_alive = format!("{shm_adr}/{ln0}alive" );
        if crate::Path::new(&read_alive).exists(){
            let read_alive = read_file_abs_adr(&read_alive);
            let alive_time = strn_2_u64(read_alive).unwrap_or(0);
            let ret = nowtime.overflowing_sub(alive_time);
            if  ret.0 > limit && !ret.1{
                let ln0 = ln;
                if ln0 == ""{continue;}
                let mut dead_tam = format!("{shm_adr}/{}", ln0);
                crate::forced_rm_dir(&mut dead_tam);
            } 
        }else {
                let mut dead_tam = format!("{shm_adr}/{}", ln);
                crate::forced_rm_dir(&mut dead_tam);
        }
    } //std::thread::spawn(||{clean_main_path()} );
   clean_main_path();
}
pub(crate) fn clean_main_path(){
    let mut main_path = bkp_main_path(None, false);
    tailOFF(&mut main_path, "_"); tailOFF(&mut main_path, "/");
    let find_tams = format!("du {main_path}");
    let run_cmd = match Command::new("bash").arg("-c").arg(&find_tams).output(){
        Ok(cmd) => cmd, _ => return errMsg0("Failed to get list of dead tams")
    };
    let mut read_alive = "".strn();
    let build_stdout = String::from_utf8_lossy(&run_cmd.stdout);
    for ln in build_stdout.lines(){
        let ln0 = ln.replace("4\t", "").replace("8\t", "");
        //for x in 0..10{
         read_alive = read_file_abs_adr(&format!("{ln0}/alive" ).__unreel_link_to_file() );
        /* if read_alive == ""{delay_ms(1); continue;}
         break;
        }*/
        if read_alive == "" {
            //errMsg0(&format!("{ln0}/alive" ).__unreel_link_to_file() );
            let mut dead_tam = crate::get_path_from_strn(format!("{}", ln0));
            if dead_tam == main_path {return;}
                crate::forced_rm_dir(&mut dead_tam);
            } 

    }
}
pub fn upd_screen_or_not(anchor: (i64, String) ) -> bool{
    static mut state: (i64, String) = (0, String::new());
    unsafe {
       // dbg!(&state);
        if state != anchor { state = anchor; return true; }}

    false
}
//fn
