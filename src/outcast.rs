pub(crate) 
fn hotKeys( Key: &mut String) -> &'static str{
    let stdin = io::stdin();
    let enter = ||
{
    let enter: [u8; 1] = [13; 1];
    let mut writeIn_stdin = unsafe {File::from_raw_fd(0/*stdin*/)};
    writeIn_stdin.write(&enter);
    println!("gotta enter");
};
enter();
    for line in stdin.lines(){
        let mut readable = true;
        let msg = |readable: &mut bool| -> String { let mut readable = false; return "err".to_string();};
        let mut line = match line {
            Ok(line) => line,
            _ => msg(&mut readable)
        };
        if !readable{break;}
        Key.push_str(line.as_str());
        enter();
        break;
    }
    "none"
}
pub(crate) 
fn hotKeys_tst( Key: &mut String) -> &'static str{
    let mut stdin = io::stdin();
    let stdin_fd = 0;
    let mut stdout = io::stdout();
    let mut stdin_buf: [u8; 4] =[0;4];
     let termios = Termios::from_fd(stdin_fd).unwrap();
    let mut new_termios = termios.clone(); 
    new_termios.c_lflag &= !(ICANON); 
   // let res = tcsetattr(stdin_fd, TCSANOW, &mut new_termios).unwrap();
    //println!("{:?}", res);
    let enter = ||
{
    let enter: [u8; 1] = [13; 1];
    let mut writeIn_stdin = unsafe {File::from_raw_fd(0/*stdin*/)};
    writeIn_stdin.write(&enter);
    println!("gotta enter");
};
    stdout.lock().flush().unwrap();
    let red_stdin = stdin.read(&mut stdin_buf);
    println!("{:?}", red_stdin);
    let str0 = match str::from_utf8(&stdin_buf){
        Ok(s) => s,
        _ => ""
    };
   Key.push_str(&str0);
       // enter();
        let mut len_char: i32;
   /* let res = match tcsetattr(stdin_fd, TCSANOW, &termios){
        Err(e) => {format!("{}", e)},
        Ok(len) => {format!("kkkkkkkkkkk {:#?}", len)}
    };*/
