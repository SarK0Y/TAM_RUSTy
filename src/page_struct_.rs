mod exts;
use exts::page_struct_uses;

use crate::{bkp_tmp_dir, complete_path, cpy_str, file_prnt, func_id18, get_path_from_strn, 
  globs18::{ins_patch_to_string, len_of_front_list, take_list_adr}, helpful_math_ops, i64_2_usize, read_front_list, read_front_list_but_ls, 
  read_proper_num_pg, rewrite_user_written_path, save_file, set_proper_num_pg, 
  swtch::{set_user_written_path_from_prnt, set_user_written_path_from_strn}, custom_traits::STRN};
self::page_struct_uses!();
pub const STOP_CODE_: i64 = 1;
pub const KONSOLE_TITLE_: i64 = 2;
pub const LEFT_SHIFT_4_CUR_: i64 = 3;
pub const CUR_CUR_POS_: i64 = 4;
pub const NUM_PAGE_: i64 = 5;
pub const NUM_COLS_: i64 = 6;
pub const COL_WIDTH_: i64 = 7;
pub const NUM_ROWS_: i64 = 8;
pub const NUM_SPACES_: i64 = 9;
pub const NUM_FILES_: i64 = 10;
pub const COUNT_PAGES_: i64 = 11;
pub const NEWS_BAR_: i64 = 12;
pub const ASK_USER_: i64 = 13;
pub const dontDelFromTableJustMark_: i64 = 14;
pub const RUNNING_: i64 = 15;
pub const VIEWER_: i64 = 16;
pub const MODE2RUN_: i64 = 17;
pub const PRNT_: i64 = 18;
pub const PROMPT_: i64 = 134;
pub const FULL_PATH_: i64 = 19;
pub const MAINPATH_: i64 = 20;
pub const FOUND_FILES_: i64 = 21;
pub const TMP_DIR_: i64 = 22;
pub const __BKSP: i64 = 23; // caller's id
pub const __DEL: i64 = 24;
pub const __INS: i64 = 1725;
pub const NUM_OF_VIEWERS: i64 = 26;
pub struct page_struct_ret{
   pub str_: String,
   pub int: i64
}
pub struct shift_cur_struct{
  pub shift: usize,
  pub str__: String
}
//#[derive(Clone, Copy)]
pub struct child2run{
   pub running: i64,
   pub viewer: i64,
   pub mode2run: i64, // 0 == internal, 1 == extrn
   pub full_path: String
}
//#[derive(Clone, Copy)]
pub struct _page_struct{
   pub stop_code: String,
   pub prnt: String,
   pub prompt: String,
   pub konsole_title: String,
   pub left_shift_4_cur: i64,
   pub cur_cur_pos: i64,
   pub num_page: i64,
   pub num_cols: i64,
   pub col_width: i64,
   pub num_rows: i64,
   pub num_spaces: i64,
   pub num_files: i64,
   pub count_pages: i64,
   pub news_bar: String,
   pub ask_User: String,
   pub c2r: child2run
}
pub(crate) fn cpy_page_struct(ps: &mut _page_struct) -> _page_struct{
   let func_id = crate::func_id18::init_page_struct_;
   let stop_code = unsafe {page_struct("", STOP_CODE_, func_id.to_i64().unwrap()).str_};;
   let konsole_title = unsafe {page_struct("", KONSOLE_TITLE_, func_id.to_i64().unwrap()).str_};
   let left_shift_4_cur =0i64; let cur_cur_pos = 0i64; let num_page = ps.num_page; let num_cols = ps.num_cols;
   let col_width = get_col_width(func_id); let num_rows = ps.num_rows;
    let num_spaces = 0i64; let num_files = 0i64;
   let count_pages = 0i64;
   let news_bar = unsafe {page_struct("", NEWS_BAR_, func_id.to_i64().unwrap()).str_};
   let ask_user = unsafe {page_struct("", ASK_USER_, func_id.to_i64().unwrap()).str_};
   let prnt = unsafe {page_struct("", PRNT_, func_id.to_i64().unwrap()).str_};
   let prompt = unsafe {page_struct("", PROMPT_, func_id.to_i64().unwrap()).str_};
   let full_path = unsafe {page_struct("", FULL_PATH_, func_id.to_i64().unwrap()).str_};
   let running = 0i64; let viewer = 0i64; let mode2run = 1i64;
   let c2r = child2run{running: running, viewer: viewer, mode2run: mode2run, full_path: full_path};
   let ps_new = _page_struct{stop_code: stop_code, prnt: prnt, prompt: prompt, konsole_title: konsole_title, left_shift_4_cur: left_shift_4_cur,
       cur_cur_pos: cur_cur_pos, num_page: num_page, num_cols: num_cols, col_width: col_width, num_rows: num_rows, num_spaces: num_spaces,
   num_files: num_files, count_pages: count_pages, news_bar: news_bar, ask_User: ask_user, c2r: c2r};
   ps_new
}

