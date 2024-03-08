use std::process::Command;
use std::io::Write;
pub(crate) fn run_term_app(cmd: String) -> bool{
let func_id = crate::func_id18::run_cmd_viewer_;
crate::set_ask_user(cmd.as_str(), func_id);
let fstdout: String; 
let path_2_cmd = crate::mk_cmd_file(cmd);
let mut stderr_path = "stderr".to_string();
stderr_path = format!("{}stderr_term_app", unsafe{crate::ps18::page_struct("", crate::ps18::MAINPATH_, -1).str_});
let fstdout = format!("{}fstdout_term_app", unsafe{crate::ps18::page_struct("", crate::ps18::TMP_DIR_, -1).str_});
crate::core18::errMsg_dbg(&stderr_path, func_id, -1.0);
let fstderr = crate::File::create(stderr_path).unwrap();
let fstdout0 = crate::File::create(fstdout).unwrap();
//let mut fstdout0 = io::BufReader::new(fstdout0);
//errMsg_dbg(&in_name, func_id, -1.0);
let run_command = Command::new("bash").arg("-c").arg(path_2_cmd)//.arg(";echo").arg(stopCode)
//let run_command = Command::new(cmd)
    .stderr(fstderr)
    .stdout(crate::Stdio::piped())
    //.stdin(crate::io::stdin())
    .output()
    .expect("can't run command in run_cmd_viewer");
if run_command.status.success(){
    crate::io::stdout().write_all(&run_command.stdout).unwrap();
    crate::io::stderr().write_all(&run_command.stderr).unwrap();
    return false;
}
true
}