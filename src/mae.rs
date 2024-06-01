use Mademoiselle_Entropia::{Mademoiselle_Entropia::cipher, help_funcs};
use crate::{custom_traits::{STRN, STRN_strip}, getkey};
pub(crate) fn encrypt_n_keep_orig_file(cmd: String){
    let func_name = "encrypt_n_keep_orig_file".strn();
    let file_to_encrypt = cmd.replace("encrypt copy", "").trim_start().trim_end().strn();
    match std::fs::copy(&file_to_encrypt, format!("{file_to_encrypt}.mae")){Ok(f) => {}, Err(e) =>  {return println!("{func_name} got {e:?}" );}}
    let fst_pswd = pswd(None, false);
    let nd_pswd = pswd(Some("Please, repeat Your password: ".strn()), false);
    if nd_pswd != fst_pswd{return println!("Sorry, Dear User, try again", );}
    let mut file = match help_funcs::get_file(format!("{file_to_encrypt}.mae")){Ok(f) => f, 
                                                            Err(e) => return println!("Sorry, can't open {file_to_encrypt}.mae: {e:?}")};
    let IK_len = crate::globs18::strn_2_usize(
        pswd(Some("Please, enter a size of IK".strn()), true)
       ).unwrap_or(256);
    let buf_size = crate::globs18::strn_2_usize(
        pswd(Some("Please, enter the buffer's size ".strn()), true)
    ).unwrap_or(10_000);
    file.encrypt(&fst_pswd, buf_size, IK_len);
    println!("Dear User, Please, hit any key to continue.. Thanks."); getkey();
}
pub(crate) fn pswd(prompt: Option<String>, echo: bool) -> String{
    let prompt0 = prompt.clone();
    let mut prompt1 = "Dear User, enter Your password: ".strn(); 
    if let Some(p) = prompt{prompt1 = p}
    print!("{prompt1}");
    let mut password = String::new();
    loop { 
        let k = getkey();
        let ansiKey: u8 = match k.as_str().bytes().next(){
        Some(val) => val,
        _ => 0
        };
        if ansiKey == crate::kcode01::ENTER{break;}
        password.push_str(k.as_str());
        if echo{print!("\r{prompt1}{}", password.clone())}   
    }
    if password == ""{
        let mut STRING = "string";
        if !echo {STRING = "password/token"}
        println!("Dear User, Please, don't leave empty {STRING}.. Thanks"); password = pswd(prompt0, echo);}
    password
}