pub(crate) fn init_page_struct() -> _page_struct{
   let func_id = crate::func_id18::init_page_struct_;
   let stop_code = unsafe {page_struct("", STOP_CODE_, func_id.to_i64().unwrap()).str_};
   let konsole_title = unsafe {page_struct("", KONSOLE_TITLE_, func_id.to_i64().unwrap()).str_};
   let left_shift_4_cur =0i64; let cur_cur_pos = 0i64; let num_page = 0i64; let num_cols = 3i64;
   let col_width = 43i64; let num_rows = 9i64; let num_spaces = 0i64; let num_files = 0i64;
   let count_pages = 0i64;
   let news_bar = unsafe {page_struct("", NEWS_BAR_, func_id.to_i64().unwrap()).str_};
   let ask_user = unsafe {page_struct("", ASK_USER_, func_id.to_i64().unwrap()).str_};
   let prnt = unsafe {page_struct("", PRNT_, func_id.to_i64().unwrap()).str_};
   let prompt = unsafe {page_struct("", PROMPT_, func_id.to_i64().unwrap()).str_};
   let full_path = unsafe {page_struct("", FULL_PATH_, func_id.to_i64().unwrap()).str_};
   let running = 0i64; let viewer = 0i64; let mode2run = 1i64;
   let c2r = child2run{running: running, viewer: viewer, mode2run: mode2run, full_path: full_path};
   let ps_new = _page_struct{stop_code: stop_code, prnt: prnt, prompt: prompt, konsole_title: konsole_title, left_shift_4_cur: left_shift_4_cur,
       cur_cur_pos: cur_cur_pos, num_page: num_page, num_cols: num_cols, col_width: col_width, num_rows: num_rows, num_spaces: num_spaces,
   num_files: num_files, count_pages: count_pages, news_bar: news_bar, ask_User: ask_user, c2r: c2r};
   ps_new
}
pub(crate) fn INS(val: &str) -> bool{
  if val == ""{return false}
  let func_id = crate::func_id18::INS_;
  let mut cur_cur_pos = get_prnt(func_id).chars().count();
  let shift = unsafe {shift_cursor_of_prnt(2, None, func_id).shift};
  if cur_cur_pos >= shift {cur_cur_pos -= shift;}
      let mut string1 = "".to_string();
      let mut prnt0: String;
      let prnt = 'ret: {loop{
        prnt0 = get_prnt(func_id);
        if prnt0 != "none"{break 'ret prnt0.as_str()}
      } };
      string1.push_str(prnt);
      let new_string = if val.len() > 1{ ins_patch_to_string(cur_cur_pos, string1, &val.strn()) }
      else {string1.push_str(val);
      crate::globs18::ins_last_char_to_string1_from_string1(cur_cur_pos, string1)};
      //loop {
          set_prnt(&new_string, func_id);
          //set_ask_user(&new_string, func_id);
        //  if get_prnt(func_id) == new_string{break;}
      //}
      //set_prompt(&new_string, func_id);
    //  io::stdout().flush().unwrap();
    //  print!("{}", get_prnt(func_id));
   /* let err_msg = format!("cur_cur_pos as i64 {} as usize {}", cur_cur_pos as i64, cur_cur_pos).blink().red().bold(); 
     if cur_cur_pos > 1000{panic!("{}", err_msg);}*/
      cur_cur_pos += 1;
      set_cur_cur_pos(cur_cur_pos as i64, func_id);
      crate::achtung("fn ins");
      //print!("{}]", get_cur_cur_pos(func_id));
      true
}
pub(crate) fn press_DEL(val: &str) -> page_struct_ret{crate::globs18::set_valid_list_as_front(); return unsafe{page_struct("prnt", 0, __DEL)}}
pub(crate) fn press_BKSP() -> page_struct_ret{
return unsafe{page_struct("prnt", 0, __BKSP)}}
/*------------------------------------------------------------------------------------------------------------------------ */
pub(crate) fn get_mainpath(func_id: i64) -> String{return unsafe{page_struct("", MAINPATH_, func_id).str_}}
pub(crate) fn get_tmp_dir(func_id: i64) -> String{
  static mut bkp: OnceCell<String> = OnceCell::new();
  if crate::C!(bkp.get()) == None{
    let fst: String = unsafe{page_struct("", TMP_DIR_, func_id).str_};
    let ret = fst.clone();
    crate::C!(bkp.set(fst));
    return ret;
  }
  let mut bkp0: &String = &String::new();
  for i in 0..1000{
    bkp0 = crate::C!(bkp.get().expect("Can't get TMP_DIR #0"));
    if bkp0 != ""{return bkp0.to_string();}
    let bkp1 = cpy_str(&bkp_tmp_dir(None, false) );
    if bkp1 != ""{return bkp1;}
  } 
  if bkp0 == ""{
    let fst: String = unsafe{page_struct("", TMP_DIR_, func_id).str_};
    //if fst == ""{panic!("Can't get TMP_DIR #1")}
    return fst;
  }

bkp0.to_string()
  }
