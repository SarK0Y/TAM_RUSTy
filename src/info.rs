use crate::repeat_char;

pub(crate) fn SYS(){
    println!("Have a nice day, DEAR USER\nSee You Soon", );
    std::process::exit(0)
}
const Author: &str = "Knyazhev Evgeney (SarK0Y)";
const Ver: &str = "1.9.84";
const Telega: &str = "https://t.me/+N_TdOq7Ui2ZiOTM6";
pub(crate) fn info(){

}
pub(crate) fn banners_line(val: &str, filler: &str){
    use terminal_size::{Width, Height, terminal_size};

let size = terminal_size();
let (mut w, h) = (80u16, 300u16);
if let Some((Width(w_), Height(h_))) = size { w = w_; }
let w_half_size: usize = (w / 2).into();
let val_half_size = val.chars().count() / 2 - w_half_size - 1;
let filler = repeat_char(w_half_size, &filler);
println!("{filler} {val} {filler}");
}