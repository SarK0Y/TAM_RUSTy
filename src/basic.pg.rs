use cli_table::{CellStruct, print_stdout, Table, Style};
use colored::Colorize;
use num_traits::ToPrimitive;
use std::collections::{HashMap, hash_map::Entry};
use once_cell::sync::Lazy;
use std::ptr::addr_of_mut;
use crate::{read_file_abs_adr, cached_data, get_num_files, ln_of_found_files_cacheless, cache_state, cache};
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
            //  if num_files == indx || "front list is empty" != res{time_to_stop = true;}
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
pub(crate) fn pg_rec_to_cache(cache: &mut HashMap<String, Vec<String>>, key: &String, val: &String){
    match cache.entry(key.to_string()){
        Entry::Occupied(mut entry) => {entry.get_mut().push(val.to_string());},
        Entry::Vacant(entry) => {entry.insert(vec!(key.to_string()));}
    }
}
pub(crate) fn pg_0_cache(cache: &mut HashMap<String, Vec<String>>, key: &String){
    match cache.entry(key.to_string()){
        Entry::Occupied(mut entry) => {entry.remove();},
        Entry::Vacant(entry) => {}
    }
}
pub(crate) fn pg_rec_from_cache(cache: &mut HashMap<String, Vec<String>>, key: &String, indx: usize) -> (String, cached_data){
    match cache.entry(key.to_string()){
        Entry::Occupied(entry) => {let  mut status = cached_data::no_rec; 
            let ret = if entry.get().len() > indx{status = cached_data::all_ok; entry.get()[indx].to_string()}else{"no such rec".to_string()}; return (ret, status);},
        Entry::Vacant(entry) => {return ("no such list was cached".to_string(), cached_data::no_list)}
    }
}
pub(crate) fn pg_rec_from_front_list(&mut self, indx: i64, fixed_indx: bool) -> String{
    let proper_indx = crate::get_proper_indx(indx, fixed_indx);
    if proper_indx.0 == usize::MAX{return "front list is empty".to_string()}
    let front_lst = self.read_file("front_list");
    let rec: (String, cached_data) = self.rec_from_cache(&front_lst, crate::i64_2_usize(indx));
    if rec.1 == cached_data::all_ok{return rec.0;}
    if rec.1 == cached_data::no_rec{
        let rec = crate::C!(crate::globs18::lists("", crate::globs18::FRONT_, proper_indx.0, crate::globs18::GET));
        self.rec_to_cache(front_lst, rec.clone());
        return rec;
    }
    else {
        let msg = self.read_file("msgs/basic/cache/clean");
        let msg0 = msg.clone(); let front_lst0 = front_lst.clone(); 
        std::thread::spawn(move||{
           crate::C!(crate::basic::mk_fast_cache(&front_lst0, cache_state::ready, &msg0));
        });
        let ret = crate::C!(crate::basic::mk_fast_cache(&front_lst, cache_state::ready, &msg));
        if ret.1 == cache_state::ready{self.cache = ret.0}
    }
    //if !list_id.1{set_ask_user("Can't access to Front list", -1); return "!!no¡".to_string()}
    return crate::C!(crate::globs18::lists("", crate::globs18::FRONT_, proper_indx.0, crate::globs18::GET))
}
pub(crate) unsafe fn mk_fast_cache(name: &String, op: cache_state, msg: &String) -> (HashMap<String, Vec<String>>, cache_state){
    static mut cache: Lazy<HashMap<String, Vec<String>>> = Lazy::new(||{HashMap::new()});
    static mut state: cache_state = cache_state::empty;
    let empty_cache: HashMap<String, Vec<String>> = HashMap::new();
    if msg == name{cache.remove(name); state = cache_state::empty}
    if state == cache_state::forming{return (empty_cache, cache_state::empty);}
    if op == cache_state::ready{if state == cache_state::ready{return (cache.clone(), cache_state::ready);}
     let mut lst_len = ln_of_found_files_cacheless(usize::MAX).1;
    if lst_len == 0{return (cache.clone(), cache_state::no_data_to_add);}
    state = cache_state::forming;
    let zero_rec = ln_of_found_files_cacheless(0).0;
    cache.insert(name.clone(), vec!(zero_rec));
    lst_len -= 1;
    for i in 1..lst_len{
        let rec = ln_of_found_files_cacheless(i).0;
        cache.entry(name.clone()).and_modify(|e|{e.push(rec)});
        //cache.entry(name.clone()).or_default().push(rec);
     }
     return (cache.clone(), cache_state::ready);
    }
return (empty_cache, cache_state::empty);}
pub(crate) fn read_file(&self, name: &str) -> String{
    let path = format!("{}/{name}", self.tmp_dir).replace("//", "/");
    read_file_abs_adr(&path)
}
pub(crate) fn read_cache_msg(&self) -> String{
    self.read_file("msg/basic/cache/clean")
}
}