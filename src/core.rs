#[macro_use]
#[path = "exts.rs"]
mod exts;
use exts::*;
use once_cell::sync::Lazy;
use ps21::set_num_files0;
use termios::ISIG;
//use gag::RedirectError;

use crate::{
    cached_ln_of_found_files, change_dir0,
    custom_traits::{fs_tools, STRN},
    front_lst, get_arg_in_cmd, helpful_math_ops,
    init::user_home_dir,
    link_ext_lsts, link_lst_dir_to, link_lst_to, no_esc_lst, run_cmd0, run_cmd_out,
    run_cmd_out_sync, run_cmd_str, session_lists, shift_cursor_of_prnt, split_once,
    swtch::{path_completed, read_user_written_path, user_wrote_path, user_wrote_path_prnt},
    swtch_esc,
    update18::{
        alive_session, background_fixing, background_fixing_count, fix_screen, upd_screen_or_not,
        update_dir_list,
    },
    STRN_strip,
};

use self::ps21::{get_mainpath, get_prnt, get_tmp_dir, set_ask_user, set_prnt};
core_use!();
pub(crate) fn bkp_tmp_dir(sav: Option<String>, set: bool) -> String {
    static mut bkp: OnceCell<String> = OnceCell::new();
    if crate::C!(bkp.get()) == None && set {
        let fst: String =
            sav.unwrap_or(unsafe { crate::page_struct("", crate::TMP_DIR_, -75811245).str_ });
        let ret = fst.clone();
        crate::C!(bkp.set(fst));
        return ret;
    }
    if unsafe { bkp.get() == None } {
        return "".strn();
    }
    crate::C!(bkp.get().expect("bkp_tmp_dir failed").to_string())
}
pub(crate) fn bkp_main_path(sav: Option<String>, set: bool) -> String {
    static mut bkp: OnceCell<String> = OnceCell::new();
    if crate::C!(bkp.get()) == None && set {
        let fst: String =
            sav.unwrap_or(unsafe { crate::page_struct("", crate::MAINPATH_, -75811245).str_ });
        let ret = fst.clone();
        crate::C!(bkp.set(fst));
        return ret;
    }
    if unsafe { bkp.get() == None } {
        return "".strn();
    }
    crate::C!(bkp.get().expect("bkp_main_path failed").to_string())
}
pub(crate) fn shm_tam_dir(set_dir: Option<String>) -> String {
    static mut tam: OnceCell<String> = OnceCell::new();
    if crate::C!(tam.get()) == None && set_dir.is_some() {
        let ret = set_dir.unwrap();
        crate::C!(tam.set(ret.clone()));
        return ret;
    }
    crate::C!(tam.get().expect("shm_tmp_dir failed").to_string())
}
pub(crate) fn up_front_list() {
    let list = read_front_list();
    if check_substrn(&list.strn(), "history") {
        swtch_esc(true, false);
    } else {
        swtch_esc(true, true);
    }
    let found_files = take_list_adr_env(&"found_files".strn());
    let active_list = take_list_adr_env(&list);
    let cmd = format!("#up_front_list\nln -sf {active_list} {found_files}");
    run_cmd_str(&cmd);
}
pub(crate) fn set_front_list(list: &str) {
    let mut list = list.strn(); crate::lst::edit_mode_lst(Some (false) );
    if list == "" {
        list = "lst".strn()
    }
    if check_substrn(&list.strn(), "history") {
        swtch_esc(true, false);
    } else {
        swtch_esc(true, true);
    }
    let mut prev = name_of_front_list("", false);
    if prev == "" {
        prev = "main0".strn()
    }
    if prev != "ls" {
        save_file(prev, "prev_list".strn());
    }
    let found_files = take_list_adr_env(&"found_files".strn());
    let active_list = take_list_adr_env(&list);
    let cmd = format!("#set_front_list\nln -sf {active_list} {found_files}");
    run_cmd_out_sync(cmd);
    mark_front_lst(&list);
    //crate::wait_4_empty_cache();
    //if list == "merge"
    name_of_front_list(&list, true);
    //background_fixing();
    crate::set_num_files_4_lst(&list.strn());
}
pub(crate) fn name_of_front_list(name: &str, set: bool) -> String {
    static mut name0: Lazy<String> = Lazy::new(|| String::new());
    if set {
        unsafe {
            *name0 = name.to_string();
        }
    }
    //popup_msg(&unsafe{name0.to_string()} );
    unsafe { name0.to_string() }
}
pub(crate) fn set_front_list2(list: &str, num_upds_scrn: usize) {
    crate::lst::edit_mode_lst(Some (false) );
    if check_substrn(&list.strn(), "history") {
        swtch_esc(true, false);
    } else {
        swtch_esc(true, true);
    }
    let prev = name_of_front_list("", false);
    if prev != "" && prev != "ls" {
        save_file(prev, "prev_list".strn());
    }
    let found_files = take_list_adr_env(&"found_files".strn());
    let active_list = take_list_adr_env(&list);
    let cmd = format!("#set_front_list\nln -sf {active_list} {found_files}");
    run_cmd_out_sync(cmd);
    mark_front_lst(list);
    crate::wait_4_empty_cache();
    //if list == "merge"
    name_of_front_list(list, true);
    //background_fixing_count(num_upds_scrn);
    crate::set_num_files_4_lst(&list.strn());
}
pub(crate) fn mark_front_lst(name: &str) {
    if check_substrn(&name.strn(), "history") {
        swtch_esc(true, false);
    } else {
        swtch_esc(true, true);
    }
    if name != "ls" {
        save_file(name.to_string(), "front_list".to_string());
    } else {
        save_file(name.to_string(), "ls.mode".to_string());
    }
}
pub(crate) fn __proper_timestamp(sav: Option<String>) -> String {
    static mut timestamp: Lazy<String> = Lazy::new(|| String::new());
    static mut fst: bool = true;
    if unsafe { fst } && sav.is_some() {
        unsafe {
            *timestamp = sav.expect("Not proper timestamp");
            fst = false
        };
        return "".strn();
    }
    unsafe { timestamp.clone() }
}
pub(crate) fn initSession() -> bool {
    let func_id = 1;
    let run_command = Command::new("bash")
        .arg("-c")
        .arg("cd ~;pwd")
        .output()
        .expect("something wrong with cd command");
    errMsg_dbg(from_utf8(&run_command.stderr).unwrap(), func_id, -1.0);
    if !run_command.status.success() {
        io::stdout().write_all(&run_command.stdout).unwrap();
        io::stderr().write_all(&run_command.stderr).unwrap();
        return false;
    }
    let mut proper_output: Vec<u8> = run_command.stdout;
    let last: usize = proper_output.len() - 1;
    let _ = proper_output.pop();
    errMsg_dbg(from_utf8(&proper_output).unwrap(), func_id, -1.0);
    let timestamp = Local::now();
    let mut proper_timestamp = format!("{}", timestamp.format("%Y-%mm-%dd_%H-%M-%S_%f"));
    /*if __proper_timestamp(None) != ""{proper_timestamp = __proper_timestamp(None)}
    else { __proper_timestamp(Some(proper_timestamp.clone() ) ); } */
    let mainpath: String = format!(
        "{}/.TAM_SESSIONS/{proper_timestamp}/",
        from_utf8(&proper_output).unwrap().to_string()
    );
    //let mainpath = escape_symbs(&mainpath);
    errMsg_dbg(&mainpath, func_id, -1.0);
    let run_shell = Command::new("mkdir").arg("-p").arg(&mainpath).output();
    if checkArg("-dbg") {
        println!("shell out = {:?}", run_shell)
    };
    if !Path::new(&mainpath).exists() {
        let res = fs::create_dir(&mainpath);
        println!("res = {:?}", res);
    }
    if !Path::new(&mainpath).exists() {
        println!("{mainpath} cannot be made");
        return false;
    }
    crate::C!(crate::page_struct(&mainpath, set(ps21::MAINPATH_), func_id));
    bkp_main_path(Some(mainpath), true);
    let mut path_2_shm = "";
    while true {
        if Path::new("/dev/shm").exists() {
            path_2_shm = "/dev/shm";
            break;
        }
        if Path::new("/run/shm").exists() {
            path_2_shm = "/run/shm";
            break;
        }
        if Path::new("/sys/shm").exists() {
            path_2_shm = "/sys/shm";
            break;
        }
        if Path::new("/proc/shm").exists() {
            path_2_shm = "/proc/shm";
            break;
        }
        if Path::new("/opt/shm").exists() {
            path_2_shm = "/opt/shm";
            break;
        }
        if Path::new("/var/shm").exists() {
            path_2_shm = "/var/shm";
            break;
        }
        panic!("no way for shared memory: device /dev/shm and its analogs don't exist or maybe Your system ain't common Linux");
    }
    crate::globs18::path_to_shm(Some(&path_2_shm.strn()));
    let globalTAM = format!("{path_2_shm}/TAM");
    Command::new("mkdir")
        .arg("-p")
        .arg(&globalTAM)
        .output()
        .expect(&"Sorry, Failed to create shared TAM dir.".bold().red());
    let globalTAMdot = format!("{path_2_shm}/TAM/.");
    Command::new("chmod")
        .arg("700")
        .arg(&globalTAMdot)
        .output()
        .expect(
            &"Sorry, Failed to gain full access to shared TAM dir."
                .bold()
                .red(),
        );
    shm_tam_dir(Some(globalTAM));
    let mut path_2_found_files_list = format!("{}/TAM_{}", path_2_shm, proper_timestamp);
    if bkp_tmp_dir(None, false) != "" {
        path_2_found_files_list = bkp_tmp_dir(None, false)
    }
    let err_msg = format!("{} wasn't created", path_2_found_files_list);
    let run_shell1 = Command::new("mkdir")
        .arg("-p")
        .arg(&path_2_found_files_list)
        .output()
        .expect(&err_msg.bold().red());
    if checkArg("-dbg") {
        println!("shell out = {:?}", run_shell1)
    };
    let err_msg = format!("{} permission denied", path_2_found_files_list);
    let run_shell2 = Command::new("chmod")
        .arg("700")
        .arg(&path_2_found_files_list)
        .output()
        .expect(&err_msg.bold().red());
    if checkArg("-dbg") {
        println!("shell out = {:?}", run_shell2)
    };
    crate::C!(crate::page_struct(
        &path_2_found_files_list,
        set(crate::TMP_DIR_),
        func_id
    ));
    let tmp_dir = path_2_found_files_list;
    bkp_tmp_dir(Some(tmp_dir.clone()), true);
    let path_2_found_files_list_dot = format!("{}/TAM_{}/.", path_2_shm, proper_timestamp);
    let err_msg = format!("{} permission denied", path_2_found_files_list_dot);
    let run_shell3 = Command::new("chmod")
        .arg("700")
        .arg(&path_2_found_files_list_dot)
        .output()
        .expect(&err_msg.bold().red());
    if checkArg("-dbg") {
        println!("shell out = {:?}", run_shell3)
    };
    let path_2_found_files_list = format!("{}/TAM_{}/found_files0", path_2_shm, proper_timestamp);
    /*let err_msg = format!("{} can't be created", path_2_found_files_list);
    let run_shell4 = Command::new("touch").arg("-f").arg(&path_2_found_files_list).output().expect(&err_msg.bold().red());*/
    //if checkArg("-dbg"){println!("shell out = {:?}", run_shell4)};
    unsafe {
        crate::page_struct(&path_2_found_files_list, set(crate::FOUND_FILES_), func_id);
        crate::page_struct("empty", set(crate::KONSOLE_TITLE_), func_id);
    }
    crate::globs18::set_main0_as_front();
    crate::set_prnt("", func_id);
    unsafe {
        crate::swtch::form_list_of_viewers(false);
    }
    crate::save_file("".to_string(), "main0.pg".to_string());
    mark_front_lst("main0");
    let mk_cache_dir = format!("mkdir -p {tmp_dir}/cache");
    crate::run_cmd0(mk_cache_dir);
    let mk_cache_dir = format!("mkdir -p {tmp_dir}/env");
    crate::run_cmd0(mk_cache_dir);
    let mk_cache_dir = format!("mkdir -p {tmp_dir}/patch");
    crate::run_cmd0(mk_cache_dir);
    let mk_cache_dir = format!("mkdir -p {tmp_dir}/logs");
    crate::run_cmd0(mk_cache_dir);
    if !link_lst_dir_to() {
        let mk_cache_dir = format!("mkdir -p {tmp_dir}/env/lst");
        crate::run_cmd0(mk_cache_dir);
    }
    session_lists();
    let mk_cache_dir = format!("mkdir -p {tmp_dir}/env/lst_opts");
    crate::run_cmd0(mk_cache_dir);
    let mk_cache_dir = format!("mkdir -p {tmp_dir}/env/lst_id");
    crate::run_cmd0(mk_cache_dir);
    let mk_cache_dir = format!("mkdir -p {tmp_dir}/env/dummy_lnks");
    crate::run_cmd0(mk_cache_dir);
    let mk_cache_dir = format!("mkdir -p {tmp_dir}/msgs/basic/cache");
    crate::run_cmd0(mk_cache_dir);
    mk_dummy_lnks();
    let key = "-history";
    if checkArg(key) {
        let link = __get_arg_in_cmd(key);
        link_lst_to(&key.substring(1, key.len()).strn(), &link);
    }
    link_ext_lsts();
    alive_session();
    change_dir0();
    crate::init::user_home_dir();
    crate::set_full_path(
        &"{Alt + F12} == Rolling guide of TAM (Topnotch Practical ways to use Console/Terminal)".bold().strn(),
        func_id,
    );
    #[cfg(feature = "mae")]
    crate::cache::lazy_cache_cleaning(None);
    crate::subs::prompt_mode(Some( crate::enums::prompt_modes::glee_uppercases ) );
    return true;
}
pub(crate) fn __get_arg_in_cmd(key: &str) -> String {
    let mut ret = "".to_string();
    let args: Vec<_> = env::args().collect();
    //let args_2_str = args.as_slice();
    for i in 1..args.len() {
        if
        /*args_2_str[i]*/
        args[i] == key {
            return args[i + 1].clone();
        }
    }
    return ret;
}
pub(crate) fn mk_dummy_lnks() {
    mk_dummy_lnk("cp");
    mk_dummy_lnk("mv");
    mk_dummy_lnk("rm");
}
pub(crate) fn mk_dummy_lnk(head: &str) {
    let cmd = "whereis ".to_string() + head;
    let ret = run_cmd_out(cmd)
        .replace(&format!("{head}:"), "")
        .trim_start()
        .to_string();
    let (ret, _) = split_once(&ret, head);
    if ret != "none" {
        let cmd = format!(
            "ln -sf {ret}/{head} {}",
            format!("{}/{head}", take_list_adr("env/dummy_lnks"))
        );
        run_cmd0(cmd);
    }
}
pub(crate) fn errMsg_dbg(msg: &str, val_func_id: i64, delay: f64) {
    if !checkArg("-dbg") {
        return;
    }
    if delay == -1.0 {
        let msg = format!(
            "{} said: {} ..please, hit any key to continue",
            crate::func_id18::get_func_name(val_func_id),
            msg
        );
        println!("{msg}");
        set_ask_user(&msg.bold().red(), val_func_id);
    }
    getkey();
}
pub(crate) fn errMsg_dbg0(msg: &str) {
    errMsg_dbg(msg, -1101, -1.0);
}
pub(crate) fn errMsg0(msg: &str) {
    errMsg(msg, -1191);
    println!("{}", msg);
    getkey();
}
pub(crate) fn errMsg(msg: &str, val_func_id: i64) {
    let msg = format!(
        "{} said: {}",
        crate::func_id18::get_func_name(val_func_id),
        msg
    );
    set_ask_user(&msg.bold().red(), val_func_id);
}
pub(crate) fn checkArg(key: &str) -> bool {
    let len_of_cmd_line = env::args().len();
    let args: Vec<String> = env::args().collect();
    let i: i64 = 0;
    for i in 0..len_of_cmd_line {
        if args[i] == key.to_string() {
            return true;
        }
    }
    return false;
}
pub(crate) fn set(item: i64) -> i64 {
    return item * -1;
}
pub(crate) fn this_item_takes_global_val(id: i64) -> i64 {
    return set(id);
}
pub(crate) fn set_proper_num_pg(num_pg: i64) {
    let front_list = format!("{}/{}", crate::get_tmp_dir(-371033), "front_list");
    let listName_dot_pg = format!("{}.pg", read_front_list());
    save_file(num_pg.to_string(), listName_dot_pg);
}
pub(crate) fn rgx_from_file(rgx: String, src: &str, out: &str) {
    let prime_path = format!("{}", get_tmp_dir(84411254));
    if prime_path == "" {
        set_ask_user("Sorry, No access to tmp directory.. Sorry", -5611115);
        return;
    }
    let (opts, rgx) = split_once(&rgx, " ");
    let src = format!("{prime_path}/{}", full_escape(&src.to_string()));
    let out = format!("{prime_path}/{}", full_escape(&out.to_string()));
    let cmd = format!("grep {opts} '{rgx}' {src} > {out}");
    run_cmd_str(cmd.as_str());
}
pub(crate) fn rgx_from_prnt(rgx: String, out: &str) {
    rgx_from_file(rgx, "prnt", out)
}
pub(crate) fn read_rgx_from_prnt(rgx: String, out: &str) -> String {
    rgx_from_prnt(rgx, out);
    read_file(out)
}
pub(crate) fn drop_ls_mode() {
    save_file("".to_string(), "ls.mode".to_string());
    up_front_list()
}
pub(crate) fn read_front_list() -> String {
    let mut active_lst = read_file("ls.mode");
    if active_lst == "" {
        active_lst = read_file("front_list");
    }
    active_lst
}
pub(crate) fn read_front_list_but_ls() -> String {
    let active_lst = read_file("front_list");
    active_lst
}
pub(crate) fn read_proper_num_pg() -> i64 {
    let front_list = format!("{}/{}", crate::get_tmp_dir(-371011), "front_list");
    let num_pg = format!("{}.pg", read_front_list());
    let num_pg = read_file(&num_pg);
    match i64::from_str_radix(num_pg.as_str(), 10) {
        Ok(num) => return num,
        _ => return 0,
    }
}
pub(crate) struct ret0 {
    pub s: [char; 512],
    pub res: bool,
}
pub(crate) fn escape_symbs(str0: &String, func_id: i64) -> String {
    if func_id != crate::func_id18::full_escape_ {
        return str0.strn();
    }
    if no_esc_lst(str0, false).is_some() {
        return str0.strn();
    }
    if check_patch_mark(str0) || !swtch_esc(false, false) {
        return str0.to_string();
    }
    let strr = str0.as_str();
    let strr = strr.replace("-", r"\-");
    let strr = strr.replace(" ", r"\ ");
    let strr = strr.replace("$", r"\$");
    let strr = strr.replace("`", r"\`");
    let strr = strr.replace("(", r"\(");
    let strr = strr.replace("}", r"\}");
    let strr = strr.replace("{", r"\{");
    let strr = strr.replace(")", r"\)");
    let strr = strr.replace("]", r"\]");
    let strr = strr.replace("[", r"\[");
    let strr = strr.replace("&", r"\&");
    let strr = strr.replace("'", r"\'");
    let strr = strr.replace(r"\\'", r"\'");
    let strr = str::replace(&strr, ":s:", " ");
    return strr.to_string();
}
pub(crate) fn escape_symbs_no_limits(str0: &String, func_id: i64) -> String {
    let strr = str0.as_str();
    let strr = strr.replace("-", r"\-");
    let strr = strr.replace(" ", r"\ ");
    let strr = strr.replace("$", r"\$");
    let strr = strr.replace("`", r"\`");
    let strr = strr.replace("(", r"\(");
    let strr = strr.replace("}", r"\}");
    let strr = strr.replace("{", r"\{");
    let strr = strr.replace(")", r"\)");
    let strr = strr.replace("]", r"\]");
    let strr = strr.replace("[", r"\[");
    let strr = strr.replace("&", r"\&");
    let strr = strr.replace("'", r"\'");
    let strr = strr.replace(r"\\'", r"\'");
    let strr = str::replace(&strr, ":s:", " ");
    return strr.to_string();
}
pub(crate) fn escape_apostrophe(str0: &String, func_id: i64) -> String {
    if func_id != crate::func_id18::full_escape_ {
        return str0.strn();
    }
    if no_esc_lst(str0, false).is_some() {
        return str0.strn();
    }
    if check_patch_mark(str0) || !swtch_esc(false, false) {
        return str0.to_string();
    }
    return str0.as_str().replace(r"'", r"\'");
}
pub(crate) fn escape_apostrophe_no_limits(str0: &String, func_id: i64) -> String {
    return str0.as_str().replace(r"'", r"\'");
}
pub(crate) fn escape_backslash(str0: &String, func_id: i64) -> String {
    if func_id != crate::func_id18::full_escape_ {
        return str0.strn();
    }
    if no_esc_lst(str0, false).is_some() {
        return str0.strn();
    }
    if check_patch_mark(str0) || !swtch_esc(false, false) {
        return str0.to_string();
    }
    return str0.as_str().replace("\\", r"\\");
}
pub(crate) fn escape_backslash_no_limits(str0: &String, func_id: i64) -> String {
    return str0.as_str().replace("\\", r"\\");
}
pub(crate) fn full_escape(str0: &String) -> String {
    let func_id = crate::func_id18::full_escape_;
    if !swtch_esc(false, false) {
        return str0.strn();
    }
    let front_list = name_of_front_list("", false);
    if check_substrn(&front_list, "history") || front_list == "cmds" || front_list == "cmds.fp" {
        swtch_esc(true, false);
        return str0.strn();
    }
    let str0 = escape_backslash(str0, func_id);
    let str0 = escape_apostrophe(&str0, func_id);
    escape_symbs(&str0, func_id)
}
pub(crate) fn full_escape_no_limits(str0: &String) -> String {
    let func_id = crate::func_id18::full_escape_;
    let front_list = name_of_front_list("", false);
    let str0 = escape_backslash_no_limits(str0, func_id);
    let str0 = escape_apostrophe_no_limits(&str0, func_id);
    escape_symbs_no_limits(&str0, func_id)
}
pub(crate) fn check_substr(orig: &String, probe: &str, start_from: usize) -> bool {
    let func_id = 3;
    let probe: String = String::from(probe.to_string());
    let substr: &str = &orig.as_str();
    let substr = substr.substring(start_from, probe.len()).to_string();
    errMsg_dbg(&substr, func_id, -1.0);
    if probe.ne(&substr) {
        return false;
    }
    return true;
}

