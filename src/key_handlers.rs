use crate::{read_front_list, save_file0, read_file, set_front_list, globs18::take_list_adr, read_file_abs_adr, errMsg0};

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