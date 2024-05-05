use crate::{repeat_char, run_cmd_out_sync};

pub(crate) fn SYS(){
    println!("Have a nice day, DEAR USER\nSee You Soon", );
    std::process::exit(0)
}
pub const Author: &str = "Knyazhev Evgeney (SarK0Y)";
const Ver: &str = "1.9.84";
const Telega: &str = "https://t.me/+N_TdOq7Ui2ZiOTM6";
pub(crate) fn info(){

}
pub(crate) fn banners_line(val: &str, filler: &str){
    use terminal_size::{Width, Height, terminal_size};
    use console::Term;
    use termion::terminal_size as sizze;

let size = sizze();//Term::size(&Term::stdout());
let (mut w, h) = (80u32, 300u32);
if (0, 0) != match size{
    Ok(ts) => ts,
    _ => (0, 0)
} { w = size.unwrap().0.into(); println!("w {w}", );}
let mut w_half_size: usize = (w / 2) as usize;
let mut val_half_size = (val.chars().count() +2) / 2 ;
if w_half_size + 1 > val_half_size{w_half_size -= (val_half_size)}
let filler_left = repeat_char(w_half_size, &filler);
let filler_right = repeat_char(w_half_size +1, &filler);
println!("{filler_left} {val} {filler_right}");
}