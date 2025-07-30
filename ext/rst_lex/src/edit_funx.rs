/* Edit func body */ 
use once_cell::sync::Lazy;
use substring::Substring;
use Mademoiselle_Entropia::custom_traits::{STRN, helpful_math_ops};
//#[cfg(feature ="dev_hell_n_fun")]
use crate::lex::{ blocks_status as blocks_state, leave_file_mark };
use crate::faav::blocks;
//#[cfg(feature ="stable")]
//use crate::stable_lex::{ blocks_status as blocks_state, leave_file_mark };
#[cfg(any(feature ="dev_hell_n_fun", feature = "stable"))]
pub fn rewrite_last_exit (stream: &String, new_end: &String ) -> String {
    let last_exit = find_last_exit ( stream ).trim().strn();
    leave_file_mark ("/tmp/enter", &last_exit );
    if last_exit.chars().count() < 3 {
        let top = stream.as_str().substring(0, stream.chars().count() - 1);
        let edit = format! ("{top}\n{new_end}\n}}");
        leave_file_mark ("/tmp/edit", &edit);
        return edit
    }
    let edit = format! ("\n{new_end}\n{last_exit}");
    let stream0 = stream.replace (&last_exit, &edit);
    leave_file_mark ("/tmp/func0", &stream0);
    leave_file_mark ("/tmp/last", &last_exit);
    leave_file_mark ("/tmp/edit", &edit);
    return stream0
}
//#[cfg(feature ="dev_hell_n_fun")]
pub fn find_last_exit (stream: &String ) -> String {
    let stream_len = stream.chars().count();
    let stop = ';';
    leave_file_mark ("/tmp/ending", "tst");
    let mut count_ending: usize = 0;
    let mut chars = stream.chars();
    let mut ending = String::new();
    let mut cursor = stream_len - 2;
    let mut block_ = blocks::new();
    let mut ch: char = ' ';
    while cursor > 0 {
        ch = chars.clone().nth (cursor ).unwrap_or (' ');
        //leave_file_mark ("/tmp/ch", &ch.to_string () );
        if ch != stop || blocks_state ( Some ( &ch ), Some (&mut block_) )  { ending.push (ch ); }
        if ch == stop && !blocks_state ( Some ( &ch ), Some (&mut block_) ) { break;}
        cursor.dec();
    }
    leave_file_mark ("/tmp/ending1", &ending);
    return ending.rev()
}

pub trait Rev {
   fn rev (&mut self) -> Self;
}
impl Rev for String {
    fn rev (&mut self) -> Self {
        leave_file_mark ("/tmp/norev", self);
        let mut rev_ = String::new();
        let mut len = self.chars().count() as isize;
        let mut chars = self.chars();
        while len > -1 {
            rev_.push ( chars.clone().nth (len.try_into ().unwrap_or(0) ).unwrap_or (' ') );
            len -= 1;
        }
        *self = rev_;
        return self.clone()
    }
}
pub fn dirty_fix (fn_body: &mut String) {
    *fn_body = fn_body
    .replace("pub fn", "pub fn ")
    .replace("mutcrate", "mut crate")
    .strn();
}