pub(crate) fn get_prnt(func_id: i64) -> String{return unsafe{page_struct("", PRNT_, func_id).str_}}
pub(crate) fn set_prnt(val: &str, func_id: i64) -> String{
  file_prnt(val.to_string());
  return unsafe{page_struct(val, crate::set(PRNT_), func_id).str_}}
pub(crate) fn get_ask_user(func_id: i64) -> String{return unsafe{page_struct("", ASK_USER_, func_id).str_}}
pub(crate) fn set_ask_user(val: &str, func_id: i64) -> String{return unsafe{page_struct(val, crate::set(ASK_USER_), func_id).str_}}
pub(crate) fn get_full_path(func_id: i64) -> String{return format!("{}",unsafe{page_struct("", FULL_PATH_, func_id).str_}) }
pub(crate) fn set_full_path(val: &str, func_id: i64) -> String{return unsafe{page_struct(val, crate::set(FULL_PATH_), func_id).str_}}
pub(crate) fn get_prompt(func_id: i64) -> String{return unsafe{page_struct("", PROMPT_, func_id).str_}}
pub(crate) fn set_prompt(val: &str, func_id: i64) -> String{return unsafe{page_struct(val, crate::set(PROMPT_), func_id).str_}}
/*------------------------------------------------------------------------------------------------------------------------ */
pub(crate) fn get_num_cols(func_id: i64) -> i64{return unsafe{page_struct_int(0, NUM_COLS_, func_id)}}
pub(crate) fn set_num_cols(val: i64, func_id: i64) -> i64{return unsafe{page_struct_int(val, crate::set(NUM_COLS_), func_id)}}
pub(crate) fn get_num_page(func_id: i64) -> i64{return read_proper_num_pg()}//unsafe{page_struct_int(0, NUM_PAGE_, func_id)}}
pub(crate) fn set_num_page(val: i64, func_id: i64) -> i64{
  let mut last_pg = where_is_last_pg();
  let mut proper_val = val;
  if val >= last_pg {proper_val = last_pg.dec();}
  set_proper_num_pg(proper_val);
  return unsafe{page_struct_int(proper_val, crate::set(NUM_PAGE_), func_id)}}
pub(crate) fn get_num_pages(func_id: i64) -> i64{return unsafe{page_struct_int(0, COUNT_PAGES_, func_id)}}
pub(crate) fn get_num_files(func_id: i64) ->i64{ return unsafe{page_struct_int(0, NUM_FILES_, func_id)}}
pub(crate) fn fix_num_files(func_id: i64) ->i64{
   let mut len_of_front = match i64::from_str_radix(crate::globs18::len_of_front_list().as_str(), 10){
    Ok(i) => i,
    _ => 0
  } - 1; 
   if len_of_front == -1{len_of_front = match i64::from_str_radix(crate::globs18::len_of_front_list_wc().as_str(), 10){ Ok(i) => i, _ => 0}; }
   return unsafe{page_struct_int(len_of_front, crate::set(NUM_FILES_), func_id)};}
   pub(crate) fn fix_num_files0(func_id: i64) ->i64{
   let len_of_front = match i64::from_str_radix(crate::globs18::len_of_front_list_wc().as_str(), 10){
    Ok(i) => i,
    _ => 1
   } - 1;
   return unsafe{page_struct_int(len_of_front, crate::set(NUM_FILES_), func_id)};}

