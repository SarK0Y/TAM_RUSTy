use std::process::Command;
use std::io::{Write, Read};
use std::thread::Builder;
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
   // .stdin(crate::Stdio::inherit())
    .spawn()
    .expect("can't run command in run_term_app");
/*if run_command.status.success(){
    crate::io::stdout().write_all(&run_command.stdout).unwrap();
    crate::io::stderr().write_all(&run_command.stderr).unwrap();
    return false;
}*/
let builder = Builder::new().stack_size(2 * 1024 * 1024).name("rw std".to_string());
 builder.spawn(move|| {
    let mut stdout = run_command.stdout.unwrap();
    let mut buf: [u8; 128] = [0; 128];
loop{
    stdout.read(&mut buf);
    crate::io::stdout().write_all(&read_std(&buf)).unwrap();
}
println!("exit rw_std");
});
//builder.join();
true
}
fn read_std(std: &[u8]) -> Vec<u8> {
    let out = String::from_utf8_lossy(std);
    let out = out.lines().collect::<Vec<_>>().join("\n");
    let out = out.replace("\r\n", "");
    out.replace('\n', "").into_bytes()
}