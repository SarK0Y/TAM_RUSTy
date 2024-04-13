use cli_table::{CellStruct, print_stdout, Table, Style};
use colored::Colorize;
use num_traits::ToPrimitive;
use std::collections::{HashMap, hash_map::Entry};
use once_cell::sync::Lazy;
use std::ptr::addr_of_mut;
use crate::{read_file_abs_adr, cached_data, get_num_files, ln_of_found_files_cacheless, cache_state, cache, popup_msg, save_file_abs_adr, checkArg, get_arg_in_cmd, globs18::{strn_2_u64, strn_2_usize}, cache_t, entry_cache_t};
//use crate::extctrl;
//use super::extctrl::*;
impl super::basic{
   pub fn build_page_(&mut self, ps: &mut crate::_page_struct){
    let func_id = crate::func_id18::build_page_;
    let mut try_entry = 0usize;
    let mut num_files = crate::get_num_files(func_id);
    while try_entry < 1_000 {
        if crate::size_of_found_files() > 4u64 {break;}
        num_files = crate::get_num_files(func_id);
        if num_files == 0i64{continue;}
        try_entry += 1; 
    }
    let mut count_down = num_files;
    if crate::size_of_found_files() == 0u64 {println!("No files found"); if !crate::checkArg("-dont-exit"){crate::C!(libc::exit(-1));}}
    let mut num_page; num_page = crate::get_num_page(func_id); // if ps.num_page != i64::MAX{num_page = ps.num_page;}else{num_page = crate::get_num_page(func_id);}
    let mut num_cols; if ps.num_cols != i64::MAX{num_cols = ps.num_cols;}else{num_cols = crate::get_num_cols(func_id);}
    let mut num_rows; if ps.num_rows != i64::MAX{num_rows = ps.num_rows;}else{num_rows = crate::get_num_rows(func_id);}
    if ps.col_width != i64::MAX{crate::set_col_width(ps.col_width, func_id);}
    let num_items_on_pages = num_cols * num_rows; let stopCode: String = crate::getStop_code__!();
    num_page = crate::calc_num_files_up2_cur_pg(); let mut filename_str: String; let mut time_to_stop = false;
    let mut row: Vec<CellStruct> = Vec::new(); let mut row_cpy: Vec<String> = Vec::new();
    println!("{}", crate::get_full_path(func_id));
    for j in 0..num_rows{
        for i in 0..num_cols{
            let mut indx = i + num_cols * j + num_page;
            //indx = num_files - count_down_files;
            let mut res: String ="".to_string();
            res = self.rec_from_front_list(indx, false);
              num_files = crate::get_num_files(func_id);
             if num_files == indx || "front list is empty" == res{time_to_stop = true;}
            // println!("build_page - probe 0");
            let full_path = res;
            //no_dup_indx = indx;
            if !crate::C!(crate::swtch::local_indx(false)){indx -= num_page;}
            let err_ret = std::ffi::OsString::from("");
            let mut err_path = || -> &std::ffi::OsString{return &err_ret};
            //println!("build_page - probe 1");
            let filename = crate::Path::new(&full_path);
            macro_rules! filename_str0{
                () => {String::from(match filename.file_name(){
                    Some(f) => f,
                    None => err_path(),
                }.to_str().unwrap()).as_str()};
            }
            if crate::globs18::eq_str(stopCode.as_str(), filename.as_os_str().to_str().unwrap()) == 0 && stopCode.len() == filename.as_os_str().to_str().unwrap().len() {println!("{}", "caught".bold().green()); 
             time_to_stop = true; break;}
            if crate::dirty!(){
               println!("cmp_str res {}", crate::globs18::eq_str(stopCode.as_str(), filename.as_os_str().to_str().unwrap()));
               println!("stop code {}, len {}; str {}, len {}", stopCode, stopCode.as_str().len(), filename.as_os_str().to_str().unwrap(), filename.as_os_str().to_str().unwrap().len());
               println!("{:?}", filename.file_name());
            }
            let mut fixed_filename: String = filename_str0!().to_string();
            crate::ins_newlines(crate::get_col_width(func_id).to_usize().unwrap(), &mut fixed_filename);
            if filename.is_dir(){filename_str =format!("{}: {}/", indx, fixed_filename);}
            else{filename_str = format!("{}: {}", indx, fixed_filename);}
            if filename_str == stopCode || filename_str == "no str gotten"{return;}
            row_cpy.push(filename_str);
            if count_down <= 0 {time_to_stop = true; break;}
            count_down -= 1;
        }
        let count_pages = crate::get_num_files(func_id) / num_items_on_pages;
        let mut new_row: Vec<Vec<CellStruct>> = Vec::new();
        new_row.push(crate::pg18::cpy_row(&mut row_cpy));
        print_stdout(new_row.table().bold(true).foreground_color(Some(cli_table::Color::Blue)));
        if time_to_stop {break;}
    }
    //println!("{}", pg.table().display().unwrap());
    println!("{}", crate::get_ask_user(func_id));
}
pub(crate) fn pg_rec_to_cache(cache: &mut cache_t, key: &String, val: &String){
    let mut entry_cache: entry_cache_t = HashMap::new();
    let len = cache.len();
    match cache.entry(key.to_string()){
        Entry::Occupied(mut entry) => {entry.get_mut().entry(len).and_modify(|e|{e.push(val.to_string())});},
        Entry::Vacant(entry) => { entry_cache.insert(len, vec!(key.to_string())); entry.insert(entry_cache);}
    }
}
pub(crate) fn pg_0_cache(cache: &mut crate::cache_t, key: &String){
    match cache.entry(key.to_string()){
        Entry::Occupied(mut entry) => {entry.remove();},
        Entry::Vacant(entry) => {}
    }
}
pub(crate) fn pg_rec_from_cache(cache: &mut cache_t, key: &String, indx: usize) -> (String, cached_data){
    match cache.entry(key.to_string()){
        Entry::Occupied(entry) => {let  mut status = cached_data::no_rec; 
            let base_indx = entry.get()[0].to_string();
            let base_indx = match usize::from_str_radix(&base_indx, 10){
                Ok(i) => i,
                _ => return ("no such rec".to_string(), status)
            };
            let ret = if entry.get().len() + base_indx > indx && indx > base_indx
                          {status = cached_data::all_ok; entry.get()[indx - base_indx].to_string()}else{"no such rec".to_string()}; return (ret, status);},
        Entry::Vacant(entry) => {return ("no such list was cached".to_string(), cached_data::no_list)}
    }
}
pub(crate) fn pg_rec_from_front_list(&mut self, indx: i64, fixed_indx: bool) -> String{
    static mut good_count: u64 = 0;
    let proper_indx = crate::get_proper_indx(indx, fixed_indx);
    if proper_indx.0 == usize::MAX{return "front list is empty".to_string()}
    let front_lst = self.read_file("front_list");
    let rec: (String, cached_data) = self.rec_from_cache(&front_lst, crate::i64_2_usize(indx));
    if rec.1 == cached_data::all_ok{crate::C!(crate::logs(&self.cache.len().to_string(), "cache.len")); unsafe{good_count +=1}; return rec.0;}
    //popup_msg("msg");
    if rec.1 == cached_data::no_rec{
        let rec = crate::C!(crate::globs18::lists("", crate::globs18::FRONT_, proper_indx.0, crate::globs18::GET));
        let front_lst0 = front_lst.clone(); let front_lst1 = front_lst0.clone();
        self.rec_to_cache(front_lst, rec.clone());
        let msg = "".to_string(); let msg0 = msg.clone(); let tmp_dir0 = self.tmp_dir.clone(); let cache_window = self.cache_window.clone();
        let indx = proper_indx.0.clone();
        std::thread::spawn(move||{
            //popup_msg("msg1");
           crate::C!(crate::basic::mk_fast_cache(&tmp_dir0, cache_window, indx, &front_lst0, cache_state::ready));
           //popup_msg("msg2");
        });
        let ret = crate::C!(crate::basic::mk_fast_cache(&self.tmp_dir, cache_window, indx, &front_lst1, cache_state::ready));
        if ret.1 == cache_state::ready{self.cache.remove(&front_lst1); self.cache.insert(ret.0.key().to_string(), match ret.0{
            Entry::Occupied(en) => {popup_msg("msg"); en.get().to_vec()},
            Entry::Vacant(en) => return rec
        });}
        return rec;
    }
    else {
        let front_lst0 = front_lst.clone(); let tmp_dir1 = self.tmp_dir.clone(); let cache_window = self.cache_window.clone(); let indx = proper_indx.0.clone();
        std::thread::spawn(move||{
            //popup_msg("msg1");
           crate::C!(crate::basic::mk_fast_cache(&tmp_dir1, cache_window, indx, &front_lst0, cache_state::ready));
           //popup_msg("msg2");
        });
        let ret = crate::C!(crate::basic::mk_fast_cache(&self.tmp_dir, self.cache_window, indx, &front_lst, cache_state::ready));
        if ret.1 == cache_state::ready{self.cache.remove(&front_lst); self.cache.insert(ret.0.key().to_string(), match ret.0{
            Entry::Occupied(en) => en.get().to_vec(),
            Entry::Vacant(en) => return "no_list".to_string()
        });}
    }
    //if !list_id.1{set_ask_user("Can't access to Front list", -1); return "!!no¡".to_string()}
    crate::C!(crate::logs(&good_count.to_string(), "bad_count"));
    return crate::C!(crate::globs18::lists("", crate::globs18::FRONT_, proper_indx.0, crate::globs18::GET))
}
pub(crate) unsafe fn mk_fast_cache<'a>(tmp_dir: &'a String, base_indx: usize, indx: usize, name: &'a String, op: cache_state) -> (Vec<String>, cache_state){
    static mut cache: Lazy<Vec<String>> = Lazy::new(||{Vec::new()});
    static mut count: u64 = 0;
    static mut state: cache_state = cache_state::empty;
    static mut seg_size: usize = 150;
    static mut fst_run: bool = false;
    if !fst_run{
        fst_run = true;
        if checkArg("-cache-seg-size"){
            let seg_size_new = String::from_iter(get_arg_in_cmd("-cache-seg-size").s).trim_end_matches('\0').to_string();
            let ret = strn_2_usize(seg_size_new);
            if ret != None{seg_size = ret.unwrap()}
        }
    }
    //return (empty_entry, cache_state::empty);    
    let adr_of_msg_clean = format!("{tmp_dir}/msgs/basic/cache/clean").replace("//", "/");
    let path_2_msg_forming = format!("{}/msgs/basic/cache/forming", tmp_dir).replace("//", "/");
    let clean = read_file_abs_adr(&adr_of_msg_clean);
    let forming = read_file_abs_adr(&path_2_msg_forming);
    if clean == *name {cache.clear(); state = cache_state::empty;}
    if state == cache_state::forming {return (vec!("".to_string()), cache_state::forming);}
    if op == cache_state::ready{if state != cache_state::empty{return (cache.to_vec(), cache_state::ready);}
     let mut lst_len = ln_of_found_files_cacheless(usize::MAX).1;
    if lst_len == 0{return (vec!("".to_string()), cache_state::no_data_to_add);}
    count += 1;
    //if count > 5{println!("{:?}", cache);}
    let cache_window = base_indx;
    let mut base_indx = 1 + base_indx / 2;
    if base_indx > indx {base_indx = 0}
    else{base_indx = indx - base_indx}
    crate::save_file_abs_adr0(name.to_string(), path_2_msg_forming.clone());
    for i in 0..seg_size{
        let rec = ln_of_found_files_cacheless(i);
        if rec.1 == lst_len{break;}
       // cache.entry(name.clone()).and_modify(|e|{e.push(rec.0)});
        cache.push(name.clone());
     }
     crate::rm_file(&path_2_msg_forming);
    // crate::popup_msg(&std::mem::size_of_val(&cache).to_string());
     return (cache.to_vec(), cache_state::ready);
    }
return (vec!("".to_string()), cache_state::empty);}
pub(crate) fn read_file(&self, name: &str) -> String{
    let path = format!("{}/{name}", self.tmp_dir).replace("//", "/");
    read_file_abs_adr(&path)
}
pub(crate) fn read_cache_msg(&self) -> String{
    self.read_file("msg/basic/cache/clean")
}
}