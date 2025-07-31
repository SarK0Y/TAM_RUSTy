It's fork of the rst_lex to clean source code up from debugging stuff. Let's look at workable example:
//build.rs for rst_lex
use std::env;
use std::fs;
use std::path::Path;
use std::io::{Read, Write};
//extern crate for_build_rs;
#[cfg(feature ="cleanup")]
use for_build_rs::lex::{stream_cleanup, leave_file_mark};
#[cfg(feature ="cleanup")]
use for_build_rs::faav::cleanup_dbg_attr;
#[cfg(feature ="cleanup")]
pub fn cleanup () {
    let mut attr = cleanup_dbg_attr::new();
    attr._1st_token = "dbg!".to_string ();
    attr.end_token = ";".to_string ();
    let src = "./src/dev_lex.rs";
    let dst = "./src/stable_lex.rs";
    let mut dev_lex = String::new ();
    let mut src_file = match fs::File::open (src) {
        Ok (f) => { f },
        Err (e) => {panic! ("cleanup for rst_lex failed to open file {src} w/ err {:?}", e)}
    };
    let _ = match src_file.read_to_string ( &mut dev_lex){
        Ok (fine) => { fine },
        Err (e) => { eprintln! ("cleanup () in rst_lex build.rs failed to read from {src} to dev_lex due to {:?} ", e); return}
    };
    dev_lex = stream_cleanup (&dev_lex, &attr._1st_token, 0, &attr.end_token);
    dev_lex = stream_cleanup (&dev_lex, &"dbg_stuff".to_string(), 0, &"dbg_stuff".to_string() );
    let mut dst_file = match fs::File::create (dst) {
        Ok (f) => { f },
        Err (e) => {panic! ("cleanup for rst_lex failed to wtite file {dst} w/ err {:?}", e)}
    };
    dev_lex = dev_lex.replace (";;", "");
    leave_file_mark ("/tmp/cleanup", &dev_lex);
    dst_file.write_all (dev_lex.as_bytes() );
}
fn main() {
    #[cfg(feature ="cleanup")]
    cleanup ();
}

 <b> Links: </b> <br>
 <b>Rolling guide of TAM (Topnotch Practical ways to use Console/Terminal):<b> https://alg0z8n8its9lovely6tricks.blogspot.com/2024/08/tam-guide-of-features-smart-tricks.html <br>
 <b>DISCORD:</b> https://discord.gg/X9RBbtCN (Alg0Z). <br>
 <b>TELEGRAM:</b> https://t.me/+N_TdOq7Ui2ZiOTM6 (Alg0Z). <br>
 <b>ALG0Z RU:</b> https://dzen.ru/alg0z <br>
 <b>ALG0Z EN:</b> https://alg0z.blogspot.com <br>
 <b>ChangeLog:</b> https://alg0z8n8its9lovely6tricks.blogspot.com/2023/09/tam-changelog.html <br>
 <b>FORUM:</b> https://www.neowin.net/forum/topic/1430114-tam/ <br>
 <b>E-MAIL:</b> sark0y@protonmail.com <br>
 <b>GITHUB:</b> https://github.com/SarK0Y/TAM_RUSTy.git <br>
 <b>YouTube:</b> https://www.youtube.com/@evgeneyknyazhev968 <br>
 <b>Twitter_X:</b> https://x.com/SarK0Y8 <br>
 Donations: https://boosty.to/alg0z/donate https://zap-hosting.com/en/shop/donation/1f0c83845d810df04ca74e56238399f7/ <br>
 # <p align=center> <b> MAKE CONSOLE GREAT AGAIN.🤘 </b> </p>
 
# my the Best Wishes to You 🙃
