use crate::{run_cmd_out_sync, getkey, swtch::print_pg_info};

/*Run it Later or Cancel Now */
pub(crate) fn rilocan(){
    let now = crate::checkArg("-now");
    let cmd = String::from_iter(crate::get_arg_in_cmd("-rilocan").s);
    let mut sleep = String::from_iter(crate::get_arg_in_cmd("-sleep").s).trim_end_matches('\0').trim_start().to_string();
    let hour = 3600u64; let min = 60u64;
    let mut len_sleep = sleep.chars().count();
    let mut sleep_val = 47u64;
    if len_sleep == 0{sleep = "47s".to_string(); len_sleep = 3}
    let mut hms = String::new();
    hms.push(sleep.chars().nth(len_sleep - 1).unwrap());
    println!("hms {}, sleep {}", hms, sleep.len());
    match hms.as_str(){
        "h" => {
            sleep = sleep.replace("h", "");
            let ret = u64_from_strn(&sleep);
            if ret.1{sleep_val = ret.0 * hour}
        },
        "m" => {
            sleep = sleep.replace("m", "");
            let ret = u64_from_strn(&sleep);
            if ret.1{sleep_val = ret.0 * min}
        }
        "s" => {
            sleep = sleep.replace("s", "");
            let ret = u64_from_strn(&sleep);
            if ret.1{sleep_val = ret.0}
        },
        _ => {
            panic!("Command must look like..\ntam.rst -rilocan \"sudo standby\" -sleep 10m -now\ntam.rst -rilocan \"sudo standby\" -sleep 215s -now\ntam.rst -rilocan \"sudo standby\" -sleep 3h")
        }
    }
        let msg = format!("You have run task {cmd} for each {sleep}\nPlease, hit any key to cancel Your task.");
        println!("{}", msg);
        std::thread::spawn(move||{
            loop {
                crate::run_term_app1(cmd.clone());
                std::thread::sleep(std::time::Duration::from_secs(sleep_val));
                println!("Executed at {}", crate::Local::now())
            }
        });
        crate::getkey();
        println!("Have a nice day, DEAR USER|n
        See You Soon 🙃", );
        std::process::exit(0);

    }
pub(crate) fn u64_from_strn(strn: &String) -> (u64, bool){
    match u64::from_str_radix(&strn, 10){
        Ok(u) => (u, true),
        _ => return (0, false)
    }
}