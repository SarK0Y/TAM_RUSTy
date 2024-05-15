use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use substring::Substring;
use regex::Regex;
use std::borrow::Borrow;
use std::panic;
use crate::{globs18::{take_list_adr, split_once_alt, check_char_in_strn, take_list_adr_env, strn_2_usize, get_item_from_front_list}, errMsg0, read_file, patch_t, split_once, read_tail, parse_paths, run_term_app, is_dir2, escape_backslash, escape_apostrophe, escape_symbs, getkey, dont_scrn_fix, popup_msg, full_escape, mk_dummy_file, ending, run_cmd0, mark_front_lst, set_front_list2, usize_2_i64, get_path_from_strn};

use std::io::BufRead;
pub(crate) fn reorder_list_4_cmd(name: &str) -> String{
    read_file(name).replace(r"\n", r"\\\n ")
}
pub(crate) fn reorder_strn_4_cmd(strn: &String) -> String{
    let strn = strn.replace(r"\n", r"\\\n ");
    let alt_nl = char::from_u32(0x0a).unwrap();
    let alt_nl = String::from(alt_nl);
    let replace_with = format!(r"\{alt_nl} ");
    let strn = strn.replace(&alt_nl, &replace_with);
    //println!("strn: {strn}");
    //getkey();
    strn
}
pub(crate) fn strn_2_vec(strn: &String, delim: &str) -> Vec<String>{
    let mut ret = Vec::<String>::new();
    let len = strn.chars().count();
    let delim = delim.chars().nth(0);
    let mut item = String::new();
    for ch in strn.chars(){
        if Some(ch) == delim{ret.push(item.trim_end().trim_start().to_string()); item.clear()}
        item.push(ch)
    }
    ret
}
pub(crate) fn __patch(old: Option<String>, new: Option<String>) -> (String, String, bool, usize){
    #[cfg(feature="in_dbg")]
    if crate::breaks("break patch", 1, true).0 == 1 && crate::breaks("break patch", 1, true).1{
        println!("break patch 1")
    }
    static mut patch: Lazy<patch_t> = Lazy::new(||{HashMap::new()});
    let mut ret = (String::new(), String::new(), false, 0usize);
    let mut old = old; let mut new = new; let empty = String::new();
    let mut old0 = old.clone().unwrap_or(empty.clone()); let mut new0 = new.clone().unwrap_or(empty);
    if old == Some("".to_string()){old = None} if new == Some("".to_string()){new = None}
    if old == Some("::clear patch::".to_string()){crate::C!(patch.clear()); return ("".to_string(), "".to_string(), false, 0)}
    if old == Some("::prnt patch::".to_string()){crate::C!(dbg!(&patch.clone())); return ("".to_string(), "".to_string(), false, 0)}
    if old == Some("::patch len::".to_string()){return ("".to_string(), "".to_string(), false, crate::C!(patch.len()))}
    if old != None && new == Some("::clear entry::".to_string()){crate::C!(patch.remove(&old.unwrap())); return ("".to_string(), "".to_string(), false, 0)}
    if old != None && new != None{
        crate::C!(patch.insert(old0, new0));
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
pub(crate) fn rec_from_patch(key: &String) -> Option<String>{
    let key = full_escape(&key);
    let ret = __patch(Some(key), None);
    if ret.2 {return Some(format!("{}::patch",ret.1));}
    None
}
pub(crate) fn patch_len() -> usize{ __patch(Some("::patch len::".to_string()), None).3 }
pub(crate) fn term_mv(cmd: &String){
    let cmd = cmd.replace("term mv", "").trim_start_matches(' ').to_string();
    let (add_opts, all_files, to) = parse_paths(&cmd);
    let finally_to =to.clone();
   /*/ let alt_nl = char::from_u32(0x0a).unwrap();
    let nl = String::from(alt_nl);
    let nl = if crate::globs18::check_char_in_strn(&cmd, alt_nl) == nl{nl}else{"\n".to_string()};*/
    let mut vec_files = lines_2_vec_no_dirs(&all_files);
    let all_files = vec_2_strn_multilined(&vec_files, 1);
    all_to_patch(&(vec_files, to));
    let dummy_file = mk_dummy_file();
    ending("mv");
    let cmd = format!("mv {add_opts} {dummy_file} {all_files} {finally_to}");    
    let state = crate::dont_scrn_fix(false).0; if state {crate::dont_scrn_fix(true);}
    crate::run_term_app_ren(cmd);
}
pub(crate) fn term_cp(cmd: &String){
    let cmd = cmd.replace("term cp", "").trim_start_matches(' ').to_string();
    let (add_opts, all_files, to) = parse_paths(&cmd);
    let finally_to =to.clone();
    let mut vec_files = lines_2_vec_no_dirs(&all_files);
    let all_files = vec_2_strn_multilined(&vec_files, 1);//reorder_strn_4_cmd(&all_files);
    all_to_patch(&(vec_files, to));
    let dummy_file = mk_dummy_file();
    ending("cp");
    let cmd = format!("cp {add_opts} {dummy_file} {all_files}\\\n {finally_to}");    
    let state = crate::dont_scrn_fix(false).0; if state {crate::dont_scrn_fix(true);}
    crate::run_term_app_ren(cmd);
}
pub(crate) fn default_term_4_shol_a(cmd: &String) -> bool{
    let if_shol_a: Vec<_> = cmd.match_indices("%a").map(|(i, _)|i).collect();
    if if_shol_a.len() == 0{return false;}
    let cmd = cmd.replace("term", "").trim_start_matches(' ').to_string();
    let (cmd_name, cmd) = split_once(&cmd, " ");
    ending(cmd_name.as_str());
    let cmd = cmd.trim_start_matches(' ').to_string();
    let (_, all_files, _) = parse_paths(&cmd);
    let mut vec_files = lines_2_vec_no_dirs(&all_files);
    let all_files = vec_2_strn_multilined(&vec_files, 1);//reorder_strn_4_cmd(&all_files);
    //let dummy_file = mk_dummy_file();
    let cmd = cmd.replace("%a", &all_files);
    let cmd = format!("{cmd_name} {cmd}");    
    let state = crate::dont_scrn_fix(false).0; if state {crate::dont_scrn_fix(true);}
    crate::run_term_app(cmd);
    true
}
fn parse_paths(cmd: &String) -> (String, String, String){
    let mut cmd = cmd.to_string();
    if match cmd.chars().nth(cmd.len() -1){Some(k) => k, _ => "0".chars().nth(0).unwrap()}.to_string() == crate::getStop_code__!(){cmd = cmd.substring(0, cmd.len() - 1).to_string()}
    let re = Regex::new(r"(?x)
                            (?<long_name_opt>--\w+\s)|
                            (?<short_name_opt>-\w+\s)
    
    ").unwrap();
    let mut opts = Vec::<String>::new();
    let mut add_opts = String::new();
    let delim = "🁶".to_string(); let mut to = String::new();
    let mut all_files = String::new();
    let mut ret = (String::new(), String::new(), String::new());
    if cmd.substring(0, 1) == "-"{
        let caps = re.captures_iter(&cmd);
        for ca in caps{
            let res = panic::catch_unwind(||{ca["long_name_opt"].to_string()});
            if res.is_ok() && ca["long_name_opt"] != "".to_string(){opts.push(ca["long_name_opt"].to_string());}
            let res = panic::catch_unwind(||{ca["short_name_opt"].to_string()});
            if res.is_ok() && ca["short_name_opt"] != "".to_string(){opts.push(ca["short_name_opt"].to_string());}
        }
        cmd = re.replace_all(&cmd, "").to_string();
        for opt in opts{add_opts.push_str(opt.as_str()); add_opts.push(' ');}
    }
    if cmd.substring(0, 2) == "%a"{
        all_files = crate::raw_read_file("found_files");
        to = cmd.replace("%a ", "").trim_start_matches(' ').to_string();
        patch_msg( Some(parse_paths::all_files) );
        let all_files = escape_backslash(&all_files);
        let all_files = escape_apostrophe(&all_files);
        let all_files = escape_symbs(&all_files);
        let to = escape_backslash(&to);
        let to = escape_apostrophe(&to);
        let to = escape_symbs(&to);
        ret.0 = format!("{}", add_opts); ret.1 = all_files; ret.2 = to; return ret;
    }
    if cmd.substring(0, 4) == "%enu"{
        all_files = crate::raw_read_file("found_files");
        to = cmd.replace("%enu ", "").trim_start_matches(' ').to_string();
        patch_msg( Some(parse_paths::each_name_unique) );
        let all_files = escape_backslash(&all_files);
        let all_files = escape_apostrophe(&all_files);
        let all_files = escape_symbs(&all_files);
        let to = escape_backslash(&to);
        let to = escape_apostrophe(&to);
        let to = escape_symbs(&to);
        ret.0 = format!("--backup=t {}", add_opts); ret.1 = all_files; ret.2 = to; return ret;
    }
    if cmd.substring(0, 1) == "/"{
        let cmd = cmd.replace("\\ ", ":@@:");
        let cmd = cmd.replace(" /", &delim); to = read_tail(&cmd, &delim);
        to = to.replace(":@@:", "\\ "); all_files = cmd.replace(&delim, "").replace(":@@:", "\\ ").replace(&to, "");
        to = format!("/{to}");
        let to = escape_backslash(&to);
        let to = escape_apostrophe(&to);
        let to = escape_symbs(&to);
        ret.0 = format!("{}", add_opts); ret.1 = all_files; ret.2 = to; return ret;
    }
    ret
}
fn all_to_patch(from_to: &(Vec<String>, String)){
    let dir = from_to.1.clone();
    //let err_msg =format!("{dir} isn't directory");
    //let mode_2_parse_paths = patch_msg(None);
    //if !is_dir2(&dir){errMsg0(&err_msg); return}
    let mut new = String::new();
    let mut count = 0u64;
    let len = from_to.0.len();
    for v in 0..len{
        let mut old = from_to.0[v].clone();
        /*if mode_2_parse_paths == parse_paths::each_name_unique{ new = format!("{dir}/{count}_{}", read_tail(&old, "/")).replace("//", "/"); count += 1 }
        else*/ { new = format!("{dir}/{}", read_tail(&old, "/")).replace("//", "/"); }
        old = old.replace(r"\\", r"\"); new = new.replace(r"\\", r"\");
        __patch(Some(old), Some(new));
    }
}
fn patch_msg(msg: Option<crate::parse_paths>) -> crate::parse_paths{
    static mut mode: crate::parse_paths = parse_paths::default;
    crate::C!(mode = msg.unwrap_or(crate::C!(mode.clone())));
    crate::C!(mode.clone())
}
pub(crate) fn prnt_patch(){ __patch(Some("::prnt patch::".to_string()), None); dont_scrn_fix(true); getkey();}
pub(crate) fn paths_2_vec(strn: &String, delim: &str) -> Vec<String>{
    let mut ret = Vec::<String>::new();
    let mut paths = strn.to_string();
#[cfg(feature="in_dbg")]
    if crate::breaks("paths 2 vec", 1, true).1 && crate::breaks("paths 2 vec", 1, true).0 == 1{crate::report(&paths, "paths 2 vec");}
    paths = strn.replace(r"\ ", ":@:");
    loop {
        let (path, paths) = split_once(&paths, " /");
        let paths = paths.replace(":@:", r"\ ").trim_end().trim_start().to_string();
        if path == "" && paths.substring(0, 1) == "/"{ret.push(paths); return ret;}
        let path = if path.substring(0, 1) == "/"{path}else {format!("/{path}")};
        let path = path.replace(":@:", r"\ ").trim_end().trim_start().to_string();
#[cfg(feature="in_dbg")]
    let path121 = path.clone();
            ret.push(path);
#[cfg(feature="in_dbg")]
    if crate::breaks("paths 2 vec0", 1, true).1 && crate::breaks("paths 2 vec0", 1, true).0 == 1{crate::report(&path121, "paths 2 vec0");}
            if paths.len() == 0{break;}
        }
    if ret.len() == 0{return strn_2_vec(strn, delim);}
    ret
}
pub(crate) fn lines_2_vec(strn: &String) -> Vec<String>{
    let mut ret = Vec::<String>::new();
    let mut paths = strn.to_string();
    let lines = paths.lines();
    for line in lines{
        ret.push(line.trim_end().trim_start().to_string())
    }
    ret
}
pub(crate) fn lines_2_vec_no_dirs(strn: &String) -> Vec<String>{
    let mut ret = Vec::<String>::new();
    let mut paths = strn.to_string();
    let lines = paths.lines();
    for line in lines{
        let line = line.trim_end().trim_start().to_string();
        let last = if line.chars().count() > 0{line.chars().count() -1}else {continue;};
   // #[cfg(feature="in_dbg")] {println!("{}", line.chars().nth(last).unwrap().to_string()); getkey();}
        if line.chars().nth(last).unwrap().to_string() == "/" {continue;}
        ret.push(line)
    }
    ret
}
pub(crate) fn vec_2_strn_multilined(vec_strn: &Vec<String>, cut_off: usize) -> String{
    let mut ret = String::new();
    let nl = char::from_u32(0x0a).unwrap().to_string();
    let mut len = vec_strn.len(); len = if len > cut_off{len - cut_off}else{len};
    for ln in vec_strn{
        let ln = ln.trim_end().trim_start().trim_end_matches('\\');
        if len == 0 {break;}
        ret.push_str(format!("\\{nl} {ln}").as_str());
        len -= 1;
    } ret
}
pub(crate) fn list_the_lists(){
    let lst = take_list_adr("lst");
    let lst_dir = take_list_adr("env/lst");
    let cmd = format!("find {lst_dir} > {lst}");
    run_cmd0(cmd);
    // check & add default lsts
    if crate::Path::new(&take_list_adr("main0")).exists(){
        let cmd = format!("echo '{}' >> {lst}", take_list_adr("main0"));
        run_cmd0(cmd);
    }
    if crate::Path::new(&take_list_adr("filter")).exists(){
        let cmd = format!("echo '{}' >> {lst}", take_list_adr("filter"));
        run_cmd0(cmd);
    }
    if crate::Path::new(&take_list_adr("cd")).exists(){
        let cmd = format!("echo '{}' >> {lst}", take_list_adr("cd"));
        run_cmd0(cmd);
    }
    if crate::Path::new(&take_list_adr("merge")).exists(){
        let cmd = format!("echo '{}' >> {lst}", take_list_adr("merge"));
        run_cmd0(cmd);
    }
    mark_front_lst("lst"); set_front_list2("lst", 0);
}
pub(crate) fn manage_lst(cmd: &String){
    if *cmd == "lst"{list_the_lists(); return;}
    let (_, mut cmd) = split_once(&cmd, " "); cmd = cmd.trim_start().trim_end().to_string();
    if cmd.substring(0, 1) == "/"{
        let item = get_path_from_strn(cmd.clone());
    let lst_dir = take_list_adr("env/lst"); let path_2_item = item.replace(&read_tail(&item, "/"), "");
    if lst_dir != path_2_item{
        let head = read_tail(&item, "/");
        let link_2_item = format!("{}/{}", take_list_adr("env/lst"), head);
        let cmd = format!("ln -sf {item} {link_2_item}");
        run_cmd0(cmd);
        mark_front_lst(&head); set_front_list2(&head, 0); return;
    }
    }
    let ret = strn_2_usize(cmd);
    if ret == None{errMsg0("Possible variants ==>> lst; lst <<index in list>>; lst /path/to/YourExternalList"); return;}
    let item_indx = usize_2_i64(ret.unwrap());
    let item = get_item_from_front_list(item_indx, true);
    let lst_dir = take_list_adr("env/lst"); let path_2_item = item.replace(&read_tail(&item, "/"), "");
    if lst_dir != path_2_item{
        let head = read_tail(&item, "/");
        let link_2_item = format!("{}/{}", take_list_adr("env/lst"), head);
        let cmd = format!("ln -sf {item} {link_2_item}");
        run_cmd0(cmd);
        mark_front_lst(&head); set_front_list2(&head, 0); return;
    }
    let head = read_tail(&item, "/");
    mark_front_lst(&head); set_front_list2(&head, 0);
}
//fn