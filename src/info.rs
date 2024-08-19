use crate::{repeat_char, run_cmd_out_sync, clear_screen, run_cmd_str, getkey};
use std::io::Write; use std::os::fd::FromRawFd;
use std::process::Command;

pub(crate) fn SYS(){
    println!("\nHave a nice day, DEAR USER\nSee You Soon 🙃", );
    std::process::exit(0)
}
pub const Author: &str = "Knyazhev Evgeney (SarK0Y)";
const Project: &str = "Project: Tiny Automation Manager.";
pub const Ver: &str = "Ver: 3.0.8";
const Telega: &str = "TELEGRAM: https://t.me/+N_TdOq7Ui2ZiOTM6 (Alg0Z)";
const Ru_blog: &str = "ALG0Z RU: https://dzen.ru/alg0z";
const En_blog: &str = "ALG0Z EN: https://alg0z.blogspot.com";
const ChangeLog: &str = "ChangeLog: https://alg0z8n8its9lovely6tricks.blogspot.com/2023/09/tam-changelog.html";
const Forum: &str = "FORUM: https://www.neowin.net/forum/topic/1430114-tam/";
const E_mail: &str = " E-MAIL: sark0y@protonmail.com";
const Github: &str = "GITHUB: https://github.com/SarK0Y/TAM_RUSTy.git";
const YouTube: &str = "YouTube: https://www.youtube.com/@evgeneyknyazhev968";
const Twitter_X: &str = "Twitter/X: https://x.com/SarK0Y8";
const Supported_platforms: &str = "Supported platforms: TAM  for Linux & alike; TAW for Windows.";
const LICENSE: &str = "License/Agreement:\n
Personal usage will cost You $0.00, but don't be shy to donate me.. or You could support me any other way You want - just call/mail me to discuss possible variants for mutual gains. 🙂\n
Commercial use takes $0.77 per month from You.. or just Your Soul 😇😜\n
my the Best Wishes to You 🙃
";
#[cfg(feature="mae")]
const LICENSE_MAE: &str = "🠳 License/Agreement for Mademoiselle Entropia 🠳\n Price for Personal usage: $0.78 per month.\n
 Price for commercial usage: $7.13.\n
Or just Your Soul 😇😜\n my the Best Wishes to You 🙃";

