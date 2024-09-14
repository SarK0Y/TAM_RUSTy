use once_cell::sync::Lazy;

pub fn fork_lag_mcs_bool (mcs: u128) -> bool {
    static mut start_time: Lazy < std::time::SystemTime > = Lazy::new(|| {std::time::SystemTime::now() }); 

    unsafe {
        let end_time = std::time::SystemTime::now();
        let end: u128 = match end_time.duration_since(std::time::UNIX_EPOCH){Ok(dur) => dur, _ => return false}.as_micros();
        let start: u128 = match start_time.duration_since(std::time::UNIX_EPOCH){Ok(dur) => dur, _ => return false }.as_micros();
        *start_time =  end_time;
        if end - start > mcs { return true} false
    }
} 
//fn
