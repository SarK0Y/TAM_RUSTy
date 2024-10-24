use chrono::format::format;
use colored::Colorize;
use num_traits::{FloatErrorKind, ToPrimitive};
use once_cell::sync::{Lazy, OnceCell};
use std::{
    fs::File,
    i64,
    io::{prelude::*, BufReader, BufWriter, ErrorKind, Read, Write},
    os::fd::AsRawFd,
    path::Path,
    usize,
};
use substring::Substring;
pub const SWTCH_RUN_VIEWER: i64 = 0;
pub const SWTCH_USER_WRITING_PATH: i64 = 1;
use crate::{
    add_cmd_in_history, complete_path,
    core18::{errMsg, get_path_from_prnt, update_user_written_path},
    cpy_str, drop_ls_mode, edit_mode_lst, escape_symbs, full_escape,
    func_id18::{mk_cmd_file_, viewer_, where_is_last_pg_},
    get_prnt,
    globs18::{get_item_from_front_list, set_ls_as_front, take_list_adr, FRONT_},
    mark_front_lst, no_view, path_exists,
    pg18::form_cmd_line_default,
    position_of_slash_in_prnt,
    ps18::{
        child2run, get_full_path, get_num_files, get_num_page, init_page_struct, page_struct_ret,
        set_ask_user,
    },
    raw_ren_file, read_file, read_front_list_but_ls, read_rgx_from_prnt, run_term_app, save_file,
    set_front_list, set_full_path, split_once, tui_or_not,
    update18::update_dir_list,
    usize_2_i64, STRN,
};
pub(crate) unsafe fn check_mode(mode: &mut i64) {
    static mut state: i64 = 0;
    if *mode == -1 {
        *mode = state;
    }
    state = *mode;
}
pub(crate) unsafe fn swtch_fn(indx: i64, cmd: String) {
    static mut fst_run: bool = true;
    static mut fn_indx: usize = 0;
    static mut fn_: OnceCell<Vec<fn(String) -> bool>> = OnceCell::new();
    if edit_mode_lst(None) {
        return;
    }
    let mut ls_mode = "".strn();
    let mut indx = indx;
    if fn_indx != crate::swtch::SWTCH_USER_WRITING_PATH as usize {
        ls_mode = read_file("ls_mode");
        if ls_mode == "ls" {
            fn_indx = crate::swtch::SWTCH_USER_WRITING_PATH as usize;
            indx != crate::swtch::SWTCH_USER_WRITING_PATH;
        }
    }
    if fst_run {
        let fn_vec: Vec<fn(String) -> bool> = Vec::new();
        fn_.set(fn_vec);
        fst_run = false;
        fn_.get_mut().unwrap().push(run_viewer); // 0
        fn_.get_mut().unwrap().push(user_writing_path); // 1
    }
    if indx < -1 {
        let indx = crate::i64_2_usize(crate::set(indx)) - 2;
        let len = fn_.get().expect("Can't unwrap fn_ in swtch_fn").len();
        if indx > len {
            set_ask_user("indx gets out of fn_ ", -178);
            return;
        }
        fn_.get().unwrap()[indx](cmd);
        let mut indx = usize_2_i64(indx);
        check_mode(&mut indx);
        return;
    }
    if indx > -1 {
        fn_indx = indx.to_usize().unwrap();
        return;
    }
    //  if indx > -1 && !cmd.is_empty(){fn_indx = indx.to_usize().unwrap();}
    let mut mode = indx;
    check_mode(&mut mode);
    let dont_run = crate::cmd_keys::dont_run_file(None);
    if dont_run && fn_indx == crate::swtch::SWTCH_RUN_VIEWER as usize {
        return;
    }
    fn_.get().unwrap()[fn_indx](cmd);
}
pub(crate) unsafe fn swtch_ps(indx: i64, ps: Option<crate::_page_struct>) -> crate::_page_struct {
    static mut fst_run: bool = true;
    static mut ps_indx: usize = 0;
    static mut ps_: OnceCell<Vec<crate::_page_struct>> = OnceCell::new();
    let dummy = init_page_struct();
    if fst_run {
        let ps_vec: Vec<crate::_page_struct> = Vec::new();
        ps_.set(ps_vec);
        fst_run = false;
    }
    if ps.is_some() {
        ps_.get_mut().unwrap().push(ps.unwrap());
    }
    if indx > -1 {
        ps_indx = indx.to_usize().unwrap();
        return dummy;
    }
    return crate::cpy_page_struct(&mut ps_.get_mut().unwrap()[ps_indx]);
}
pub(crate) fn renFile() -> bool {
    // crate::save_file("".to_string(), "prev_list".to_string());
    let front_list = read_front_list_but_ls();
    set_front_list(&front_list);
    let actual_link_to = check_symlink();
    if actual_link_to == "ls" {
        set_ask_user(
            &"Wrong list was activated".bold().red().to_string(),
            12154487,
        );
        return false;
    }
    let rgx = "-Eio ren\\s+[0-9]+".to_string();
    let prnt = crate::read_prnt();
    let head = read_rgx_from_prnt(cpy_str(&rgx), "head_of_prnt").replace("\n", "");
    if head == "" {
        set_ask_user(
            "example: ren <indx of file> <new name <short one or full path to>>",
            1003671,
        );
        return false;
    }
    let (_, file_indx) = split_once(&head, " ");
    let file_indx = file_indx.trim_start_matches(" ");
    let file_indx = file_indx.trim_end_matches("\n");
    let file_indx = file_indx.trim_end_matches(" ");
    let file_indx = match i64::from_str_radix(&file_indx, 10) {
        Ok(n) => n,
        _ => 0i64,
    };
    let old_name = full_escape(&get_item_from_front_list(file_indx, true));
    if old_name.len() == 0 || Path::new(&old_name).is_dir() {
        set_ask_user(&"file has wrong type".bold().red().to_string(), 12154487);
        return false;
    }
    let mut new_name = full_escape(
        &prnt
            .replace(&crate::cpy_str(&head), "")
            .trim_start_matches(" ")
            .to_string(),
    );
    let is_last_ch_slash = new_name.chars().count() - 1;
    let is_last_ch_slash = new_name.chars().nth(is_last_ch_slash).unwrap().to_string();
    let mut slash = "".to_string();
    if is_last_ch_slash == "/" {
        slash.push('/');
    }
    if is_last_ch_slash == "/" || Path::new(&new_name).is_dir() {
        let fname = Path::new(&old_name).file_name().unwrap().to_str().unwrap();
        new_name = format!("{new_name}{slash}{fname}");
    }
    save_file(cpy_str(&old_name), "old_name".to_string());
    save_file(cpy_str(&new_name), "new_name".to_string());
    if Path::new(&new_name).exists() {
        crate::clear_screen();
        let err_msg = format!(
            "{} is existed. Would You like to overwrite it? Type 'Yes, I do' or n.",
            new_name
        )
        .bold()
        .red();
        println!("{err_msg}");
        let mut ans = String::new();
        crate::io::stdin()
            .read_line(&mut ans)
            .expect("renFile failed to read console");
        if ans == "Yes, I do" {
            set_ask_user("moving file..", -74554152);
            crate::globs18::renew_lists(cpy_str(&new_name));
            raw_ren_file(old_name, new_name);
            return true;
        } else {
            return false;
        }
    }
    let mut path_2_new = Path::new(&new_name);
    let is_dir = new_name.chars().count() - 1;
    let is_dir = new_name.chars().nth(is_dir);
    if is_dir
        .expect("failed to unwrap is_dir in renFile")
        .to_string()
        != "/"
    {
        path_2_new = match Path::new(&new_name).parent() {
            Some(n) => n,
            _ => Path::new(""),
        };
    }
    let path_2_new0 = path_2_new.to_str().unwrap().to_string();
    if !path_exists(cpy_str(&path_2_new0)) {
        crate::raw_mkdir(cpy_str(&path_2_new0));
        if !path_exists(cpy_str(&path_2_new0)) {
            let err_msg = format!("renFile failed to create {path_2_new0}");
            save_file(cpy_str(&err_msg), "renFile.err".to_string());
            errMsg(&err_msg, -1125414);
            return false;
        }
    }
    let msg = format!("new name: {new_name}");
    set_ask_user(&msg, -74554152);
    let msg = format!("old name: {old_name}");
    set_full_path(&msg, -74554152);
    crate::globs18::renew_lists(cpy_str(&new_name));
    raw_ren_file(old_name, new_name);
    true
}
fn viewer_n_adr(app: String, file: String) -> bool {
    use crate::custom_traits::STRN_strip;
    let func_id = crate::func_id18::viewer_;
    if app == "none" {
        crate::core18::errMsg(
            "To run file w/ viewer, You need to type '<indx of viewer> /path/to/file'",
            func_id,
        );
        return false;
    }
    let msg = || -> bool {
        crate::core18::errMsg(
            "To run file w/ viewer, You need to type '<indx of viewer> <index of file>'",
            func_id,
        );
        return false;
    };
    let app_indx = match usize::from_str_radix(app.as_str(), 10) {
        Ok(v) => v,
        _ => return msg(),
    };
    let filename_len = file.chars().count();
    let mut file = file;
    let patch_mark_len = "::patch".to_string().chars().count();
    if crate::Path::new(&file).exists() {
        file = full_escape(&file);
    } else {
        file.strip_all_symbs();
    }
    let viewer = get_viewer(app_indx, -1, true);
    let mut cmd = String::new();
    cmd = format!("{} {} > /dev/null 2>&1", viewer, file);
    add_cmd_in_history(&format!("term {cmd}"));
    if crate::term_app::run_new_win_bool ( None ) { crate::term_app::new0__(&cmd ); return true;}
    if tui_or_not(cpy_str(&cmd), &mut file) {
            cmd = format!("{} {}", viewer, file);
            return run_term_app(cmd);
        }
        return crate::run_cmd_viewer(cmd);
    }
    pub(crate) fn run_viewer(cmd: String) -> bool {
        let mut cmd = cmd;
        if crate::term_app::run_new_win_bool( None ) {
            cmd = cmd.substring(1, usize::MAX).strn();
        }
        if stop_run_viewer(&cmd) {
            return false;
        }
        let func_id = crate::func_id18::viewer_;
        if cmd.as_str().substring(0, 1) == "/" {
            let app_indx = "0".to_string();
            let file_indx = cmd;
            return viewer_n_adr(app_indx, file_indx);
        }
        let (mut app_indx, mut file_indx) = crate::split_once(&cmd, " ");
        if app_indx == "none" || file_indx == "none" {
            set_ask_user(
                "To run file w/ viewer, You need to type '<indx of viewer> <index of file>'",
                func_id,
            );
        }
        if file_indx == "none" {
            file_indx = app_indx;
            app_indx = 0.to_string();
        }
        if file_indx.as_str().substring(0, 1) == "/" {
            return viewer_n_adr(app_indx, file_indx);
        }
        if app_indx.as_str().substring(0, 1) == "/" {
            file_indx = app_indx;
            app_indx = 0.to_string();
            return viewer_n_adr(app_indx, file_indx);
        }
        let msg = || -> bool {
            crate::core18::errMsg(
                "To run file w/ viewer, You need to type '<indx of viewer> <index of file>'",
                func_id,
            );
            return false;
        };
        let app_indx = match usize::from_str_radix(app_indx.as_str(), 10) {
            Ok(v) => v,
            _ => return msg(),
        };
        let file_indx = match i64::from_str_radix(file_indx.as_str(), 10) {
            Ok(v) => v,
            _ => return msg(),
        };
        //let file_indx: i64 = crate::globs18::get_proper_indx(file_indx).1;
        let mut filename = get_item_from_front_list(file_indx, true);
        let filename_len = filename.chars().count();
        let patch_mark_len = "::patch".to_string().chars().count();
        if filename_len > patch_mark_len
            && filename.substring(filename_len - patch_mark_len, filename_len) == "::patch"
        {
            filename = filename.replace("::patch", "");
        } else {
            filename = full_escape(&filename);
        }
        let viewer = get_viewer(app_indx, -1, true);
        let mut cmd = format!("{} {} > /dev/null 2>&1", viewer, filename);
        add_cmd_in_history(&format!("term {cmd}"));
        if tui_or_not(cpy_str(&cmd), &mut filename) || tui_mode(app_indx, None).unwrap() {
            cmd = format!("{} {}", viewer, filename);
            if crate::term_app::run_new_win_bool ( None ) { crate::term_app::new0__(&cmd ); return true}
            return run_term_app(cmd);
        }
        return crate::run_cmd_viewer(cmd);
    }
    pub(crate) fn get_viewer(indx: usize, func_id: i64, thread_safe: bool) -> String {
        if mode_default_viewers(None) {
            return "xdg-open".strn();
        }
        let mut func_id_loc = func_id;
        if thread_safe {
            let rnd = get_rnd_u64();
            let mut msk: u64 = !u64::from(1u64 << 63);
            let msg = format!("{:b}", msk);
            if crate::dirty!() {
                println!("{}", msg.as_str());
            }
            let mut handle_err = move || -> i64 {
                let ret = msk & rnd.0;
                let ret = ret.to_i64().unwrap();
                msk = 0;
                return ret;
            };
            if !rnd.1 {
                errMsg("/dev/urandom and /dev/random either don't exist or aren't achivable on Your system", func_id);
                return "none".to_string();
            }
            func_id_loc = match rnd.0.to_i64() {
                Some(v) => v,
                _ => handle_err(),
            };
            if msk == 0 {
                func_id_loc *= -1;
            }
        }

    let ret = unsafe { share_usize(indx, func_id_loc) };
    if ret.1 {
        return unsafe { 
            { let viewer = crate::page_struct("", crate::VIEWER_, func_id_loc).str_; crate::nvim::add_keys_2_cmd( &viewer ) }
        };
    }
    "locked".to_string()
}
    pub(crate) fn display_viewer(indx: usize, func_id: i64, thread_safe: bool) -> String {
        if mode_default_viewers(None) {
            return "xdg-open".strn();
        }
        let mut func_id_loc = func_id;
        if thread_safe {
            let rnd = get_rnd_u64();
            let mut msk: u64 = !u64::from(1u64 << 63);
            let msg = format!("{:b}", msk);
            if crate::dirty!() {
                println!("{}", msg.as_str());
            }
            let mut handle_err = move || -> i64 {
                let ret = msk & rnd.0;
                let ret = ret.to_i64().unwrap();
                msk = 0;
                return ret;
            };
            if !rnd.1 {
                errMsg("/dev/urandom and /dev/random either don't exist or aren't achivable on Your system", func_id);
                return "none".to_string();
            }
            func_id_loc = match rnd.0.to_i64() {
                Some(v) => v,
                _ => handle_err(),
            };
            if msk == 0 {
                func_id_loc *= -1;
            }
        }
    let ret = unsafe { share_usize(indx, func_id_loc) };
    if ret.1 {
        return unsafe { 
             let viewer = crate::page_struct("", crate::VIEWER_, func_id_loc).str_; 
                if mode_display_full_names_of_viewers ( None ) {crate::nvim::add_keys_2_cmd( &viewer ) } else { viewer }
        };
    }
    "locked".to_string()
}

