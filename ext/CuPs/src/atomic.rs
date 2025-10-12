use once_cell::sync::Lazy;
use std::os::fd::FromRawFd;
use std::mem::ManuallyDrop;
use std::io::Write;
use crate::enums::named_mutex; 
use crate::smart_lags::new_custom_mutex;
use Mademoiselle_Entropia::custom_traits::STRN;
pub fn stdin_write <T: ToString + STRN > (strn: Option < T > ) {
  //  static mut fd0: Lazy < Option < std::fs::File > > = Lazy::new (|| { None });
  //  static mut fst_run: bool = true;
    unsafe {
  //      if fst_run {
            let mut file = std::fs::File::from_raw_fd(0/*stdin*/);
      //      let mut undead = ManuallyDrop::new (file );
    //        *fd0 = Some( ManuallyDrop::into_inner (undead) );
            
    //    }
        let fn_name = "stdin_write".strn();
    let mut mutex_state = false;
    let mut mutex: crate::enums::custom_mutex = crate::enums::custom_mutex::new( &fn_name );
    if let Some ( x ) = crate::smart_lags::mamed_mutexes (&fn_name, named_mutex::set, &mut mutex ) {mutex_state = x}
    //else { crate::smart_lags::mamed_mutexes (&fn_name, named_mutex::set, &mut mutex ); mutex_state = true;}
    //if crate::smart_lags::mamed_mutexes (&fn_name, named_mutex::get ).is_none() {mutex = false }
    let mut cond = unsafe { ( *mutex.owner == u64::MAX || *mutex.owner == mutex.id ) };
    while cond == false {
        cond = unsafe { ( *mutex.owner == u64::MAX || *mutex.owner == mutex.id ) };
        if let Some ( x ) = crate::smart_lags::mamed_mutexes (&fn_name, named_mutex::set, &mut mutex ) {mutex_state = x}
        unsafe { dbg! (*mutex.owner ); dbg! (mutex.id ); }
    }
    //libc::pthread_cancel( abort.as_pthread_t() ); 
    //if strn.is_none () { (*fd0.as_mut().unwrap()).flush (); return }
    //if strn.is_none () { fd0.as_ref().unwrap().write("".strn().as_bytes() ); return }
    if strn.is_none () { file.write("".strn().as_bytes() ); return }
    //fd0.as_ref().unwrap().write(strn.unwrap().strn().as_bytes() );
    file.write(strn.unwrap().strn().as_bytes() );
    std::mem::forget ( file );
  //  std::mem::drop (writeIn_stdin);
    crate::smart_lags::forcely_set_mamed_mutexes (&fn_name, named_mutex::unset, &mut mutex ); //dbg!("end");
    }
}
pub fn delay_mcs(sleep: u64){
    std::thread::sleep(std::time::Duration::from_micros(sleep));
}
pub fn delay_ns(sleep: u64){
    std::thread::sleep(std::time::Duration::from_nanos(sleep));
}
pub fn glob_delay (delay: Option <u64> ) -> u64{
    static mut sav: u64 = 500;
    unsafe {
        if let Some (x) = delay {
            sav = x;
        } return sav
    }
}
//fn
/*
////////////////////// stable variant ///////////////////////////
pub fn stdin_write <T: ToString + STRN > (strn: Option < T > ) {
  //  static mut fd0: Lazy < Option < std::fs::File > > = Lazy::new (|| { None });
  //  static mut fst_run: bool = true;
    unsafe {
  //      if fst_run {
            let mut file = std::fs::File::from_raw_fd(0/*stdin*/);
      //      let mut undead = ManuallyDrop::new (file );
    //        *fd0 = Some( ManuallyDrop::into_inner (undead) );
            
    //    }
        let fn_name = "stdin_write".strn();
    let mut mutex_state = false;
    let mut mutex: crate::enums::custom_mutex = crate::enums::custom_mutex::new( &fn_name );
    if let Some ( x ) = crate::smart_lags::mamed_mutexes (&fn_name, named_mutex::set, &mut mutex ) {mutex_state = x}
    //else { crate::smart_lags::mamed_mutexes (&fn_name, named_mutex::set, &mut mutex ); mutex_state = true;}
    //if crate::smart_lags::mamed_mutexes (&fn_name, named_mutex::get ).is_none() {mutex = false }
    let mut cond = unsafe { ( *mutex.owner == u64::MAX || *mutex.owner == mutex.id ) };
    while cond == false {
        cond = unsafe { ( *mutex.owner == u64::MAX || *mutex.owner == mutex.id ) };
        if let Some ( x ) = crate::smart_lags::mamed_mutexes (&fn_name, named_mutex::set, &mut mutex ) {mutex_state = x}
        unsafe { dbg! (*mutex.owner ); dbg! (mutex.id ); }
    }
    //libc::pthread_cancel( abort.as_pthread_t() ); 
    //if strn.is_none () { (*fd0.as_mut().unwrap()).flush (); return }
    //if strn.is_none () { fd0.as_ref().unwrap().write("".strn().as_bytes() ); return }
    if strn.is_none () { file.write("".strn().as_bytes() ); return }
    //fd0.as_ref().unwrap().write(strn.unwrap().strn().as_bytes() );
    file.write(strn.unwrap().strn().as_bytes() );
    std::mem::forget ( file );
  //  std::mem::drop (writeIn_stdin);
    crate::smart_lags::forcely_set_mamed_mutexes (&fn_name, named_mutex::unset, &mut mutex ); //dbg!("end");
    }
}
////////////////////// stable variant ///////////////////////////

