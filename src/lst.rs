use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use substring::Substring;
use regex::Regex;
use std::borrow::Borrow;
use crate::{globs18::{take_list_adr, split_once_alt}, errMsg0, read_file, patch_t, split_once, read_tail};

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
pub(crate) fn parse_paths(cmd: &String) -> (String, String){
    let mut cmd = cmd.to_string();
    let re = Regex::new(r"(?x)
                            (?<long_name_opt>--\w+)|
                            (?<short_name_opt>-\w+)
    
    ").unwrap();
    let mut opts = Vec::<String>::new();
    let mut add_opts = String::new();
    let delim = "🁶".to_string(); let mut to = String::new();
    let mut all_files = String::new();
    let mut ret = (String::new(), String::new());
    if cmd.substring(0, 1) == "-"{
        let caps = re.captures_iter(&cmd);
        for ca in caps{
            if ca["long_name_opt"] != "".to_string(){opts.push(ca["long_name_opt"].to_string());}
            if ca["short_name_opt"] != "".to_string(){opts.push(ca["short_name-opt"].to_string());}
        }
        cmd = re.replace_all(&cmd, "").to_string();
        for opt in opts{add_opts.push_str(opt.as_str()); add_opts.push(' ');}
    }
    if cmd.substring(0, 3) == "@@a"{
        all_files = crate::raw_read_file("found_files");
        to = cmd.replace("@@a ", "\\ ").trim_start_matches(' ').to_string();
        ret.0 = format!("{}{}", add_opts, reorder_strn_4_cmd(&all_files)); ret.1 = to; return ret
    }
    if cmd.substring(0, 1) == "/"{
        let cmd = cmd.replace("\\ ", ":@@:");
        let cmd = cmd.replace(" /", &delim); to = format!("/{}", read_tail(&cmd, &delim));
        to = to.replace(":@@:", "\\ "); all_files = reorder_strn_4_cmd(&cmd.replace(&delim, "").replace(":@@:", "\\ ").replace(&to, ""));
        ret.0 = format!("{}{}", add_opts, all_files); ret.1 = to; return ret
    }
    ret
}
pub(crate) fn to_patch(from_to: &(Vec<String>, String)){
    let dir = from_to.1.clone();
    if !crate::Path::new(&dir).is_dir(){return}
    let len = from_to.0.len();
    for v in 0..len{
        let old = from_to.0[v].clone();
        let new = format!("{dir}/{}", read_tail(&old, "/")).replace("//", "/");
        patch(Some(old), Some(new));
    }
}