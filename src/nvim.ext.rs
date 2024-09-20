use std::fmt::format;
use once_cell::sync::Lazy;
use crate::{mkdir, split_once, STRN};

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
pub fn nvim_version () -> Option < (String, String ) > {
   let mut nvv = r"nvim --version|grep -i 'nvim\sv[0-9]*\.[0-9]*\.[0-9]*'".strn();
   let ret = crate::run_cmd_out_sync( nvv );
    if ret != "" { nvv = ret }
    else { crate::errMsg0( "Err: Unknown Neovim is installed on Your system."); return None}
   let (_, nvv ) = crate::split_once(&nvv, " ") ;
   let nvv = nvv.replace ("v", "");
   let (high, nvv) = split_once(&nvv, ".");
   let (middle, nvv) = split_once(&nvv, ".");
   let low = nvv;


    None
}
//fn
