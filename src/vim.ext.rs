use std::fmt::format;
use once_cell::sync::Lazy;
use crate::{konsole, mkdir, read_midway_data, split_once, STRN};
pub fn get_socket () -> String {
    static mut fst: bool = true;
    static mut socket: Lazy < String > = Lazy::new( || {"".strn() });
    unsafe {
        if fst {
            let path = format!( "{}/env/vim", crate::core18::bkp_tmp_dir( None, false ) );
            fst = false; *socket = format! ("{path}/sock");
            mkdir( path );
        } socket.strn()
    }
}
pub fn vim_remote (cmd: &String) {
   let cmd = cmd.trim_start_matches( "vimr ").strn();
    let nvc = nvimc();
    let base_ver = (0u16, 10u32, 1u16);
    if nvc == "" {
        match status( None ) {
            crate::enums::vim::unknown => { crate::errMsg0( "Err: Unknown Vim is installed on Your system."); return; }
            crate::enums::vim::too_old => { crate::errMsg0( &format!("Required minimal version of Neovim: {:#?}", base_ver) ); return; }
            crate::enums::vim::not_found => { crate::errMsg0 ( "Dear User, Vim with proper version is not found on Your machine." ); return }
            crate::enums::vim::ok => {}
        }
    }
    let cmd = cmd.replace (" ", "<space>");
    let cmd = format! ( r"{nvc} {cmd}" );
    crate::swtch_tam_konsole();
    crate::run_cmd_out_sync( cmd );
}
pub fn vim_remote_file_in_new_tab (cmd: &String) {
    use substring::Substring;
   let cmd = cmd.trim_start_matches( "vimt ").trim_start().strn();
    let mut file_name = if cmd.substring(0, 1) == "/" { cmd } else {
        let mut indx: i64; 
        if let Ok (x) = cmd.parse:: <i64> () {indx = x} else {
            crate::errMsg0("Example: c{vimt /path/to/file} or c{vimt <file's index>}"); return;
        }
        let name = crate::get_item_from_front_list( indx, true);
        if name == "no str gotten" || name == "" {crate::errMsg0 ("Failed to fetch item from front list"); return;} name
    };    
    let nvc = nvimc();
    let base_ver = (0u16, 10u32, 1u16);
    if nvc == "" {
        match status( None ) {
            crate::enums::vim::unknown => { crate::errMsg0( "Err: Unknown Vim is installed on Your system."); return; }
            crate::enums::vim::too_old => { crate::errMsg0( &format!("Required minimal version of Neovim: {:#?}", base_ver) ); return; }
            crate::enums::vim::not_found => { crate::errMsg0 ( "Dear User, Vim with proper version is not found on Your machine." ); return }
            crate::enums::vim::ok => {}
        }
    }
    crate::swtch_tam_konsole();
    let cmd = format! ( "{nvc} '<esc>:tabe<space>{file_name}<cr>'");
    crate::run_cmd_out_sync( cmd );
}

fn nvimc () -> String {
     static mut nvr: Lazy< String > = Lazy::new( || { "".strn() });
    static mut set: bool = false;
    unsafe {
        if set {return nvr.clone() }
        if *nvr == "" {*nvr = choose_ver(); set = true; return nvr.clone() }
    }
    
    "".strn()
}
fn choose_ver () -> String {
    let variants = vec! ["vim", "vim.app", "vim.AppImage"];
    for var in variants {
        let variant = vim_version( var );
        if let Some ( app_name_n_ver ) = variant {return format! ("{} --servername {} --remote-send", app_name_n_ver.0, get_socket() ) }
    }
    "".strn()
}
pub fn vim_version (app_name: &str) -> Option < (String, String ) > {
    let mut nvv = format! (r"{app_name} --version" );
    let ret = crate::run_cmd_out_sync( nvv ).trim_end().strn();
    if ret == "" { status( Some( crate::enums::vim::not_found ) );  return None}
    return Some ( ( app_name.strn(), "unknown".strn () ) ); 
/*    let mut nvv = format! (r"{app_name} --version|grep -i 'vim\sv[0-9]*\.[0-9]*\.[0-9]*'" );
   let ret = crate::run_cmd_out_sync( nvv ).trim_end().strn();
    if ret != "" { nvv = ret }
    else {status( Some( crate::enums::nvim::unknown ) ); return None}
   let (_, nvv ) = crate::split_once(&nvv, " ") ;
   let nvv = nvv.replace ("v", "");
   let ( high, nvv) = split_once(&nvv, ".");
   let ( middle, nvv) = split_once(&nvv, ".");
   let low = nvv;
   let base_ver = (0u16, 10u32, 1u16);
   let high: Option< u16 > = match high.parse:: <u16> () {Ok(x) => Some(x), _ => None };
   if high == None {status( Some( crate::enums::nvim::unknown ) ); return None}
   if high.unwrap() < base_ver.0 {  status( Some( crate::enums::nvim::too_old ) ); return None}
   if high.unwrap() > base_ver.0 { status( Some( crate::enums::nvim::ok ) );  return Some( (app_name.strn(), format! ("{}.{}.{}", high.unwrap(), middle, low ) ) )}
   let middle: Option< u32 > = match middle.parse:: <u32> () {Ok(x) => Some(x), _ => None };
   if middle == None { status( Some( crate::enums::nvim::unknown ) ); return None}
   if middle.unwrap() < base_ver.1 {status( Some( crate::enums::nvim::too_old ) ); return None}
   if middle.unwrap() > base_ver.1 {  status( Some( crate::enums::nvim::ok ) );  return Some( (app_name.strn(), format! ("{}.{}.{}", high.unwrap(), middle.unwrap(), low ) ) )}
   let low: Option< u16 > = match low.parse:: <u16> () {Ok(x) => Some(x), _ => None };
   if low == None {status( Some( crate::enums::nvim::unknown ) ); return None}
   if low.unwrap() < base_ver.2 { status( Some( crate::enums::nvim::too_old ) ); return None}
   if low.unwrap() >= base_ver.2 {  status( Some( crate::enums::nvim::ok ) ); return Some( (app_name.strn(), format! ("{}.{}.{}", high.unwrap(), middle.unwrap(), low.unwrap() ) ) )} */
    None
}
pub fn status (new_state: Option < crate::enums::vim >) -> crate::enums::vim {
    static mut state: Lazy< crate::enums::vim > = Lazy::new( || { crate::enums::vim::unknown });
    unsafe {
        if let Some( x ) = new_state {
            *state = x
        } state.clone()
    }
}
//fn
