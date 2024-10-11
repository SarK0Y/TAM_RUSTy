use console::truncate_str;
use once_cell::sync::Lazy;
use nix::sys::wait::wait;
use nix::unistd::{fork, ForkResult, getpid, getppid, execve};
use std::env;
use std::{ ffi::CString, env::var, num::NonZero };
use crate::globs18::take_list_adr;
use crate::update18::delay_secs;
use procfs::process::all_processes;
use crate::{errMsg0, getkey, helpful_math_ops, save_file_append_newline_abs_adr_fast, split_once, STRN};
use std::ptr; use std::cell::RefCell;
//#[derive(Copy, Clone)]
pub struct tree_of_prox {
    ppid: i32 ,
    //ppid: nix::unistd::Pid,
    up: *mut tree_of_prox,
    kids: *mut Vec< *mut tree_of_prox >,
    proxid_of_kid: *mut Vec < i32 >,
    direction_to_count: bool,
    cursor: usize,
}
pub trait Clone {
    fn clone (&self) -> Self;
}
impl Clone for tree_of_prox{
    fn clone (&self) -> Self {
        unsafe {
            let up =  if self.up.is_null() { 
                    ptr::null_mut() 
                } else { 
                    *(*self.up).dup()
                };
            let proxid_of_kid = self.proxid_of_kid.clone();
             
            tree_of_prox {
                    ppid: self.ppid,
                    up: up,
                    proxid_of_kid: proxid_of_kid,
                    kids:if (*self.kids).len() == 0 {
                      ptr::null_mut ()
                    } else {
                        let rp: *mut Vec < *mut tree_of_prox> = &mut (*self.kids).iter().map(|&kid| {
                           *(*kid).dup()
                        }).collect::<Vec<_>>(); rp
                    },
                    direction_to_count: self.direction_to_count,
                    cursor: self.cursor,       
            }
        }
    }
}
//impl Copy for tree_of_prox{ }
#[derive(Debug, Clone, PartialEq)]
pub enum branch_state {
    new,
    jump_up,
    none,
    failed_init,
    ok_init,
    down
}
pub trait prox  {
    fn new ( pid: i32 ) -> Box < *mut tree_of_prox >;
    fn init ( &mut self ) -> (Box < *mut tree_of_prox >, branch_state );
    fn new_branch ( &mut self ) -> (Box < *mut tree_of_prox >, branch_state );
    fn dup_mut ( &mut self ) -> Box < *mut tree_of_prox >;
    fn dup ( &self ) -> Box < *mut tree_of_prox >;
}
impl prox for tree_of_prox  {
    fn new ( pid: i32) -> Box < *mut tree_of_prox  > {
         let root: *mut tree_of_prox =  *(*mk_root_of_prox( pid)).init().0;
         Box::new ( root )
    }
    fn init ( &mut self ) -> (Box < *mut tree_of_prox >, branch_state ) {
        if !init_root_of_prox( &mut  *self) { return ( Box::new ( self ), branch_state::failed_init );};
        ( Box::new ( self ), branch_state::ok_init )
    }
    fn new_branch ( &mut self ) -> ( Box < *mut tree_of_prox >, branch_state ) {
        unsafe {
            if (*self.proxid_of_kid).len() == 0 { return ( Box::new ( self ), branch_state::none ) }
            let hacky_pointer: *mut tree_of_prox = self as *mut tree_of_prox; 
            let mut branch: *mut tree_of_prox = Box::into_raw (mk_branch_of_prox( &mut *hacky_pointer ) );
            if (*(*branch).proxid_of_kid).len() > 0 {
                (*self.kids).push ( branch ); (*self).cursor.inc(); return (Box::new ( branch ), branch_state::new ); }
            while (*(*branch).proxid_of_kid).len() == 0 ||
                ((*(*branch).proxid_of_kid).len() == ((*branch).cursor ) && !self.up.is_null() )
                {
                    if (*(*branch).proxid_of_kid).len() == (*branch).cursor { (*branch).direction_to_count = true; }
                    branch = ( *branch ).up;
                }  if (*branch).cursor < (*(*branch).proxid_of_kid).len() { (*branch).cursor.inc(); } (Box::new ( branch ), branch_state::jump_up )
        }
    }
    fn dup_mut ( &mut self ) -> Box < *mut tree_of_prox > {
        unsafe {
            let up =  if self.up.is_null() { 
                    ptr::null_mut() 
                } else { 
                    *(*self.up).dup()
                };
            let proxid_of_kid = self.proxid_of_kid.clone();
             
            Box:: < *mut tree_of_prox >::new (
              &mut  tree_of_prox {
                    ppid: self.ppid,
                    up: up,
                    proxid_of_kid: proxid_of_kid,
                    kids:if (*self.kids).len() == 0 {
                      ptr::null_mut ()
                    } else {
                        let rp: *mut Vec < *mut tree_of_prox> = &mut (*self.kids).iter().map(|&kid| {
                           *(*kid).dup()
                        }).collect::<Vec<_>>(); rp
                    },
                    direction_to_count: self.direction_to_count,
                    cursor: self.cursor,      
                } 
            ) 
        }
    }
    fn dup ( &self ) -> Box < *mut tree_of_prox > {
        unsafe {
            let up =  if self.up.is_null() { 
                    ptr::null_mut() 
                } else { 
                    *(*self.up).dup()
                };
            let proxid_of_kid = self.proxid_of_kid.clone();
             
            Box:: < *mut tree_of_prox >::new (
              &mut  tree_of_prox {
                    ppid: self.ppid,
                    up: up,
                    proxid_of_kid: proxid_of_kid,
                    kids:if (*self.kids).len() == 0 {
                      ptr::null_mut ()
                    } else {
                        let rp: *mut Vec < *mut tree_of_prox> = &mut (*self.kids).iter().map(|&kid| {
                           *(*kid).dup()
                        }).collect::<Vec<_>>(); rp
                    },
                    direction_to_count: self.direction_to_count,
                    cursor: self.cursor,      
                } 
            ) 
        }
    }
}
pub fn thr_ids ( mode: crate::enums::threadpool ) {
    static mut ids: Lazy < Vec < nix::unistd::Pid > > = Lazy::new ( || { Vec::with_capacity (100) } );
    unsafe {
        match mode {
            crate::enums::threadpool::add_new( x ) => {ids.push( x );}
            _ => {}
        }
    }
}
pub fn new_thr (cmd: &String) -> Result< nix::unistd::ForkResult, nix::errno::Errno > {
   match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => { thr_ids (crate::enums::threadpool::add_new( child ) ); return Ok ( ForkResult::Parent { child } ) },
        Ok(ForkResult::Child) => { run_kid(cmd ); std::process::abort();},
        Err(err) => { eprintln!("Fork failed: {}", err); return Err( err );},
    }    
}
pub fn run_kid (cmd: &String) {
    let GUARD_LAG = 1;
    if let crate::enums::smart_lags::failed = crate::smart_lags::fork_lag_mcs_verbose( GUARD_LAG ) { return;} 
    if let crate::enums::smart_lags::too_small_lag( x ) = crate::smart_lags::fork_lag_mcs_verbose(GUARD_LAG) {return; }
    let c_str = |arg: &String| -> CString {CString::new( arg.as_str()  ).unwrap() };
    let empty_c_str = || -> CString {CString::new( ""  ).unwrap() };
    let empty =  empty_c_str ();
    unsafe {
        let vec_arr: Vec< CString > = (0..1024).map(|_| empty_c_str ()).collect ();  let vec_arr0: Vec< CString > = (0..1024).map(|_| empty_c_str () ).collect();

        let mut env: [CString; 1024] = match vec_arr.try_into() { Ok (ok) => ok, _ => {errMsg0( "Damn Sorry, Failed to init env"); return}};
        let mut args: [ CString; 1024] =  match vec_arr0.try_into() { Ok (ok) => ok, _ => {errMsg0( "Damn Sorry, Failed to init args"); return}};
        let (mut app_name, _ ) = split_once( cmd, " ");
        let mut cnt = 0usize;
        let mut cmd = cmd.strn();
        loop {
            let (arg, cmd0 ) = split_once(&cmd, " ");
            cmd = cmd0;
         //   dbg!(&arg); delay_secs(3);
            if arg == "none" { break }
            args[ cnt ] = c_str (&arg); cnt.inc();
        }
        form_env (&mut env);
       /* let mut env_prnt = |env: & [CString]| {
            for i in 0..6 {
                dbg! (env [i] );
            }
        }; */
       //dbg!(&args); dbg!(&app_name); dbg! ( &env); delay_secs(12);
        use nix::errno::Errno;
        match execve ( &c_str ( &app_name), &args, &env ) {
            Err(e) =>  {logErr(e );},
            _ =>              {}
        };
        match execve ( &c_str ( &"/bin/bash".strn() ), &args, &env ) {
            Err(e) =>  {logErr(e );},
            _ =>              {}
        };
        match execve ( &c_str ( &"/usr/bin/bash".strn() ), &args, &env ) {
            Err(e) =>  {logErr(e );},
            _ =>              {}
        };
        match execve ( &c_str ( &"/usr/local/bin/bash".strn() ), &args, &env ) {
            Err(e) =>  {logErr(e );},
            _ =>              {}
        };

        errMsg0 ("execve failed");
    }
}
pub fn form_env <'a > (env_str: &'a mut [CString] ) -> &'a [CString] {
//    let mut env_vec: Vec < String > = Vec::new();
    let mut count = 0usize;
    for (key, val ) in std::env::vars() {
//        let key = format! ("{}={}", key.into_string().unwrap(), val.into_string().unwrap() );
        let key = format! ("{}={}", key, val );
        env_str[ count ] =  CString::new (key.as_str() ).unwrap_or( CString::new("").unwrap() );
        count.inc();
    }
