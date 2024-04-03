use cli_table::TableStruct;

use crate::{exts::pg_uses, ps18::{set_prnt, get_cur_cur_pos, set_prompt, get_prnt, shift_cursor_of_prnt, set_full_path, set_ask_user, get_col_width, where_is_last_pg, get_num_files, child2run}, core18::{achtung, errMsg_dbg, ins_newlines, checkArg, popup_msg, calc_num_files_up2_cur_pg}, globs18::{ins_last_char_to_string1_from_string1, rm_char_from_string, ins_last_char_to_string1_from_string1_ptr, len_of_front_list, Ins_key, show_ls, sieve_list, get_proper_indx, merge, clear_merge}, split_once, swtch::{run_viewer, swtch_fn, local_indx, read_user_written_path, user_writing_path, renFile}, update18::lets_write_path, ln_of_found_files, size_of_found_files, key_f12, get_path_from_prnt, get_path_from_strn, read_prnt, read_file, get_num_page, run_term_app, set_front_list, clean_cache, wait_4_empty_cache, change_dir, shol_on, process_tag};
self::pg_uses!();

fn cpy_row(row: &mut Vec<String>) -> Vec<CellStruct>{
    let mut row_: Vec<CellStruct> = Vec::new(); 
    for i in 0..row.len(){
        row_.push(row[i].as_str().cell());
    }
    row.clear();
    row_
}

pub(crate) 
fn build_page(ps: &mut crate::_page_struct){
    let func_id = crate::func_id18::build_page_;
    let mut try_entry = 0usize;
    let mut num_files = get_num_files(func_id);
    while try_entry < 1_000 {
        if size_of_found_files() > 4u64 {break;}
        num_files = get_num_files(func_id);
        if num_files == 0i64{continue;}
        try_entry += 1; 
    }
    let mut count_down = num_files;
    if size_of_found_files() == 0u64 {println!("No files found"); if !checkArg("-dont-exit"){crate::C!(libc::exit(-1));}}
    let mut num_page; num_page = crate::get_num_page(func_id); // if ps.num_page != i64::MAX{num_page = ps.num_page;}else{num_page = crate::get_num_page(func_id);}
    let mut num_cols; if ps.num_cols != i64::MAX{num_cols = ps.num_cols;}else{num_cols = crate::get_num_cols(func_id);}
    let mut num_rows; if ps.num_rows != i64::MAX{num_rows = ps.num_rows;}else{num_rows = crate::get_num_rows(func_id);}
    if ps.col_width != i64::MAX{crate::set_col_width(ps.col_width, func_id);}
    let num_items_on_pages = num_cols * num_rows; let stopCode: String = crate::getStop_code__!();
    num_page = calc_num_files_up2_cur_pg(); let mut filename_str: String; let mut time_to_stop = false;
    let mut row: Vec<CellStruct> = Vec::new(); let mut row_cpy: Vec<String> = Vec::new();
    //let mut row: OnceCell<Vec<CellStruct>> = OnceCell::new(); row.set(row_nested);
   // pg.table().forecolor(Color::red());
    println!("{}", crate::get_full_path(func_id));
    for j in 0..num_rows{
        for i in 0..num_cols{
            let mut indx = i + num_cols * j + num_page;
            //indx = num_files - count_down_files;
            let mut res: String ="".to_string();
            let full_path_fn = move || -> String {//for i in 0..10_0 {
              res = crate::globs18::get_item_from_front_list(indx, false);
              num_files = get_num_files(func_id);
              if num_files == indx || "front list is empty" != res{time_to_stop = true; return res;}
            // println!("build_page - probe 0");
             return "".to_string()};
            let full_path = full_path_fn();
            //no_dup_indx = indx;
            if !crate::C!(local_indx(false)){indx -= num_page;}
            let err_ret = std::ffi::OsString::from("");
            let mut err_path = || -> &std::ffi::OsString{return &err_ret};
            //println!("build_page - probe 1");
            let filename = Path::new(&full_path);
            macro_rules! filename_str0{
                () => {String::from(match filename.file_name(){
                    Some(f) => f,
                    None => err_path(),
                }.to_str().unwrap()).as_str()};
            }
            if crate::globs18::eq_str(stopCode.as_str(), filename.as_os_str().to_str().unwrap()) == 0 && stopCode.len() == filename.as_os_str().to_str().unwrap().len() {println!("{}", "caught".bold().green()); 
             time_to_stop = true; break;}
            if crate::dirty!(){
               println!("cmp_str res {}", crate::globs18::eq_str(stopCode.as_str(), filename.as_os_str().to_str().unwrap()));
               println!("stop code {}, len {}; str {}, len {}", stopCode, stopCode.as_str().len(), filename.as_os_str().to_str().unwrap(), filename.as_os_str().to_str().unwrap().len());
               println!("{:?}", filename.file_name());
            }
            let mut fixed_filename: String = filename_str0!().to_string();
            ins_newlines(get_col_width(func_id).to_usize().unwrap(), &mut fixed_filename);
            if filename.is_dir(){filename_str =format!("{}: {}/", indx, fixed_filename);}
            else{filename_str = format!("{}: {}", indx, fixed_filename);}
            if filename_str == stopCode || filename_str == "no str gotten"{return;}
            row_cpy.push(filename_str);
            if count_down <= 0 {time_to_stop = true; break;}
            count_down -= 1;
        }
        let count_pages = crate::get_num_files(func_id) / num_items_on_pages;
        let mut new_row: Vec<Vec<CellStruct>> = Vec::new();
        new_row.push(cpy_row(&mut row_cpy));
        print_stdout(new_row.table().bold(true).foreground_color(Some(cli_table::Color::Blue)));
        if time_to_stop {break;}
    }
    //println!("{}", pg.table().display().unwrap());
    println!("{}", crate::get_ask_user(func_id));
}

