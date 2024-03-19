use crate::{update18::update_dir_list, find_files_ls, TMP_DIR_, bkp_tmp_dir, run_cmd_out_sync, set_front_list, read_file, drop_ls_mode, save_file, popup_msg, ln_of_found_files, ln_of_list};

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
    if check_base_str.len() < base_path.len() || indx == usize::MAX {crate::find_files_cd(&path_escaped);}
    drop_ls_mode();
    set_front_list("cd");
    if !set {return;}
    save_file(path, "env/cd".to_string());
}