pub fn mode_default_viewers(mode: Option<bool>) -> bool {
    static mut state: bool = false;
    unsafe {
        if let Some(x) = mode {
            state = x;
        }
        state
    }
}
pub fn roll_header () {
    mode_display_full_names_of_viewers( Some ( 
        !mode_display_full_names_of_viewers(None )
    ) );
}
pub fn mode_display_full_names_of_viewers(mode: Option<bool>) -> bool {
    static mut state: bool = false;
    unsafe {
        if let Some(x) = mode {
            state = x;
        }
        state
    }
}

pub fn tui_mode(indx: usize, set_state: Option<bool>) -> Option<bool> {
    static mut mode_to_run_app: Lazy<Vec<bool>> = Lazy::new(|| vec![]);
    unsafe {
        if let Some(x) = set_state {
            mode_to_run_app.push(x);
            return None;
        }
        Some(mode_to_run_app[indx])
    }
}
pub(crate) fn get_num_of_viewers(func_id: i64) -> i64 {
    return unsafe { crate::page_struct("", crate::NUM_OF_VIEWERS, func_id).int };
}
pub(crate) fn add_viewer(val: &str, func_id: i64) -> String {
    return unsafe { crate::page_struct(val, crate::set(crate::VIEWER_), func_id).str_ };
}
pub(crate) unsafe fn share_usize(val: usize, func_id: i64) -> (usize, bool) {
    static mut owner_id: i64 = i64::MIN;
    static mut actual_val: usize = 0;
    if owner_id == func_id && val == usize::MAX {
        owner_id = i64::MIN;
        return (actual_val, true);
    }
    if owner_id == i64::MIN {
        owner_id = func_id;
        actual_val = val;
        return (val, true);
    }
    (usize::MAX, false)
}
unsafe fn drop_2_dev_null() -> bool {
    static mut drop: bool = true;
    drop = !drop;
    return !drop;
}
pub(crate) unsafe fn path_completed(set_state: bool, ret_state: bool) -> bool {
    static mut state: bool = false;
    if ret_state {
        return state;
    }
    state = set_state;
    state
}
pub(crate) unsafe fn front_list_indx(val: i64) -> (i64, bool) {
    static mut actual_indx: i64 = FRONT_;
    if val == i64::MAX {
        return (actual_indx, true);
    }
    //if val > -1 {actual_indx = val; return (val, true);}
    (i64::MAX, false)
}

