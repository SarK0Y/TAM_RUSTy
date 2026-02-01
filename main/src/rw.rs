use std::mem;
use std::io::Read;
use std::fs;
use std::error::Error;
pub fn read_file_to_vec < T > (path: &String) -> Result <Vec < T >, Box <dyn Error> > {
    let t_size = mem::size_of::<T>();
    let u8_size = mem::size_of:: <u8>();
    let ratio = t_size / u8_size;
    unsafe {
        let mut read_file = fs::read (&path)?;
        let capacity = read_file.capacity() / ratio;
        let len = read_file.len() / ratio;
        let mut ptr: *mut T = read_file.as_mut_ptr() as *mut T;
        let mut ret_vec = Vec::<T>::from_raw_parts (ptr, capacity, len,); 
        mem::forget (read_file);
        Ok(ret_vec)
    }
}
pub fn checkForkChilds () -> bool {
    let only_childs = crate::take_list_adr ("now_only_forked_childs");
    return std::path::Path::new (&only_childs).exists ()
}
pub fn flag_1st_proc () {
    if checkForkChilds () { return }
    let only_childs = crate::take_list_adr ("now_only_forked_childs");
    crate::mk_empty_file (&only_childs);
}
pub fn file_exist7 < T: ToString > (rel_name: T ) -> bool {
    let rel_name0 = rel_name.to_string();
   // dbg! (&rel_name0);
    let rel_name = crate::take_list_adr (&rel_name0);
   // dbg! (&rel_name);
    return std::path::Path::new (&rel_name).exists()
}
pub fn del_file < T: ToString > (rel_name: T ) {
    let rel_name = rel_name.to_string();
    let rel_name = crate::take_list_adr (&rel_name);
    std::fs::remove_file (&rel_name);
}
pub fn close_this_child7 () {
    if file_exist7 ("now_only_forked_childs") {
        std::process::exit (0);
    }
}
//fn