fn end_termios(termios: &Termios) {
    let res = match tcsetattr(0, TCSANOW, &termios) {
        Err(e) => {
            format!("{}", e)
        }
        Ok(len) => {
            format!("{:?}", len)
        }
    };
    io::stdout().flush().unwrap();
}

#[inline(always)]
pub(crate) fn custom_cmd_4_find_files(custom_cmd: String) -> bool {
    let func_id: i64 = 2;
    let mut list_of_found_files: Vec<String> = vec![];
    let output = format!("{}/found_files", unsafe {
        crate::ps18::page_struct("", crate::ps18::TMP_DIR_, -1).str_
    });
    let stopCode: String =
        unsafe { crate::ps18::page_struct("", crate::ps18::STOP_CODE_, -1).str_ };
    let mut cmd: String = format!(
        "#!/bin/bash\n{} > {};echo '{stopCode}' >> {}",
        custom_cmd, output, output
    );
    crate::run_cmd0(cmd);
    return true;
}
#[inline(always)]
pub(crate) fn find_files_ls(custom_cmd: String) -> bool {
    let func_id: i64 = 2;
    let mut list_of_found_files: Vec<String> = vec![];
    let output = format!("{}/ls", unsafe {
        crate::ps18::page_struct("", crate::ps18::TMP_DIR_, -1).str_
    });
    let stopCode: String =
        unsafe { crate::ps18::page_struct("", crate::ps18::STOP_CODE_, -1).str_ };
    let mut cmd: String = format!(
        "#!/bin/bash\n{} > {};echo '{stopCode}' >> {}",
        custom_cmd, output, output
    );
    crate::run_cmd0(cmd);
    return true;
}
pub(crate) fn find_files_ls_no_stop_code(custom_cmd: String) -> bool {
    let func_id: i64 = 2;
    let mut list_of_found_files: Vec<String> = vec![];
    let output = format!("{}/ls", unsafe {
        crate::ps18::page_struct("", crate::ps18::TMP_DIR_, -1).str_
    });
    let stopCode: String =
        unsafe { crate::ps18::page_struct("", crate::ps18::STOP_CODE_, -1).str_ };
    let mut cmd: String = format!("#!/bin/bash\n{} > {}", custom_cmd, output);
    crate::run_cmd0(cmd);
    return true;
}
//#[inline(always)]
pub(crate) fn find_files_cd(path: &String) -> bool {
    let func_id: i64 = 2;
    let mut list_of_found_files: Vec<String> = vec![];
    let output = format!(
        "{}/cd",
        crate::C!(crate::ps18::page_struct("", crate::ps18::TMP_DIR_, -1).str_)
    );
    let stopCode = getStop_code__!();
    let cmd: String = format!(
        "#!/bin/bash\n#cd\nfind -L {} -maxdepth 1 > {};echo '{stopCode}' >> {output}",
        path, output
    );
    crate::run_cmd_out_sync(cmd);
    return true;
}
pub(crate) fn find_files_cd_cpy_ls(path: &String) -> bool {
    let func_id: i64 = 2;
    let cmd = format!("find -L {path} -maxdepth 1");
    find_files_ls(cmd);
    let cd = format!(
        "{}/cd",
        crate::C!(crate::ps18::page_struct("", crate::ps18::TMP_DIR_, -1).str_)
    );
    let ls = format!(
        "{}/ls",
        crate::C!(crate::ps18::page_struct("", crate::ps18::TMP_DIR_, -1).str_)
    );
    if read_file_abs_adr(&ls) == getStop_code__!() {
        return false;
    }
    let cmd: String = format!("#!/bin/bash\n#ls cpy to cd\ncp -f {ls} {cd}");
    crate::run_cmd_out_sync(cmd);
    return true;
}
pub(crate) fn getkey() -> String {
    let mut Key: String = "".to_string();
    let xccnt = unsafe { exec_cmd_cnt(false) };
    let mut stdin = io::stdin().lock();
    let stdin_fd = 0;
    let mut stdout = io::stdout();
    let mut stdin_buf: [u8; 16] = [0; 16];
    let mut stdin_buf0: [u8; 1] = [1; 1];
    let termios = match Termios::from_fd(stdin_fd) {
        Ok(t) => t,
        _ => return "".to_string(),
    };
    let mut new_termios = termios.clone();
    stdout.lock().flush().unwrap();
    //new_termios.c_lflag &= !(ICANON | ECHO | ISIG);
    new_termios.c_lflag &= !(ICANON | ECHO);
    let enter = || {
        let enter: [u8; 1] = [13; 1];
        let mut writeIn_stdin = unsafe {
            File::from_raw_fd(0 /*stdin*/)
        };
        //writeIn_stdin.write(&enter);
        //println!("gotta enter");
    };
    //loop {
    let res = match tcsetattr(stdin_fd, TCSANOW, &new_termios) {
        Err(e) => {
            format!("{}", e)
        }
        Ok(len) => {
            format!("kkkkkkkkkkk {:#?}", len)
        }
    };
    let red_stdin = match stdin.read(&mut stdin_buf) {
        Ok(red) => red,
        Err(e) => {
            errMsg0(&format!("{e:?}"));
            return "".strn();
        }
    };
    //stdout.lock().flush().unwrap();
    let mut j = 1usize;
    /* stdin.read(&mut stdin_buf0);
        if stdin_buf0[0] == b'\x1b'{
        stdin_buf[0] = stdin_buf0[0];
            while stdin_buf0[0] != 0 && j < 8 {
                stdin.read(&mut stdin_buf0);
                stdin_buf[j] = stdin_buf0[0];
                j.inc();
            }
            dbg!(stdin_buf);
    } else {
        popup_msg("jj");
        stdin.read(&mut stdin_buf[1..]);
        stdin_buf[0] = stdin_buf0[0];
    }*/
    end_termios(&termios);
    //if crate::dirty!() {println!("len of red {:?}", red_stdin);}
    let str0 = match str::from_utf8(&stdin_buf) {
        Ok(s) => s,
        _ => "",
    };
    let msg = format!("getch {} {:?}", str0, stdin_buf);
    if stdin_buf != [0; 16] {
        let mut i = 0;
        loop {
            let ch = match str0.chars().nth(i) {
                Some(ch) => ch,
                _ => return Key,
            };
            if ch == '\0' {
                return Key;
            }
            Key.push(ch);
            i += 1;
        }
    }
    //}
    Key
}
pub(crate) fn cpy_str(in_str: &String) -> String {
    in_str.to_string()
}
pub(crate) fn complete_path(dir: &str, opts: &str, no_grep: bool) {
    let dir = dir.trim_end().trim_start();
    let proper_dir = crate::full_escape(&dir.to_string());
    update_dir_list(&proper_dir, opts, no_grep);
    let not_full_path = get_path_from_prnt(); //read_user_written_path();
    let num_of_ln_in_dir_lst = ln_of_found_files(usize::MAX).1;
    let mut get_prnt_dbg: fn(i64) -> String = get_prnt;
    let mut prnt = String::from("");
    //for i in 0..100{
    prnt.push_str(read_prnt().as_str());
    //  if prnt != ""{break;}
    //}
    if prnt.len() == 0 {
        set_ask_user("prnt is empty", -47);
    }
    if num_of_ln_in_dir_lst < 2 {
        let mut full_path = ln_of_found_files(0).0;
        let is_dir = Path::new(&full_path.strip_all_symbs()).is_dir();
        if is_dir {
            full_path.push('/');
        }
        if full_path == "no str gotten" {
            return;
        }
        prnt = prnt.replace(&not_full_path, &full_path);
        let msg = format!("prnt: {}", prnt);
        //popup_msg(msg.as_str());
        set_prnt(&prnt, -47);
        let prnt = read_prnt();
        set_ask_user(&prnt, -47);
        rewrite_user_written_path(&full_path);
        //unsafe{crate::swtch::path_completed(true, false);}
        let proper_dir = crate::full_escape(&full_path.to_string());
        update_dir_list(&proper_dir, opts, no_grep);
    }
}
pub(crate) fn update_user_written_path(e: std::io::Error) -> File {
    println!("{:?}", e);
    let user_written_path = user_wrote_path_prnt();
    let err_msg = format!(
        "update_user_written_path() can't create {}",
        user_written_path
    );
    rm_file(&user_written_path);
    File::options()
        .create_new(true)
        .write(true)
        .read(true)
        .open(user_written_path)
        .expect(&err_msg)
}
pub(crate) fn rewrite_user_written_path(new_path: &String) {
    let user_written_path = user_wrote_path();
    let err_msg = format!(
        "update_user_written_path() can't create {}",
        user_written_path
    );
    rm_file(&user_written_path);
    let mut write_path = File::options()
        .create_new(true)
        .write(true)
        .read(true)
        .open(user_written_path)
        .expect(&err_msg);
    write_path.write_all(new_path.as_bytes());
}
pub(crate) fn rm_user_written_path(func_id: i64) {
    let user_written_path = user_wrote_path_prnt();
    let err_msg = format!(
        "update_user_written_path() can't delete {}",
        user_written_path
    );
    rm_file(&user_written_path);
    let existed = Path::new(&user_written_path).exists();
    // if existed{set_ask_user(&err_msg, func_id);}
    let user_written_path = user_wrote_path();
    let err_msg = format!(
        "update_user_written_path() can't delete {}",
        user_written_path
    );
    rm_file(&user_written_path);
    let existed = Path::new(&user_written_path).exists();
    //   if existed{set_ask_user(&err_msg, func_id);}
}
pub(crate) fn rm_file(file: &String) -> bool {
    let err_msg = format!("rm_file can't remove {}", file);
    let rm_cmd = Command::new("rm")
        .arg("-f")
        .arg(file)
        .output()
        .expect(&err_msg);
    true
}
pub(crate) fn forced_rm_dir(dir: &mut String) -> bool {
    let err_msg = format!("forced_rm_dir can't remove {}", dir);
    dir.replace("//", "/");
    /*let rm_cmd = Command::new("rm").arg("-fr").arg(dir)
    .output()
    .expect(&err_msg);*/
    let rm_cmd = format!("rm -fr {dir}");
    let output = run_cmd_out_sync(rm_cmd);
    let rm_cmd = format!("rmdir {dir}");
    let output = run_cmd_out_sync(rm_cmd);
    //let output = String::from_utf8_lossy(&rm_cmd.stderr);// errMsg0(&output);
    println!("{output}");
    true
}
pub(crate) fn read_midway_data_4_ls() -> bool {
    // return true;
    let func_id = crate::func_id18::read_midway_data_;
    let mut added_indx = 0usize;
    loop {
        let stopCode = getStop_code__!();
        let filename = format!("{}/found_files", unsafe {
            crate::ps18::page_struct("", crate::ps18::TMP_DIR_, -1).str_
        });
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        for (indx, line) in reader.lines().enumerate() {
            if indx <= added_indx && added_indx > 0 {
                continue;
            }
            added_indx = indx;
            let line = line.unwrap();
            let ret = crate::globs18::add_2_front_list(&line, -1);
            //let line_dbg = get_item_from_front_list(usize_2_i64(indx), false);
            if dirty!() {
                println!("line {}", line)
            }
            crate::ps18::set_num_files0(func_id, indx);
            if line == stopCode {
                crate::ps18::fix_num_files(func_id);
                return true;
            }
        }
        if dirty!() {
            println!("midway ended")
        }
    }
    false
}
//#[io_cached]
pub(crate) fn ln_of_found_files(get_indx: usize) -> (String, usize) {
    let ls_mode = read_front_list();
    if ls_mode != "ls" {
        if !checkArg("-no-cache") {
            if get_indx < usize::MAX {
                return cached_ln_of_found_files(get_indx);
            }
        }
    }
    let stopCode = getStop_code__!();
    let filename = format!("{}/found_files", unsafe {
        crate::ps18::page_struct("", crate::ps18::TMP_DIR_, -1).str_
    });
    let file = match File::open(filename) {
        Ok(f) => f,
        _ => return ("no str gotten".to_string(), 0),
    };
    let reader = BufReader::new(file);
    let mut len = 0usize;
    for (indx, line) in reader.lines().enumerate() {
        if indx == get_indx {
            return (line.unwrap(), indx);
        }
        len = indx;
    }
    return ("no str gotten".to_string(), len);
}
pub(crate) fn ln_of_found_files01(get_indx: usize) -> (String, usize) {
    let ls_mode = read_front_list();
    let stopCode = getStop_code__!();
    let filename = format!("{}/found_files", unsafe {
        crate::ps18::page_struct("", crate::ps18::TMP_DIR_, -1).str_
    });
    let file = match File::open(filename) {
        Ok(f) => f,
        _ => return ("no str gotten".to_string(), 0),
    };
    let reader = BufReader::new(file);
    let mut len = 0usize;
    for (indx, line) in reader.lines().enumerate() {
        if indx == get_indx {
            return (line.unwrap(), indx);
        }
        len = indx;
    }
    return ("no str gotten".to_string(), len);
}
pub(crate) fn ln_of_found_files_cacheless(get_indx: usize) -> (String, usize) {
    let stopCode = getStop_code__!();
    let found_files = format!("{}/found_files", unsafe {
        crate::ps18::page_struct("", crate::ps18::TMP_DIR_, -1).str_
    });
    let file = match crate::File::open(&found_files) {
        Ok(f) => f,
        _ => return ("no str gotten".to_string(), 0),
    };
    let reader = BufReader::new(file);
    let mut len = 0usize;
    for (indx, line) in reader.lines().enumerate() {
        if indx == get_indx {
            return (line.unwrap(), indx);
        }
        len = indx;
    }
    return ("no str gotten".to_string(), len);
}
pub(crate) fn ln_of_list(get_indx: usize, list: &str) -> (String, usize) {
    /* if ls_mode != "ls"{
    if !checkArg("-no-cache") {
        if get_indx < usize::MAX{return cached_ln_of_found_files(get_indx);}}
    }*/
    let filename = take_list_adr_env(&list);
    let file = match File::open(filename) {
        Ok(f) => f,
        _ => return ("no str gotten".to_string(), usize::MAX),
    };
    let reader = BufReader::new(file);
    let mut len = 0usize;
    for (indx, line) in reader.lines().enumerate() {
        if indx == get_indx {
            return (line.unwrap(), indx);
        }
        len = indx;
    }
    return ("no str gotten".to_string(), len);
}
pub(crate) fn size_of_found_files() -> u64 {
    let stopCode = getStop_code__!();
    let filename = format!("{}/found_files", unsafe {
        crate::ps18::page_struct("", crate::ps18::TMP_DIR_, -1).str_
    });
    match fs::metadata(filename) {
        Ok(op) => op,
        _ => return 0u64,
    }
    .len()
}
pub(crate) fn get_path_from_strn(strn: String) -> String {
    let len: usize = strn.chars().count();
    let mut ret = String::new();
    let mut yes_path = false;
    for i in 0..len {
        let char0 = strn.chars().nth(i).unwrap();
        if char0 == '/' {
            yes_path = true;
        }
        if yes_path {
            ret.push(char0);
        }
    }
    ret
}
pub(crate) fn get_path_from_prnt() -> String {
    let got_path = read_prnt();
    let len: usize = got_path.chars().count();
    let mut ret = String::new();
    let mut yes_path = false;
    for i in 0..len {
        let char0 = got_path.chars().nth(i).unwrap();
        if char0 == '/' {
            yes_path = true;
        }
        if yes_path {
            ret.push(char0);
        }
    }
    ret
}
pub(crate) fn logs(data: &String, logname: &str) {
    if !checkArg("-logs") {
        return;
    }
    let args: Vec<_> = env::args().collect();
    let args_2_str = args.as_slice();
    for i in 1..args.len() {
        if args_2_str[i] == "-logs" {
            let typelog = args_2_str[i + 1].clone();
            if typelog != "" {
                if eq_str(typelog.as_str(), logname) != 0
                /*typelog != logname*/
                {
                    return;
                }
            }
            let logname = format!("logs/{logname}");
            save_file_append_newline(data, &logname);
        }
    }
}
pub(crate) fn save_file0(content: String, fname: String) -> bool {
    let fname = format!("{}/{}", crate::get_tmp_dir(-157), fname);
    let mut dir = fname.clone();
    tailOFF(&mut dir, "/");
    std::fs::create_dir(&dir);
    let cmd = format!("echo '{}' > {}", content, fname);
    run_cmd_str(cmd.as_str());
    true
}
pub(crate) fn save_file(content: String, fname: String) -> bool {
    logs(&fname, "save_file");
    let fname = format!("{}/{}", crate::get_tmp_dir(-157), fname);
    let cmd = format!("touch -f {fname}");
    //run_cmd_str(cmd.as_str());
    let anew_file = || -> File {
        rm_file(&fname);
        return File::options()
            .create_new(true)
            .write(true)
            .open(&fname)
            .expect(&fname);
    };
    let mut file: File = match File::options()
        .create(false)
        .read(true)
        .truncate(true)
        .write(true)
        .open(&fname)
    {
        Ok(f) => f,
        _ => return save_file_abs_adr0(content, fname),
    };
    file.write(content.as_bytes()).expect("save_file failed");
    true
}
pub(crate) fn rewrite_file_abs_adr(content: String, fname: String) -> bool {
    logs(&fname, "rewrite_file_abs_adr");
    let anew_file = || -> File {
        rm_file(&fname);
        return match File::options().create_new(true).write(true).open(&fname) {
            Ok(f) => f,
            _ => {
                errMsg0(&format!("Failed to create {fname}... Sorry"));
                return File::options()
                    .create_new(true)
                    .write(true)
                    .open(&fname)
                    .expect(&format!("Failed again to create {fname}... Sorry"));
            }
        };
    };
    let mut file: File = match File::options()
        .create(false)
        .read(true)
        .truncate(true)
        .write(true)
        .open(&fname)
    {
        Ok(f) => f,
        _ => return false,
    };
    file.write(content.as_bytes())
        .expect("rewrite_file_abs_adr failed");
    true
}
pub(crate) fn save_file_abs_adr0(content: String, fname: String) -> bool {
    let mut path_to_file = fname.clone();
    tailOFF(&mut path_to_file, "/");
    mkdir(path_to_file);
    let cmd = format!("touch -f {fname} 2>&1");
    run_cmd_str(cmd.as_str());
    let cmd = format!("echo '{content}' > {fname} 2>&1");
    run_cmd_str(cmd.as_str());
    true
}
//#[inline(always)]
pub(crate) fn save_file_abs_adr(content: String, fname: String) -> bool {
    let mut path_to_file = fname.clone();
    tailOFF(&mut path_to_file, "/");
    mkdir(path_to_file);
    let cmd = format!("touch -f {fname} 2>&1");
    run_cmd_str(cmd.as_str());
    let anew_file = || -> File { return File::create(&fname).expect(&fname) };
    let existing_file = || -> File {
        let timestamp = Local::now();
        let fname = format!("{}", timestamp.format("%Y-%mm-%dd_%H-%M-%S_%f"));
        return File::options()
            .create_new(true)
            .write(true)
            .open(&fname)
            .expect(&fname);
    };
    let mut file: File = match File::options().read(true).write(true).open(&fname) {
        Ok(f) => f,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => anew_file(),
            std::io::ErrorKind::AlreadyExists => {
                rm_file(&fname);
                File::create_new(&fname).unwrap()
            }
            _ => existing_file(),
        },
    };
    file.write(content.as_bytes()).expect("save_file failed");
    true
}
pub(crate) fn save_file_append_newline<T, T1>(content: T, fname: T1) -> bool
where
    T: STRN + ToString,
    T1: STRN,
{
    let content = format!("{}\n", content.strn());
    let fname = format!("{}/{}", crate::get_tmp_dir(-157), fname.strn());
    let mut dir = fname.clone();
    tailOFF(&mut dir, "/");
    let mk_dir = format!("mkdir -p {}", dir);
    run_cmd_str(mk_dir.as_str());
    let cmd = format!("touch -f {fname}");
    run_cmd_str(cmd.as_str());
    let anew_file = || -> File { return File::create(&fname).expect(&fname) };
    let existing_file = || -> File {
        let timestamp = Local::now();
        let fname = format!("{}", timestamp.format("%Y-%mm-%dd_%H-%M-%S_%f"));
        return File::options()
            .create_new(true)
            .write(true)
            .open(&fname)
            .expect(&fname);
    };
    let mut file: File = match File::options()
        .read(true)
        .append(true)
        .write(true)
        .open(&fname)
    {
        Ok(f) => f,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => anew_file(),
            std::io::ErrorKind::AlreadyExists => File::options()
                .read(true)
                .append(true)
                .write(true)
                .open(&fname)
                .unwrap(),
            _ => existing_file(),
        },
    };
    let comtent = format!("{}\n", content);
    file.write_all(content.as_bytes())
        .expect("save_file failed");
    true
}
pub(crate) fn save_file_append(content: String, fname: String) -> bool {
    let fname = format!("{}/{}", crate::get_tmp_dir(-157), fname);
    let cmd = format!("touch -f {fname}");
    //run_cmd_str(cmd.as_str());
    let anew_file = || -> File {
        rm_file(&fname);
        return File::options()
            .create_new(true)
            .write(true)
            .open(&fname)
            .expect(&fname);
    };
    let existing_file = || -> File {
        let timestamp = Local::now();
        let fname = format!("{}", timestamp.format("%Y-%mm-%dd_%H-%M-%S_%f"));
        return File::options()
            .create_new(true)
            .write(true)
            .open(&fname)
            .expect(&fname);
    };
    let mut file: File = match File::options()
        .create(false)
        .read(true)
        .append(true)
        .write(true)
        .open(&fname)
    {
        Ok(f) => f,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => anew_file(),
            std::io::ErrorKind::AlreadyExists => File::options()
                .read(true)
                .append(true)
                .write(true)
                .open(&fname)
                .unwrap(),
            _ => existing_file(),
        },
    };
    file.write(content.as_bytes()).expect("save_file failed");
    true
}
pub(crate) fn save_file_append_abs_adr(content: String, fname: String) -> bool {
    let cmd = format!("touch -f {fname}");
    //run_cmd_str(cmd.as_str());
    let anew_file = || -> File {
        return match File::options().create_new(true).write(true).open(&fname) {
            Ok(f) => f,
            Err(e) => match e.kind() {
                std::io::ErrorKind::AlreadyExists => File::options()
                    .read(true)
                    .append(true)
                    .write(true)
                    .open(&fname)
                    .unwrap(),
                _ => File::options()
                    .create_new(true)
                    .write(true)
                    .open(&fname)
                    .unwrap(),
            },
        };
    };
    let existing_file = || -> File {
        let timestamp = Local::now();
        let fname = format!("{}", timestamp.format("%Y-%mm-%dd_%H-%M-%S_%f"));
        return File::options()
            .create_new(true)
            .write(true)
            .open(&fname)
            .expect(&fname);
    };
    let mut file: File = match File::options()
        .create(false)
        .read(true)
        .append(true)
        .write(true)
        .open(&fname)
    {
        Ok(f) => f,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => anew_file(),
            std::io::ErrorKind::AlreadyExists => File::options()
                .read(true)
                .append(true)
                .write(true)
                .open(&fname)
                .unwrap(),
            _ => existing_file(),
        },
    };
    file.write(content.as_bytes()).expect("save_file failed");
    true
}
pub(crate) fn save_file_append_newline_abs_adr(content: String, fname: String) -> bool {
    let cmd = format!("touch -f {fname}");
    let content = format!("{}\n", content);
    //run_cmd_str(cmd.as_str());
    let anew_file = || -> File {
        return match File::options().create_new(true).write(true).open(&fname) {
            Ok(f) => f,
            Err(e) => match e.kind() {
                std::io::ErrorKind::AlreadyExists => File::options()
                    .read(true)
                    .append(true)
                    .write(true)
                    .open(&fname)
                    .unwrap(),
                _ => File::options()
                    .create_new(true)
                    .write(true)
                    .open(&fname)
                    .unwrap(),
            },
        };
    };
    let existing_file = || -> File {
        let timestamp = Local::now();
        let fname = format!("{}", timestamp.format("%Y-%mm-%dd_%H-%M-%S_%f"));
        return File::options()
            .create_new(true)
            .write(true)
            .open(&fname)
            .expect(&fname);
    };
    let mut file: File = match File::options()
        .create(false)
        .read(true)
        .append(true)
        .write(true)
        .open(&fname)
    {
        Ok(f) => f,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => anew_file(),
            std::io::ErrorKind::AlreadyExists => File::options()
                .read(true)
                .append(true)
                .write(true)
                .open(&fname)
                .unwrap(),
            _ => existing_file(),
        },
    };
    file.write(content.as_bytes()).expect("save_file failed");
    true
}
pub(crate) fn save_file_append_newline_abs_adr_fast(content: &String, fname: &String) -> bool {
    let cmd = format!("touch -f {fname}");
    let content = format!("{}\n", content);
    //run_cmd_str(cmd.as_str());
    let anew_file = || -> File {
        return match File::options().create_new(true).write(true).open(&fname) {
            Ok(f) => f,
            Err(e) => match e.kind() {
                std::io::ErrorKind::AlreadyExists => File::options()
                    .read(true)
                    .append(true)
                    .write(true)
                    .open(&fname)
                    .unwrap(),
                _ => File::options()
                    .create_new(true)
                    .write(true)
                    .open(&fname)
                    .unwrap(),
            },
        };
    };
    let existing_file = || -> File {
        let timestamp = Local::now();
        let fname = format!("{}", timestamp.format("%Y-%mm-%dd_%H-%M-%S_%f"));
        return File::options()
            .create_new(true)
            .write(true)
            .open(&fname)
            .expect(&fname);
    };
    let mut file: File = match File::options()
        .create(false)
        .read(true)
        .append(true)
        .write(true)
        .open(&fname)
    {
        Ok(f) => f,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => anew_file(),
            std::io::ErrorKind::AlreadyExists => File::options()
                .read(true)
                .append(true)
                .write(true)
                .open(&fname)
                .unwrap(),
            _ => existing_file(),
        },
    };
    file.write(content.as_bytes()).expect("save_file failed");
    true
}
pub(crate) fn read_file(fname: &str) -> String {
    let fname = format!("{}/{}", crate::get_tmp_dir(-157), fname);
    //let err = format!("failed to read {}", fname);
    let mut file: File = match File::open(&fname) {
        Ok(f) => f,
        Err(e) => return "".to_string(), //format!("{:?}", e)
    };
    let mut ret = String::new();
    file.read_to_string(&mut ret);
    ret.trim_end().to_string()
}
pub(crate) fn raw_read_file(fname: &str) -> String {
    let fname = format!("{}/{}", crate::get_tmp_dir(-157), fname);
    //let err = format!("failed to read {}", fname);
    let mut file: File = match File::open(&fname) {
        Ok(f) => f,
        Err(e) => return "".to_string(), //format!("{:?}", e)
    };
    let mut ret = String::new();
    file.read_to_string(&mut ret);
    ret
}
pub(crate) fn read_file_abs_adr(fname: &String) -> String {
    //let err = format!("failed to read {}", fname);
    let path = Path::new(fname);
    let mut file: File = match File::open(&path) {
        Ok(f) => f,
        Err(e) => return "".to_string(), //format!("{:?}", e)
    };
    let mut ret = String::new();
    file.read_to_string(&mut ret);
    ret.trim_end().to_string()
}
pub(crate) fn read_file_abs_adr0(fname: &String) -> String {
    let cmd = format!("cat {fname}");
    run_cmd_out(cmd)
}
pub(crate) fn raw_read_file_abs_adr(fname: &String) -> String {
    //let err = format!("failed to read {}", fname);
    let mut file: File = match File::open(fname) {
        Ok(f) => f,
        Err(e) => return "".to_string(), //format!("{:?}", e)
    };
    let mut ret = String::new();
    file.read_to_string(&mut ret);
    ret
}
pub(crate) fn raw_read_prnt() -> String {
    raw_read_file("prnt")
}
pub(crate) fn read_prnt() -> String {
    read_file("prnt")
}
pub(crate) fn file_prnt(content: String) {
    save_file(cpy_str(&content), "prnt".to_string());
    let path = get_path_from_strn(content);
    save_file(path, "user_wrote_path".to_string());
}
pub(crate) fn position_of_slash_in_prnt() -> usize {
    let got_path = read_prnt();
    let ret = usize::MAX;
    let mut i = 0usize;
    loop {
        let char0 = match got_path.chars().nth(i) {
            Some(ch) => ch,
            _ => return ret,
        };
        if char0 == '/' {
            return i;
        }
        i += 1;
    }
}
pub(crate) fn i64_2_usize(v: i64) -> usize {
    let mut ret = 0usize;
    let unit = 1i64;
    let shl = 1usize;
    let mut v = v;
    let i64_len: usize = size_of::<i64>() * 8;
    for i in 0..i64_len {
        if v == 0 {
            break;
        }
        if v & unit == 1 {
            ret += (shl << i);
        }
        v = v >> 1;
    }
    ret
}
pub(crate) fn usize_2_i64(v: usize) -> i64 {
    let mut ret = 0i64;
    let unit = 1usize;
    let shl = 1i64;
    let mut v = v;
    let usize_len: usize = size_of::<usize>() * 8;
    for i in 0..usize_len {
        if v == 0 {
            break;
        }
        if v & unit == 1 {
            ret += (shl << i);
        }
        v = v >> 1;
    }
    ret
}
pub(crate) fn getch() -> char {
    let mut ch: char = '\0';
    let mut stdin = io::stdin();
    let stdin_fd = 0;
    let mut stdout = io::stdout();
    let mut stdin_buf: [u8; 6] = [0; 6];
    let termios = Termios::from_fd(stdin_fd).unwrap();
    let mut new_termios = termios.clone();
    stdout.lock().flush().unwrap();
    new_termios.c_lflag &= !(ICANON | ECHO);
    let enter = || {
        let enter: [u8; 1] = [13; 1];
        let mut writeIn_stdin = unsafe {
            File::from_raw_fd(0 /*stdin*/)
        };
        writeIn_stdin.write(&enter);
        println!("gotta enter");
    };
    loop {
        let res = match tcsetattr(stdin_fd, TCSANOW, &new_termios) {
            Err(e) => {
                format!("{}", e)
            }
            Ok(len) => {
                format!("kkkkkkkkkkk {:#?}", len)
            }
        };
        let red_stdin = stdin.read(&mut stdin_buf);
        stdout.lock().flush().unwrap();
        end_termios(&termios);
        if crate::dirty!() {
            println!("len of red {:?}", red_stdin.unwrap());
        }
        let str0 = match str::from_utf8(&stdin_buf) {
            Ok(s) => s,
            _ => "",
        };
        let msg = format!("getch {} {:?}", str0, stdin_buf);
        achtung(&msg);
        if stdin_buf != [0; 6] {
            return str0.chars().nth(0).unwrap();
        }
    }
    ch
}
pub(crate) fn popup_msg(msg: &str) {
    if crate::checkArg("-no-popup") {
        return;
    }
    let msg = format!("notify-send '{}'", msg);
    crate::run_cmd_str(&msg);
}
pub(crate) fn achtung(msg: &str) {
    if !crate::checkArg("-dbg") || !crate::checkArg("-use-achtung") {
        return;
    }
    let msg = format!("notify-send '{}'", msg);
    crate::run_cmd_str(&msg);
}
pub(crate) fn calc_num_files_up2_cur_pg() -> i64 {
    let func_id = crate::func_id18::calc_num_files_up2_cur_pg_;
    let ps = unsafe { crate::swtch::swtch_ps(-1, None) };
    let mut num_page;
    if ps.num_page != i64::MAX {
        num_page = ps.num_page;
    } else {
        num_page = crate::get_num_page(func_id);
    }
    let mut num_cols;
    if ps.num_cols != i64::MAX {
        num_cols = ps.num_cols;
    } else {
        num_cols = crate::get_num_cols(func_id);
    }
    let mut num_rows;
    if ps.num_rows != i64::MAX {
        num_rows = ps.num_rows;
    } else {
        num_rows = crate::get_num_rows(func_id);
    }
    if ps.col_width != i64::MAX {
        crate::set_col_width(ps.col_width, func_id);
    }
    //let num_items_on_pages = num_cols * num_rows; let stopCode: String = crate::getStop_code__!();
    let counted_files = num_page * num_cols * num_rows;
    return counted_files;
}
pub(crate) fn calc_num_files_up2_cur_pg01() -> i64 {
    let func_id = crate::func_id18::calc_num_files_up2_cur_pg_;
    let ps = unsafe { crate::swtch::swtch_ps(-1, None) };
    let mut num_page;
    if ps.num_page != i64::MAX {
        num_page = ps.num_page;
    } else {
        num_page = crate::get_num_page(func_id);
    }
    let mut num_cols;
    if ps.num_cols != i64::MAX {
        num_cols = ps.num_cols;
    } else {
        num_cols = crate::get_num_cols(func_id);
    }
    let mut num_rows;
    if ps.num_rows != i64::MAX {
        num_rows = ps.num_rows;
    } else {
        num_rows = crate::get_num_rows(func_id);
    }
    if ps.col_width != i64::MAX {
        crate::set_col_width(ps.col_width, func_id);
    }
    //let num_items_on_pages = num_cols * num_rows; let stopCode: String = crate::getStop_code__!();
    let counted_files = (num_page + 1) * num_cols * num_rows;
    return counted_files.clone().dec();
}
pub(crate) fn check_substring(orig: String, probe: String, start_from: usize) -> bool {
    let substr: &str = &orig.as_str();
    let substr = substr.substring(start_from, probe.len() - 1).to_string();
    if probe.ne(&substr) {
        return false;
    }
    return true;
}
pub(crate) fn put_in_name() -> String {
    // return dbg_put_in_name();
    //#[cfg(in_dbg="in-dbg")]
    #![inline(always)]
    if checkArg("-dbg1") || checkArg("-dbg") {
        return crate::core18::put_in_name__dbg();
    }
    let mut ret: String = String::new();
    let len_of_cmd_line = env::args().len() - 1;
    let args: Vec<String> = env::args().collect();
    let i: i64 = 0;
    for i in 0..len_of_cmd_line {
        if args[i] == "-in_name".to_string() || args[i] == "-in-name".to_string() {
            let cmd = format!("|{}", crate::form_grep_cmd(&args[i + 1]));
            ret.push_str(&cmd);
        }
    }
    return ret;
}
//#[cfg(in_dbg="in-dbg")]
#[inline(always)]
pub(crate) fn put_in_name__dbg() -> String {
    popup_msg("run dbg_put_in_name");
    let mut ret: String = String::new();
    let len_of_cmd_line = env::args().len();
    let args: Vec<String> = env::args().collect();
    let i: i64 = 0;
    let dbg = format!("put_in_name::len_cmd_line {}", len_of_cmd_line);
    achtung(&dbg);
    save_file_abs_adr(dbg, "/tmp/dbg_put_in_name0".to_string());
    for i in 0..len_of_cmd_line {
        if args[i] == "-in_name".to_string() || args[i] == "-in-name".to_string() {
            let cmd = format!("|{}", crate::form_grep_cmd(&args[i + 1]));
            ret.push_str(&cmd);
            println!("put_in_name::{i} {}", ret);
        }
    }
    println!("put_in_name:: {}", ret);
    let dbg = format!("put_in_name:: {}", ret);
    let dbg1 = dbg.clone();
    save_file_abs_adr(dbg, "/tmp/dbg_put_in_name".to_string());
    popup_msg(&ret);
    //return panic!("{}", crate::cpy_str(&dbg1));
    return ret;
}
pub(crate) fn ins_newlines(len_of_line: usize, origString: &mut String) {
    let num_of_loops = origString.len() / len_of_line;
    for i in 1..num_of_loops {
        let indx = i * len_of_line;
        origString.push('\n');
        ins_last_char_to_string1_from_string1_ptr(indx, origString);
    }
}
pub(crate) fn raw_ren_file(src: String, dst: String) {
    let cmd = format!("mv {src} {dst}");
    run_cmd_str(cmd.as_str());
}
pub(crate) fn mkdir(name: String) {
    #[cfg(feature = "in_dbg")]
    if name.len() > 20 && check_substrn(name.substring(2, name.len()), "dev") {
        crate::in_dbg0::just_break();
    }
    let name = full_escape(&name);
    let cmd = format!("mkdir -p {name}");
    run_cmd_str(cmd.as_str());
}
pub(crate) fn raw_mkdir(name: String) {
    let out = format!("{}/raw_mkdir.out", get_tmp_dir(4512154749974));
    let cmd = format!("mkdir -p {name} > {out} 2>&1");
    run_cmd_str(cmd.as_str());
}
pub(crate) fn path_exists(path: String) -> bool {
    let timestamp = Local::now();
    let proper_timestamp = format!("{}", timestamp.format("%Y-%mm-%dd_%H-%M-%S_%f"));
    let out: String = format!(
        "{}/path_exists{}.out",
        get_tmp_dir(10008745),
        proper_timestamp
    );
    let cmd = format!("find -L {path}");
    let is_path = run_cmd_out(cmd);
    let path = path.len(); //chars().count();
                           //clear_screen();
                           //println!("{is_path}");
    let is_path = is_path.len() + 1; //chars().count();
                                     //println!("{path} and {is_path}");
                                     //getkey();
    if is_path == path {
        return true;
    }
    false
}
pub(crate) fn link_list_2_front(name: &str) {
    let front = take_list_adr("found_files");
    let list = take_list_adr_env(name);
    let cmd = format!("ln -sf {list} {front}");
    run_cmd_str(cmd.as_str());
}
pub(crate) fn from_ls_2_front(ls_mode: String) {
    let front = read_front_list_but_ls();
    //let ls_mode = take_list_adr("ls.mode");
    rm_file(&ls_mode);
    set_front_list(front.as_str());
    C!(crate::swtch::swtch_fn(0, "".to_string()));
}
pub(crate) fn tailOFF(strn: &mut String, delim: &str) -> bool {
    let len = strn.chars().count();
    let mut ret = String::new();
    let empty = String::new();
    let delim = delim.to_string().chars().nth(0).unwrap();
    for i in 0..len {
        let ch = match strn.chars().nth(i) {
            Some(ch) => ch,
            _ => empty.chars().nth(0).unwrap(),
        };
        if ch == delim && i < len - 1 {
            ret = "".to_string();
            continue;
        }
        ret.push(ch);
    }
    if ret.len() == 0 {
        return false;
    }
    //*strn = strn.replace(&ret, "").trim_end_matches(delim).to_string();
    let ret_delim = format!("{ret}{delim}sss");
    strn.push_str("sss");
    ret.push_str("sss");
    *strn = strn.replace(&ret_delim, "");
    *strn = strn.replace(&ret, "");
    true
}
pub(crate) fn read_tail(strn: &String, delim: &str) -> String {
    let len = strn.chars().count();
    let mut ret = String::new();
    let empty = String::new();
    let delim = delim.to_string().chars().nth(0).unwrap();
    for i in 0..len {
        let ch = match strn.chars().nth(i) {
            Some(ch) => ch,
            _ => empty.chars().nth(0).unwrap(),
        };
        if ch == delim && i < len - 1 {
            ret = "".to_string();
            continue;
        }
        ret.push(ch);
    }
    ret
}
pub(crate) fn is_dir(path: &String) -> bool {
    if checkArg("-dbg1") {
        return dbg_is_dir(path);
    }
    if checkArg("-dbg") || checkArg("-dbg3") {
        return dbg_is_dir3(path);
    }
    let last_symb_indx = path.chars().count() - 1;
    let slash = path.chars().nth(last_symb_indx).unwrap();
    let is_slash = String::from(slash);
    if is_slash != "/" {
        if checkArg("-dbg2") {
            return dbg_is_dir2(path);
        }
        return is_dir2(path);
    }
    true
}
pub(crate) fn dbg_is_dir(path: &String) -> bool {
    let last_symb_indx = path.chars().count() - 1;
    let slash = path.chars().nth(last_symb_indx).unwrap();
    let is_slash = String::from(slash);
    popup_msg(&format!("is_slash = {is_slash}"));
    if is_slash != "/" {
        if checkArg("-dbg2") {
            return dbg_is_dir2(path);
        }
        return is_dir2(path);
    }
    true
}

