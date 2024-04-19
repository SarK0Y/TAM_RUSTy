use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use crate::{globs18::take_list_adr, errMsg0, read_file, patch_t};

use std::io::BufRead;
pub(crate) fn reorder_list_4_cmd(name: &str) -> String{
    read_file(name).replace(r"\n", r"\\\n ")
}
pub(crate) fn strn_2_vec(strn: &String, delim: &String) -> Vec<String>{
    let mut ret = Vec::<String>::new();
    let len = strn.chars().count();
    let delim = delim.chars().nth(0);
    let mut item = String::new();
    for ch in strn.chars(){
        if Some(ch) == delim{ret.push(item.clone()); item.clear()}
        item.push(ch)
    }
    ret
}
pub(crate) fn patch(old: Option<String>, new: Option<String>) -> (String, String, bool){
    static mut patch: Lazy<patch_t> = Lazy::new(||{HashMap::new()});
    let mut ret = (String::new(), String::new(), false);
    if old != None && new != None{
        let old = old.unwrap(); let new = new.unwrap();
        crate::C!(patch.insert(old, new));
        return ("".to_string(), "".to_string(), false)
    } 
    if old != None && new == None{
        let old = old.unwrap();
        match crate::C!(patch.entry(old)){
            Entry::Occupied(en) => {},
            _ => {}
        }
    }
("".to_string(), "".to_string(), false)
}