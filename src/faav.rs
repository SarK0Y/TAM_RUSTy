use once_cell::sync::Lazy;
use crate::custom_traits::STRN; use crate::custom_traits::helpful_math_ops;
pub fn one_time_sav_prnt (prnt: Option <String > ) -> Option < String > {
    static mut state: Lazy <String> = Lazy::new (|| { String::new() });
    static mut count_to_reset: u32 = 1;
    unsafe {
        if let Some( x ) = prnt.clone() {
            if x == "" { return None;}
            *state = x; count_to_reset = 1; return prnt ;}
        if count_to_reset == 0 { return None;} count_to_reset.dec();
        Some ( state.clone() )
    }
}
pub fn new_obj_id () -> u128 {
    static mut id: u128 = 0;
    unsafe {
        if id == u128::MAX { id = 0; return u128::MAX; }
        id.inc(); return id - 1;
    }
}