pub(crate) fn clear_screen(){
    if checkArg("-dbg") || checkArg("-dirty"){return;}
    let run_command = Command::new("clear")
    .output()
    .expect("can't clear screen");
if run_command.status.success(){
    io::stdout().write_all(&run_command.stdout).unwrap();
    io::stderr().write_all(&run_command.stderr).unwrap();
}
}
pub(crate) 
fn hotKeys(Key: &mut String, ext: &Option<&mut crate::__ext_msgs::_ext_msgs>) -> String{
    let func_id = crate::func_id18::hotKeys_;
    //if unsafe {crate::swtch::path_completed(true, true)}{unsafe {crate::swtch::path_completed(false, false);}; return "dontPass".to_string();}
    //let mut Key =String::new();
    let mut cmd = String::new();
    let ext_is_alive = if Some(ext) == None{false}else{true};
    if !ext_is_alive{Key.push_str(crate::getkey().as_str());}
    if crate::globs18::eq_ansi_str(&kcode::F1, Key.as_str()) == 0 {
        return crate::globs18::F1_key();
    } 
    if crate::globs18::eq_ansi_str(&kcode::DOWN_ARROW, Key.as_str()) == 0 {
        return "pp".to_string();
    }
    if crate::globs18::eq_ansi_str(&kcode::UP_ARROW, Key.as_str()) == 0 {
        return "np".to_string();
    }
    if crate::globs18::eq_ansi_str(&kcode::RIGHT_ARROW, Key.as_str()) == 0 {
       // achtung(Key.as_str());
        unsafe {shift_cursor_of_prnt(1, func_id).shift};
        return "dontPass".to_string();
    }
    if crate::globs18::eq_ansi_str(&kcode::Alt_l, Key.as_str()) == 0 {
       show_ls();
        return "dontPass".to_string();
    }
    if crate::globs18::eq_ansi_str(&kcode::LEFT_ARROW, Key.as_str()) == 0 {
    unsafe {shift_cursor_of_prnt(-1, func_id).shift};
    //io::stdout().lock().flush().unwrap();
    achtung("left arrow");
    return "dontPass".to_string();}
    if crate::globs18::eq_ansi_str(&kcode::INSERT, Key.as_str()) == 0 {
        let cmd = format!("{}>::insert",crate::globs18::Ins_key());
        return cmd;
    }
    if crate::globs18::eq_ansi_str(&kcode::F3, Key.as_str()) == 0 {
        crate::globs18::F3_key();
        return "dontPass".to_string();
    }
    if "/" == Key.as_str() {
        let prev_list = crate::read_front_list();
        let prev = read_file("prev_list");
        if prev == ""{crate::save_file(prev_list, "prev_list".to_string());}
        let mut Key_cpy =String::from(Key.to_string()); let mut Key_ = String::from(Key.to_string()); lets_write_path(Key_cpy); crate::INS(&Key_);
    return "/".to_string();}
    if crate::globs18::eq_ansi_str(&kcode::Alt_0, Key.as_str()) == 0 {
    crate::C!(local_indx(true));
        let msg = format!("alt_0 num page {}", crate::get_num_page(-1));
       // popup_msg(&msg);
    return "dontPass".to_string();}
    if crate::globs18::eq_ansi_str(&kcode::F12, Key.as_str()) == 0{
        key_f12(func_id); return "dontPass".to_string();} 
    if crate::globs18::eq_ansi_str(&kcode::DELETE, Key.as_str()) == 0{
        let shift = unsafe {shift_cursor_of_prnt(1, func_id).shift};
        let mut indx = get_prnt(func_id).chars().count();
        if shift <= indx {indx -= shift;}
        let prnt = rm_char_from_string(indx, &get_prnt(func_id));
        set_prnt(prnt.as_str(), func_id);
        return "dontPass".to_string();} 
    let ansiKey: u8 = match Key.as_str().bytes().next(){
        Some(val) => val,
        _ => 0
    };
    if ansiKey == 0{return crate::get_prnt(func_id);}
    if crate::dirty!(){println!("ansi {}, Key {:?}", ansiKey, Key);}
    if kcode::ENTER == ansiKey{crate::globs18::Enter(); return crate::get_prnt(func_id);} 
    if kcode::BACKSPACE == ansiKey{crate::press_BKSP(); return "dontPass".to_string();} 
    if kcode::ESCAPE == ansiKey{println!("esc pressed");}
    if kcode::TAB == ansiKey{println!("tab pressed");}  
   crate::INS(&Key);
       // enter();
       let user_written_path = read_user_written_path().replace("//", "/");
       if user_written_path != "/" && Path::new(&user_written_path).exists() && ln_of_found_files(usize::MAX).1 < 2usize {return get_prnt(func_id);}
        let path = get_path_from_prnt();
        if path.len() == 0{return "dontPass".to_string();}
        if ext_is_alive {if ext.as_ref().unwrap().dontPass{return "dontPass".to_string();}}
        return Key.to_string();
//return get_prnt(func_id);
}
pub fn manage_pages(ext: &Option<&mut crate::__ext_msgs::_ext_msgs>){
let mut Key: String = "".to_string(); 
let mut count: u64 = 0;
let mut bal =String::new();
    loop{
        //set_prnt(&bal, -1);
        let mut ps: crate::_page_struct = unsafe {crate::swtch::swtch_ps(-1, None)};
        let mut data = "".to_string();
        let num_pg = get_num_page(-5555555121);
        let num_pgs = where_is_last_pg();
        crate::swtch::print_viewers();
        crate::swtch::print_pg_info();
        if num_pg < num_pgs || num_pgs ==0 {build_page(&mut ps);}
        println!("{}", get_prnt(-1));
        Key  = "".to_string(); 
        exec_cmd(custom_input(&mut Key, ext));
        clear_screen();
    }
}
pub(crate) fn repeat_char(num_of_times: usize, this_char: &str) -> String{
    let mut ret = String::new();
    for i in 1..num_of_times {ret.push_str(this_char);}
    ret
}
pub(crate) fn wipe_cmd_line(len_2_wipe: usize){
    return;
    let many_spaces = repeat_char(len_2_wipe, " ");
    println!("\r{}", many_spaces);
}
pub(crate) fn form_cmd_line(prompt: String, prnt: String){
    //let whole_line_len = prompt.len() + prnt.len() + 2;
    let print_whole_line = format!("\r{}: {}", prompt, prnt);
    print!("{}", print_whole_line);
}
pub(crate) fn form_cmd_newline(prompt: String, prnt: String){
    let print_whole_line = format!("{}: {}\n", prompt, prnt);
    io::stdout().write_all(&print_whole_line.as_bytes());
}
pub(crate) fn form_cmd_newline_default(){
    let func_id = crate::func_id18::form_cmd_line_default_;
    let prompt = crate::get_prompt(func_id); let mut ret = unsafe {crate::shift_cursor_of_prnt(3, func_id)};
    let shift = ret.str__;
    let mut prnt = get_prnt(func_id);
    let full_path = read_user_written_path();
    let partial_path = get_path_from_strn(crate::cpy_str(&prnt));
    if partial_path != ""{
        if partial_path.chars().count() < full_path.chars().count(){
        prnt = prnt.replace(&partial_path, &full_path);
        }
    }
    //else {prnt = format!("{} {}", prnt, full_path);}
    if full_path.len() > 0{set_prnt(&prnt, func_id);}
    let len = prnt.chars().count();
    if ret.shift == len {prnt = format!("👉{}", prnt)}
    else if ret.shift < len {ret.shift = len - ret.shift;
    prnt.push('👈');
    prnt = ins_last_char_to_string1_from_string1(ret.shift, prnt);}
    let whole_line_len = prompt.len() + prnt.len() + 2;
    prnt.push_str(shift.as_str());
    wipe_cmd_line(whole_line_len);
    form_cmd_newline(prompt, prnt)
}

