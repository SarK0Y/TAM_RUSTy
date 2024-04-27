use once_cell::sync::Lazy;
use crate::{checkArg, getkey};
pub(crate) fn dont_clean_bash(set: bool) -> bool{
    static mut state: bool = false;
    static mut fst_run: bool = false;
    if !crate::C!(fst_run){
        crate::C!(fst_run = true);
        if checkArg("-dont-clean-bash"){crate::C!(state = true)}
        println!("dont_clean_bash status: {}", crate::C!(state))
    }
    if set{crate::C!(state = !state); println!("dont_clean_bash status: {}", crate::C!(state)); }
    crate::C!(state)
}
pub(crate) fn dbg(set: bool) -> bool{
    static mut state: bool = false;
    static mut fst_run: bool = false;
    if !crate::C!(fst_run){
        crate::C!(fst_run = true);
        if checkArg("-dbg"){crate::C!(state = true)}
        println!("dbg status: {}", crate::C!(state))
    }
    if set{crate::C!(state = !state); println!("dbg status: {}", crate::C!(state));}
    crate::C!(state)
}
pub(crate) fn switch_cmd_keys(cmd: &String){
    let cmd = cmd.replace("key::", "").trim_start_matches(' ').to_string();
    match cmd.as_str(){
        "dont clean bash" => {dont_clean_bash(true); getkey();}
        "dbg" => {dbg(true); getkey();}
        "dont scrn fix" => {dont_scrn_fix(true); getkey();}
        _ => {}
    }
}
pub(crate) fn dont_scrn_fix(set: bool) -> (bool, bool){
    static mut state: bool = false;
    static mut fst_run: bool = false;
    static mut local_set: bool = false;
    if !crate::C!(fst_run){
        crate::C!(fst_run = true);
        if checkArg("-dont-scrn-fix"){crate::C!(state = true)}
        println!("dont_scrn_fix status: {}", crate::C!(state))
    }
    if set{unsafe {state = !state; local_set = true}; println!("dont_scrn_fix status: {}", crate::C!(state));}
    crate::C!((state, local_set))
}
//fn