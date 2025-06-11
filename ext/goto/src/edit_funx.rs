/* Edit func body */ 
use once_cell::sync::Lazy;
use substring::Substring;
use Mademoiselle_Entropia::custom_traits::{STRN, helpful_math_ops};
use crate::lex::blocks_status as blocks_state;
pub fn rewrite_last_exit (stream: &String, new_end: &String ) -> String {
    let last_exit = find_last_exit ( stream );
    let edit = format! ("\n{new_end}\n{last_exit}");
    let stream = stream.replace (&last_exit, &edit);
    return stream
}
pub fn find_last_exit (stream: &String ) -> String {
    let stream_len = stream.chars().count();
    let stop = Some (';');
    let mut count_ending: usize = 0;
    let mut chars = stream.chars();
    let mut ending = String::new();
    let mut cursor = stream_len - 1;
    let mut ch: Option < char > = Some (' ');
    loop {
        ch = chars.nth (cursor );
        if ch != stop || blocks_state (ch.as_ref() )  { ending.push (ch.unwrap_or (' ') ); }
        if ch == stop { break;}
        cursor.dec();
    }
    return ending.rev()
}
pub trait Rev {
   fn rev (&mut self) -> Self;
}
impl Rev for String {
    fn rev (&mut self) -> Self {
        let mut rev_ = String::new();
        let mut len = self.chars().count() as isize;
        let mut chars = self.chars();
        while len > -1 {
            rev_.push ( chars.nth (len.try_into ().unwrap() ).unwrap () );
            len -= 1;
        }
        *self = rev_;
        return self.clone()
    }
}
