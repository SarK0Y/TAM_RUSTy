use crate::custom_traits::helpful_math_ops;
use crate::{enums, lst, smart_lags, STRN};
use nix::fcntl::FallocateFlags;
use once_cell::sync::Lazy;
use std::collections::HashMap;
pub fn fork_lag_mcs_bool(mcs: u128) -> bool {
    static mut start_time: Lazy<std::time::SystemTime> = Lazy::new(|| std::time::SystemTime::now());

    unsafe {
        let end_time = std::time::SystemTime::now();
        let end: u128 = match end_time.duration_since(std::time::UNIX_EPOCH) {
            Ok(dur) => dur,
            _ => return false,
        }
        .as_micros();
        let start: u128 = match start_time.duration_since(std::time::UNIX_EPOCH) {
            Ok(dur) => dur,
            _ => return false,
        }
        .as_micros();
        *start_time = end_time;
        let end = crate::key_handlers::delta(end, start);
        if end > mcs {
            return true;
        }
        false
    }
}
pub fn fork_lag_mcs_verbose(mcs: u128) -> crate::enums::smart_lags {
    static mut start_time: Lazy<std::time::SystemTime> = Lazy::new(|| std::time::SystemTime::now());

    unsafe {
        let end_time = std::time::SystemTime::now();
        let end: u128 = match end_time.duration_since(std::time::UNIX_EPOCH) {
            Ok(dur) => dur,
            _ => return enums::smart_lags::failed,
        }
        .as_micros();
        let start: u128 = match start_time.duration_since(std::time::UNIX_EPOCH) {
            Ok(dur) => dur,
            _ => return enums::smart_lags::failed,
        }
        .as_micros();
        *start_time = end_time;
        let end = crate::key_handlers::delta(end, start);
        if end > mcs {
            return enums::smart_lags::well_done(end);
        }
        enums::smart_lags::too_small_lag(end)
    }
}
pub fn screen_lag(lag: Option<u128>) -> u128 {
    static mut state: u128 = 2000;
    unsafe {
        if let Some(x) = lag {
            state = x
        }
        state
    }
}
pub fn set_screen_lag(cmd: String) {
    let cmd = cmd.replace("screen lag", "").trim_end().trim_start().strn();
    if let Ok(lag) = cmd.parse::<u128>() {
        screen_lag(Some(lag));
    }
}
use crate::enums::named_mutex;
pub fn mamed_mutexes(
    name: &String,
    op: named_mutex,
    mutex: &mut crate::enums::custom_mutex,
) -> Option < bool > {
    static mut lst_mutexes: Lazy<HashMap<String, bool>> = Lazy::new(|| HashMap::new());
    if let crate::enums::smart_lags::well_done(_) = fork_lag_mcs_verbose(7111) {
    } else {
        return None;
    }
    dbg! ("check");
    unsafe {
        match op {
            named_mutex::get => {
                if lst_mutexes.contains_key(name) {
                    if *mutex.owner == mutex.id { return Some( true );}
                    return Some( false );
                }
                return None;
            }
            named_mutex::set => {
                dbg! ("check");
                if let Some(x) = lst_mutexes.get_mut(name) {
                    let cond = ( *mutex.owner == u64::MAX || *mutex.owner == mutex.id );
                    if cond { *mutex.owner = mutex.id; *x = true; mutex.status = *x  };
                    return Some(*x);
                }
                let cond = ( *mutex.owner == u64::MAX || *mutex.owner == mutex.id );
                if cond { *mutex.owner = mutex.id; mutex.status = true};
                lst_mutexes.insert(name.strn(), true); 
                dbg! ("check");
                return Some(true);
            }
            named_mutex::unset => {
                if let Some(x) = lst_mutexes.get_mut(name) {
                    let cond = ( *mutex.owner == u64::MAX || *mutex.owner == mutex.id );
                    if cond { *mutex.owner = mutex.id; *x = false; mutex.status = *x };
                    return Some(*x);
                }
                let cond = ( *mutex.owner == u64::MAX || *mutex.owner == mutex.id );
                if cond { *mutex.owner = u64::MAX; mutex.status = false };
                lst_mutexes.insert(name.strn(), false);
                return Some(false);
            }
            _ => {}
        }
    }
    None
}
pub fn mutex_group_register(op: crate::enums::mutex_group) -> (*mut u64, usize) {
    static mut lst_mutexes: Lazy<HashMap<String, usize > > = Lazy::new(|| HashMap::new());
    static mut register: Lazy<Vec<*mut u64>> = Lazy::new(|| Vec::new());
    unsafe {
        if let crate::enums::mutex_group::set (x ) = op {
            if lst_mutexes.contains_key ( x ) {
                let line = lst_mutexes.get ( x ).unwrap ();
                let pointer: *mut u64 = register [ *line ]; 
                return (pointer, *line);
            }
            let mut shared: std::mem::ManuallyDrop < Box < u64 > > = std::mem::ManuallyDrop::new (Box::new (u64::MAX ) );
            //Box::into_raw ( std::mem::ManuallyDrop::into_inner (shared) );
            let pointer: *mut u64 = &mut **shared;
            register.push (pointer);
            lst_mutexes.insert ( x.strn(), register.len () - 1 );
            return (pointer, register.len () - 1);
        }
    }
    (std::ptr::null_mut (), 0)
}
pub trait new_custom_mutex {
    fn new ( name: &String) -> crate::enums::custom_mutex;
}
impl new_custom_mutex for crate::enums::custom_mutex {
    fn new ( name: &String ) -> crate::enums::custom_mutex {
        let reg = mutex_group_register( crate::enums::mutex_group::set ( name ) );
        Self {
            line_in_register: reg.1,
            owner: reg.0,
            id: crate::faav::new_obj_id(),
            status: false,
            rank: 0
        }
    }
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

pub fn mamed_mutexes(
    name: &String,
    op: named_mutex,
    mutex: &mut crate::enums::custom_mutex,
) -> Option < bool > {
    static mut lst_mutexes: Lazy<HashMap<String, bool>> = Lazy::new(|| HashMap::new());
    if let crate::enums::smart_lags::well_done(_) = fork_lag_mcs_verbose(200111) {
    } else {
        return None;
    }
    unsafe {
        match op {
            named_mutex::get => {
                if lst_mutexes.contains_key(name) {
                    return Some(*lst_mutexes.get(name).unwrap());
                }
                return None;
            }
            named_mutex::set => {
                if let Some(x) = lst_mutexes.get_mut(name) {
                    *x = true;
                    return Some(*x);
                }
                lst_mutexes.insert(name.strn(), true);
                return Some(true);
            }
            named_mutex::unset => {
                if let Some(x) = lst_mutexes.get_mut(name) {
                    *x = false;
                    return Some(*x);
                }
                lst_mutexes.insert(name.strn(), false);
                return Some(false);
            }
            _ => {}
        }
    }
    None
}
///////////// rethink or just kick out ////////////////////
*/
