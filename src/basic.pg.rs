use cli_table::{CellStruct, print_stdout, Table, Style};
use colored::Colorize;
use num_traits::ToPrimitive;
use std::collections::{HashMap, hash_map::Entry};
use once_cell::sync::{Lazy, OnceCell};
use std::ptr::addr_of_mut;
use crate::{cache, cache_state, cache_t, cached_data, checkArg, entry_cache_t, get_arg_in_cmd, get_num_files, get_num_page, getkey, globs18::{get_item_from_front_list, seg_size, strn_2_u64, strn_2_usize}, i64_2_usize, ln_of_found_files_cacheless, patch_len, popup_msg, read_file, read_file_abs_adr, read_front_list, rec_from_patch, rm_file, save_file_abs_adr, set_num_page, swtch::check_symlink, update18::fix_screen_count};
use crate::custom_traits::{STRN, helpful_math_ops};
//use super::extctrl::*;
impl super::basic{
   pub fn build_page_(&mut self, ps: &mut crate::_page_struct){
    let func_id = crate::func_id18::build_page_;
    let mut try_entry = 0usize;
    let mut num_files = crate::get_num_files(func_id);
    let dbg_point = self.read_file("stop_point").trim_end().to_string();
    #[cfg(feature="in_dbg")]
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
    let mut num_page; num_page = crate::calc_num_files_up2_cur_pg(); // if ps.num_page != i64::MAX{num_page = ps.num_page;}else{num_page = crate::get_num_page(func_id);}
    let mut num_cols; if ps.num_cols != i64::MAX{num_cols = ps.num_cols;}else{num_cols = crate::get_num_cols(func_id);}
    let mut num_rows; if ps.num_rows != i64::MAX{num_rows = ps.num_rows;}else{num_rows = crate::get_num_rows(func_id);}
    if ps.col_width != i64::MAX{crate::set_col_width(ps.col_width, func_id);}
    let num_items_on_pages = num_cols * num_rows; let stopCode: String = crate::getStop_code__!();
    let mut filename_str: String; let mut time_to_stop = false;
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
             if num_files == indx || "front list is empty" == res || "no str gotten" == res.to_lowercase(){
                time_to_stop = true;
            }
            // println!("build_page - probe 0");
            let full_path = res;
            //no_dup_indx = indx;
            display_indx = indx;
            if !crate::C!(crate::swtch::local_indx(false)){display_indx = indx - num_page;}
            let err_ret = std::ffi::OsString::from("");
            let mut err_path = || -> &std::ffi::OsString{return &err_ret};
            //println!("build_page - probe 1");
            let mut filename = crate::Path::new(&full_path);
            let filename_str0 = || -> String{
                    let front_list = read_front_list();
                 if front_list != "history"{
                   return String::from(match filename.file_name(){
                    Some(f) => f,
                    None => err_path(),
                }.to_str().unwrap()).as_str().strn();
            } else {return filename.as_os_str().to_str().unwrap().strn()};
            };
            if filename.as_os_str().to_str() == None{filename = crate::Path::new("")}
            if crate::globs18::eq_str(stopCode.as_str(), filename.as_os_str().to_str().unwrap()) == 0 && stopCode.len() == filename.as_os_str().to_str().unwrap().len() {println!("{}", "caught".bold().green()); 
             time_to_stop = true; break;}
            if crate::dirty!(){
               println!("cmp_str res {}", crate::globs18::eq_str(stopCode.as_str(), filename.as_os_str().to_str().unwrap()));
               println!("stop code {}, len {}; str {}, len {}", stopCode, stopCode.as_str().len(), filename.as_os_str().to_str().unwrap(), filename.as_os_str().to_str().unwrap().len());
               println!("{:?}", filename.file_name());
            }
            let mut fixed_filename: String = filename_str0().to_string();
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
    let failed = ("no such segment was cached".to_string(), cached_data::no_list);
    match cache.entry(key.to_string()){
        Entry::Occupied(entry) => {let key= entry.get().contains_key(&seg_num); if key {
            let len = entry.get()[&seg_num].len(); if len <= offset {return failed;}
            if patch_len() > 0{
               let rec = match rec_from_patch(&entry.get()[&seg_num][offset].clone()){
                Some(val) => val,
                None => entry.get()[&seg_num][offset].clone()
               }; return  (rec, cached_data::all_ok);
            } return (entry.get()[&seg_num][offset].clone(), cached_data::all_ok);
        }else {return failed;}},
        Entry::Vacant(entry) => {return failed;}
    }
}
pub(crate) fn pg_rec_from_front_list(&mut self, indx: i64, fixed_indx: bool) -> String{
    static mut good_count: u64 = 0;
    let proper_indx = /*(i64_2_usize(indx), indx);*/crate::get_proper_indx(indx, fixed_indx);
    if proper_indx.0 == usize::MAX{return "front list is empty".to_string()}
    let front_lst = read_front_list();
     let adr_of_msg_clean = format!("{}/msgs/basic/cache/clean", self.tmp_dir).replace("//", "/");
#[cfg(feature="in_dbg")]
     if read_file("break").trim_end().to_string() == "001"{
        println!("break 001");
     }
#[cfg(feature="in_dbg")]
     if read_file("panic") == "yes"{
        panic!("wtffffff");
     }
     let clean = read_file_abs_adr(&adr_of_msg_clean);
    if clean.len() > 0{
        self.cache.remove(&clean);
        rm_file(&adr_of_msg_clean);
    // self.cache.remove_entry(&clean);
    }
    let rec: (String, cached_data) = self.rec_from_cache(&front_lst, i64_2_usize(indx));
    if rec.1 == cached_data::all_ok{crate::C!(crate::logs(&self.cache.len().to_string(), "cache.len")); unsafe{good_count +=1}; return rec.0;}
    //popup_msg("msg");
     let front_lst0 = front_lst.clone(); let front_lst1 = front_lst0.clone(); let front_lst2 = front_lst0.clone();
     let mut cache_entry: entry_cache_t = HashMap::new();
     let indx = i64_2_usize(indx);//proper_indx.0.clone();
     let seg_num = indx / self.seg_size;
     let no_offset = indx % self.seg_size; let no_offset = indx - no_offset;
    if rec.1 == cached_data::no_rec{
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
            let vecc = ret.0.unwrap().clone(); let vecc1 = vecc.clone();
            cache_entry.insert(indx / self.seg_size, vecc);
            match self.cache.entry(front_lst2){
            Entry::Occupied(mut en) => {en.get_mut().insert(seg_num, vecc1); },
            Entry::Vacant(en) => {en.insert(cache_entry);}
         }
        }
        return get_item_from_front_list(proper_indx.1, false);;
    }
    else {
        //fix_screen_count(1);
        let front_lst0 = front_lst.clone(); let tmp_dir1 = self.tmp_dir.clone(); let cache_window = self.cache_window.clone();
        let no_offset = indx % self.seg_size; let no_offset = indx - no_offset;
        let ret = crate::C!(crate::basic::mk_fast_cache(&self.tmp_dir, indx, &front_lst, cache_state::ready));
        if ret.1 == cache_state::ready{
           // popup_msg("msg1"); // hits only here 
           let vecc = ret.0.unwrap().clone(); let vecc1 = vecc.clone();
            cache_entry.insert(indx / self.seg_size, vecc1);
            match self.cache.entry(front_lst1){
            Entry::Occupied(mut en) => {en.get_mut().insert(seg_num, vecc); return get_item_from_front_list(crate::usize_2_i64(indx), fixed_indx);},
            Entry::Vacant(en) => {en.insert(cache_entry); return get_item_from_front_list(proper_indx.1, false);}
         }
    }
    //if !list_id.1{set_ask_user("Can't access to Front list", -1); return "!!no¡".to_string()}
    crate::C!(crate::logs(&good_count.to_string(), "bad_count"));
    return crate::C!(crate::globs18::lists("", crate::globs18::FRONT_, proper_indx.0, crate::globs18::GET))
}}
pub(crate) unsafe fn mk_fast_cache<'a>(tmp_dir: &'a String, indx: usize, name: &'a String, op: cache_state) -> (Option<Vec<String>>, cache_state){
    //static mut cache: Lazy<Vec<String>> = Lazy::new(||{Vec::new()});
    static mut cache: OnceCell<Vec<String>> = OnceCell::new();
    static mut count: u64 = 0;
    static mut state: cache_state = cache_state::empty;
    static mut seg_size: usize = 150;
    static mut fst_run: bool = false;
    let mut fixed_indx = false;
    match op {
        cache_state::ready  => {fixed_indx = true},
        cache_state::ready0 => {fixed_indx = false}
        _ => {}
    }
    //std::thread::sleep(std::time::Duration::from_millis(1));
    if !fst_run{
        fst_run = true;
        if checkArg("-cache-seg-size"){
            let seg_size_new = String::from_iter(get_arg_in_cmd("-cache-seg-size").s).trim_end_matches('\0').to_string();
            let ret = strn_2_usize(seg_size_new);
            if ret != None{seg_size = ret.unwrap()}
        }
        let mut vec0 = Vec::with_capacity(10000);
        cache.set(vec0);
    }
    if state == cache_state::forming {return (None, cache_state::forming);}
    if state == cache_state::taken || (cache.get() != None && cache.get().unwrap().len() > 0){
        cache.take(); // = OnceCell::new(); 
        let mut vec0 = Vec::with_capacity(10000);
        cache.set(vec0);
        state = cache_state::empty
    }
        let cache0 =cache.get_mut();
        if cache0 == None{return (None, cache_state::cache_seg_corrupted);}
        let mut cache0 = cache0.expect("cache in mk_fast_cache");
    //return (empty_entry, cache_state::empty);    
    let path_2_msg_forming = format!("{}/msgs/basic/cache/forming", tmp_dir).replace("//", "/");
    let forming = read_file_abs_adr(&path_2_msg_forming);
    if op == cache_state::ready{if state == cache_state::ready{state = cache_state::taken; return (Some(cache0.to_vec()), cache_state::ready);}
     let mut lst_len = match crate::globs18::strn_2_usize(crate::globs18::len_of_front_list_wc()){Some(i) => i, _ => 0};
     
    if lst_len == 0{return (None, cache_state::no_data_to_add);}
    count += 1;
    //if count > 5{println!("{:?}", cache);}
    let seg_num = indx / seg_size;
  //  let msg = format!("seg# {seg_num}"); popup_msg(&msg);
    let mut indx = indx;
    let offset = indx % seg_size; indx -= offset; 
    let upto = seg_size + indx;
    if state == cache_state::forming {return (None, cache_state::forming);}
    let prev_state = state.clone();
    state = cache_state::forming;
    crate::save_file_abs_adr0(name.to_string(), path_2_msg_forming.clone());
    if cache0.len() > 0{popup_msg("bad cache"); cache0.clear(); popup_msg(&cache0.len().to_string())}
    for i in indx..upto{
        let rec =  get_item_from_front_list(crate::usize_2_i64(i), false);//ln_of_found_files_cacheless(i);
        if i == lst_len{break;}
        if rec == "no str gotten"{continue}
       // cache.entry(name.clone()).and_modify(|e|{e.push(rec.0)});
        cache0.push(rec);
        //println!("{}", cache0[i]);
     }
     if cache0.len() > 150{panic!("cannot drop cache seg prev {:?} cur {:?} len {}", prev_state, state, cache0.len())};
     state = cache_state::ready;
     crate::rm_file(&path_2_msg_forming);
     /*if get_num_page(-577714581011) == 0{
        let cache_iter = cache0.clone();
        let mut ii = 0;
        for v in cache_iter{
            println!("{ii}: {v}");
            ii +=1
        }
        println!("cache size: {}", cache0.len());
        getkey();
     }*/
    // crate::popup_msg(&std::mem::size_of_val(&cache).to_string());
     return (Some(cache0.to_vec()), cache_state::ready);
    }
return (None, cache_state::empty);}
pub(crate) fn read_file(&self, name: &str) -> String{
    let path = format!("{}/{name}", self.tmp_dir).replace("//", "/");
    read_file_abs_adr(&path)
}
pub(crate) fn read_cache_msg(&self) -> String{
    self.read_file("msg/basic/cache/clean")
}
}