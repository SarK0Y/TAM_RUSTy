use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use substring::Substring;
use crate::{globs18::take_list_adr, errMsg0, read_file, patch_t, split_once};

use std::io::BufRead;
pub(crate) fn reorder_list_4_cmd(name: &str) -> String{
    read_file(name).replace(r"\n", r"\\\n ")
}
pub(crate) fn reorder_strn_4_cmd(strn: &String) -> String{
    strn.replace(r"\n", r"\\\n ")
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
    if old == Some("clear patch".to_string()){crate::C!(patch.clear()); return ("".to_string(), "".to_string(), false)}
    if old != None && new == Some("clear entry".to_string()){crate::C!(patch.remove(&old.unwrap())); return ("".to_string(), "".to_string(), false)}
    if old != None && new != None{
        let old = old.unwrap(); let new = new.unwrap();
        crate::C!(patch.insert(old, new));
        return ("".to_string(), "".to_string(), false)
    } 
    if old != None && new == None{
        let old = old.unwrap(); let old0 = old.clone();
        match crate::C!(patch.entry(old)){
            Entry::Occupied(en) => { ret.0 = old0; ret.1 = en.get().to_string(); ret.2 = true; },
            _ => {}
        }
    }
ret
}
pub(crate) fn term_mv(cmd: &String){
    let cmd = cmd.replace("term mv", "").trim_start_matches(' ').to_string();
    
}
pub(crate) fn parse_paths(cmd: &String){
    let cmd = cmd.replace(r"\\ ", r":@@:");
    let (from, to) = split_once(&cmd, " ");
    let from = from.replace(r":@@:", ""); let to = to.replace(r":@@:", "");
    let mut all_files = String::new();
    if from == "@@a"{all_files = crate::raw_read_file("found_files")}
    if from.substring(0, 1) == "/"{}
}