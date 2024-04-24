use crate::{read_front_list, save_file0, read_file, set_front_list};

pub(crate) fn key_slash(){
    let front_list = read_front_list();
    save_file0(front_list, "prev_list".to_string());
}
pub(crate) fn pre_Enter(){
    let front_list = read_front_list();
    if front_list != "ls"{return}
    let prev_list = read_file("prev_list");
    set_front_list(&prev_list);
    crate::free_term_msg();
}