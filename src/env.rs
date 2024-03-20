use crate::{update18::update_dir_list, find_files_ls, TMP_DIR_, bkp_tmp_dir, run_cmd_out_sync, set_front_list, read_file, drop_ls_mode, save_file, popup_msg, ln_of_found_files, ln_of_list, tailOFF, set_ask_user, globs18::get_item_from_front_list};

pub(crate) fn change_dir(cmd: String, set: bool){
    if cmd == "cd"{
        let pwd = read_file("env/cd");
        if pwd == ""{return};
        let pwd = format!("cd {pwd}");
        change_dir(pwd, false);   
    }
    let path = cmd.replace("cd", "").trim_start().to_string();
    let path_escaped = crate::escape_symbs(&path);
    if path == ""{return}
    let (base_path, indx) = ln_of_list(0, "cd");
    let check_base_str = base_path.replace(&path, "");
    {crate::find_files_cd(&path_escaped);}
    drop_ls_mode();
    set_front_list("cd");
    if !set {return;}
    save_file(path, "env/cd".to_string());
}
pub(crate) fn dir_up(){
    let mut pwd = read_file("env/cd");
    tailOFF( &mut pwd, "/");
    if pwd == ""{return};
    change_dir(pwd, true);
}
pub(crate) fn dir_down(cmd: String){
    let cmd = cmd.replace(".", "").trim_end().to_string();
    let indx = match i64::from_str_radix(&cmd, 10){
        Ok(i) => i,
        _ => {set_ask_user("to enter dir, please, write 'dot index of directory' (for example, .7)", 145211752); return}
    };
    let fname = get_item_from_front_list(indx, true);
    if fname == "no str gotten"{set_ask_user("dir_down failed", 145211752); return;}
    change_dir(fname, true);
}