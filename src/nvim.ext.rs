use std::fmt::format;
use once_cell::sync::Lazy;
use crate::{mkdir, STRN};

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
//fn
