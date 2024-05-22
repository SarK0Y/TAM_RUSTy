use once_cell::sync::Lazy;
use crate::{checkArg, getkey};
pub(crate) fn dont_clean_bash(roll: bool) -> bool{
    static mut state: bool = false;
    static mut fst_run: bool = false;
    if !crate::C!(fst_run){
        crate::C!(fst_run = true);
        if checkArg("-dont-clean-bash"){crate::C!(state = true)}
        println!("dont_clean_bash status: {}", crate::C!(state))
    }
    if roll{crate::C!(state = !state); println!("dont_clean_bash status: {}", crate::C!(state)); }
    crate::C!(state)
}
pub(crate) fn dbg(roll: bool) -> bool{
    static mut state: bool = false;
    static mut fst_run: bool = false;
    if !crate::C!(fst_run){
        crate::C!(fst_run = true);
        if checkArg("-dbg"){crate::C!(state = true)}
        if !silent(){println!("dbg status: {}", crate::C!(state));}
    }
    if roll{crate::C!(state = !state); if !silent(){println!("dbg status: {}", crate::C!(state));}}
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
pub(crate) fn dont_scrn_fix(roll: bool) -> (bool, bool){
    static mut state: bool = false;
    static mut fst_run: bool = false;
    static mut local_set: bool = false;
    if !crate::C!(fst_run){
        crate::C!(fst_run = true);
        if checkArg("-dont-scrn-fix"){crate::C!(state = true)}
        if !silent(){println!("dbg status: {}", crate::C!(state));}
    }
    if roll{unsafe {state = !state; local_set = true}; if !silent(){println!("dbg status: {}", crate::C!(state));}}
    crate::C!((state, local_set))
}
pub(crate) fn no_view(set: bool, new_state: bool) -> bool{
    static mut state: bool = false;
    static mut fst_run: bool = false;
    if !crate::C!(fst_run){
        crate::C!(fst_run = true);
        if checkArg("-no-view"){crate::C!(state = true)}
        println!("dbg status: {}", crate::C!(state))
    }
    if set{crate::C!(state = new_state); if !silent(){println!("dbg status: {}", crate::C!(state));}}
    crate::C!(state)
}
pub(crate) fn be_silent(set: bool, new_state: bool) -> bool{
    static mut state: bool = false;
    static mut fst_run: bool = false;
    if set{crate::C!(state = new_state);}
    crate::C!(state)
}
pub(crate) fn swtch_esc(set: bool, new_state: bool) -> bool{
    static mut state: bool = false;
    static mut fst_run: bool = false;
    if set{crate::C!(state = new_state);}
    crate::C!(state)
}
pub(crate) fn silent() -> bool{be_silent(false, false)}
//fn