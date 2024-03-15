use std::io::BufRead;
use crate::{get_num_page, get_num_cols, read_front_list, globs18::take_list_adr, save_file_append, i64_2_usize, save_file, where_is_last_pg, save_file_append_abs_adr, run_cmd_out};
pub(crate) fn cached_ln_of_found_files(get_indx: usize) -> (String, usize){
     let stopCode = crate::getStop_code__!();
     let last_pg = where_is_last_pg();
     let num_pg = get_num_page(27786521);
     let cols = get_num_cols(27786521);
     let rows = crate::get_num_rows(27786521);
     let front_list = read_front_list();
     let tmp_dir = crate::C!(crate::ps18::page_struct("", crate::ps18::TMP_DIR_, -1).str_);
        let found_files = format!("{tmp_dir}/found_files");
        let cached_list = format!("cache/{front_list}.{num_pg}");
        let is_cached = take_list_adr(&cached_list);
    let num_pg0 = if num_pg == 0{0}else{num_pg -1};
    let prev_pg =format!(".{}", num_pg0);
    let nxt_pg =format!(".{}", num_pg +1);
    let cur_pg =format!(".{}", num_pg);
    let nxt_cache = is_cached.replace(&cur_pg, &nxt_pg);
    if !crate::Path::new(&nxt_cache).exists() && num_pg + 1 < last_pg{
        let num_pg = num_pg +1;
        let cols = cols;
        let rows = rows;
        let found_files = found_files.clone();
        std::thread::spawn(move||{
            let get_index = num_pg * cols * rows;
            cache_pg(get_indx, nxt_cache, found_files, cols, rows);
        });
    }
    let prev_cache = is_cached.replace(&cur_pg, &prev_pg);
    if !crate::Path::new(&prev_cache).exists(){
        let num_pg = num_pg0;
        let cols = cols;
        let rows = rows;
        let found_files = found_files.clone();
        std::thread::spawn(move||{
            let get_index = num_pg * cols * rows;
            cache_pg(get_indx, prev_cache, found_files, cols, rows);
        });
    }
    
        if !crate::Path::new(&is_cached).exists(){
        let mut recs_on_pg = cols * rows;
        let file = crate::File::open(&found_files).unwrap();
        let reader = crate::BufReader::new(file);
        let mut len = 0usize;
        let mut ret = (String::new(), 0usize);
        let mut ret0 = (String::new(), 0usize);
        for (indx, line) in reader.lines().enumerate() {
            let line0 = line.unwrap().as_mut().to_string();
            if indx >= get_indx && recs_on_pg > 0{
                if indx == get_indx{
                ret = (crate::cpy_str(&line0), indx);}
                let proper_line = format!("{}\n", line0.clone());
            save_file_append(proper_line, cached_list.clone());
            recs_on_pg -= 1;
        }   
         len = indx;
        }
        if ret == ret0 {return ("no str gotten".to_string(), len);}
    }
        let base_indx: usize = i64_2_usize(num_pg * cols * rows);
        let mut get_indx = get_indx;
        let mut len = 0usize;
        if get_indx < base_indx {return ("no str gotten".to_string(), len);}
        get_indx -= base_indx;
        let dbg = |e: std::io::Error| -> String{save_file(format!("{}\n{:?}", is_cached, e), "dbg_cached".to_string()); return String::new()};
        let cached_list = is_cached.clone();
        let file = match crate::File::open(cached_list){
            Ok(f) => f,
            Err(e) => return (dbg(e), 0)
        };
        let reader = crate::BufReader::new(file);
    for (indx, line) in reader.lines().enumerate() {
        if indx == get_indx{return (line.unwrap(), indx + base_indx);}
        len = indx;
    }
    return ("no str gotten".to_string(), len);
}
pub(crate) fn cache_pg(get_indx: usize, cached_list: String, found_files: String, cols: i64, rows: i64) {
     save_file_append(format!("{}\n", cached_list.to_string()), "cached_list.dbg".to_string());
        if crate::Path::new(&cached_list).exists(){return}
        save_file_append(format!("{}\n", get_indx.to_string()), "cache_pg.indx".to_string());
        let mut recs_on_pg = cols * rows;
        let file = crate::File::open(&found_files).unwrap();
        let reader = crate::BufReader::new(file);
        let mut len = 0usize;
        let mut ret = (String::new(), 0usize);
        let mut ret0 = (String::new(), 0usize);
        for (indx, line) in reader.lines().enumerate() {
            let line0 = line.unwrap().as_mut().to_string();
            if indx >= get_indx && recs_on_pg > 0{
                let proper_line = format!("{}\n", line0.clone());
            save_file_append_abs_adr(proper_line, cached_list.clone());
            save_file("content".to_string(), "cache_pg".to_string());
            recs_on_pg -= 1;
        }   
        }
}
pub(crate) fn clean_cache(){
    let cmd = format!("rm -f {}", take_list_adr("cache/*"));
    run_cmd_out(cmd);
}