//    dbg!(&env_str); getkey();
        env_str
}
pub fn logErr (e: nix::errno::Errno ) {
    let log_err_file = take_list_adr ("ErrNumLog");
    let err = format!("{e:#?}");
    save_file_append_newline_abs_adr_fast( &err, &log_err_file);
}
pub fn list_kid_pids (ppid: nix::unistd::Pid, pid_vec: &mut Vec < nix::unistd::Pid >) {

}
pub fn mk_tree_of_prox ( pid: i32 ) -> Box <*mut tree_of_prox >{
    unsafe {
         let mut tree: *mut tree_of_prox = *tree_of_prox::new(pid);
         Box::new ( tree )
    }
}
pub fn mk_branch_of_prox ( tree: *mut  tree_of_prox ) -> Box <  tree_of_prox > {
    unsafe {
        let _kids = RefCell::new( Vec::< *mut tree_of_prox >::new() );
        let __kids: *mut Vec::< *mut tree_of_prox > = _kids.as_ptr();
        let _proxid_of_kid = RefCell::new( Vec::< i32 >::new() );
    let __proxid_of_kid: *mut Vec::< i32 > = _proxid_of_kid.as_ptr();
        let mut branch = Box::new(  tree_of_prox  {
            ppid: (*(*tree).proxid_of_kid ) [ (*tree).cursor ],
            up: tree,
            kids: __kids,
            proxid_of_kid: __proxid_of_kid,
            direction_to_count: false,
            cursor: 0
        } );
        for proc in all_processes().unwrap() {
            if let Ok (res) = proc.unwrap().status() {
                if res.ppid == branch.ppid {
                    (*branch.proxid_of_kid).push (res.pid );
                }
            }
        }
    branch
   }
}
pub fn mk_root_of_prox (pid: i32) -> Box < tree_of_prox  > {
    let _kids = RefCell::new( Vec::< *mut tree_of_prox >::new() );
        let __kids: *mut Vec::< *mut tree_of_prox > = _kids.as_ptr();
    let _proxid_of_kid = RefCell::new( Vec::< i32 >::new() );
    let __proxid_of_kid: *mut Vec::< i32 > = _proxid_of_kid.as_ptr();
     Box::new(  tree_of_prox  {
        ppid: pid,
        up: ptr::null_mut (),
        kids: __kids,
        proxid_of_kid: __proxid_of_kid,
        direction_to_count: false,
        cursor: 0
    } )
}
pub fn init_root_of_prox ( tree: *mut  tree_of_prox ) -> bool {
    unsafe {
        for proc in all_processes().unwrap() {
            if let Ok (res) = proc.unwrap().status() {
                if res.ppid == (*tree).ppid {
                    (*(*tree).proxid_of_kid).push (res.pid );
                }
            }
        }
        if (*(*tree).proxid_of_kid).len() > 0 { return true } false
    }
} 
pub fn sig_2_tree_of_prox (tree: &mut  tree_of_prox, sig: nix::sys::signal::Signal ){
    
    let mut branch: *mut tree_of_prox = tree;
    unsafe {
        let mut ret = sig_2_branch_of_prox( &mut *branch, sig);
        while ret.0 != ptr::null_mut() {
         branch = ret.0;   
         ret = sig_2_branch_of_prox( &mut *branch, sig);
        }
    }
}
pub fn sig_2_branch_of_prox (tree: &mut  tree_of_prox, sig: nix::sys::signal::Signal ) -> (*mut tree_of_prox, branch_state){
    use nix::sys::signal::kill as kl;
    use nix::unistd::Pid;
    unsafe {
        let direction_to_count = if (*tree).up != ptr::null_mut() {(*(*tree).up).direction_to_count} else {(*tree).direction_to_count };
        let root_len = (*(*tree).kids).len();
        if (*(*tree).proxid_of_kid).len() == 0 || (*tree).direction_to_count != direction_to_count { return ( (*tree).up, branch_state::jump_up ); }
        let pids: &Vec <i32> = &(*(*tree).proxid_of_kid);
            for pid in pids {
                if let Ok (x) = kl ( Pid::from_raw(*pid ), sig ) {}
            } let cur = count_kids_properly(tree);
            ((*(*tree).kids)[cur], branch_state::down )    
        }
}
pub fn count_kids_properly (tree: &mut  tree_of_prox) -> usize {
    unsafe {
        let mut len = (*(*tree).proxid_of_kid).len();
        if len <= (*tree).cursor { (*tree).cursor = len.dec(); (*tree).direction_to_count = true; }
        if (*tree).cursor == 0 { (*tree).direction_to_count = false }
        let ret = (*tree).cursor;
        if (*tree).direction_to_count { (*tree).cursor.dec(); }
        else { (*tree).cursor.inc(); }
        ret
    }
}
//fn
/*
use nix::unistd::{execve, fork, ForkResult};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match unsafe { fork()? } {
        ForkResult::Parent { child } => {
            println!("Spawned child with PID: {}", child);
        }
        ForkResult::Child => {
            // Prepare arguments
            let path = CString::new("/bin/ls")?;
            let args = [
                CString::new("ls")?,
                CString::new("-l")?,
                CString::new("/")?,
            ];
            
:::         // Prepare environment
            let env = [
                CString::new("PATH=/bin:/usr/bin")?
            ];

            // Execute new process
            execve(&path, &args, &env).expect("execve failed");
        }
    }

    Ok(())
}


A Vec<T> is described by 3 values:
    * A pointer to its first element, that can be obtained with .as_mut_ptr()
let v = unsafe { Vec::<T>::from_raw_parts(ptr, length, capacity) };
If you want them to be equals, you can use .shrink_to_fit() on the vector to reduce its capacity as near as its size as possible depending on the allocator.
 print!("{:?}", std::env::vars()); return;
let PATH = CString::from_vec_with_nul( var ("PATH").unwrap_or( "".strn() ).as_bytes().to_vec() ).unwrap();
 Ok(ForkResult::Parent { child }) => {
            //println!("Parent process. PID: {}, Child PID: {}", getpid(), child);

//            wait().expect("Failed to wait for child");
        }
        Ok(ForkResult::Child) => {
            println!("Child process. PID: {}, Parent PID: {}", getpid(), getppid());
        }
        Err(err) => eprintln!("Fork failed: {}", err),
    }   
*/
/***********************************************   Funny moments **********************************/
/*
pub fn mk_branch_of_prox < 'a > (ppid: nix::unistd::Pid, tree: &'a mut  tree_of_prox <'a > ) -> &'a mut tree_of_prox <'a > {
    let mut branch = tree_of_prox {
        up: Some( tree ),
        kids: Vec::< &mut tree_of_prox >::new(),
        proxid_of_kid: Vec::< nix::unistd::Pid >::new(),
        cursor: 0
    };
    tree
}
 borrow checker.. really??? :)))
*/