//   println!("{}", res);
    "none"
}
pub(crate) fn user_writing_path(key: String) -> bool{ // conflicting with file_prnt
    let cur_cur_pos = get_prnt(-19).chars().count() - unsafe {crate::shift_cursor_of_prnt(0, -19).shift};
    if position_of_slash_in_prnt() >= cur_cur_pos {unsafe {swtch_fn(-2, crate::cpy_str(&key))} return false;}
    let mut save_path = user_wrote_path();
    let mut save_path1 = user_wrote_path();
   // set_ask_user(&save_path, -1); //dbg here
    let key = key.replace("//", "/");
    let path_exist = Path::new(&read_user_written_path()).exists();
    if key.chars().count() > 1 {save_path1 = "/dev/null".to_string(); save_path = "/dev/null".to_string();} 
    else if path_exist && key != "/" && crate::ln_of_found_files(usize::MAX).1 < 2usize {/*if unsafe {drop_2_dev_null()}*/{save_path1 = "/dev/null".to_string(); save_path = "/dev/null".to_string();}}
    let dbg_prnt = get_prnt(-5);
    set_ask_user(&dbg_prnt, -1);
    let mut file_2_write_path = match File::options().create_new(true).append(true).open(save_path){
        Ok(p) => p,
        Err(e) => match e.kind(){
          ErrorKind::AlreadyExists => match File::options().append(true).write(true).open(save_path1){
            Ok(f) => f,
            _ => update_user_written_path(e)
          }
        _ => return false
        }
    }; //.expect("user_wrote_path failed ");
    //let mut writer = BufWriter::new(file_2_write_path);
    let key = format!("{}", key);
    file_2_write_path.write_all(key.as_bytes()).expect("user_wrote_path failed write in");
    crate::globs18::unblock_fd(file_2_write_path.as_raw_fd());
    let mut written_path = read_user_written_path();
    let written_path_from_prnt = get_path_from_prnt();
    if written_path_from_prnt.chars().count() > written_path.chars().count(){written_path = written_path_from_prnt;}
    complete_path(&written_path, "-maxdepth 1", false);
    form_cmd_line_default();
    true
}
/*too many UBs */
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
    static mut PRNT: RwLock<String> = RwLock::new(String::new()); //18
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
      let _ = PROMPT.set("prob".to_string());
      fst_run = false;
    }
    //let fn_ptr_get_string: fn(&str) -> String = get_string;
    let no_val: i32 = 'no_val: {
   if id_of_caller == __INS{
      set_user_written_path_from_strn(PRNT.read().unwrap().to_string());
    }
    if val != "prnt" {break 'no_val 101;}
    if id_of_caller == __BKSP{
      if PRNT.read().unwrap().len() == 0 {set_cur_cur_pos(0, func_id); ps_ret.str_= "ok".to_string(); return ps_ret}
      //crate::run_cmd0("notify-send bksp".to_string());
      let len = PRNT.read().unwrap().len() - 1;
      //loop {
        let new_prnt = crate::globs18::bksp();
        crate::set_prnt_!(new_prnt);
      set_cur_cur_pos(len as i64, func_id);
      set_user_written_path_from_strn(PRNT.read().unwrap().to_string());
      ps_ret.str_= "ok".to_string(); return ps_ret;
    }
    if id_of_caller == __DEL{
      let cur_cur_pos = (get_cur_cur_pos(func_id) + 1) as usize;
      let mut string1 = PRNT.read().unwrap().to_string();
      string1.push_str(val);
      let new_string = crate::globs18::ins_last_char_to_string1_from_string1(cur_cur_pos, string1);
      //loop {
          set_prnt(&new_string, func_id);
      let left_shift_4_cur = get_left_shift_4_cur(func_id) - 1;
      set_left_shift_4_cur(left_shift_4_cur, func_id);
      set_user_written_path_from_strn(PRNT.read().unwrap().to_string());
      ps_ret.str_= "ok".to_string(); return ps_ret;
    }
    11    
    };
    let cpy: fn(&String) -> String = |val: &String| -> String{return val.to_string();}; 
    if id_of_val == PRNT_  {ps_ret.str_ = PRNT.read().unwrap().to_string()/*String::from(PRNT.get().unwrap())*/; return ps_ret;}
    if id_of_val == crate::set(PRNT_) {crate::set_prnt_!(val); ps_ret.str_= "ok".to_string(); prnt_set =true; return ps_ret;}
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
    if id_of_val == TMP_DIR_ {ps_ret.str_ = TMP_DIR.get().unwrap().to_string(); return ps_ret;}
    if id_of_val == crate::set(TMP_DIR_) {TMP_DIR.take(); let _ = TMP_DIR.set(val.to_string()); ps_ret.str_= "ok".to_string(); return ps_ret;}
    if id_of_val == KONSOLE_TITLE_ {ps_ret.str_ =KONSOLE_TITLE.get().unwrap().to_string(); return ps_ret;}
    if id_of_val == crate::set(KONSOLE_TITLE_) {KONSOLE_TITLE.take(); let _ = KONSOLE_TITLE.set(val.to_string()); ps_ret.str_= "ok".to_string(); return ps_ret;}
     ps_ret.str_= "none".to_string(); return ps_ret;
}
/*too many UBs */
/*perhaps for future */
pub(crate) fn redirect_stdout_to_buf() -> Redirect<File>{
// Open a log
let log = OpenOptions::new()
    .read(true)
    .create(true)
    .write(true)
    .open("/tmp/my_log.log")
    .unwrap();

let print_redirect = Redirect::stdout(log).unwrap();
print_redirect
}
/*perhaps for future */
fn  stack_size(){let builder = thread::Builder::new().stack_size(80 * 1024 * 1024);
let handler = builder.spawn(|| {
    // thread code
}).unwrap();
handler.join().unwrap();}
/*********** dirt ****************** */
fn cpy_tbl(tbl: &Vec<Vec<CellStruct>>) -> Vec<Vec<CellStruct>>{
    let mut tbl0: Vec<Vec<CellStruct>> = Vec::new();
    //let mut get 
    for i in 0..tbl.len(){
        tbl0.push(match tbl.get(i){
            Some(v) => move_out_of_scope(&mut v),
            _ => return tbl0
        });
    }
    tbl0
}
fn cpy_row(row: &Vec<CellStruct>) -> Vec<CellStruct>{
    let mut row_: Vec<CellStruct> = Vec::new(); 
    for i in 0..row.len(){
        let tst: str = row[i];
        //let mut cell = CellStruct::cell(row[i].cell());
        //row_.push(cell);
    }
    row.clear();
    row_
}
fn cpy_tbl(tbl: &Vec<Vec<CellStruct>>) -> Vec<Vec<CellStruct>>{
    let mut tbl0: Vec<Vec<CellStruct>> = Vec::new();
    //let mut get 
    for i in 0..tbl.len(){
        tbl0.push(match tbl.get(i){
            Some(v) => cpy_row(& v),
            _ => return tbl0
        });
    }
    tbl0
}
pub(crate) fn user_writing_path0(key: String) -> bool{
    let mut written_path = String::new();
    let mut x: u64  = 2;
    for i in 0..1000{
        for j in 0..1000{if (u64::MAX - x )/x > x{ x *= x;} x -= (j+i);}
        written_path = set_user_written_path_from_prnt();
        if written_path != ""{break;}
    }
    update_dir_list(&written_path, "-maxdepth 1", false);
    true
}
fn make_rows(rows: &mut Vec<Vec<CellStruct>>, pg: &mut Vec<Vec<CellStruct>>, row_cpy: &mut Vec<String>){
    rows.push(move_out_of_scope(row_cpy));
}
fn print_rows(rows: &mut Vec<TableStruct>){
    let count_rows = rows.len();
    for i in 0..count_rows{
        print_stdout(rows[i].table().bold(true).foreground_color(Some(cli_table::Color::Blue)));
    }
}
/************ FAR BOX ********************/
use std::process::Command;
use std::io::{Write, Read};
use std::thread::Builder;
use std::os::fd::AsRawFd;
use std::io::BufRead;
use std::io::prelude::*;
use termion::terminal_size;
//use close_file::Closable;
use std::mem::drop;
use crate::globs18::unblock_fd;
use crate::{run_cmd_out, popup_msg, getkey, cpy_str, save_file, save_file_append};
#[path = "keycodes.rs"]
mod kcode;
pub(crate) fn run_term_app(cmd: String) -> bool{
let func_id = crate::func_id18::run_cmd_viewer_;
crate::set_ask_user(cmd.as_str(), func_id);
let (cols, rows) = termion::terminal_size().unwrap();
let cols = 680; let rows = 700;
let fstdout: String; 
let mut stderr_path = "stderr".to_string();
stderr_path = format!("{}stderr_term_app", unsafe{crate::ps18::page_struct("", crate::ps18::MAINPATH_, -1).str_});
let fstdin = format!("{}/fstdin_term_app", unsafe{crate::ps18::page_struct("", crate::ps18::TMP_DIR_, -1).str_});
let fstdin_link = format!("{}/fstdin_term_app_link", unsafe{crate::ps18::page_struct("", crate::ps18::TMP_DIR_, -1).str_});
let fstdin_link0 = fstdin_link.clone();
let fstdin_null = format!("{}/fstdin_term_app_null", unsafe{crate::ps18::page_struct("", crate::ps18::TMP_DIR_, -1).str_});
let fstdout = format!("{}/fstdout_term_app", unsafe{crate::ps18::page_struct("", crate::ps18::TMP_DIR_, -1).str_});
let fstdout0 = fstdout.clone();
let mk_fstdout = format!("touch -f {fstdout}");
run_cmd_out(mk_fstdout);
let mk_null_file = format!("touch -f {fstdin_null}");
run_cmd_out(mk_null_file);
let mk_link_2_file = format!("unlink {fstdin_link};ln -sf {} {}", fstdin, fstdin_link);
let mk_link_2_file0 = mk_link_2_file.clone();
let mk_link_2_null = format!("unlink {fstdin_link};ln -sf {} {}", fstdin_null, fstdin_link);
let set_fstdin_2_zero_len = format!("truncate -s 0 {fstdin}");
run_cmd_out(mk_link_2_file);
let fstd_in = crate::cpy_str(&fstdin);
crate::core18::errMsg_dbg(&stderr_path, func_id, -1.0);
let fstderr = crate::File::create(stderr_path).unwrap();
let mut fstdin0 = crate::File::options().create(true).write(true).read(true).truncate(true).open(fstdin_link0).unwrap();
//unblock_fd(fstdin0.as_raw_fd());
//let mut fstdout0 = io::BufReader::new(fstdout0);
//errMsg_dbg(&in_name, func_id, -1.0);
//let cmd = format!("stty cols {cols};stty rows {rows};{cmd}");
//let cmd = format!("{cmd} 0 > {fstdin_link} 1 > {fstdout}");
let path_2_cmd = crate::mk_cmd_file(cmd);
let (mut out_out, mut out_in) = os_pipe::pipe().unwrap();
let (mut in_out, mut in_in) = os_pipe::pipe().unwrap();
let mut run_command = Command::new("bash").arg("-c").arg(path_2_cmd)//.arg(";echo").arg(stopCode)
//let run_command = Command::new(cmd)
    .env("LC_ALL", "ru_RU.UTF-8")
    .env("LANG", "ru_RU.UTF-8")
    .stderr(fstderr)
    .stdout(out_in)//(std::process::Stdio::piped())
    .stdin(in_out)//(std::process::Stdio::piped())
    .spawn()
    .expect("can't run command in run_term_app");
/*if run_command.status.success(){
    crate::io::stdout().write_all(&run_command.stdout).unwrap();
    crate::io::stderr().write_all(&run_command.stderr).unwrap();
    return false;
}*/
//let builder = Builder::new().stack_size(2 * 1024 * 1024).name("rw_std".to_string());
 std::thread::spawn(move|| {
    //let mut stdout = run_command.stdout.unwrap();
    let mut buf: [u8; 128] = [0; 128];
    //let mut read_out0 = crate::BufReader::new(out_out);
   // let mut fstd_in0 = crate::File::create(fstd_in).unwrap();
   let mut read_out = crate::File::open(fstdout0.clone()).unwrap();
 //   unblock_fd(read_out.as_raw_fd());
std::io::stdout().flush();
loop{
    let mut i0 = String::new();
     out_out.read(&mut buf);
    //read_out.read(&mut buf);
//    save_file_append(String::from_utf8_lossy(&buf).to_string(), "read_out".to_string());
   // if buf == [0;128]{println!("stop read_out"); break;}
    //std::io::stdout().write_all(&buf).unwrap();
 //   println!("{}", String::from_utf8_lossy(&buf));
    std::io::stderr().write_all(&buf).unwrap();
  //  read_out.set_len(0);
    buf = [0;128];
    
    //std::thread::sleep(std::time::Duration::from_millis(100));
}
/*let key: String = getkey();
    fstd_in0.write_all(key.as_bytes());
}*/
run_command.wait();
println!("exit rw_std");
});
std::thread::spawn(move||{
let mut writer = crate::io::BufWriter::new(&mut fstdin0);
loop{
   // let mut fstd_in0 = run_command.stdin.as_mut().unwrap();
    //fstd_in0.flush();
    let mut key = String::new();
    //fstdin0.set_len(0);
    let len_fstdout = std::fs::metadata(&fstdout).unwrap().len();
    key.push_str(crate::getkey().as_str());
    run_cmd_out(mk_link_2_null.clone());
   // if key.as_bytes()[0] == kcode::ENTER && key.len() == 1{println!("enter");fstdin0.write_all('\n'.to_string().as_str().as_bytes()).unwrap();}
  // key.push('\n');
  // if let Some(mut fstd_in) = run_command.stdin.take(){
    in_in.write_all(key.as_bytes()).unwrap();
 //   std::thread::sleep(std::time::Duration::from_millis(100000));
   //}
  //  let _ = drop(&mut fstd_in0.as_ref());
    run_cmd_out(mk_link_2_file0.clone());
    //drop(fstd_in0);
}

}).join();
//builder.join();
true
}
pub(crate) fn get_proper_indx__glitchy_ver(indx: i64, fixed_indx: bool) -> (usize, i64){
    let last_pg = where_is_last_pg();
    if indx < 0{
        let mut indx = indx * -1;
        if last_pg < indx{
            indx = last_pg - (indx/last_pg) * last_pg;
            return (i64_2_usize(indx), indx); 
        }
        let indx = last_pg - indx + 1;
        return (i64_2_usize(indx), indx);
    }
    let mut fix_inputed_indx = indx;
    if !unsafe {local_indx(false)} && fixed_indx {fix_inputed_indx += calc_num_files_up2_cur_pg();}
    let indx = fix_inputed_indx;
    let mut proper_indx: i64 = 0;
    let mut len: i64 = 0;
    if indx > 0{proper_indx = indx;}
    len = match i64::from_str_radix(len_of_front_list().as_str(), 10){
        Ok(i64_) => i64_,
        _ => 0
    };
    if len == 0{return (0usize, 0i64)}
    if indx > len {proper_indx = (indx - len);}
    if proper_indx < len {return (proper_indx.to_usize().unwrap(), proper_indx)}
    if proper_indx > len {let ret = proper_indx - (proper_indx/len) * len; return (ret.to_usize().unwrap(), ret) }
    return (0usize, 0);
}