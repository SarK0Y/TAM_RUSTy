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
pub fn new_obj_id () -> u64 {
    static mut id: u64 = 0;
    unsafe {
        if id == u64::MAX { id = 0; return 0; }
        id.inc(); return id - 1;
    }
}
pub fn kill_prox_chain (state: Option < bool >) -> bool {
    static mut state0: bool = false;
    unsafe {
        let prev = state0;
        if let Some ( x ) = state {state0 = x; } 
        else { state0 = false } prev
    }
}
pub fn fin_prox_loop (state: Option < bool >) -> bool {
    static mut state0: bool = false;
    unsafe {
        let prev = state0;
        if let Some ( x ) = state {state0 = x; } 
        else { state0 = false } prev
    }
}
pub fn proc_exited (state: Option < bool >) -> bool {
    static mut state0: bool = false;
    unsafe {
        let prev = state0;
        if let Some ( x ) = state {state0 = x; }
        else { state0 = false } prev
    }
}
pub fn lock_control_c (state: Option < bool >) -> bool {
    static mut state0: bool = false;
    unsafe {
        if let Some ( x ) = state {state0 = x; } state0
    }
}
pub fn count_getkey (state: Option < i64 >) -> i64 {
    static mut state0: i64 = 0;
    unsafe {
        if let Some ( x ) = state {
            if x == 0 { state0 = 0; return 0;}
            state0 += x; 
        } state0 
    }
}
//fn