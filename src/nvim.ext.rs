use std::fmt::format;
use once_cell::sync::Lazy;
use crate::{mkdir, read_midway_data, split_once, STRN};

pub fn add_keys_2_cmd (cmd: &String) -> String {
    match cmd.as_str() {
        "nvim" | "nvim.app" | "nvim.AppImage" => return  format! ("{cmd} --listen {}", get_socket() ).strn(),
        _ => return cmd.strn()
    }
    "".strn()
}
pub fn get_socket () -> String {
    static mut fst: bool = true;
    static mut socket: Lazy < String > = Lazy::new( || {"".strn() });
    unsafe {
        if fst {
            let path = format!( "{}/env/nvim", crate::core18::bkp_tmp_dir( None, false ) );
            fst = false; *socket = format! ("{path}/sock");
            mkdir( path );
        } socket.strn()
    }
}
pub fn nvim_remote (cmd: &String) {
    static mut nvr: Lazy< String > = Lazy::new( || {format! ("") });
    let cmd = cmd.trim_start_matches( "nvr ").strn();
}
pub fn nvimc (nvim: Option < String >) -> String {
    "".strn()
}
pub fn nvim_version (app_name: &str) -> Option < (String, String ) > {
    let mut nvv = format! (r"{app_name} --version|grep -i 'nvim\sv[0-9]*\.[0-9]*\.[0-9]*'" );
   let ret = crate::run_cmd_out_sync( nvv );
    if ret != "" { nvv = ret }
    else { crate::errMsg0( "Err: Unknown Neovim is installed on Your system."); return None}
   let (_, nvv ) = crate::split_once(&nvv, " ") ;
   let nvv = nvv.replace ("v", "");
   let ( high, nvv) = split_once(&nvv, ".");
   let ( middle, nvv) = split_once(&nvv, ".");
   let low = nvv;
   let base_ver = (0u16, 10u32, 1u16);
   let high: Option< u16 > = match high.parse:: <u16> () {Ok(x) => Some(x), _ => None };
   if high == None { crate::errMsg0( "Err: Unknown Neovim is installed on Your system."); return None}
   if high.unwrap() < base_ver.0 { crate::errMsg0( &format!("Required minimal version of Neovim: {:#?}", base_ver) ); return None}
   if high.unwrap() > base_ver.0 { return Some( (app_name.strn(), format! ("{}.{}.{}", high.unwrap(), middle, low ) ) )}
   let middle: Option< u32 > = match middle.parse:: <u32> () {Ok(x) => Some(x), _ => None };
   if middle == None { crate::errMsg0( "Err: Unknown Neovim is installed on Your system."); return None}
   if middle.unwrap() < base_ver.1 { crate::errMsg0( &format!("Required minimal version of Neovim: {:#?}", base_ver) ); return None}
   if middle.unwrap() > base_ver.1 { return Some( (app_name.strn(), format! ("{}.{}.{}", high.unwrap(), middle.unwrap(), low ) ) )}
   let low: Option< u16 > = match low.parse:: <u16> () {Ok(x) => Some(x), _ => None };
   if low == None { crate::errMsg0( "Err: Unknown Neovim is installed on Your system."); return None}
   if low.unwrap() < base_ver.2 { crate::errMsg0( &format!("Required minimal version of Neovim: {:#?}", base_ver) ); return None}
   if low.unwrap() > base_ver.2 { return Some( (app_name.strn(), format! ("{}.{}.{}", high.unwrap(), middle, low.unwrap() ) ) )}

    None
}
//fn
