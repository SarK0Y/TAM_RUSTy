use Mademoiselle_Entropia::{Mademoiselle_Entropia::cipher, help_funcs};
use crate::{amaze_me, custom_traits::{STRN_strip, STRN}, getkey};
pub(crate) fn encrypt_n_keep_orig_file(cmd: &String){
    let func_name = "encrypt_n_keep_orig_file".strn();
    let file_to_encrypt = cmd.replace("encrypt copy", "").trim_start().trim_end().strn().strip_all_symbs();
    match std::fs::copy(&file_to_encrypt, format!("{file_to_encrypt}.mae")){Ok(f) => {}, Err(e) =>  {return println!("{func_name} got {e:?}" );}}
    let fst_pswd = pswd(None);
    let nd_pswd = pswd(Some("\rPlease, repeat Your password: ".strn()) );
    if nd_pswd != fst_pswd{return println!("Sorry, Dear User, try again", );}
    let mut file = match help_funcs::get_file(&format!("{file_to_encrypt}.mae")){Ok(f) => f, 
                                                            Err(e) => return println!("Sorry, can't open {file_to_encrypt}.mae: {e:?}")};
    let IK_len = crate::globs18::strn_2_usize(
        open_typing(Some("\rPlease, enter a size of IK".strn()) )
       ).unwrap_or(256);
    let buf_size = crate::globs18::strn_2_usize(
        open_typing(Some("\rPlease, enter the buffer's size ".strn()) )
    ).unwrap_or(10_000);
    file.encrypt(&fst_pswd, IK_len, buf_size);
    crate::save_file0(format!("{}.mae", file_to_encrypt), "mae".strn());
    #[cfg(feature="in_dbg")] println!("pswd {fst_pswd}");
    println!("Dear User, Please, hit any key to continue.. Thanks."); getkey();
}
pub(crate) fn decrypt_copy(cmd: &String){
    let func_name = "decrypt_copy".strn();
    let file_to_decrypt = cmd.replace("decrypt copy", "").trim_start().trim_end().strn().strip_all_symbs();
    match std::fs::copy(&file_to_decrypt, file_to_decrypt.replace(".mae", "")){Ok(f) => {}, Err(e) =>  {return println!("{func_name} got {e:?}" );}}
    let fst_pswd = pswd(None);
    let mut file = match help_funcs::get_file(
        &format!("{}", file_to_decrypt.replace(".mae", "") )
    ){Ok(f) => f, Err(e) => return println!("Sorry, can't open {file_to_decrypt}: {e:?}")};
    let buf_size = crate::globs18::strn_2_usize(
        open_typing(Some("\nPlease, enter the buffer's size ".strn()) )
    ).unwrap_or(10_000);
    file.decrypt(&fst_pswd, buf_size);
    crate::save_file0(file_to_decrypt.replace(".mae", ""), "decrypted".strn());
#[cfg(feature="in_dbg")] println!("pswd {fst_pswd}");
    println!("Dear User, Please, hit any key to continue.. Thanks."); getkey();
}
pub(crate) fn kb_input(prompt: Option<String>, echo: bool) -> String{
    let prompt0 = prompt.clone();
    println!("");
    let mut prompt1 = "\rDear User, enter Your password: ".strn(); 
    if let Some(p) = prompt{prompt1 = p}
    print!("{prompt1}");
    let mut typed = String::new();
    loop { 
        let k = getkey();
        let ansiKey: u8 = match k.as_str().bytes().next(){
        Some(val) => val,
        _ => 0
        }; // possible problem could be ahead
        if ansiKey == crate::kcode01::ENTER{break;}
        typed.push_str(k.as_str());
        if echo{print!("\r{prompt1}{}", typed.clone())}   
    }
    if typed == ""{
        let mut STRING = "string";
        if !echo {STRING = "password/token"}
        println!("Dear User, Please, don't leave empty {STRING}.. Thanks");}
    typed
}
pub(crate) fn pswd(prompt: Option<String>) -> String{
    kb_input(prompt, false)
}
pub(crate) fn open_typing(prompt: Option<String>) -> String{
    kb_input(prompt, true)
}
pub(crate) fn mk_dummy_filo(name: &str, content: &str, len: usize){
    let func_name = "mk_dummy_filo".strn();
    let name = name.trim_end().trim_end_matches('\0');
    let content = content.trim_end().trim_end_matches('\0');
    match std::fs::File::options().write(true).read(true).create_new(true).open(&name){Ok(f) => f,
                                                         Err(e) => return println!("{func_name} got {e:?}")};
    let mut file =  match help_funcs::get_file(&name.strn()){Ok(f) => f, _ => return};
    use Mademoiselle_Entropia::help_funcs::dummy_file;
    file.populate_w_strn(content, len, 40*1024);
    println!("{name} created with length {len}");
}
pub(crate) fn mk_empty_fil0(name: &str ){
    let func_name = "mk_empty_fil0".strn();
    let name = name.trim_end().trim_end_matches('\0');
    match std::fs::File::options().write(true).read(true).create_new(true).open(&name){Ok(f) => f,
                                                         Err(e) => return println!("{func_name} got {e:?}")};
    let mut file =  match help_funcs::get_file(&name.strn()){Ok(f) => f, _ => return};
}
pub fn surprise_me(cmd: Option < amaze_me > ) -> Option <u64>{
    if cmd == None {return None;}
    static mut state: u64 = 0;
    use Mademoiselle_Entropia::true_rnd::get_true_rnd_u64 as u64_;
    unsafe {
        let in_val = cmd.unwrap();
        /*  ret_indx_n_do_none,
    do_ur_stuff,
    warming(/*times*/ u64),*/
        match in_val {
            amaze_me::ret_indx_n_do_none => {return Some (state);}
            amaze_me::do_ur_stuff => {
                state ^= u64_();
                let indx_mode = crate::swtch::local_indx(false);
                if indx_mode {crate::swtch::local_indx(true);}
                let count = crate::ps18::get_num_files(510974534);
                let file_indx = state % count as u64;
                let mut item = crate::globs18::get_item_from_front_list_times( file_indx as i64, true, 100 );
                let cmd = format!("0 {item}");
                crate::swtch::run_viewer(cmd);
                if indx_mode {crate::swtch::local_indx(true);} 
                crate::set_prnt( &format!("surprise me {file_indx} {item}"), 510974534 );
                return Some( file_indx );
            }
            amaze_me::warming(c) => {
                state ^= warming_up( c );
                let indx_mode = crate::swtch::local_indx(false);
                if indx_mode {crate::swtch::local_indx(true);}
                let count = crate::ps18::get_num_files(510974534);
                let file_indx = state % count as u64;
                let mut item = crate::globs18::get_item_from_front_list_times( file_indx as i64, true, 100 );
                let cmd = format!("0 {item}");
                crate::swtch::run_viewer(cmd);
                if indx_mode {crate::swtch::local_indx(true);} 
                crate::set_prnt( &format!("surprise me {file_indx}"), 510974534 );
                return Some( file_indx );
            }
        }       
    }
    None
}
#[cfg(feature = "in_dbg")]
pub fn surprise_me_dry_run(cmd: Option < amaze_me > ) -> Option <u64>{
    if cmd == None {return None;}
    static mut state: u64 = 0;
    use Mademoiselle_Entropia::true_rnd::get_true_rnd_u64 as u64_;
    unsafe {
        let in_val = cmd.unwrap();
        /*  ret_indx_n_do_none,
    do_ur_stuff,
    warming(/*times*/ u64),*/
        match in_val {
            amaze_me::ret_indx_n_do_none => {return Some (state);}
            amaze_me::do_ur_stuff => {
                state ^= u64_();
                let indx_mode = crate::swtch::local_indx(false);
                if indx_mode {crate::swtch::local_indx(true);}
                let count = crate::ps18::get_num_files(510974534);
                let file_indx = state % count as u64;
                let mut item = crate::globs18::get_item_from_front_list_times( file_indx as i64, true, 100 );
                let cmd = format!("0 {item}");
                if indx_mode {crate::swtch::local_indx(true);} 
                crate::set_prnt( &format!("surprise me dry run {file_indx} {item}"), 510974534 );
                return Some( file_indx );
            }
            amaze_me::warming(c) => {
                state ^= warming_up( c );
                let indx_mode = crate::swtch::local_indx(false);
                if indx_mode {crate::swtch::local_indx(true);}
                let count = crate::ps18::get_num_files(510974534);
                let file_indx = state % count as u64;
                let mut item = crate::globs18::get_item_from_front_list_times( file_indx as i64, true, 100 );
                let cmd = format!("0 {item}");
                crate::swtch::run_viewer(cmd);
                if indx_mode {crate::swtch::local_indx(true);} 
                crate::set_prnt( &format!("surprise me {file_indx}"), 510974534 );
                return Some( file_indx );
            }
        }       
    }
    None
}

pub fn warming_up(times: u64) -> u64{
    use Mademoiselle_Entropia::true_rnd::get_true_rnd_u64 as u64_;
    let mut ret =0u64;
    for i in 0..=times{
        ret ^= u64_();
    } ret
}
//fn