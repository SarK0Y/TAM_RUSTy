use crate::{add_cmd_in_history, count_ln, custom_traits::{find_substrn, helpful_math_ops, turn_2_i64, turn_2_usize, vec_tools, STRN_usize, STRN}, 
drop_ls_mode, errMsg0, get_cur_cur_pos, getkey, globs18::{enum_not_escaped_spaces_in_strn, enum_not_escaped_spaces_in_strn_up_to, get_item_from_front_list, id_suffix, len_of_front_list, 
    take_list_adr}, history_buffer, history_buffer_size, ln_of_found_files, popup_msg, read_file, read_file_abs_adr, read_front_list, read_prnt, run_cmd0, save_file0, save_file_abs_adr, set_ask_user, set_cur_cur_pos, set_front_list, set_prnt, shift_cursor_of_prnt, stop_term_msg, COUNT_PAGES_};
use crossterm::event::PopKeyboardEnhancementFlags;
use num_traits::ops::overflowing::OverflowingSub;
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
        let alt_prev_list = format!("{}/prev_list", crate::bkp_tmp_dir(None, false)).replace("//", "/");
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
    history_buffer(Some(prnt), 0, false);
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
    if file_indx.as_str().substring(0, 6) == "no esc"{crate::swtch_esc(true, false); return empty;}
    if file_indx.as_str().substring(0, 6) == "en esc"{crate::swtch_esc(true, true); return empty;}
    if file_indx.as_str().substring(0, 5) == "en ls"{crate::swtch_ls(true, true); return empty;}
    if file_indx.as_str().substring(0, 5) == "no ls"{crate::swtch_ls(true, false); return empty;}
    if file_indx.as_str().substring(0, 7) == "delim::"{crate::delim(Some(file_indx) ); return empty;}
    if file_indx.trim_end() == "scroll ln in pg"{crate::scroll_ln_in_pg( true ); return empty;}
    #[cfg(feature="in_dbg")]
    if file_indx.as_str().substring(0, 8) == "::report"{crate::report(&"".to_string(), ""); return empty;}
    #[cfg(feature="in_dbg")]
    if file_indx.as_str().substring(0, 3) == "br:"{crate::manage_breaks(&file_indx); return empty;}
    #[cfg(feature="in_dbg")]
    if file_indx.as_str().substring(0, 4) == "::br"{crate::just_break(); return empty;}
    #[cfg(feature="in_dbg")]
    if file_indx.as_str().substring(0, 7) == "::front"
    {let fp = crate::core18::name_of_front_list(&"".strn(), false); println!("{} {}", fp, crate::globs18::check_substrn(&fp, "history") ); getkey(); 
                                                                                                                                          return empty;}
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
pub(crate) fn F1_key() -> String{
    let mut prnt: String = read_prnt();
   crate::globs18::set_main0_as_front();
   crate::ps18::fix_num_files(-13971);
   crate::clean_cache("main0");
   crate::core18::rm_file(&take_list_adr("msgs/term/state"));
format!("go2 {}", read_file("main0.pg"))
}
pub(crate) fn key_f12(func_id: i64){
    unsafe {crate::shift_cursor_of_prnt(0, None, func_id)};
    crate::set_prnt("", func_id);
    crate::core18::rm_file(&take_list_adr("msgs/term/state"));
    crate::rm_user_written_path(func_id);
    count_ln(false, false, false);
}
pub(crate) fn PgDown(){
    let mut cur_cur_pos = crate::i64_2_usize(get_cur_cur_pos(74444418691));
    let len = read_prnt().chars().count();
     let mut som = Some( delta(len,cur_cur_pos) );
    if som == Some(0){som = Some(len)}
    unsafe {shift_cursor_of_prnt(0, som, 74444418691)};
    if cur_cur_pos == 0{ return;}
    let delim0 = delim(None);
    let enum_spaces = if delim0 == ""{enum_not_escaped_spaces_in_strn_up_to(&read_prnt(), cur_cur_pos) }
                             else {read_prnt().enum_entry_points_of_substrn( &delim0 ).up2(cur_cur_pos)};
    if enum_spaces.len() == 0{
        //popup_msg(&cur_cur_pos.strn());
        let cur_cur_pos0 = delta(cur_cur_pos, 10);
        cur_cur_pos = delta(cur_cur_pos, len);
        som = Some(cur_cur_pos );
        set_cur_cur_pos(crate::usize_2_i64(cur_cur_pos0 ),74444418691); 
        unsafe {shift_cursor_of_prnt(0, som, 74444418691).shift }; return;}
    let mut dt = usize::MAX;
    //popup_msg(&cur_cur_pos.to_string());
    let mut i = enum_spaces.len() - 1;
    let mut pass = false;
    //loop {
        let cur_cur_pos = unsafe {shift_cursor_of_prnt(0, None, 74444418691).shift};
        if cur_cur_pos == enum_spaces[0]{ return;}
        pass = false;
        set_cur_cur_pos(crate::usize_2_i64(enum_spaces[i]),74444418691);
        {unsafe {shift_cursor_of_prnt(0, Some(len - enum_spaces[i]), 74444418691).shift}; return
        } 
    //    if i == 0{break;}
        i -= 1;
  //  }
}
pub(crate) fn PgUp(){
    let mut cur_cur_pos = crate::i64_2_usize(get_cur_cur_pos(74444418691));
    let len = read_prnt().chars().count();
     let mut som = Some( delta(cur_cur_pos, len) );
    if som == Some(0){som = Some(len)}
    unsafe {shift_cursor_of_prnt(0, som, 74444418691)};
    //if cur_cur_pos == 0{ return;}
    let delim0 = delim(None);
    let enum_spaces = if delim0 == ""{crate::globs18::enum_not_escaped_spaces_in_strn_down_to(&read_prnt(), cur_cur_pos) }
                             else {read_prnt().enum_entry_points_of_substrn( &delim0 ).down2(cur_cur_pos)};
    if enum_spaces.len() == 0{
        popup_msg(&cur_cur_pos.strn());
        let cur_cur_pos0 =cur_cur_pos + 10;
        if cur_cur_pos > len {return}
        cur_cur_pos = delta(cur_cur_pos, len);
        som = Some(cur_cur_pos );
        set_cur_cur_pos(crate::usize_2_i64(cur_cur_pos0 ),74444418691);
        unsafe {shift_cursor_of_prnt(0, som, 74444418691).shift }; return;}
    let mut dt = usize::MAX;
    //popup_msg(&cur_cur_pos.to_string());
    let loops = enum_spaces.len();
    let mut i = 0;
    let mut pass = false;
    loop {
        let cur_cur_pos = unsafe {shift_cursor_of_prnt(0, None, 74444418691).shift};
        if cur_cur_pos == enum_spaces[0]{ return; }
        pass = false;
        set_cur_cur_pos(crate::usize_2_i64(enum_spaces[i]),74444418691);
        {unsafe {shift_cursor_of_prnt(0, Some(len - enum_spaces[i]), 74444418691).shift}; return
        } 
        if i == loops{break;}
        i += 1;
    }
}
pub(crate) fn F9_key() {
    let mut block_ring_buffer = false;
    let ringbuf_size = if !crate::scroll_ln_in_pg(false) {history_buffer_size(None)} else {0};
    let mut ln_indx0 = count_ln(true, true, false);
    let mut ln_indx = if !crate::scroll_ln_in_pg(false){len_of_front_list().usize0().dec().dec()  }
    else {block_ring_buffer = true; crate::calc_num_files_up2_cur_pg01().usize0() };
   // popup_msg(&ln_indx0.strn() );
    let mut indx: usize = delta(ln_indx, ln_indx0);
    //if ln_indx.1{count_ln(false, false, false); indx = 0}
    let mut ln = "".strn();
    if let Some(ln0) = history_buffer(None, ln_indx0, block_ring_buffer){
        ln = ln0;
    } else {
        let mut count_out = 93usize;
        while  count_out > 0 {
            ln = crate::ln_of_found_files01(indx + ringbuf_size).0;
            if (ln == "" || ln == "no str gotten" || ln == crate::getStop_code__!() )  {indx.dec(); }
            else { break;}
            count_out.dec();
        }
        let mut ln0 = crate::ln_of_found_files01(indx + ringbuf_size );
        let mut count_out = 93usize;
        while ln0.1 < indx + ringbuf_size && count_out > 0 {
            indx = indx.dec();
            ln0 = crate::ln_of_found_files01(indx + ringbuf_size );
            count_out.dec();
        } 
        set_ask_user("Now, Dear User, You're scrolling list.", 999714);
        ln = crate::ln_of_found_files01(indx + ringbuf_size ).0;
        if ln == "no str gotten"{ln = crate::ln_of_found_files01(count_ln(true, true, true) ).0;}
        if ln == "no str gotten"{count_ln(false, false, false); ln = crate::ln_of_found_files01(0 ).0;}
        //popup_msg("ring"); popup_msg(&ringbuf_size.strn() ); popup_msg(&indx.strn() );
        }
    set_prnt(&ln, 999714);

}
pub(crate) fn F8_key() {
    let mut block_ring_buffer = false;
    let ringbuf_size = if !crate::scroll_ln_in_pg(false) {history_buffer_size(None)} else {0};
    let mut ln_indx0 = count_ln(true, false, false);
    let lst_size = if !crate::scroll_ln_in_pg(false){len_of_front_list().usize0() }
    else {block_ring_buffer = true; crate::calc_num_files_up2_cur_pg01().usize0() };// + ln_indx0 };
    let ln_indx = lst_size.overflowing_sub( ln_indx0 ); //if !crate::scroll_ln_in_pg(false) {lst_size.overflowing_sub( ln_indx0 )} else{lst_size.overflowing_add( 0 )};
    let mut in_history = false;
    let mut prev_indx = usize::MAX;
    let mut indx: usize = ln_indx.0;
    let mut count_out = 15usize;
    if ln_indx.1{count_ln(false, false, false); indx = 0}
     let mut ln = "".strn();
     let nl = char::from_u32(0x0a);
    if  let Some(ln0) = history_buffer(None, ln_indx0, block_ring_buffer) {
        ln = ln0; 
    } else {
        while  true {
            let ln1 = crate::ln_of_found_files01(indx + ringbuf_size);
            if ln1.0 == "" || ln1.0 == crate::getStop_code__!() || nl == ln1.0.chars().nth(0) 
            {indx.inc(); count_out.dec(); if count_out == 0 {break;} continue;}
            if prev_indx == ln1.1 || ln1.0 == "no str gotten" {
                let indxx = count_ln(true, false, false);
                if let Some(ln0) = history_buffer(None, indxx, block_ring_buffer){ 
                    ln = ln0; in_history = true; count_ln(false, false, false);
                }
                break;
            }
            else {prev_indx = ln1.1}
            break;
        }
        if !in_history { ln = crate::ln_of_found_files01(indx + ringbuf_size ).0; if ln == "no str gotten" {ln = "You got in ring-buffer.".strn()} }
    }
    set_prnt(&ln, 999714);

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
pub(crate) fn delim(cmd: Option<String> ) -> String{
        let delim = take_list_adr("delim");
    if cmd == None{
       let delim = read_file_abs_adr(&delim);
        return delim
    }
    let cmd = cmd.unwrap().replace("delim::", "").trim_end().strn();
    save_file_abs_adr(cmd.clone(), delim);
    cmd
}
//fn