//////////////////// abuses stdin ///////////////////////////////
pub fn stdin_write <T: ToString + STRN > (strn: Option < T > ) {
    static mut fd0: Lazy < Option < Box <std::fs::File > > > = Lazy::new (|| { None });
    static mut fst_run: bool = true;
    unsafe {
        if fst_run {
            let mut file = Box::new (std::fs::File::from_raw_fd(0/*stdin*/));
            let mut undead = ManuallyDrop::new (file );
            *fd0 = Some( ManuallyDrop::into_inner (undead) );
          }
        let fn_name = "stdin_write".strn();
    let mut mutex_state = false;
    let mut mutex: crate::enums::custom_mutex = crate::enums::custom_mutex::new( &fn_name );
    if let Some ( x ) = crate::smart_lags::mamed_mutexes (&fn_name, named_mutex::set, &mut mutex ) {mutex_state = x}
    //else { crate::smart_lags::mamed_mutexes (&fn_name, named_mutex::set, &mut mutex ); mutex_state = true;}
    //if crate::smart_lags::mamed_mutexes (&fn_name, named_mutex::get ).is_none() {mutex = false }
    let mut cond = unsafe { ( *mutex.owner == u64::MAX || *mutex.owner == mutex.id ) };
    while cond == false {
        cond = unsafe { ( *mutex.owner == u64::MAX || *mutex.owner == mutex.id ) };
        if let Some ( x ) = crate::smart_lags::mamed_mutexes (&fn_name, named_mutex::set, &mut mutex ) {mutex_state = x}
        unsafe { dbg! (*mutex.owner ); dbg! (mutex.id ); }
    }
    //libc::pthread_cancel( abort.as_pthread_t() ); 
    //if strn.is_none () { (*fd0.as_mut().unwrap()).flush (); return }
    if strn.is_none () { fd0.as_mut().unwrap().flush (); return }
   // if strn.is_none () { file.write("".strn().as_bytes() ); return }
    fd0.as_mut().unwrap().write(strn.unwrap().strn().as_bytes() );
    //file.write(strn.unwrap().strn().as_bytes() );
  //  std::mem::drop (writeIn_stdin);
    crate::faav::fin_prox_loop( Some (true) );
    crate::smart_lags::forcely_set_mamed_mutexes (&fn_name, named_mutex::unset, &mut mutex ); //dbg!("end");
    }
}
//////////////////// abuses stdin ///////////////////////////////
*/