pub(crate) fn form_cmd_line_default(){
    let func_id = crate::func_id18::form_cmd_line_default_;
    let prompt = crate::get_prompt(func_id); let mut ret = unsafe {crate::shift_cursor_of_prnt(3, func_id)};
    let shift = ret.str__;
    let mut prnt = get_prnt(func_id);
    let full_path = read_user_written_path();
    let partial_path = get_path_from_strn(crate::cpy_str(&prnt));
    if partial_path != ""{
        if partial_path.chars().count() < full_path.chars().count(){
        prnt = prnt.replace(&partial_path, &full_path);
        }
    }
    //else {prnt = format!("{} {}", prnt, full_path);}
    if full_path.len() > 0{set_prnt(&prnt, func_id);}
    let len = prnt.chars().count();
    if ret.shift == len {prnt = format!("👉{}", prnt)}
    else if ret.shift < len {ret.shift = len - ret.shift;
    prnt.push('👈');
    prnt = ins_last_char_to_string1_from_string1(ret.shift, prnt);}
    let whole_line_len = prompt.len() + prnt.len() + 2;
    prnt.push_str(shift.as_str());
    wipe_cmd_line(whole_line_len);
    form_cmd_line(prompt, prnt)
}
pub(crate) fn custom_input(Key: &mut String, ext: &Option<&mut crate::__ext_msgs::_ext_msgs>) -> String{
    let mut Key = Key;
    form_cmd_line_default();
    return hotKeys(&mut Key, ext);
}
pub(crate) unsafe fn exec_cmd_cnt(count_: bool) -> u64{
    static mut count: u64 = 0;
    if count_ {count += 1;}
    count
}
pub(crate) fn exec_cmd(cmd: String){
    let func_id = crate::func_id18::exec_cmd_;
    //println!("cmd {} func {}, prnt {}", cmd, crate::func_id18::get_func_name(func_id), crate::get_prnt(func_id));
    if cmd == "dontPass" {return;}
    let mut cmd = cmd;
    let sub_cmd = extract_sub_cmd(&mut cmd);
    if cmd.as_str().substring(0, 3) == "sl:"{

//        process_tag(key)
    }
    if cmd == "np"{
        unsafe{exec_cmd_cnt(true)};
        let num_page = crate::get_num_page(func_id) + 1;
        crate::set_num_page(num_page, func_id);
        return;
    }
    if crate::globs18::eq_ansi_str(cmd.as_str().substring(0, 2), "pp") == 0{
       // unsafe{exec_cmd_cnt(true)};
        let mut num_page = crate::get_num_page(func_id);
        if num_page > 0{num_page -= 1;}
        crate::set_num_page(num_page, func_id);
        return;
    }
    if crate::globs18::eq_ansi_str(cmd.as_str().substring(0, 3), "go2") == 0{
        let (_, opt) = split_once(cmd.as_str(), " ");
        if opt == "none" {set_ask_user("wrong use of go2: go2 <indx of page>", func_id); return;}
        let pg_num: i64 = match i64::from_str_radix(&opt, 10){
            Ok(val) => val,
            _ => {set_ask_user("wrong use of go2: go2 <indx of page>", func_id); return}
        };
        let global_indx_or_not = crate::C!(local_indx(false));
        if !global_indx_or_not {crate::C!(local_indx(true));}
        let pg_num = get_proper_indx(pg_num, true);
        crate::set_num_page(pg_num.1, func_id);
        if !global_indx_or_not {crate::C!(local_indx(true));}
        return;
    }
    if cmd.as_str().substring(0, 5) == "sieve"{
        sieve_list(crate::cpy_str(&cmd));
        return;
    }
    if cmd.as_str().substring(0, 4) == "ren "{
        renFile();
        return;
    }
    if cmd.as_str().substring(0, 2) == "cd"{
        if sub_cmd != "insert"{change_dir(cmd, true); return;}
        crate::C!(swtch_fn(-1, cmd));
        return;
    }
      if cmd.as_str().substring(0, 2) == "fp"{
        let (_, opt) = split_once(cmd.as_str(), " ");
        if opt == "none" {set_full_path("wrong use of fp: fp <indx of file>", func_id); return;}
        let file_indx: i64 = match i64::from_str_radix(&opt, 10){
            Ok(val) => val,
            _ => {set_full_path("wrong use of fp: fp <indx of file>", func_id); return}
        };
        let file_full_name =  crate::globs18::get_item_from_front_list(file_indx, true);
        let file_full_name = format!("Full path: {}", file_full_name);
        set_full_path(&file_full_name, func_id);
        return;
    }
    if cmd.as_str().substring(0, 2) == ".."{
        crate::dir_up(); return
    }
    if cmd.as_str().substring(0, 1) == "."{
        crate::dir_down(cmd); return
    }
    if cmd.as_str().substring(0, 3) == "mrg"{
        if sub_cmd != "insert"{merge(cmd); return;}
        crate::C!(swtch_fn(-1, cmd));
        return;
    }
    if cmd == "cl mrg" || cmd == "clear merge"{
        clear_merge();
    }
    if cmd == "show mrg"{
        set_front_list("merge");
    }
    if cmd.as_str().substring(0, 4) == "term"{
        let subcmd = extract_sub_cmd_by_mark(&cmd, ":>:".to_string());
        if subcmd != "no_upd_scrn"{crate::term(&cmd); return}
        crate::term(&cmd);
    }
    crate::C!(swtch_fn(-1, cmd));
}
fn extract_sub_cmd(cmd: &mut String) -> String{
    let len_cmd = cmd.chars().count();
    let mark = ">::".to_string();
    let len_mark = mark.chars().count();
    let mut mark_iter = 0usize;
    let mut sub_cmd = String::new();
    for i in 0..len_cmd{
        let ch = cmd.chars().nth(i).unwrap();
        if mark_iter == len_mark {sub_cmd.push(ch);} else {if Some(ch) == mark.chars().nth(mark_iter){mark_iter += 1}}
    }
    let erase = format!("{}{}", mark, sub_cmd);
    *cmd = cmd.replace(&erase, "");
    sub_cmd
}
fn extract_sub_cmd_by_mark(cmd: &String, mark: String) -> String{
    let len_cmd = cmd.chars().count();
    let len_mark = mark.chars().count();
    let mut mark_iter = 0usize;
    let mut sub_cmd = String::new();
    for i in 0..len_cmd{
        let ch = cmd.chars().nth(i).unwrap();
        if mark_iter == len_mark {sub_cmd.push(ch);} else {if Some(ch) == mark.chars().nth(mark_iter){mark_iter += 1}}
    }
    sub_cmd
}