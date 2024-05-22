use crate::{read_front_list, save_file0, read_file, set_front_list, globs18::{take_list_adr, id_suffix, enum_not_escaped_spaces_in_strn, enum_not_escaped_spaces_in_strn_up_to}, read_file_abs_adr, errMsg0, stop_term_msg, run_cmd0, read_prnt, shift_cursor_of_prnt, set_cur_cur_pos, get_cur_cur_pos, popup_msg, drop_ls_mode, add_cmd_in_history};
use substring::Substring; use std::io;
pub(crate) fn key_slash(){
    let front_list = read_front_list();
    if front_list == "ls"{return;}
    save_file0(front_list, "prev_list".to_string());
}
pub(crate) fn pre_Enter(){
    let front_list = read_front_list();
    let ls = std::path::PathBuf::from("ls0");
    let found_files = take_list_adr("found_files");
    let behind_found_files = crate::read_tail(&match std::fs::read_link(&front_list){
        Ok(link) => link,
        _ => ls
    }.into_os_string().into_string().unwrap(), "/");
    if front_list != "ls" && behind_found_files != "ls" {return}
    let mut prev_list = read_file("prev_list");
    if prev_list == ""{
        let alt_prev_list = format!("{}/prev_list", crate::bkp_tmp_dir()).replace("//", "/");
        prev_list = read_file_abs_adr(&alt_prev_list);
        if prev_list == ""{
            errMsg0("Operation failed.. Sorry :((");
            save_file0("stop".to_string(), "msgs/term/state".to_string());
            return;
        }
    }
    set_front_list(&prev_list);
    crate::free_term_msg();
}
pub(crate) fn Enter(){
    let mut prnt = crate::get_prnt(-881454);
    let (term, _) = crate::split_once(&prnt, " ");
    if term == "term"{
        prnt = format!("{prnt}:>:no_upd_scrn");
        //set_prnt(&prnt, -881454);
    }
    drop_ls_mode();
    let mut mode = 0i64;
    crate::C!(crate::swtch::check_mode(&mut mode));
    if mode == crate::swtch::SWTCH_USER_WRITING_PATH{mode = crate::swtch::SWTCH_RUN_VIEWER}
    crate::C!(crate::swtch::swtch_fn(mode, "".to_string()));
    add_cmd_in_history();
}
pub(crate) fn Ins_key() -> String{
    stop_term_msg();
    let mut prnt: String = crate::read_prnt();
    let path = crate::get_path_from_strn(crate::cpy_str(&prnt));
    let mut file_indx = String::new(); let empty = String::new();
    let spaces = crate::repeat_char(63, " ");
    println!(" \rPlease, enter indx of dir/file name to autocomplete: {}", spaces);
    io::stdin().read_line(&mut file_indx).expect("Ins_key failed to read console");
    if file_indx.as_str().substring(0, 5) == "key::"{crate::switch_cmd_keys(&file_indx); return empty;}
    if file_indx.as_str().substring(0, 12) == "::prnt patch"{crate::prnt_patch(); return empty;}
    if file_indx.as_str().substring(0, 9) == "::drop ls"{crate::drop_ls_mode(); return empty;}
    #[cfg(feature="in_dbg")]
    if file_indx.as_str().substring(0, 8) == "::report"{crate::report(&"".to_string(), ""); return empty;}
    #[cfg(feature="in_dbg")]
    if file_indx.as_str().substring(0, 3) == "br:"{crate::manage_breaks(&file_indx); return empty;}
    #[cfg(feature="in_dbg")]
    if file_indx.as_str().substring(0, 4) == "::br"{crate::just_break(); return empty;}
    let file_indx = file_indx.as_str().substring(0, file_indx.len() -1);
    let mut err_msg = "".to_string();
    let mut handle_err =|e: std::num::ParseIntError| -> i64 {err_msg = format!("{:?}", e); -1i64};
    let file_indx = match i64::from_str_radix(&file_indx, 10){
        Ok(int) => int,
        Err(e) => handle_err(e)
    };
    if file_indx == -1i64{crate::set_ask_user(&err_msg, -1); return "none done".to_string();}
    let mut file = crate::get_item_from_front_list(file_indx, true);
    let is_dir = crate::Path::new(&file).is_dir();
    if is_dir {file.push('/');}
    prnt = prnt.replace(&path, &file);
    crate::set_prnt(&prnt, -1);
    prnt
}
pub(crate) fn swtch_tam_konsole(){
    let id_suffix = id_suffix(); let cmd = format!("krunner '{id_suffix}'");
    run_cmd0(cmd);
}
pub(crate) fn PgDown(){
    let cur_cur_pos = crate::i64_2_usize(get_cur_cur_pos(74444418691));
    let len = read_prnt().chars().count();
     let mut som = Some(len - cur_cur_pos);
    if som == Some(0){som = Some(len)}
    unsafe {shift_cursor_of_prnt(0, som, 74444418691)};
    if cur_cur_pos == 0{ return;}
    let enum_spaces = enum_not_escaped_spaces_in_strn_up_to(&read_prnt(), cur_cur_pos);
    if enum_spaces.len() == 0{return;}
    let mut dt = usize::MAX;
    //popup_msg(&cur_cur_pos.to_string());
    let mut i = enum_spaces.len() - 1;
    let mut pass = false;
    loop {
        let cur_cur_pos = unsafe {shift_cursor_of_prnt(0, None, 74444418691).shift};
        if cur_cur_pos == enum_spaces[0]{ return;}
        pass = false;
        set_cur_cur_pos(crate::usize_2_i64(enum_spaces[i]),74444418691);
        {unsafe {shift_cursor_of_prnt(0, Some(len - enum_spaces[i]), 74444418691).shift}; return
        } 
        if i == 0{break;}
        i -= 1;
    }
}
pub(crate) fn PgUp(){
    let cur_cur_pos = crate::i64_2_usize(get_cur_cur_pos(74444418691));
    let len = read_prnt().chars().count();
     let mut som = Some(len - cur_cur_pos);
    if som == Some(0){som = Some(len)}
    unsafe {shift_cursor_of_prnt(0, som, 74444418691)};
    if cur_cur_pos == 0{ return;}
    let enum_spaces = crate::globs18::enum_not_escaped_spaces_in_strn_down_to(&read_prnt(), cur_cur_pos);
    if enum_spaces.len() == 0{return;}
    let mut dt = usize::MAX;
    //popup_msg(&cur_cur_pos.to_string());
    let loops = enum_spaces.len();
    let mut i = 0;
    let mut pass = false;
    loop {
        let cur_cur_pos = unsafe {shift_cursor_of_prnt(0, None, 74444418691).shift};
        if cur_cur_pos == enum_spaces[0]{ return;}
        pass = false;
        set_cur_cur_pos(crate::usize_2_i64(enum_spaces[i]),74444418691);
        {unsafe {shift_cursor_of_prnt(0, Some(len - enum_spaces[i]), 74444418691).shift}; return
        } 
        if i == loops{break;}
        i += 1;
    }
}
pub(crate) fn delta <T>(fst: T, nd: T) -> T where T: PartialEq + Eq + std::ops::Sub<Output=T> + std::cmp::PartialOrd{
    if fst > nd{return fst - nd;}
    return nd - fst
}
pub(crate) fn mm <T>(val: T, m: T) -> T where T: std::ops::Sub<Output=T> + std::cmp::PartialOrd{
    if val >= m {return val - m;}
    val
}
fn ret_type_of<T>(_: &T) -> &str {
   std::any::type_name::<T>()
}
//fn