use std::io::BufRead;
use crate::{get_num_page, get_num_cols, read_front_list, globs18::take_list_adr, save_file_append, i64_2_usize, save_file};
pub(crate) fn cached_ln_of_found_files(get_indx: usize) -> (String, usize){
     let stopCode = crate::getStop_code__!();
     let num_pg = get_num_page(27786521);
     let cols = get_num_cols(27786521);
     let rows = crate::get_num_rows(27786521);
     let front_list = read_front_list();
     let tmp_dir = crate::C!(crate::ps18::page_struct("", crate::ps18::TMP_DIR_, -1).str_);
        let filename = format!("{tmp_dir}/found_files");
        let cached_list = format!("cache/{front_list}.{num_pg}");
        let is_cached = take_list_adr(&cached_list);
        if !crate::Path::new(&is_cached).exists(){
        let mut recs_on_pg = cols * rows;
        let file = crate::File::open(&filename).unwrap();
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
        let get_indx = get_indx - base_indx;
        let dbg = |e: std::io::Error| -> String{save_file(format!("{}\n{:?}", is_cached, e), "dbg_cached".to_string()); return String::new()};
        let cached_list = is_cached.clone();
        let file = match crate::File::open(cached_list){
            Ok(f) => f,
            Err(e) => return (dbg(e), 0)
        };
        let reader = crate::BufReader::new(file);
        let mut len = 0usize;
    for (indx, line) in reader.lines().enumerate() {
        if indx == get_indx{return (line.unwrap(), indx + base_indx);}
        len = indx;
    }
    return ("no str gotten".to_string(), len);
}
