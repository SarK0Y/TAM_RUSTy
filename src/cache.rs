use std::io::BufRead;
use std::sync::mpsc::channel;
use crate::{bkp_tmp_dir, cache_t, custom_traits::{STRN, helpful_math_ops}, errMsg0, get_num_cols, get_num_page, 
globs18::{seg_size, take_list_adr}, i64_2_usize,
 ln_of_found_files, ln_of_found_files_cacheless, popup_msg, read_front_list, rm_file, run_cmd_out, save_file, save_file_append, save_file_append_abs_adr, 
 where_is_last_pg};
 use once_cell::sync::Lazy;
pub(crate) fn cached_ln_of_found_files(get_indx: usize) -> (String, usize){
     let stopCode = crate::getStop_code__!();
     let last_pg = where_is_last_pg();
     let num_pg = get_indx / seg_size();//get_num_page(27786521);
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
    let cur_cache = crate::cpy_str(&is_cached);
    let prev_cache = is_cached.replace(&cur_pg, &prev_pg);
    if !crate::Path::new(&prev_cache).exists(){
        //let num_pg = num_pg0;
        //let cols = cols;
        //let rows = rows;
        let found_files = found_files.clone();
        std::thread::spawn(move||{
            cache_pg_prev(get_indx, prev_cache, found_files, cols, rows);
        }).join();
    }
    
        if !crate::Path::new(&is_cached).exists(){
        let mut recs_on_pg1 = seg_size();
        let mut recs_on_pg2 = recs_on_pg1*2;
        let file = match crate::File::open(&found_files){
            Ok(f) => f,
            _ => return ln_of_found_files_cacheless(get_indx)
        };
        let reader = crate::BufReader::new(file);
        let mut len = 0usize;
        let mut ret = (String::new(), 0usize);
        let mut ret0 = (String::new(), 0usize);
        let get_indx_offset = get_indx % recs_on_pg1;
        for (indx, line) in reader.lines().enumerate() {
            let line0 = line.unwrap_or("".strn() ).as_mut().strn();
            if indx >= get_indx && recs_on_pg2 > 0{
                if indx == get_indx_offset{
                ret = (crate::cpy_str(&line0), indx);}
                let proper_line = format!("{}\n", line0.clone());
                if recs_on_pg2 > recs_on_pg1{
                    save_file_append_abs_adr(proper_line, cur_cache.clone());
                }else {save_file_append_abs_adr(proper_line, nxt_cache.clone());}
            recs_on_pg2 -= 1;
        }   
         len = indx;
        }
        if ret == ret0 {return ("no str gotten".to_string(), len);}
    }
        let get_indx_offset = get_indx % seg_size();
        let base_indx: usize = get_indx - get_indx_offset;
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
        if indx == get_indx{let ret = String::from(&line.unwrap()); let ret = crate::rec_from_patch(&ret).unwrap_or(ret); return (ret, indx + base_indx);}
        len = indx;
    }
    let mut ret = ln_of_found_files_cacheless(get_indx); ret.0 = crate::rec_from_patch(&ret.0).unwrap_or(ret.0); len = ret.1;
    return (ret.0, len);
}
pub(crate) fn cache_pg(get_indx: usize, cached_list: String, found_files: String, cols: i64, rows: i64) {
     //save_file_append(format!("{}\n", cached_list.to_string()), "cached_list.dbg".to_string());
        if crate::Path::new(&cached_list).exists(){return}
        //save_file_append(format!("{}\n", get_indx.to_string()), "cache_pg.indx".to_string());
        let mut recs_on_pg = seg_size();
        let file = match crate::File::open(&found_files){
            Ok(f) => f,
            _ => return
        };
        let reader = crate::BufReader::new(file);
        let mut len = 0usize;
        let mut ret = (String::new(), 0usize);
        let mut ret0 = (String::new(), 0usize);
        for (indx, line) in reader.lines().enumerate() {
            let line0 = line.unwrap().as_mut().to_string();
            if indx >= get_indx && recs_on_pg > 0{
                let proper_line = format!("{}\n", line0.clone());
            save_file_append_abs_adr(proper_line, cached_list.clone());
            recs_on_pg -= 1;
        }   
        }
}
pub(crate) fn cache_pg_prev(get_indx: usize, cached_list: String, found_files: String, cols: i64, rows: i64) {
     //save_file_append(format!("{}\n", cached_list.to_string()), "cached_list.dbg".to_string());
        if crate::Path::new(&cached_list).exists(){return}
        //save_file_append(format!("{}\n", get_indx.to_string()), "cache_pg.indx".to_string());
        let mut recs_on_pg = seg_size();
        let align_indx = (get_indx / recs_on_pg) * recs_on_pg;
        let file = match crate::File::open(&found_files){
            Ok(f) => f,
            _ => return
        };
        let msg = format!("align {align_indx}, get_indx {get_indx}");
        let reader = crate::BufReader::new(file);
        let mut len = 0usize;
        let mut ret = (String::new(), 0usize);
        let mut ret0 = (String::new(), 0usize);
        for (indx, line) in reader.lines().enumerate() {
            let line0 = line.unwrap_or("no str".strn() ).as_mut().to_string();
            if align_indx > indx &&  align_indx - indx == recs_on_pg && recs_on_pg > 0{
                let proper_line = format!("{}\n", line0.clone());
            save_file_append_abs_adr(proper_line, cached_list.clone());
            recs_on_pg -= 1;
        }   
        }
}
pub(crate) fn clean_cache(msg: &str){
    let tmp_dir = bkp_tmp_dir(None, false);
    let mk_msg_dir = format!("{tmp_dir}/msgs/basic/cache");
    let clean_cache = format!("{tmp_dir}/cache/{msg}*");
    let msg_of_clean_cache = format!("{mk_msg_dir}/clean");
    let cmd = format!("mkdir -p {mk_msg_dir};rm -f {clean_cache};echo '{msg}' > {msg_of_clean_cache}");
    std::thread::spawn(||{run_cmd_out(cmd);});
}
pub(crate) fn clean_all_cache(){
    let tmp_dir = bkp_tmp_dir(None, false);
    let clean_cache = format!("{tmp_dir}/cache/*");
    let cmd = format!("rm -f {clean_cache}");
    std::thread::spawn(||{run_cmd_out(cmd);});
}
pub(crate) fn wait_4_empty_cache() -> bool{
    let cache_dir = take_list_adr("cache/");
    let files = format!("{}*", cache_dir);
    let cmd = format!("rm -f {files}");
    let mut dir_arr = [true;2];
    let (send, recv) = channel::<bool>();
    let mut i = 0usize;
    let mut ret = || -> std::result::Result<std::fs::DirEntry, std::io::Error>{return std::fs::read_dir("/").unwrap().next().unwrap();};
    let mut status = ||{dir_arr[i] = false; i += 1; i = i ^ 2; send.send(dir_arr[0]);};
    loop {
       let dir: std::fs::DirEntry = match std::path::PathBuf::from(cache_dir.clone()).read_dir().expect("wait_4_empty_cache can't read dir").next(){
            Some(i0) => i0,
            _ => {status();let ret0 = recv.recv().unwrap(); println!("{}", ret0); if !ret0 {return dir_arr[0];}; ret()}
        }.unwrap();
        run_cmd_out(cmd.clone());
        println!("{:?}", dir);
       // let dir_arr0 = dir_arr;
        //if !dir_arr0[0]{return true;}
    }
}
pub(crate) fn upd_fast_cache(cache: &mut cache_t){
    *cache = std::collections::HashMap::new();
}
pub(crate) fn history_buffer_size(new_buf_size: Option < usize >) -> usize {
    static mut buf_sz: usize = 0;
    if new_buf_size == None {history_buffer(None, usize::MAX, false); return unsafe { buf_sz } }
    unsafe { buf_sz  = new_buf_size.unwrap(); buf_sz}

}
pub(crate) fn history_buffer(item: Option<String>, indx: usize, no_ret: bool) -> Option < String >{
    static mut buf: Lazy<Vec <String> > = Lazy::new(||{Vec::with_capacity(20) });//Lazy< AllocRingBuffer <String> > = Lazy::new(||{AllocRingBuffer::new(20)});
    static mut order: Lazy< Vec <usize> > = Lazy::new(||{vec!(0) });
    if no_ret{ return None;}
    if indx == usize::MAX {history_buffer_size(Some(unsafe { buf.len() } ) ); return None }
    if unsafe{ buf.len() } == indx && item.is_none() { return None }
    if item == None && unsafe{ buf.len() } > indx {
        let indx0: usize = if indx < unsafe {order.len() }{unsafe { order[indx]} } else {
            if unsafe { order.len() == 1 } {unsafe { order [0] } } else{ return None } };
        let ret = unsafe{ buf[indx0].clone()  };
        return Some(ret);}
    let item0 = item.clone(); let item2 = item.clone();
    let item1 = item.clone().unwrap_or("".strn() );
    for i in 0..unsafe { buf.len() }{
        if unsafe { buf[i].strn() } == item1 {
         if unsafe{ buf.len() } > 1 {
            let indx = i;
            let mut vecc: Vec<usize> = Vec::with_capacity(20);
            vecc.push(0); 
            for j in 1..unsafe{ buf.len() } {
                if unsafe{ order.len() > j } && j != indx{
                    vecc.push(unsafe{ order[j] });
                }
            }
            if indx < unsafe { order.len() } { vecc[0] =unsafe { order[indx]}; }
            else {vecc[0] = indx}
            unsafe { *order = vecc.clone()};
        }
            return item2 }
    }
    
    let mut vecc: Vec<usize> = Vec::with_capacity(20);
    let mut len: usize = unsafe { buf.len() }; let len_ord = unsafe { order.len() };
    if len == 20{ unsafe { order.clear(); order.push(0);}
        let mut ringbuf: Vec<String> = Vec::with_capacity(20);
        unsafe { ringbuf.push(match item {Some(it) => it, _ => return None} ); buf.pop();
         };
         len = unsafe { buf.len() };
         for y in 0..len{
            unsafe{ ringbuf.push(buf [y].clone() ); order.push(y.clone().inc() );}
         }unsafe{ *buf = ringbuf};
         return Some(unsafe { buf[0].clone() });
    }
    unsafe { buf.push(match item {Some(it) => it, _ => return None} ) }; 
    for i in 0..len{
         if unsafe { order.len() <= i} &&  unsafe { order.len() > 0} {
            let ord = unsafe { order[i].inc() };
            vecc.push(ord);
         } else {
            unsafe { order.push(i) };
            vecc.push(i);
         }
    }
    unsafe { *order = vecc};
    item0
}
//fn