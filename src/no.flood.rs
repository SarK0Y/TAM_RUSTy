use once_cell::sync::Lazy;

use crate::{enums, smart_lags};

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
//fn