pub(crate) fn set_num_files(func_id: i64) ->i64{
   let len_of_front = i64::from_str_radix(crate::globs18::len_of_front_list().as_str(), 10).unwrap();
   let mut list_len_adr = crate::globs18::take_list_adr_len(&read_front_list());
   save_file(len_of_front.to_string(), list_len_adr); 
   return unsafe{page_struct_int(len_of_front, crate::set(NUM_FILES_), func_id)};}
pub(crate) fn set_num_files0(func_id: i64, len_of_front: usize) ->i64{
   let mut list_len_adr = read_front_list();
   list_len_adr.push_str(".len");
   save_file(len_of_front.to_string(), list_len_adr); 
   return unsafe{page_struct_int(crate::usize_2_i64(len_of_front), crate::set(NUM_FILES_), func_id)};}
pub(crate) fn get_col_width(func_id: i64) -> i64{return unsafe{page_struct_int(0, COL_WIDTH_, func_id)}}
pub(crate) fn set_col_width(val: i64, func_id: i64) -> i64{return unsafe{page_struct_int(val, crate::set(COL_WIDTH_), func_id)}}
pub(crate) fn get_num_rows(func_id: i64) -> i64{return unsafe{page_struct_int(0, NUM_ROWS_, func_id)}}
pub(crate) fn set_num_rows(val: i64, func_id: i64) -> i64{return unsafe{page_struct_int(val, crate::set(NUM_ROWS_), func_id)}}
pub(crate) fn get_cur_cur_pos(func_id: i64) -> i64{return unsafe{page_struct_int(0, CUR_CUR_POS_, func_id)}}
pub(crate) fn set_cur_cur_pos(val: i64, func_id: i64) -> i64{
  return unsafe{page_struct_int(val, crate::set(CUR_CUR_POS_), func_id)}}
