use crate::{__get_arg_in_cmd, fix_num_files, mk_cnt};
use crate::update18::upd_screen_or_not;
use crate::{update18::update_dir_list, find_files_ls, TMP_DIR_, bkp_tmp_dir, run_cmd_out_sync, set_front_list, 
    read_file, drop_ls_mode, save_file, popup_msg, ln_of_found_files, ln_of_list, tailOFF, set_ask_user, globs18::get_item_from_front_list,
     set_full_path, is_dir, checkArg, clean_cache};
     use crate::custom_traits::{helpful_math_ops, STRN, STRN_strip}; use substring::Substring;

pub(crate) fn change_dir(cmd: String, set: bool){
    clean_cache("cd");
    if cmd == "cd"{
        let pwd = read_file("env/cd");
        if pwd == ""{return};
        let pwd = format!("cd {pwd}");
        change_dir(pwd, false);   
    }
    let path = cmd.replace("cd", "").trim_start().trim_end().strn();
    if path == ""{return};
    let mut path_escaped = path.clone();
    if !crate::Path::new(&path_escaped).exists() { path_escaped = path_escaped.strip_all_symbs(); }
    //if !crate::Path::new(&path_escaped).exists() {path_escaped = crate::full_escape(&path);}
    if !crate::find_files_cd_cpy_ls(&crate::full_escape(&path_escaped ) ) {crate::errMsg0(&format!("{path_escaped} is empty") ); return};
    drop_ls_mode();
    crate::set_front_list2("cd", 0);
    let mut path_display = format!("Working directory: {}", path);
    set_full_path(&path_display, 4051778415);
    if !set {return;}  upd_screen_or_not((-1, "".strn() ) ); mk_cnt();
    save_file(path, "env/cd".to_string());
    crate::fix_num_files0(-179127);
}
pub(crate) fn change_dir0(){
    let mut path = "".strn();
    if checkArg("-cd"){path = __get_arg_in_cmd("-cd")}
    if path == ""{return}
    let path = path.trim_start().trim_end().strn();
    let path_escaped = crate::full_escape(&path);
    crate::find_files_cd_cpy_ls(&path_escaped);
    let mut path_display = format!("Working directory: {path}");
    set_full_path(&path_display, 4051778415);
    save_file(path, "env/cd".to_string());
}
pub(crate) fn dir_up(){
    let mut pwd = read_file("env/cd");
    if pwd.chars().nth(pwd.chars().count().dec()) == Some('/'){pwd = pwd.substring(0, pwd.chars().count().dec().dec()).strn()}
    tailOFF( &mut pwd, "/");
    if pwd == ""{return};
    change_dir(pwd, true);
}
pub(crate) fn dir_down(cmd: String){
    if checkArg("-dbg") || checkArg("-dbg1"){dbg_dir_down(cmd); return;}
    let cmd = cmd.replace(".", "").trim_end().to_string();
    let indx = match i64::from_str_radix(&cmd, 10){
        Ok(i) => i,
        _ => {set_ask_user("to enter dir, please, write 'dot index of directory' (for example, .7)", 145211752); return}
    };
    let mut fname = get_item_from_front_list(indx, true);
    if fname == "no str gotten"{set_ask_user("dir_down failed", 145211752); return;}
    if !crate::is_dir2(&fname){tailOFF(&mut fname, "/");}
    upd_screen_or_not((-1, "".strn() ) );
    change_dir(fname, true);
}
pub(crate) fn dbg_dir_down(cmd: String){
    let cmd = cmd.replace(".", "").trim_end().to_string();
    let indx = match i64::from_str_radix(&cmd, 10){
        Ok(i) => i,
        _ => {set_ask_user("to enter dir, please, write 'dot index of directory' (for example, .7)", 145211752); return}
    };
    let mut fname = get_item_from_front_list(indx, true);
    if fname == "no str gotten"{set_ask_user("dir_down failed", 145211752); return;}
    if !crate::is_dir2(&fname) {tailOFF(&mut fname, "/");}
    popup_msg(&fname);
    change_dir(fname, true);
}
//fn
