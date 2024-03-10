use std::process::Command;
use std::io::{Write, Read};
use std::thread::Builder;
use std::os::fd::AsRawFd;
use termion::terminal_size;
//use close_file::Closable;
use std::mem::drop;
use crate::globs18::unblock_fd;
use crate::{run_cmd_out, popup_msg, getkey, cpy_str};
pub(crate) fn run_term_app(cmd: String) -> bool{
let func_id = crate::func_id18::run_cmd_viewer_;
crate::set_ask_user(cmd.as_str(), func_id);
let (cols, rows) = termion::terminal_size().unwrap();
let cols = 680; let rows = 700;
let fstdout: String; 
let cmd = format!("stty cols {cols};stty rows {rows};{cmd}");
let path_2_cmd = crate::mk_cmd_file(cmd);
let mut stderr_path = "stderr".to_string();
stderr_path = format!("{}stderr_term_app", unsafe{crate::ps18::page_struct("", crate::ps18::MAINPATH_, -1).str_});
let fstdin = format!("{}/fstdin_term_app", unsafe{crate::ps18::page_struct("", crate::ps18::TMP_DIR_, -1).str_});
let fstdin_link = format!("{}/fstdin_term_app_link", unsafe{crate::ps18::page_struct("", crate::ps18::TMP_DIR_, -1).str_});
let fstdin_null = format!("{}/fstdin_term_app_null", unsafe{crate::ps18::page_struct("", crate::ps18::TMP_DIR_, -1).str_});
let mk_null_file = format!("touch -f {fstdin_null}");
run_cmd_out(mk_null_file);
let mk_link_2_file = format!("ln -sf {} {}", fstdin, fstdin_link);
let mk_link_2_file0 = mk_link_2_file.clone();
let mk_link_2_null = format!("ln -sf {} {}", fstdin_null, fstdin_link);
run_cmd_out(mk_link_2_file);
let fstd_in = crate::cpy_str(&fstdin);
crate::core18::errMsg_dbg(&stderr_path, func_id, -1.0);
let fstderr = crate::File::create(stderr_path).unwrap();
let mut fstdin0 = crate::File::options().create(true).write(true).read(true).truncate(true).open(fstdin_link).unwrap();
let mut fstdin0 = crate::io::stdin();
//unblock_fd(fstdin0.as_raw_fd());
//let mut fstdout0 = io::BufReader::new(fstdout0);
//errMsg_dbg(&in_name, func_id, -1.0);
let run_command = Command::new("bash").arg("-c").arg(path_2_cmd)//.arg(";echo").arg(stopCode)
//let run_command = Command::new(cmd)
    .stderr(fstderr)
    .stdout(std::process::Stdio::piped())
    .stdin(std::process::Stdio::piped())
    .spawn()
    .expect("can't run command in run_term_app");
/*if run_command.status.success(){
    crate::io::stdout().write_all(&run_command.stdout).unwrap();
    crate::io::stderr().write_all(&run_command.stderr).unwrap();
    return false;
}*/
//let builder = Builder::new().stack_size(2 * 1024 * 1024).name("rw_std".to_string());
 std::thread::spawn(move|| {
    let mut stdout = run_command.stdout.unwrap();
    let mut buf: [u8; 128] = [0; 128];
   // let mut fstd_in0 = crate::File::create(fstd_in).unwrap();
//loop {
loop{
    stdout.read(&mut buf);
    if buf == [0;128]{break;}
    std::io::stdout().write_all(&read_std(&buf)).unwrap();
  //  std::io::stderr().write_all(&read_std(&buf)).unwrap();
    buf = [0;128];
}
/*let key: String = getkey();
    fstd_in0.write_all(key.as_bytes());
}*/
println!("exit rw_std");
});
std::thread::spawn(move||{
    let mut fstd_in0 = run_command.stdin.unwrap();
    let mut writer = crate::io::BufWriter::new(&mut fstd_in0);
loop{
    let key: String = getkey();
    writer.write_all(key.as_bytes());
 //   run_cmd_out(mk_link_2_null.clone());
  //  fstd_in0.write_all(key.as_bytes());
   // run_cmd_out(mk_link_2_file0.clone());
    //drop(fstd_in0);
}

}).join();
//builder.join();
true
}
fn read_std(std: &[u8]) -> Vec<u8> {
    let out = String::from_utf8_lossy(std);
    let out = out.lines().collect::<Vec<_>>().join("\n");
    let out = out.replace("\r\n", "");
    out.replace('\n', "").into_bytes()
}