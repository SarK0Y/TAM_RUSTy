use ansi_term::unstyle;
use once_cell::sync::Lazy;
use syn::token::Unsafe;
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use substring::Substring;
use regex::Regex;
use std::borrow::Borrow;
use std::panic;
use crate::custom_traits::{STRN, STRN_strip, fs_tools};
#[cfg(feature = "mae")]
use Mademoiselle_Entropia::help_funcs::get_file;
use crate::update18::delay_ms;
use crate::{helpful_math_ops, run_cmd_out_sync, save_file, save_file0, save_file_append, save_file_append_newline, set_prnt, tailOFF, turn_2_i64};
use crate::{globs18::{take_list_adr, split_once_alt, check_char_in_strn, take_list_adr_env, strn_2_usize, get_item_from_front_list}, errMsg0, read_file, patch_t, split_once, read_tail, parse_paths, run_term_app, is_dir2, escape_backslash, escape_apostrophe, escape_symbs, getkey, dont_scrn_fix, popup_msg, full_escape, mk_dummy_file, ending, run_cmd0, mark_front_lst, set_front_list2, usize_2_i64, get_path_from_strn, name_of_front_list, no_esc_t};

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
pub(crate) fn clear_patch(){
    __patch(Some("::clear patch::".strn()), None);
}
pub(crate) fn no_esc_lst(rec: &String, insert: bool) -> Option<String>{
    static mut no_esc: Lazy<no_esc_t> = Lazy::new(||{HashSet::new()});
    if insert{
        unsafe{
            no_esc.insert(rec.strn());
        } return None
    }
    unsafe{match no_esc.get(rec){
        Some(j) => Some("".strn()),
        _ => None
    }}
}
pub(crate) fn rec_from_patch(key: &String) -> Option<String>{
    let ret = __patch(Some(key.strn()), None);
    //if ret.2 {return Some(format!("{}::patch",ret.1));}
    if ret.2 {return Some(format!("{}",ret.1));}
    None
}
pub(crate) fn patch_len() -> usize{ __patch(Some("::patch len::".to_string()), None).3 }
pub(crate) fn term_mv(cmd: &String){
    let cmd = cmd.replace("term mv", "").trim_start_matches(' ').to_string();
    let (add_opts, all_files, to) = parse_paths(&cmd);
    let mut finally_to =to.clone();
   /*/ let alt_nl = char::from_u32(0x0a).unwrap();
    let nl = String::from(alt_nl);
    let nl = if crate::globs18::check_char_in_strn(&cmd, alt_nl) == nl{nl}else{"\n".to_string()};*/
    let mut vec_files = lines_2_vec_no_dirs(&all_files);
    let mut all_files = vec_2_strn_multilined(&vec_files, 1);//reorder_strn_4_cmd(&all_files);
    if !crate::Path::new(&finally_to.strip_all_symbs()).is_dir(){
        all_files = vec_2_strn_multilined_no_esc(&vec_files, 1);
    }
    all_to_patch(&(vec_files, to));
    let dummy_file = mk_dummy_file();
    let mut cmd = String::new();
    let ided_cmd = take_list_adr("env/dummy_lnks/mv");
    if crate::Path::new(&ided_cmd).exists(){ending("/"); cmd = format!("{ided_cmd} {add_opts} {all_files}\\\n {finally_to}");} 
    else {ending("mv"); cmd = format!("mv {add_opts} {dummy_file} {all_files}\\\n {finally_to}");}
    let state = crate::dont_scrn_fix(false).0; if state {crate::dont_scrn_fix(true);}
    crate::run_term_app_ren(cmd);
}
pub(crate) fn term_cp(cmd: &String){
    let cmd = cmd.replace("term cp", "").trim_start_matches(' ').to_string();
    let (add_opts, all_files, to) = parse_paths(&cmd);
    let mut from = all_files.clone();
    let mut finally_to =to.clone();
    let mut vec_files = lines_2_vec_no_dirs(&all_files);
    let mut all_files = vec_2_strn_multilined(&vec_files, 1);//reorder_strn_4_cmd(&all_files);
    if !crate::Path::new(&finally_to.strip_all_symbs()).is_dir(){
        all_files = vec_2_strn_multilined_no_esc(&vec_files, 1);
    }
    let dummy_file = mk_dummy_file();
    let mut cmd = String::new();
    let ided_cmd = take_list_adr("env/dummy_lnks/cp");
    if crate::Path::new(&ided_cmd).exists(){ending("/"); cmd = format!("{ided_cmd} {add_opts} {all_files}\\\n {finally_to}");} 
    else {ending("cp"); cmd = format!("cp {add_opts} {dummy_file} {all_files}\\\n {finally_to}");}
    let state = crate::dont_scrn_fix(false).0; if state {crate::dont_scrn_fix(true);}
    crate::run_term_app_ren(cmd);
    lst_copied(from.strip_all_symbs(), finally_to.strip_all_symbs());
}
pub(crate) fn term_rm(cmd: &String){
    let cmd = cmd.replace("term rm", "").trim_start_matches(' ').to_string();
    let (add_opts, mut all_files, to) = parse_paths(&cmd);
    all_files = if all_files == ""{ to} else { format! ("{} {}", all_files, to) };
    let to = "".strn();
    let mut vec_files = lines_2_vec_no_dirs(&all_files);
    let mut all_files = vec_2_strn_multilined_no_esc(&vec_files, 0);//reorder_strn_4_cmd(&all_files);
    if vec_files.len() > 1{
        all_files = vec_2_strn_multilined(&vec_files, 0);
    }
    all_files = all_files.replace(r"//", r"/").strn(); ending("rm");
    all_to_patch(&(vec_files, to.clone() ));
    let dummy_file = mk_dummy_file();
    let mut cmd = String::new();
    let ided_cmd = take_list_adr("env/dummy_lnks/rm");
    if crate::Path::new(&ided_cmd).exists(){ cmd = format!("{ided_cmd} {add_opts} {all_files}");} 
    else { cmd = format!("rm {add_opts} {dummy_file} {all_files}");}
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
        ret.0 = format!("{}", add_opts); ret.1 = all_files; ret.2 = to; return ret;
    }
    if cmd.substring(0, 4) == "%enu"{
        all_files = crate::raw_read_file("found_files");
        to = cmd.replace("%enu ", "").trim_start_matches(' ').to_string();
        patch_msg( Some(parse_paths::each_name_unique) );
        ret.0 = format!("--backup=t {}", add_opts); ret.1 = all_files; ret.2 = to; return ret;
    }
    if cmd.substring(0, 1) == "/"{
        let cmd = cmd.replace("\\ ", ":@@:");
        let cmd = cmd.replace(" /", &delim); to = read_tail(&cmd, &delim);
        to = to.replace(":@@:", "\\ "); all_files = cmd.replace(&delim, "").replace(":@@:", "\\ ").replace(&to, "");
        to = format!("/{to}");
        ret.0 = format!("{}", add_opts); ret.1 = all_files; ret.2 = to; return ret;
    }
    ret
}
fn all_to_patch(from_to: &(Vec<String>, String)){
    let mut path = from_to.1.clone();
    //let err_msg =format!("{dir} isn't directory");
    //let mode_2_parse_paths = patch_msg(None);
    //if !is_dir2(&dir){errMsg0(&err_msg); return}
    let mut new = String::new();
    let mut count = 0u64;
    let len = from_to.0.len();
    for v in 0..len{
        let mut old = from_to.0[v].clone();
        if crate::Path::new(&path.strip_all_symbs()).is_dir(){ new = format!("{path}/{}", read_tail(&old, "/")).replace("//", "/"); }
        else{ new = format!("{path}")}
       // old = old.replace(r"\\", r"\"); new = new.replace(r"\\", r"\");
       if ending("" ) == "rm"{new = format!("{old}::D")}
       new = new.replace("//", "/"); old = old.replace("//", "/"); 
        __patch(Some(old.strip_all_symbs()), Some( new.strip_all_symbs() ) );
    }
    ending("none");
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
pub(crate) fn lst_copied(all: String, to: String){
    let lines = all.lines();
    if crate::Path::new(&to).is_dir(){
        for line in lines{
            let mut line0 = line.strn(); line0.strip_all_symbs();
            tailOFF(&mut line0, "/");
            save_file_append_newline(format!("{to}/{line0}").replace("//", "/").strn(), "copied".strn());
        } return;
    } save_file_append_newline(to, "copied".strn());
}
pub(crate) fn lines_2_vec_no_dirs(strn: &String) -> Vec<String>{
    let mut ret = Vec::<String>::new();
    let mut paths = strn.to_string();
    let lines = paths.lines();
    for line in lines{
        let line = line.trim_end().trim_start().to_string();
        let last = if line.chars().count() > 0{line.chars().count() -1}else {continue;};
   // #[cfg(feature="in_dbg")] {println!("{}", line.chars().nth(last).unwrap().to_string()); getkey();}
        if crate::Path::new(&line).is_dir() {continue;}
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
        let ln = full_escape(&ln.strn());
        ret.push_str(format!("\\{nl} {ln}").as_str());
        len.dec();
    } ret
}
pub(crate) fn vec_2_strn_multilined_no_esc(vec_strn: &Vec<String>, cut_off: usize) -> String{
    let mut ret = String::new();
    let nl = char::from_u32(0x0a).unwrap().to_string();
    let mut len = vec_strn.len(); len = if len > cut_off{len - cut_off}else{len};
    for ln in vec_strn{
        let ln = ln.trim_end().trim_start().trim_end_matches('\\');
        if len == 0 {break;}
        ret.push_str(format!("\\{nl} {ln}").as_str());
        len.dec();
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
     if crate::Path::new(&take_list_adr("mae")).exists(){
        let cmd = format!("echo '{}' >> {lst}", take_list_adr("merge"));
        run_cmd0(cmd);
    }
     if crate::Path::new(&take_list_adr("decrypted")).exists(){
        let cmd = format!("echo '{}' >> {lst}", take_list_adr("merge"));
        run_cmd0(cmd);
    }
    mark_front_lst("lst"); set_front_list2("lst", 0);
}
pub(crate) fn manage_lst(cmd: &String){
    let cmd0 =cmd.to_string();
    let (_, mut cmd) = split_once(&cmd, " "); cmd = cmd.trim_start().trim_end().to_string();
    let full_adr_lst = take_list_adr_env(&cmd);
    if crate::Path::new(&full_adr_lst).exists(){cmd = full_adr_lst}
    if cmd.substring(0, 1) == "/"{
        let item = get_path_from_strn(cmd.clone());
        if match std::fs::metadata
                       (&item){Ok(it) => it, _ => return errMsg0(&format!("{item} is empty"))}.len() < 2 {errMsg0(&format!("{item} is empty")); return;}
    let lst_dir = take_list_adr("env/lst"); let path_2_item = item.replace(&read_tail(&item, "/"), "");
    if lst_dir != path_2_item{
        let head = read_tail(&item, "/");
        let item = full_escape(&item);
        let link_2_item = full_escape(&format!("{}/{}", take_list_adr("env/lst"), head) );
        if link_2_item != item{
            let cmd = format!("ln -sf {item} {link_2_item}");
            run_cmd0(cmd);
        }
        mark_front_lst(&head); set_front_list2(&head, 0); crate::fix_num_files(711284191);return;
    }
    }
    if cmd0 == "lst"{list_the_lists(); mark_front_lst("lst"); set_front_list2("lst", 0); return;}
    if name_of_front_list("", false) != "lst"{errMsg0("Please, enter «lst» command, then You will be able to switch lists."); return;}
    let ret = strn_2_usize(cmd);
    if ret == None{errMsg0("Possible variants ==>> lst; lst <<index in list>>; lst /path/to/YourExternalList"); return;}
    let item_indx = usize_2_i64(ret.unwrap());
    let item = get_item_from_front_list(item_indx, true);
    if match std::fs::metadata(&item){Ok(it) => it, _ => return errMsg0(&format!("{item} is empty"))}.len() < 2 {errMsg0(&format!("{item} is empty")); return;}
    let lst_dir = take_list_adr("env/lst"); let path_2_item = item.replace(&read_tail(&item, "/"), "");
    let head = read_tail(&item, "/");
    mark_front_lst(&head); set_front_list2(&head, 0); crate::fix_num_files(711284191);
}
pub(crate) fn add_cmd_in_history(prnt: &String){
    if crate::globs18::check_strn_in_lst("history", prnt){return}
    crate::save_file_append_newline(prnt.strn(), "history".strn());
}
pub(crate) fn link_lst_to(lst: &String, adr: &String){
    let func_name = "link_lst_to";
    let full_adr_to_lst = take_list_adr_env(lst);
    if !crate::Path::new(&adr).exists(){crate::File::create_new(&adr);}
    match std::os::unix::fs::symlink(adr, full_adr_to_lst){Ok(j) => j,
                                        Err(e) => return println!("{func_name} got {e:?}")};
}
pub(crate) fn __link_lst_to(lst: &String, adr: &String){
    let func_name = "link_lst_to";
    let full_adr_to_lst = take_list_adr_env(lst);
    if !crate::Path::new(&adr).exists(){crate::File::create_new(&adr);}
    let cmd = format!("ln -sf {adr} {}", full_adr_to_lst);
    run_cmd0(cmd);
}
pub(crate) fn clean_fast_cache(yes: Option<bool>) -> bool{
    static mut state: bool = false;
    if let Some(yes) = yes {unsafe{ state =  yes; return state; }}
    unsafe { state }

}
pub(crate) fn count_ln(yes: bool, inc: bool, get_size: bool) -> usize{
    static mut count: usize = 0;
    if get_size {return unsafe { count } }
    if !yes {unsafe { count = 0 }; return 0;}
    let ret = unsafe { count };
    if yes && inc{unsafe { count.inc() };}
    if yes && !inc{unsafe { count.dec() };}
    ret
}
pub(crate) fn mk_lst(cmd: &String){
    let dst = cmd.replace("mk lst", "").trim_start().trim_end().strn();
    let dst = take_list_adr_env(&dst);
    let src = take_list_adr_env("found_files").unreel_link_to_file();
    match std::fs::copy(src, dst){Ok(done) => done, Err(e) => return println!("{e:?}")};
}
#[cfg(not(feature = "mae"))]
pub(crate) fn del_ln_from_lst(cmd: &String){
    let ln_num = cmd.replace("del ", "").trim_end().trim_start().i640();
    let ln = get_item_from_front_list(ln_num, true);
    let mut front_lst = take_list_adr("found_files").unreel_link_to_file();
    let mut front_lst_tmp = front_lst.clone(); front_lst_tmp.push_str(".tmp");
    let cmd = format!("cat {}|grep -Ev '{}' > {front_lst_tmp}", full_escape(&front_lst), full_escape(&ln) );
    run_cmd_out_sync(cmd);
    let cmd = format!("mv {front_lst_tmp} {}", full_escape(&front_lst) );
    run_cmd_out_sync(cmd); tailOFF(&mut front_lst, "/");
    crate::clean_all_cache(); clean_fast_cache(Some(true) );
}
#[cfg(feature = "mae" )]
pub(crate) fn del_ln_from_lst(cmd: &String){
    use crate::{globs18::get_proper_indx, save_file_append_newline_abs_adr, save_file_append_newline_abs_adr_fast};

    let ln_num = cmd.replace("del ", "").trim_end().trim_start().i640();
    let ln_num = get_proper_indx(ln_num, true);
    let mut front_lst = take_list_adr("found_files").unreel_link_to_file();
    let mut front_lst_tmp = front_lst.clone(); front_lst_tmp.push_str(".tmp");
    let mut open_front_lst = match get_file(&front_lst){Ok(f) => f, Err(e) => {errMsg0(&format!("{e:?}") ); return}};
    let mut reader = crate::BufReader::new(open_front_lst);
    for (indx, ln) in reader.lines().enumerate(){
        if indx == ln_num.0 {continue;}
        save_file_append_newline_abs_adr_fast(&ln.unwrap_or("".strn()), &front_lst_tmp);
    }
    let cmd = format!("mv {front_lst_tmp} {}", full_escape(&front_lst) );
    run_cmd_out_sync(cmd); tailOFF(&mut front_lst, "/");
    crate::clean_all_cache(); clean_fast_cache(Some(true) );
}
pub(crate) fn edit_ln_in_lst(cmd: &String){
    let ln_num = cmd.replace("edit ", "").trim_end().trim_start().i640();
    let ln = get_item_from_front_list(ln_num, true);
    #[cfg(not(feature = "mae"))] save_file0(ln.strn(), "edit.tmp".strn());
    #[cfg(feature = "mae" )] let ln_num = crate::globs18::get_proper_indx(ln_num, true).0;
    #[cfg(feature = "mae" )] save_file0(ln_num.strn(), "edit.ln.tmp".strn());
    //let mut front_lst = take_list_adr("found_files").unreel_link_to_file();
    set_prnt(&ln, 984419357);
    edit_mode_lst(Some (true) );
}
#[cfg(not(feature = "mae"))]
pub(crate) fn edit_ln_in_lst_fin_op(){
    let old_ln = read_file("edit.tmp");
    let mut front_lst = take_list_adr_env("found_files");
    delay_ms(112);
    front_lst = front_lst.unreel_link_to_file();
    let mut tmp = front_lst.clone(); tmp.push_str(".tmp");
    edit_mode_lst(Some (false) );
    let new_ln = crate::read_prnt();
    let cmd = format!("cat {}|sed 's${}${}$g' > {tmp}", &front_lst, &old_ln, &new_ln);
    run_cmd_out_sync(cmd.clone());
   // errMsg0(&cmd);
    match std::fs::rename(tmp, front_lst){Ok (op) => op, Err(e) => return errMsg0(&format!("cmd\n {cmd}\n{e:?}") )};
    crate::clean_all_cache(); clean_fast_cache(Some(true) );
}
#[cfg(feature = "mae")]
pub(crate) fn edit_ln_in_lst_fin_op(){
    use crate::save_file_append_newline_abs_adr_fast; use crate::custom_traits::STRN_usize;
    let ln_num = read_file("edit.ln.tmp").usize0();
    let mut front_lst = take_list_adr_env("found_files");
    delay_ms(112);
    front_lst = front_lst.unreel_link_to_file();
    let mut tmp = front_lst.clone(); tmp.push_str(".tmp");
    edit_mode_lst(Some (false) );
    let new_ln = crate::read_prnt();
    let mut front_lst_tmp = front_lst.clone(); front_lst_tmp.push_str(".tmp");
    let mut open_front_lst = match get_file(&front_lst){Ok(f) => f, Err(e) => {errMsg0(&format!("{e:?}") ); return}};
    let mut reader = crate::BufReader::new(open_front_lst);
    for (indx, ln) in reader.lines().enumerate(){
        if indx == ln_num {save_file_append_newline_abs_adr_fast(&new_ln, &front_lst_tmp); continue;}
        save_file_append_newline_abs_adr_fast(&ln.unwrap_or("".strn()), &front_lst_tmp);
    }
   // errMsg0(&cmd);
    match std::fs::rename(tmp, front_lst){Ok (op) => op, Err(e) => return errMsg0(&format!("{e:?}") )};
    crate::clean_all_cache(); clean_fast_cache(Some(true) );
}
pub(crate) fn edit_mode_lst(active: Option < bool >) -> bool{
    static mut state: bool = false;
    if let Some(x) = active{
        unsafe {state = x}
    }
    unsafe {state}
}
pub fn full_clean_cache(){
    crate::clean_all_cache(); clean_fast_cache(Some(true) );
}
//fn