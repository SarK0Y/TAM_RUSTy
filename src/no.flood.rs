use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::{enums, smart_lags, STRN};
use crate::custom_traits::helpful_math_ops;
pub fn fork_lag_mcs_bool (mcs: u128) -> bool {
    static mut start_time: Lazy < std::time::SystemTime > = Lazy::new(|| {std::time::SystemTime::now() }); 

    unsafe {
        let end_time = std::time::SystemTime::now();
        let end: u128 = match end_time.duration_since(std::time::UNIX_EPOCH){Ok(dur) => dur, _ => return false}.as_micros();
        let start: u128 = match start_time.duration_since(std::time::UNIX_EPOCH){Ok(dur) => dur, _ => return false }.as_micros();
        *start_time =  end_time;
        let end = crate::key_handlers::delta( end, start);
        if end > mcs { return true} false
    }
} 
pub fn fork_lag_mcs_verbose (mcs: u128) -> crate::enums::smart_lags {
    static mut start_time: Lazy < std::time::SystemTime > = Lazy::new(|| {std::time::SystemTime::now() }); 

    unsafe {
        let end_time = std::time::SystemTime::now();
        let end: u128 = match end_time.duration_since(std::time::UNIX_EPOCH){Ok(dur) => dur, _ => return enums::smart_lags::failed }.as_micros();
        let start: u128 = match start_time.duration_since(std::time::UNIX_EPOCH){Ok(dur) => dur, _ => return enums::smart_lags::failed }.as_micros();
        *start_time =  end_time;
        let end = crate::key_handlers::delta( end, start);
        if end > mcs { return enums::smart_lags::well_done( end )} enums::smart_lags::too_small_lag( end )
    }
}
pub fn screen_lag (lag: Option < u128 >) -> u128 {
    static mut state: u128 = 2000;
    unsafe {
        if let Some ( x ) = lag { state = x } state
    }
}
pub fn set_screen_lag (cmd: String) {
    let cmd = cmd.replace("screen lag", "").trim_end().trim_start().strn();
    if let Ok (lag ) = cmd.parse:: <u128>() { screen_lag( Some ( lag ) ); }
}
use crate::enums::named_mutex;
pub fn mamed_mutexes (name: &String, op: named_mutex, mutex: &mut crate::enums::custom_mutex ) -> Option < bool >
{
    static mut lst_mutexes: Lazy < HashMap < String, bool > > = Lazy::new (|| {HashMap::new ()});
    if let crate::enums::smart_lags::well_done( _ ) = fork_lag_mcs_verbose( 200111) {  } else { return None }
    unsafe {
        match op {
            named_mutex::get => {
                if lst_mutexes.contains_key ( name ) { return Some( *lst_mutexes.get ( name ).unwrap () ) }
                return None
            }
            named_mutex::set => {
                if let Some ( x ) = lst_mutexes.get_mut ( name ) {
                    *x = true; return Some ( *x )
                }
                lst_mutexes.insert (name.strn(), true); return Some ( true )
            }
            named_mutex::unset => {
                if let Some ( x ) = lst_mutexes.get_mut ( name ) {
                    *x = false; return Some ( *x )
                }
                lst_mutexes.insert (name.strn(), false); return Some ( false )
            }
            _ => {}
        }
    }
    None
}
//fn
/*
///////////// rethink or just kick out ////////////////////
pub fn check_named_mutexes_failed (timeout: Option < u64 >) -> (u64, u64 ) {
    static mut timeout0: u64 = 100;
    static mut count: u64 = 0;
    unsafe {
        if let Some ( x ) = timeout {
        if x == 0 { count = x; return ( timeout0, 0) }
        timeout0 = x ; count = 0; return (timeout0, 0);
        }
        if count == timeout0 { return ( timeout0, timeout0 )} 
        count.inc (); return (timeout0, count -1)
    }
}
///////////// rethink or just kick out ////////////////////
*/