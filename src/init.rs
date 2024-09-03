use once_cell::sync::Lazy;
use crate::{custom_traits::STRN, run_cmd, run_cmd_out_sync};
pub fn user_home_dir() -> String {
    static mut home: Lazy <String> = Lazy::new(|| {"".strn() } );
    static mut fst: bool = true;
    unsafe {
        if fst {
            let cmd = format!("cd ~/;pwd");
            *home = run_cmd_out_sync(cmd);
            fst = false;
            } home.clone()    
    }    
}
