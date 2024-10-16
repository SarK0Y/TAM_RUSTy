use once_cell::sync::Lazy;
use crate::{__get_arg_in_cmd, checkArg, getkey, link_lst_to, set_front_list, set_front_list2, split_once, STRN};
pub(crate) fn dont_clean_bash(roll: bool) -> bool{
    static mut state: bool = false;
    static mut fst_run: bool = false;
    if !crate::C!(fst_run){
        crate::C!(fst_run = true);
        if checkArg("-dont-clean-bash"){crate::C!(state = true)}
#[cfg(feature = "in_dbg")] println!("dont_clean_bash status: {}", crate::C!(state))
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
pub(crate) fn scroll_ln_in_pg(roll: bool ) -> bool{
    static mut state: bool = false;
    static mut fst_run: bool = false;
    if !crate::C!(fst_run){
        crate::C!(fst_run = true);
        if checkArg("-scroll-ln-in-pg"){crate::C!(state = true)}
        println!("dbg status: {}", crate::C!(state))
    }
    if roll{crate::C!(state = !state); if !silent(){println!("dbg status: {}", crate::C!(state));}}
    crate::C!(state)
}
pub(crate) fn be_silent(set: bool, new_state: bool) -> bool{
    static mut state: bool = false;
    static mut fst_run: bool = false;
    if set{crate::C!(state = new_state);}
    crate::C!(state)
}
pub fn screen_state (mode: Option <bool>) -> bool {
    static mut state: bool = true;
    unsafe {
        if let Some (x) = mode { state = x } state
    }
}
pub(crate) fn silent() -> bool{be_silent(false, false)}
pub(crate) fn swtch_esc(set: bool, new_state: bool) -> bool{
    static mut state: bool = true;
    if set{crate::C!(state = new_state);}
    crate::C!(state)
}
pub(crate) fn swtch_ls(set: bool, new_state: bool) -> bool{
    static mut state: bool = true;
    static mut fst_run: bool = false;
    if set{crate::C!(state = new_state);}
    /*if !crate::C!(state) && set {
        let prev = crate::read_file( "prev_list" );
        set_front_list( &prev );
    }*/
    crate::C!(state)
}
pub(crate) fn link_ext_lsts(){
    let args: Vec<_> = std::env::args().collect();
for i in 1..args.len(){
    if /*args_2_str[i]*/args[i] == "-link-lst-to" { link_ext_lst(&args[i + 1] );}
}
}
pub(crate) fn link_ext_lst(cmd: &String){
    let (lst, adr) = split_once(cmd, " ");
    link_lst_to(&lst, &adr);
}
pub(crate) fn front_lst(cmd: &String){
    let (lst, adr) = split_once(cmd, " ");
    link_lst_to(&lst, &adr);
    crate::core18::set_front_list( &lst );
}
pub fn dont_run_file (control: Option < bool >) -> bool{
    static mut state: bool = false;
    unsafe {
        if let Some (x) = control {state = x} state
    }
}
pub fn prompt () -> String{
    static mut strn0: Lazy < String > = Lazy::new( || {"Your command, please: ".strn() });
    if checkArg( "-prompt" ) {
        unsafe { *strn0 = __get_arg_in_cmd("-prompt" ); }
    } unsafe { strn0.strn() }
} 
//fn
