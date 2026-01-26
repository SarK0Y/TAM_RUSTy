use once_cell::sync::Lazy;
use crate::custom_traits::STRN; use crate::custom_traits::helpful_math_ops;
use rug::{Assign, Integer as rugint, float::Constant as rugconst, Float as rugfloat, ops::SubFrom};
use crate::errMsg0 as _msg;
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
pub fn sav_uid (uid: Option <String > ) -> Option < String > {
    static mut state: Lazy <String> = Lazy::new (|| { String::new() });
    unsafe {
        if let Some( x ) = uid.clone() {
            if x == "" { return None;} *state = x;
        } Some ( state.clone() )
    }
}
pub fn __delim_for_newline (delim: Option <String > ) -> Option < String > {
    static mut state: Lazy <String> = Lazy::new (|| { "_457=@x_".strn() });
    unsafe {
        if let Some( x ) = delim.clone() {
            if x == "" { return None;} *state = x;
        } Some ( state.clone() )
    }
}
pub fn log_file_printIt (name: Option <String > ) -> Option < String > {
    static mut state: Lazy <String> = Lazy::new (|| { String::new() });
    unsafe {
        if let Some( x ) = name.clone() {
            if x == "" { return None;} *state = x;
        } Some ( state.clone() )
    }
}
pub fn new_obj_id () -> u64 {
    static mut id: u64 = 0;
    unsafe {
        if id == u64::MAX { id = 0; return 0; }
        id.inc(); return id - 1;
    }
}
pub fn npf_lock_ (ceil: Option < i64 >) -> Option < i64 > {
    static mut lock: bool = false;
    static mut id: i64 = 0;
    static mut max_id: i64 = 0;
    unsafe {
        if let Some (x) = ceil { 
            if x < 0 {id = 0; lock = false; return Some (id); }
            max_id = x; return None;
        }
        if lock { return None }
        if id == i64::MAX { id = 0; return Some (0); }
        id.inc(); 
        if id >= max_id { lock = true; return None; } return Some ( id - 1 );
    }
}
pub fn __orig_strn (name: Option <String > ) -> Option < String > {
    static mut state: Lazy <String> = Lazy::new (|| { String::new() });
    unsafe {
        if let Some( x ) = name.clone() {
            if x == "" { return None;} *state = x;
        } Some ( state.clone() )
    }
}
pub fn yes_newline_in_filename (state: Option < bool >) -> bool {
    static mut lock: bool = false;
    unsafe {
        if let Some (x) = state { 
            lock = x; return lock.clone();
        } return lock.clone()
    }
}
pub fn fork_tam_mode () -> bool {
    static mut yes: bool = false;
    static mut _1st: bool = true;
    unsafe {
        if _1st {
            if crate::checkArg ("-fork-mode") {yes = true;}
            _1st = false;
        } return yes
    }
}
pub fn npf_lock (state: Option < bool >) -> bool {
    static mut lock: bool = false;
    unsafe {
        if let Some (x) = state { 
            lock = x; return lock.clone();
        } return lock.clone()
    }
}
pub fn npf_split (state: Option < bool >) -> bool {
    static mut yes: bool = false;
    unsafe {
        if let Some (x) = state { 
            yes = x; return yes.clone();
        } return yes.clone()
    }
}
pub fn npf_sq (state: Option < bool >) -> bool {
    static mut yes: bool = false;
    unsafe {
        if let Some (x) = state { 
            yes = x; return yes.clone();
        } return yes.clone()
    }
}
pub fn npf_bar ( bit_id: usize, ceil: Option < usize >) -> bool {
    static mut max_id: usize = 0;
    unsafe {
        if let Some (x) = ceil { 
            let prev = max_id;
            max_id = x; 
            let msg = format! ("max bit_id {}", x);
            if prev > 0 { _msg (&msg); } return true;
        }
        if bit_id > max_id { return false; } return true;
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
pub fn lock_surprise_me (state: Option < bool >) -> bool {
    static mut state0: bool = false;
    unsafe {
        if let Some ( x ) = state {state0 = x; } state0
    }
}
pub fn end_read_midway (state: Option < bool >) -> bool {
    static mut state0: bool = false;
    unsafe {
        if let Some ( x ) = state {state0 = x; } state0
    }
}
pub fn count_ln_in_surprise_me_lst(yes: bool, inc: bool, get_size: bool) -> usize{
    static mut count: usize = 0;
    if get_size {return unsafe { count } }
    if !yes {unsafe { count = 0 }; return 0;}
    let ret = unsafe { count };
    if yes && inc{unsafe { count.inc() };}
    if yes && !inc{unsafe { count.dec() };}
    ret
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
pub fn too_long_len_for_bash (state: Option < usize >) -> usize {
    static mut state0: usize = 57;
    unsafe {
        if let Some ( x ) = state {
            if x == 0 { state0 = 0; return 0;}
            state0 += x; 
        } state0 
    }
}

pub fn real_e (state: Option < rugfloat >, prec: u64) -> rugfloat {
    static mut state0: Lazy< rugfloat > = Lazy::new (|| {rugfloat::with_val_64(3, 0.0)} );
    unsafe {
        if state.is_some () {
            *state0 = rugfloat::with_val_64(prec, state.unwrap() );
        } state0.clone() 
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
pub fn over_cdft (pointer: Option <*mut crate::enums::custom_dft>) -> Option <*mut crate::enums::custom_dft> {
    static mut state: Lazy < Option <*mut crate::enums::custom_dft > > = Lazy::new (|| {None});
    unsafe {
        if pointer.is_some() { *state = pointer} state.clone()
    }
}
pub fn over_samples (pointer: Option <*mut [f32]>) -> Option <*mut [f32]> {
    static mut state: Lazy < Option <*mut [f32] > > = Lazy::new (|| {None});
    unsafe {
        if pointer.is_some() { *state = pointer} state.clone()
    }
}
pub fn over_samples0 (pointer: Option <*mut [f32]>) -> Option <*mut [f32]> {
    static mut state: Lazy < Option <*mut [f32] > > = Lazy::new (|| {None});
    unsafe {
        if pointer.is_some() { *state = pointer} state.clone()
    }
}
pub fn over_uv (pointer: Option <*const crate::enums::universum_vox_morph>) -> Option <*const crate::enums::universum_vox_morph> {
    static mut state: Lazy < Option <*const crate::enums::universum_vox_morph > > = Lazy::new (|| {None});
    unsafe {
        if pointer.is_some() { *state = pointer} state.clone()
    }
}
pub enum ManageViewers <'a> {
    get_by_indx (usize),
    get_by_name (&'a String),
    add (String),
    out_strn (String),
    out_ref_strn (&'a String),
    out_usize (usize),
    no_action_needed,
    show_lst,
    null
}
pub fn full_addr_of_viewer (_in: ManageViewers) -> ManageViewers {
    static mut list: Lazy <Vec <String> > = Lazy::new (||{ Vec::new() });
    unsafe {
        match _in {
            ManageViewers::add (x) => {
                list.push (x);
            },
            ManageViewers::get_by_indx (y) => {
                if y < list.len () {
                    return ManageViewers::out_strn (list[ y ].clone())
                } return ManageViewers::null
            },
            ManageViewers::get_by_name ( n ) => {
                for name in list.iter () {
                    if name.contains ( n ) {
                        return ManageViewers::out_strn (name.clone() )
                    } 
                } return ManageViewers::null
            },
#[cfg (feature = "in_dbg")]
            ManageViewers::show_lst => {
                dbg! (&list);
                crate::just_break ();
            }
            _ => {}
        }
    }
    return ManageViewers::null
}
//fn
