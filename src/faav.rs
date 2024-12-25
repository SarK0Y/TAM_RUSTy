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
pub fn freq_range_status (data: Option <crate::enums::freq_range>, unset: bool ) -> Option <crate::enums::freq_range> {
    static mut state: Lazy< Option <crate::enums::freq_range> > = Lazy::new(||{None});
    unsafe {
        if data.is_some() {*state = data.clone();}
        if unset == true {*state = None; return None;}
        dbg!(state.is_some() );
        state.clone()
    }
}
pub fn geom_status (data: Option < Vec <f32> >, unset: bool ) -> Vec <f32> {
    static mut state: Lazy< Vec <f32> > = Lazy::new(||{Vec::new () });
    unsafe {
        if data.is_some() {
            for da in data.as_ref().unwrap() {
                state.push ( *da );
            }
            }
        if unset == true {state.clear();}
        //dbg!(state.is_some() );
        state.clone()
    }
}
pub fn pocket_geom (data: &Vec <f32> ){
    geom_status(Some (data.clone() ), false);
}
pub fn read_saved_geom() -> Vec <f32> {
    geom_status(None, false)
}
pub fn unset_geom (){
    geom_status(None, true);
}
pub fn morph_status (data: &Option < ( wavers::Samples <f32>, i32)>, unset: bool ) -> Option < ( wavers::Samples <f32>, i32)> {
    static mut state: Lazy< Option < ( wavers::Samples <f32>, i32)> > = Lazy::new(||{None});
    unsafe {
        if data.is_some() {*state = data.clone();}
        if unset == true {*state = None; return None;}
        dbg!(state.is_some() );
        state.clone()
    }
}
pub fn get_morph_state () -> Option < ( wavers::Samples <f32>, i32)> {
    morph_status(&None, false)
}
pub fn set_morph_state (data: &Option < ( wavers::Samples <f32>, i32) > )  {
    morph_status(data, false);
}
pub fn unset_morph_state () {
    morph_status(&None, true);
}
//fn