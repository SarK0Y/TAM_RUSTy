use cli_table::{CellStruct, print_stdout, Table, Style};
use colored::Colorize;
use num_traits::ToPrimitive;
use std::collections::{HashMap, hash_map::Entry};
use once_cell::sync::{Lazy, OnceCell};
use std::ptr::addr_of_mut;
use crate::{read_file_abs_adr, cached_data, get_num_files, ln_of_found_files_cacheless, cache_state, cache, popup_msg, save_file_abs_adr, checkArg, get_arg_in_cmd, globs18::{strn_2_u64, strn_2_usize, seg_size}, cache_t, entry_cache_t, i64_2_usize, getkey, get_num_page};
//use crate::extctrl;
//use super::extctrl::*;
impl super::basic{
   pub fn build_page_(&mut self, ps: &mut crate::_page_struct){
    let func_id = crate::func_id18::build_page_;
    let mut try_entry = 0usize;
    let mut num_files = crate::get_num_files(func_id);
    let dbg_point = self.read_file("stop_point").trim_end().to_string();
    if dbg_point == "001"{
        println!("stop 001");
       // panic!("kkkkkkkmmmmmmmmmm,,,,,,,,,,,,,,");
    }
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
    let mut display_indx = 0i64;
    for j in 0..num_rows{
        for i in 0..num_cols{
            let mut indx = i + num_cols * j + num_page;
            //indx = num_files - count_down_files;
            let mut res: String ="".to_string();
            while res == "" {res = self.rec_from_front_list(indx, true);}
              num_files = crate::get_num_files(func_id);
             if num_files == indx || "front list is empty" == res{time_to_stop = true;}
            // println!("build_page - probe 0");
            let full_path = res;
            //no_dup_indx = indx;
            display_indx = indx;
            if !crate::C!(crate::swtch::local_indx(false)){display_indx = indx - num_page;}
            let err_ret = std::ffi::OsString::from("");
            let mut err_path = || -> &std::ffi::OsString{return &err_ret};
            //println!("build_page - probe 1");
            let mut filename = crate::Path::new(&full_path);
            macro_rules! filename_str0{
                () => {String::from(match filename.file_name(){
                    Some(f) => f,
                    None => err_path(),
                }.to_str().unwrap()).as_str()};
            }
            if filename.as_os_str().to_str() == None{filename = crate::Path::new("")}
            if crate::globs18::eq_str(stopCode.as_str(), filename.as_os_str().to_str().unwrap()) == 0 && stopCode.len() == filename.as_os_str().to_str().unwrap().len() {println!("{}", "caught".bold().green()); 
             time_to_stop = true; break;}
            if crate::dirty!(){
               println!("cmp_str res {}", crate::globs18::eq_str(stopCode.as_str(), filename.as_os_str().to_str().unwrap()));
               println!("stop code {}, len {}; str {}, len {}", stopCode, stopCode.as_str().len(), filename.as_os_str().to_str().unwrap(), filename.as_os_str().to_str().unwrap().len());
               println!("{:?}", filename.file_name());
            }
            let mut fixed_filename: String = filename_str0!().to_string();
            crate::ins_newlines(crate::get_col_width(func_id).to_usize().unwrap(), &mut fixed_filename);
            if filename.is_dir(){filename_str =format!("{}: {}/", display_indx, fixed_filename);}
            else{filename_str = format!("{}: {}", display_indx, fixed_filename);}
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
    let offset = indx % seg_size();
    let seg_num = indx / seg_size();
    let failed = ("no such list was cached".to_string(), cached_data::no_list);
    match cache.entry(key.to_string()){
        Entry::Occupied(entry) => {let key= entry.get().contains_key(&seg_num); if key {
            let len = entry.get()[&seg_num].len(); if len <= offset {return failed;}
                return  (entry.get()[&seg_num][offset].clone(), cached_data::all_ok);
        }else {return failed;}},
        Entry::Vacant(entry) => {return failed;}
    }
}
pub(crate) fn pg_rec_from_front_list(&mut self, indx: i64, fixed_indx: bool) -> String{
    static mut good_count: u64 = 0;
    let proper_indx = /*(i64_2_usize(indx), indx);*/crate::get_proper_indx(indx, fixed_indx);
    if proper_indx.0 == usize::MAX{return "front list is empty".to_string()}
    let front_lst = self.read_file("front_list");
    if indx == 12248{
       println!("check point");
    }
    let rec: (String, cached_data) = self.rec_from_cache(&front_lst, i64_2_usize(indx));
    if rec.1 == cached_data::all_ok{crate::C!(crate::logs(&self.cache.len().to_string(), "cache.len")); unsafe{good_count +=1}; return rec.0;}
    //popup_msg("msg");
     let adr_of_msg_clean = format!("{}/msgs/basic/cache/clean", self.tmp_dir).replace("//", "/");
     let clean = read_file_abs_adr(&adr_of_msg_clean);
     let front_lst0 = front_lst.clone(); let front_lst1 = front_lst0.clone(); let front_lst2 = front_lst0.clone();
     if clean == front_lst {self.cache.remove(&front_lst);}
     let mut cache_entry: entry_cache_t = HashMap::new();
     let indx = i64_2_usize(indx);//proper_indx.0.clone();
     let seg_num = indx / self.seg_size;
    if rec.1 == cached_data::no_rec{
        let rec = crate::C!(crate::globs18::lists("", crate::globs18::FRONT_, proper_indx.0, crate::globs18::GET));
        //self.rec_to_cache(front_lst, rec.clone());
        let msg = "".to_string(); let msg0 = msg.clone(); let tmp_dir0 = self.tmp_dir.clone(); let cache_window = self.cache_window.clone();
        std::thread::spawn(move||{
            //popup_msg("msg1");
           crate::C!(crate::basic::mk_fast_cache(&tmp_dir0, indx, &front_lst0, cache_state::ready));
           //popup_msg("msg2");
        });
        let ret = crate::C!(crate::basic::mk_fast_cache(&self.tmp_dir, indx, &front_lst1, cache_state::ready));
        if ret.1 == cache_state::ready{
            popup_msg("msg");
            cache_entry.insert(indx / self.seg_size, ret.0.clone());
            match self.cache.entry(front_lst2){
            Entry::Occupied(mut en) => {en.get_mut().insert(seg_num, ret.0);},
            Entry::Vacant(en) => {en.insert(cache_entry);}
         }
        }
        return rec;
    }
    else {
        let front_lst0 = front_lst.clone(); let tmp_dir1 = self.tmp_dir.clone(); let cache_window = self.cache_window.clone(); let indx = proper_indx.0.clone();
        std::thread::spawn(move||{
            //popup_msg("msg1");
           crate::C!(crate::basic::mk_fast_cache(&tmp_dir1, indx, &front_lst0, cache_state::ready));
           //popup_msg("msg2");
        });
        let ret = crate::C!(crate::basic::mk_fast_cache(&self.tmp_dir, indx, &front_lst, cache_state::ready));
        if ret.1 == cache_state::ready{
           // popup_msg("msg1"); // hits only here 
            cache_entry.insert(indx / self.seg_size, ret.0.clone());
            match self.cache.entry(front_lst1){
            Entry::Occupied(mut en) => {en.get_mut().insert(seg_num, ret.0);},
            Entry::Vacant(en) => {en.insert(cache_entry); return crate::C!(crate::globs18::lists("", crate::globs18::FRONT_, proper_indx.0, crate::globs18::GET));}
         }
    }
    //if !list_id.1{set_ask_user("Can't access to Front list", -1); return "!!no¡".to_string()}
    crate::C!(crate::logs(&good_count.to_string(), "bad_count"));
    return crate::C!(crate::globs18::lists("", crate::globs18::FRONT_, proper_indx.0, crate::globs18::GET))
}}
pub(crate) unsafe fn mk_fast_cache<'a>(tmp_dir: &'a String, indx: usize, name: &'a String, op: cache_state) -> (Vec<String>, cache_state){
    //static mut cache: Lazy<Vec<String>> = Lazy::new(||{Vec::new()});
    static mut cache: OnceCell<Vec<String>> = OnceCell::new();
    static mut count: u64 = 0;
    static mut state: cache_state = cache_state::taken;
    static mut seg_size: usize = 150;
    static mut fst_run: bool = false;
    let empty_lst = vec!("".to_string());
    if !fst_run{
        fst_run = true;
        if checkArg("-cache-seg-size"){
            let seg_size_new = String::from_iter(get_arg_in_cmd("-cache-seg-size").s).trim_end_matches('\0').to_string();
            let ret = strn_2_usize(seg_size_new);
            if ret != None{seg_size = ret.unwrap()}
        }
        let mut vec0 = Vec::with_capacity(10000);
        cache.set(vec0);
    }else{
        if state == cache_state::forming {return (cache.get_mut().expect("cache in mk_fast_cache").clone(), cache_state::forming);}
        if state == cache_state::taken{
           // cache.take(); // = OnceCell::new(); 
            cache.get_mut().unwrap().clear(); state = cache_state::empty}}
    //return (empty_entry, cache_state::empty);    
   let mut cache0 =cache.get_mut().expect("cache in mk_fast_cache");
    let path_2_msg_forming = format!("{}/msgs/basic/cache/forming", tmp_dir).replace("//", "/");
    let forming = read_file_abs_adr(&path_2_msg_forming);
    if op == cache_state::ready{if state == cache_state::ready{state = cache_state::taken; return (cache0.to_vec(), cache_state::ready);}
     let mut lst_len = crate::globs18::strn_2_usize(crate::globs18::len_of_front_list_wc()).unwrap(); //ln_of_found_files_cacheless(usize::MAX).1;
    if lst_len == 0{return (vec!("".to_string()), cache_state::no_data_to_add);}
    count += 1;
    state = cache_state::forming;
    //if count > 5{println!("{:?}", cache);}
    let seg_num = (indx + 1) / seg_size;
  //  let msg = format!("seg# {seg_num}"); popup_msg(&msg);
    let upto = seg_size + indx;
    crate::save_file_abs_adr0(name.to_string(), path_2_msg_forming.clone());
    for i in indx..upto{
        let rec = ln_of_found_files_cacheless(i);
        if rec.1 == lst_len{break;}
       // cache.entry(name.clone()).and_modify(|e|{e.push(rec.0)});
        cache0.push(rec.0);
        //println!("{}", cache0[i]);
     }
     state = cache_state::ready;
     crate::rm_file(&path_2_msg_forming);
     /*if get_num_page(-577714581011) == 452{
        popup_msg("452");
        let cache_iter = cache0.clone();
        for v in cache_iter{
            println!("{v}")
        }
        getkey();
     }
    */
    // crate::popup_msg(&std::mem::size_of_val(&cache).to_string());
     return (cache0.to_vec(), cache_state::ready);
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