pub(crate) unsafe fn local_indx(set_new_state: bool) -> bool {
    static mut actual_state: bool = true;
    if !set_new_state {
        return actual_state;
    }
    actual_state = !actual_state;
    return actual_state;
}
pub(crate) fn get_rnd_u64() -> (u64, bool) {
    let mut get_rnd_device: File = match File::open("/dev/urandom") {
        Ok(File) => File,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::open("/dev/random") {
                Ok(File) => File,
                _ => return (0, false),
            },
            _ => return (0, false),
        },
    };
    let mut buf: [u8; 8] = [0; 8];
    get_rnd_device.read(&mut buf);
    let mut rnd_u64: u64 = 0;
    for i in 0..buf.len() {
        let shl = u64::from(*buf.get(i).unwrap());
        rnd_u64 += u64::from(shl << i * 8);
    }
    return (rnd_u64, true);
}
pub(crate) unsafe fn form_list_of_viewers(drop_1st_run: bool) {
    static mut fst_run: bool = true;
    if drop_1st_run {
        fst_run = true;
    }
    if !fst_run {
        return;
    }
    fst_run = false;
    let args: Vec<_> = crate::env::args().collect();
    let arg = args.as_slice();
    for i in 1..args.len() {
        if arg[i] == "-view_w" || arg[i] == "-view-w" || arg[i] == "-tui-app" {
            let viewer: String = (args[i + 1]).chars().collect();
            add_viewer(&viewer, -1);
            if arg[i] == "-tui-app" {
                tui_mode(0, Some(true))
            } else {
                tui_mode(0, Some(false))
            };
        }
    }
}
pub(crate) fn print_viewers() {
    if !crate::cmd_keys::screen_state(None) {
        return;
    }
    let num_of_viewers = get_num_of_viewers(-1).to_usize().unwrap();
    for i in 0..num_of_viewers {
        print!("|||{}: {}", i, display_viewer(i, -1, true));
    }
    println!("")
}
pub fn print_pg_info() {
    if !crate::cmd_keys::screen_state(None) {
        return;
    }
    let num_page = get_num_page(-1);
    let num_files = get_num_files(-1);
    let last_pg = crate::where_is_last_pg();
    let info = format!(
        "Number of files/pages {}/{} p. {} Front list: {}",
        num_files,
        last_pg,
        num_page,
        crate::name_of_front_list("", false)
    );
    println!("{}", info);
}
pub(crate) fn user_wrote_path() -> String {
    return Path::new(&unsafe {
        format!("{}/user_wrote_path", unsafe {
            crate::ps18::page_struct("", crate::ps18::TMP_DIR_, -1).str_
        })
    })
    .to_str()
    .unwrap()
    .to_string();
}
pub(crate) fn user_wrote_path_prnt() -> String {
    return Path::new(&unsafe {
        format!("{}/user_wrote_path_prnt", unsafe {
            crate::ps18::page_struct("", crate::ps18::TMP_DIR_, -1).str_
        })
    })
    .to_str()
    .unwrap()
    .to_string();
}
pub(crate) fn set_user_written_path_from_strn(strn: String) -> bool {
    let save_path = user_wrote_path();
    let save_path1 = user_wrote_path();
    let save_path2 = user_wrote_path();
    let strn = crate::get_path_from_strn(strn);
    // set_ask_user(&save_path, -1); //dbg here
    let mut file_2_write_path = match File::options().create(true).open(save_path) {
        Ok(p) => p,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::options().create_new(true).open(save_path1) {
                Ok(f) => f,
                _ => update_user_written_path(e),
            },
            _ => File::options()
                .write(true)
                .open(save_path2)
                .expect("set_user_written_path_from_strn failed"),
        },
    }; //.expect("user_wrote_path failed ");
       //let mut writer = BufWriter::new(file_2_write_path)
    file_2_write_path.set_len(0);
    file_2_write_path
        .write_all(strn.as_bytes())
        .expect("user_wrote_path failed write in");
    crate::globs18::unblock_fd(file_2_write_path.as_raw_fd());
    let written_path = full_escape(&read_user_written_path());
    // save_file(written_path.to_string(), "written_path.dbg".to_string());
    update_dir_list(&written_path, "-maxdepth 1", false);
    true
}
pub(crate) fn set_user_written_path_from_prnt() -> String {
    let save_path = user_wrote_path();
    let save_path1 = user_wrote_path();
    let path_from_prnt = get_path_from_prnt();
    //set_ask_user(&save_path, -1); //dbg here
    let mut file_2_write_path = match File::options().create_new(true).open(save_path) {
        Ok(p) => p,
        Err(e) => update_user_written_path(e),
    };
    let key = format!("{}", path_from_prnt);
    file_2_write_path
        .write_all(path_from_prnt.as_bytes())
        .expect("user_wrote_path failed write in");
    crate::globs18::unblock_fd(file_2_write_path.as_raw_fd());
    let written_path = read_user_written_path();
    if written_path == "" {
        drop_ls_mode();
        crate::F3_key();
    } else {
        set_ls_as_front();
    }
    save_file(
        written_path.to_string(),
        "written_path_prnt.dbg".to_string(),
    );
    update_dir_list(&full_escape(&written_path), "-maxdepth 1", false);
    written_path
}

