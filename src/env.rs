use crate::{update18::update_dir_list, find_files_ls, TMP_DIR_, bkp_tmp_dir, run_cmd_out_sync, set_front_list, read_file, drop_ls_mode, save_file};

pub(crate) fn change_dir(cmd: String){
    if cmd == "cd"{
        let pwd = read_file("env/cd");
        if pwd == ""{return};
        change_dir(pwd);   
    }
    let path = cmd.replace("cd", "").trim_start().to_string();
    crate::find_files_cd(&path);
    drop_ls_mode();
    set_front_list("cd");
    save_file(path, "env/cd".to_string());
}