const donate: &str = "Donations: https://boosty.to/alg0z/donate";
pub(crate) fn info(){
    KonsoleTitle(&"TAM RUSTy".to_string());
    clear_screen();
  // println!("{}", crate::from_utf8(&tst.unwrap().stdout).unwrap());
banners_line(Project, "◑"); println!("");
banners_line(Ver, "◑"); println!("");
banners_line(Telega, "◑"); println!("");
banners_line(Ru_blog, "◑"); println!("");
banners_line(En_blog, "◑"); println!("");
banners_line(ChangeLog, "◑"); println!("");
banners_line(Forum, "◑"); println!("");
banners_line(E_mail, "◑"); println!("");
banners_line(Github, "◑"); println!("");
banners_line(YouTube, "◑"); println!("");
banners_line(Twitter_X, "◑"); println!("");
banners_line(Supported_platforms, "◑"); println!("");
println!("{}", LICENSE);
#[cfg(feature="mae")]
println!("{}", LICENSE_MAE);
banners_line(donate, "◑"); println!("\n");
banner(280);
}
pub(crate) fn info1(){
    clear_screen();
 {crate::be_silent(true, true); crate::dont_scrn_fix(true); crate::no_view(true, true);}
banners_line(Project, "◑"); println!("");
banners_line(Ver, "◑"); println!("");
banners_line(Telega, "◑"); println!("");
banners_line(Ru_blog, "◑"); println!("");
banners_line(En_blog, "◑"); println!("");
banners_line(ChangeLog, "◑"); println!("");
banners_line(Forum, "◑"); println!("");
banners_line(E_mail, "◑"); println!("");
banners_line(Github, "◑"); println!("");
banners_line(YouTube, "◑"); println!("");
banners_line(Twitter_X, "◑"); println!("");
banners_line(Supported_platforms, "◑"); println!("");
println!("{}", LICENSE);
#[cfg(feature="mae")]
println!("{}", LICENSE_MAE);
banners_line(donate, "◑"); println!("\n");
banners_line2(Author, ">", "<");
println!("\nDear User, Please, hit any key to continue.. Thanks.");
getkey();
{crate::dont_scrn_fix(true); crate::no_view(true, false);}
}
pub(crate) fn banners_line(val: &str, filler: &str){
    use terminal_size::{Width, Height, terminal_size};
    use console::Term;
    use termion::terminal_size as sizze;
    use std::io::Write;
let size = sizze();//Term::size(&Term::stdout());
let (mut w, h) = (80u32, 300u32);
if (0, 0) != match size{
    Ok(ts) => ts,
    _ => (0, 0)
} { w = size.unwrap().0.into(); }
let mut w_half_size: usize = (w / 2) as usize;
let mut val_half_size = (val.chars().count() +2) / 2 ;
if w_half_size + 1 > val_half_size{w_half_size -= (val_half_size)}
let filler_left = repeat_char(w_half_size, &filler);
let filler_right = repeat_char(w_half_size +1, &filler);
let banner = format!("\r{filler_left} {val} {filler_right}");
std::io::stdout().write_all(banner.as_bytes());
}
pub(crate) fn banners_line2(val: &str, filler_L: &str, filler_R: &str){
    use terminal_size::{Width, Height, terminal_size};
    use console::Term;
    use termion::terminal_size as sizze;
    use std::io::Write;
let size = sizze();//Term::size(&Term::stdout());
let (mut w, h) = (80u32, 300u32);
if (0, 0) != match size{
    Ok(ts) => ts,
    _ => (0, 0)
} { w = size.unwrap().0.into(); }
let mut w_half_size: usize = (w / 2) as usize;
let mut val_half_size = (val.chars().count() +2) / 2 ;
if w_half_size + 1 > val_half_size{w_half_size -= (val_half_size)}
let filler_left = repeat_char(w_half_size, &filler_L);
let filler_right = repeat_char(w_half_size +1, &filler_R);
let banner = format!("\r{filler_left} {val} {filler_right}");
std::io::stdout().write_all(banner.as_bytes());
}
pub(crate) fn banner(delay: u64){
    loop {
        banners_line("© Evgeney Knyazhev 2023-2024", "\\");
         std::thread::sleep(std::time::Duration::from_millis(delay));
        banners_line("© Evgeney Knyazhev 2023-2024", "-");
         std::thread::sleep(std::time::Duration::from_millis(delay));
         banners_line("© SarK0Y 2023-2024", "8");
         std::thread::sleep(std::time::Duration::from_millis(delay));
         banners_line("© SarK0Y 2023-2024", "∞");
         std::thread::sleep(std::time::Duration::from_millis(delay));
         banners_line("© Evgeney Knyazhev 2023-2024", "|");
         std::thread::sleep(std::time::Duration::from_millis(delay));
        banners_line("© Evgeney Knyazhev 2023-2024", "+");
         std::thread::sleep(std::time::Duration::from_millis(delay));
         banners_line("© Evgeney Knyazhev 2023-2024", "/");
         std::thread::sleep(std::time::Duration::from_millis(delay));
         banners_line("© SarK0Y 2023-2024", "\\");
         std::thread::sleep(std::time::Duration::from_millis(delay));
         banners_line("© SarK0Y 2023-2024", "+");
         std::thread::sleep(std::time::Duration::from_millis(delay));
         banners_line("© SarK0Y 2023-2024", "/");
         std::thread::sleep(std::time::Duration::from_millis(delay));
         banners_line("© SarK0Y 2023-2024", "∞");
         std::thread::sleep(std::time::Duration::from_millis(delay));
    }
}
pub(crate) fn KonsoleTitle(title: &String){
    use console::Term; 
    Term::set_title(&Term::stdout(), title);
    let title = format!("import os; os.system('echo \"\\033]30;{}\\007\"')", title);
    let writeTermTitle = Command::new("python3").arg("-c").arg(title).stderr(std::process::Stdio::piped()).stdout(std::process::Stdio::piped())
    .output();
    std::io::stdout().write_all(&writeTermTitle.unwrap().stdout);
}
pub fn ver(){
    use crate::custom_traits::STRN;
    let mae = if cfg!(feature = "mae"){ "mae".strn() } else { "".strn() };
    let val: String = format!("{} {}", crate::info::Ver, mae);
    crate::set_ask_user(&val, 30050017);
}
//fn