pub(crate) fn user_writing_path(key: String) -> bool {
    unsafe {
        set_ls_as_front();
        front_list_indx(crate::globs18::LS_);
    }
    let key = key.replace("//", "/");
    let mut written_path = read_user_written_path();
    let written_path_from_prnt = get_path_from_prnt();
    if written_path_from_prnt.chars().count() > written_path.chars().count() {
        written_path = written_path_from_prnt;
        complete_path(&written_path, "-maxdepth 1", false);
        form_cmd_line_default();
        return true;
    }
    if key == "/" && written_path == written_path_from_prnt {
        written_path = format!("{written_path}/")
    }
    complete_path(&written_path, "-maxdepth 1", false);
    form_cmd_line_default();
    true
}
pub(crate) fn read_user_written_path() -> String {
    let save_path = user_wrote_path();
    let mut file_2_read_path = match File::open(save_path) {
        Ok(f) => f,
        Err(e) => return "".to_string(),
    };
    let mut reader = BufReader::new(file_2_read_path);
    let mut ret = String::new();
    reader
        .read_to_string(&mut ret)
        .expect("read_user_written_path failed write in");
    ret
}
pub(crate) fn check_symlink() -> String {
    let found_files = take_list_adr("found_files");
    std::fs::read_link(&found_files)
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap()
        .to_string()
}
fn stop_run_viewer(cmd: &String) -> bool {
    if no_view(false, false) {
        return true;
    }
    if cmd.as_str().substring(0, 5) == "term " {
        return true;
    }
    false
}
//fn