pub(crate) fn get_left_shift_4_cur(func_id: i64) -> i64{return unsafe{page_struct_int(0, LEFT_SHIFT_4_CUR_, func_id)}}
pub(crate) fn set_left_shift_4_cur(val: i64, func_id: i64) -> i64{return unsafe{page_struct_int(val, crate::set(LEFT_SHIFT_4_CUR_), func_id)}}
pub(crate) unsafe fn page_struct_int(val: i64, val_id: i64, caller_id: i64) -> i64{
    static mut LEFT_SHIFT_4_CUR: usize = 0; // 3
    static mut CUR_CUR_POS: usize = 0; //4
    static mut NUM_PAGE: i64 = 0; //5
    static mut NUM_COLS: i64 = 3; //6
    static mut COL_WIDTH: i64 = 55; //7
    static mut NUM_ROWS: i64 = 9; //8
    static mut NUM_SPACES: i64 = 4; //9
    static mut NUM_FILES: i64 = 0; //10
    static mut COUNT_PAGES: i64 = 0; //11
    if val_id == NUM_PAGE_ {return NUM_PAGE}
    if val_id == crate::set(NUM_PAGE_) {NUM_PAGE = val; return val;}
    if val_id == NUM_COLS_ {return NUM_COLS}
    if val_id == crate::set(NUM_COLS_) {NUM_COLS = val; return val;}
    if val_id == NUM_ROWS_ {return NUM_ROWS}
    if val_id == crate::set(NUM_ROWS_) {NUM_ROWS = val; return val}
    if val_id == NUM_FILES_ {return NUM_FILES}
    if val_id == crate::set(NUM_FILES_) { let mut val = val; if val == 0 {val += 1;};
#[cfg(feature="in_dbg")] if crate::breaks("show num files", 1, true).0 == 1 && crate::breaks("show num files", 1, true).1{crate::report(&format!("{val}"), "num files");}
      NUM_FILES = val; return val;}
    if val_id ==  COUNT_PAGES_ {return COUNT_PAGES}
    if val_id == crate::set(COUNT_PAGES_) {COUNT_PAGES = val; return val;}
    if val_id ==  CUR_CUR_POS_ {return CUR_CUR_POS as i64}
    if val_id == crate::set(CUR_CUR_POS_) {CUR_CUR_POS = val as usize; return val;}
    if val_id ==  LEFT_SHIFT_4_CUR_ {return LEFT_SHIFT_4_CUR as i64}
    if val_id == crate::set(LEFT_SHIFT_4_CUR_) {LEFT_SHIFT_4_CUR = val as usize; return val;}
    if val_id == COL_WIDTH_ {return COL_WIDTH}
    if val_id == crate::set(COL_WIDTH_) {COL_WIDTH = val; return val;}
 return -1;  
}
pub(crate) unsafe fn shift_cursor_of_prnt(shift: i64, just_set_shift: Option<usize>, func_id: i64) -> shift_cur_struct{ // shift == 0 to get cursor position, -1 to move left for one char, 1 to move right /
  static mut num_of_shifts: usize = 0;                                          // i64::MIN/MAX to set num_of_shifts = 0/END, 2 to ret num_of_shifts w/ no string /
  let mut str__ = String::from("");                                             // 3 to ret str of shifts
  if just_set_shift.is_some(){
    { num_of_shifts = just_set_shift.unwrap() }; return shift_cur_struct{
      str__: "".to_string(),
      shift: num_of_shifts
    };
  }
  let len = get_prnt(func_id).chars().count();
  let mut ret = shift_cur_struct{shift: num_of_shifts, str__: str__};
  if shift == i64::MIN{num_of_shifts = len;}
  if shift == i64::MAX{num_of_shifts = 0;}
  if shift == 2 {return ret;}
  if shift == 0 || shift == 3{
  macro_rules! shift {
      () => {
      repeat_char(num_of_shifts, "\x1b[D").as_str()      
      };
  }
  if shift == 0{
    if len > num_of_shifts {ret.shift= len - num_of_shifts;}
    if ret.shift == 0{ret.shift = len}
  }
    ret.str__.push_str(shift!());
    return ret
  }
  if shift == -1 {if num_of_shifts <= len {num_of_shifts += 1;}else{num_of_shifts = len;}}
  if shift ==  1 {
    if num_of_shifts > 0 {num_of_shifts -= 1;}
  }
  ret
}
pub(crate) unsafe fn page_struct(val: &str, id_of_val: i64, id_of_caller: i64) -> page_struct_ret
{
   let func_id = crate::func_id18::page_struct_;
    let mut int_:i64 = 0; let mut str__ = String::new();
    let msg = format!("echo 'once prnt {}' > /tmp/str", str__);
    //crate::run_cmd0(msg);
    let mut ps_ret = page_struct_ret{str_: str__, int: int_};
    let vec_str: Vec<String> = vec![val.to_string()];
    static mut STOP_CODE: OnceCell<String> = OnceCell::new(); // 1
    static mut KONSOLE_TITLE: OnceCell<String> = OnceCell::new(); // 2
    static mut fst_run: bool = true;
    static mut prnt_set: bool = false;
    static mut LEFT_SHIFT_4_CUR: i64 = 0; // 3
    static mut CUR_CUR_POS: i64 = 0; //4
    static mut NUM_PAGE: i64 = 0; //5
    static mut NUM_COLS: i64 = 3; //6
    static mut COL_WIDTH: i64 = 70; //7
    static mut NUM_ROWS: i64 = 9; //8
    static mut NUM_SPACES: i64 = 4; //9
    static mut NUM_FILES: i64 = 0; //10
    static mut COUNT_PAGES: i64 = 0; //11
    static mut NEWS_BAR: OnceCell<String> = OnceCell::new(); //12
    static mut ASK_USER: OnceCell<String> = OnceCell::new(); //13
    static mut dontDelFromTableJustMark: bool = true; //14
    static mut RUNNING: OnceCell<Vec<Command>> = OnceCell::new(); //15
    static mut VIEWER: OnceCell<Vec<String>> = OnceCell::new(); //16
    static mut MODE2RUN: OnceCell<(String, String)> = OnceCell::new(); //17
    static mut PRNT: UnsafeCell<String> = UnsafeCell::new(String::new()); //18
    static mut PROMPT: OnceCell<String> = OnceCell::new(); //
    static mut FULL_PATH: OnceCell<String> = OnceCell::new(); //19
    static mut MAINPATH: OnceCell<String> = OnceCell::new(); //20
    static mut FOUND_FILES: OnceCell<String> = OnceCell::new(); //21
    static mut TMP_DIR: OnceCell<String> = OnceCell::new(); //22
    //let mut tst: String = "5".to_string();
    if fst_run {
      println!("fst func id {}", id_of_caller);
      let _ = STOP_CODE.set("∇".to_string());
      FULL_PATH.set("".to_string());
      ASK_USER.set("".to_string());
      let mut viewer_vec: Vec<String> = Vec::new();
      VIEWER.set(viewer_vec);
     // let msg = format!("notify-send 'once prnt {}'", PRNT.get().unwrap()[0]);
     // crate::run_cmd0(msg);
      let _ = PROMPT.set("Your command, Please: ".to_string());
      fst_run = false;
    }
    //let fn_ptr_get_string: fn(&str) -> String = get_string;
    let no_val: i32 = 'no_val: {
   if id_of_caller == __INS{
      set_user_written_path_from_strn(cpy_str(&*PRNT.get()));
    }
    if val != "prnt" {break 'no_val 101;}
    if id_of_caller == __BKSP{
      if PRNT.get().as_ref().expect("Can't get zero len of prnt").len() == 0 {set_cur_cur_pos(0, func_id); ps_ret.str_= "ok".to_string(); return ps_ret}
      //crate::run_cmd0("notify-send bksp".to_string());
      let len = PRNT.get().as_ref().expect("Can't get len of prnt").len() - 1;
      //loop {
        let mut new_prnt = crate::globs18::bksp();
        let new_path = get_path_from_strn(crate::cpy_str(&new_prnt));
        rewrite_user_written_path(&new_path);
        crate::set_prnt(&new_prnt, func_id);
      set_cur_cur_pos(len as i64, func_id);
      set_user_written_path_from_strn(cpy_str(&*PRNT.get()));
      ps_ret.str_= "ok".to_string(); return ps_ret;
    }
    if id_of_caller == __DEL{
      let cur_cur_pos = (get_cur_cur_pos(func_id) + 1) as usize;
      let mut string1 = cpy_str(&*PRNT.get());
      string1.push_str(val);
      let new_string = crate::globs18::ins_last_char_to_string1_from_string1(cur_cur_pos, string1);
      //loop {
          set_prnt(&new_string, func_id);
      let left_shift_4_cur = get_left_shift_4_cur(func_id) - 1;
      set_left_shift_4_cur(left_shift_4_cur, func_id);
      set_user_written_path_from_strn(cpy_str(&*PRNT.get()));
      ps_ret.str_= "ok".to_string(); return ps_ret;
    }
    11    
    };
    let cpy: fn(&String) -> String = |val: &String| -> String{return val.to_string();}; 
    if id_of_val == PRNT_  {ps_ret.str_.push_str(cpy_str(&*PRNT.get()).as_str());/*String::from(PRNT.get().unwrap())*/; return ps_ret;}
    if id_of_val == crate::set(PRNT_) {crate::set_prnt_!(&val.to_string()); file_prnt(val.to_string()); 
       if *PRNT.get() != val {println!("set_prtn failed")} ps_ret.str_= "ok".to_string(); prnt_set =true; return ps_ret;}
    if id_of_val == NUM_OF_VIEWERS  {ps_ret.int = VIEWER.get().unwrap().len().to_i64().unwrap(); return ps_ret;}
    if id_of_val == VIEWER_  {
      let indx = share_usize(usize::MAX, id_of_caller);
      if !indx.1{ps_ret.str_= "none".to_string(); return ps_ret;} let indx = indx.0;
      ps_ret.str_ = cpy(&VIEWER.get().unwrap()[indx]);/*String::from(PRNT.get().unwrap())*/; return ps_ret;}
    if id_of_val == crate::set(VIEWER_) {VIEWER.get_mut().unwrap().push(val.to_string()); ps_ret.str_= "ok".to_string(); prnt_set =true; return ps_ret;}
    if id_of_val == NUM_PAGE_ {ps_ret.int = NUM_PAGE; return ps_ret;}
    if id_of_val == crate::set(NUM_PAGE_) {NUM_PAGE = i64::from_str_radix(val, 10).expect("failed number of a page"); return ps_ret;}
    if id_of_val == NUM_COLS_ {ps_ret.int = NUM_COLS; return ps_ret;}
    if id_of_val == crate::set(NUM_COLS_) {NUM_COLS = i64::from_str_radix(val, 10).expect("failed number of columns"); return ps_ret;}
    if id_of_val == NUM_ROWS_ {ps_ret.int = NUM_ROWS; return ps_ret;}
    if id_of_val == crate::set(NUM_ROWS_) {NUM_ROWS = i64::from_str_radix(val, 10).expect("failed number of rows"); return ps_ret;}
    if id_of_val == NUM_FILES_ {ps_ret.int = NUM_FILES; return ps_ret;}
    if id_of_val == crate::set(NUM_FILES_) {NUM_FILES = i64::from_str_radix(val, 10).expect("failed number of files"); return ps_ret;}
    if id_of_val ==  COUNT_PAGES_ {ps_ret.int = COUNT_PAGES; return ps_ret;}
    if id_of_val == crate::set(COUNT_PAGES_) {COUNT_PAGES = i64::from_str_radix(val, 10).expect("failed number of pages"); return ps_ret;}
    if id_of_val == STOP_CODE_ {ps_ret.str_ =STOP_CODE.get().unwrap().to_string(); return ps_ret;}
    if id_of_val == crate::set(STOP_CODE_) {STOP_CODE.take(); let _ = STOP_CODE.set(val.to_string()); ps_ret.str_= "ok".to_string(); return ps_ret;}
    if id_of_val == PROMPT_ {ps_ret.str_ =PROMPT.get().unwrap().to_string(); return ps_ret;}
    if id_of_val == crate::set(PROMPT_) {PROMPT.take(); let _ =PROMPT.set(val.to_string()); ps_ret.str_= "ok".to_string(); return ps_ret;}
    if id_of_val == FULL_PATH_ {ps_ret.str_ =FULL_PATH.get().unwrap().to_string(); return ps_ret;}
    if id_of_val == ASK_USER_ {ps_ret.str_ =ASK_USER.get().unwrap().to_string(); return ps_ret;}
    if id_of_val == crate::set(ASK_USER_) {ASK_USER.take(); let _ =ASK_USER.set(val.to_string()); ps_ret.str_= "ok".to_string(); return ps_ret;}
    if id_of_val == crate::set(FULL_PATH_) {FULL_PATH.take(); let _ =FULL_PATH.set(val.to_string()); ps_ret.str_= "ok".to_string(); return ps_ret;}
    if id_of_val == MAINPATH_ {if MAINPATH.get() != None{ps_ret.str_ = MAINPATH.get().unwrap().to_string(); return ps_ret;}}
    if id_of_val == crate::set(MAINPATH_) {MAINPATH.take(); let _ = MAINPATH.set(val.to_string());  ps_ret.str_= "ok".to_string(); return ps_ret;}
    if id_of_val == FOUND_FILES_ {ps_ret.str_ = FOUND_FILES.get().unwrap().to_string(); return ps_ret;}
    if id_of_val == crate::set(FOUND_FILES_) {FOUND_FILES.take(); let _ = FOUND_FILES.set(val.to_string());  ps_ret.str_= "ok".to_string(); return ps_ret;}
    if id_of_val == TMP_DIR_ {ps_ret.str_ = match TMP_DIR.get(){
      Some(s) => s,
      _ => ""

    }.to_string(); return ps_ret;}
    if id_of_val == crate::set(TMP_DIR_) {KONSOLE_TITLE.take(); TMP_DIR.set(val.to_string()); ps_ret.str_= "ok".to_string(); return ps_ret;}
    if id_of_val == KONSOLE_TITLE_ {ps_ret.str_ =KONSOLE_TITLE.get().unwrap().to_string(); return ps_ret;}
    if id_of_val == crate::set(KONSOLE_TITLE_) {KONSOLE_TITLE.take(); let _ = KONSOLE_TITLE.set(val.to_string()); ps_ret.str_= "ok".to_string(); return ps_ret;}
     ps_ret.str_= "none".to_string(); return ps_ret;
}
pub(crate) fn where_is_last_pg() -> i64{
  let func_id = func_id18::where_is_last_pg_;
  let len = match i64::from_str_radix(&len_of_front_list(), 10){
    Ok(i) => i,
    _ => 0
  };
  let num_rows = get_num_rows(func_id);
  let num_cols = get_num_cols(func_id);
  let mut last_pg: i64 = len / (num_cols * num_rows);
  let mut residue = last_pg * num_cols * num_rows;
  if residue < len {last_pg.inc();}
  last_pg
}