pub(crate) fn dbg_is_dir3(path: &String) -> bool {
    let c_path: std::ffi::CString = match CString::new(path.to_string()) {
        Ok(s) => s,
        _ => CString::new("/").unwrap(),
    };
    unsafe {
        let mut stat: libc::stat = std::mem::zeroed();
        if libc::stat(c_path.as_ptr(), &mut stat) >= 0 {
            popup_msg(&format!("{}", stat.st_mode));
        }
    };
    true
}
pub(crate) fn dbg_is_dir2(path: &String) -> bool {
    popup_msg("mjj");
    let maybe_dir = full_escape(&format!("{path}/."));
    let cmd = format!("find -L {maybe_dir} -maxdepth 1|grep -io {maybe_dir}|uniq");
    let ret = run_cmd_out(cmd);
    println!("dbg_is_dir2 ==>> {}", ret);
    popup_msg(&ret);
    save_file(ret.clone(), "dbg_is_dir2".to_string());
    if ret == "" {
        return false;
    }
    true
}
pub(crate) fn is_dir2(path: &String) -> bool {
    let maybe_dir = full_escape(&format!("{path}/."));
    let cmd = format!("find -L {maybe_dir} -maxdepth 1|grep -Eio '/\\.'|uniq");
    //let cmd = cmd.replace(r"\'/.", r"\'/\\.");
    let ret = run_cmd_out_sync(cmd).trim_end().trim_start().to_string();
    logs(&ret, "is_dir2");
    if ret.len() == 2 {
        return true;
    }
    false
}
//fn
