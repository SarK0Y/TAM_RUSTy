use once_cell::sync::Lazy;
use nix::sys::wait::wait;
use nix::unistd::{fork, ForkResult, getpid, getppid, execve};
use std::env;
use std::{ ffi::CString, env::var, num::NonZero };
use crate::globs18::take_list_adr;
use crate::update18::delay_secs;
use procfs::process::all_processes;
use crate::{errMsg0, getkey, helpful_math_ops, save_file_append_newline_abs_adr_fast, split_once, STRN};
use std::ptr;
//#[derive(Copy, Clone)]
pub struct tree_of_prox {
    ppid: i32 ,
    //ppid: nix::unistd::Pid,
    up: *mut tree_of_prox,
    kids: *mut Vec<  *mut tree_of_prox >,
    proxid_of_kid: Vec < i32 >,
    cursor: usize,
}
pub trait prox  {
    fn new (&mut self, pid: i32 ) -> Box < *mut tree_of_prox >;
    fn init ( &mut self ) -> Box < *mut tree_of_prox >;
    fn new_branch ( &mut self ) -> Box < *mut tree_of_prox >;
    fn dup ( &mut self ) -> Box < *mut tree_of_prox >;
}
impl prox for tree_of_prox  {
    fn new (&mut self, pid: i32) -> Box < *mut tree_of_prox  > {
         *self = *mk_root_of_prox( pid);
         Box::new ( self)
    }
    fn init ( &mut self ) -> Box < *mut tree_of_prox > {
        Box::new ( self )
    }
    fn new_branch ( &mut self ) -> Box < *mut tree_of_prox > {
        unsafe {
            let hacky_pointer: *mut tree_of_prox = self as *mut tree_of_prox; 
            /*let mut branch*/ *self = *mk_branch_of_prox( &mut *hacky_pointer ); 
            if self.proxid_of_kid.len() > 0 { return Box::new ( self );}
            while self.proxid_of_kid.len() == 0 ||
                (self.proxid_of_kid.len() == self.cursor && !self.up.is_null())
                {
                 //   *self = *(*hacky_pointer.clone()).up.clone ();
                }  Box::new (self)
        }
    }
    fn dup ( &mut self ) -> Box < *mut tree_of_prox > {
        unsafe {
            let up =  if self.up.is_null() { 
                    ptr::null_mut() 
                } else { 
                    Box::into_raw(Box::new((*self.up).dup())) 
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
                        let rp: *mut = &mut (*self.kids).iter().map(|&kid| {
                           (*kid).dup()
                        }).collect::<Vec<_>>(); rp
                    },
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
pub fn mk_tree_of_prox ( tree: Box < *mut  tree_of_prox > ) -> *mut tree_of_prox {
    unsafe {
         (*tree).as_mut().unwrap().new( getpid().as_raw() );
        *tree
    }
}
pub fn mk_branch_of_prox ( tree: *mut  tree_of_prox ) -> Box <  tree_of_prox > {
    unsafe {
        let mut branch = Box::new(  tree_of_prox  {
            ppid: (*tree).proxid_of_kid [ (*tree).cursor ],
            up: tree,
            kids: Vec::< *mut tree_of_prox >::new(),
            proxid_of_kid: Vec::< i32 >::new(),
            cursor: 0
        } );
        for proc in all_processes().unwrap() {
            if let Ok (res) = proc.unwrap().status() {
                if res.ppid == branch.ppid {
                    branch.proxid_of_kid.push (res.pid );
                }
            }
        }
    branch
   }
}
pub fn mk_root_of_prox (pid: i32) -> Box < tree_of_prox  > {
     Box::new(  tree_of_prox  {
        ppid: pid,
        up: ptr::null_mut (),
        kids: Vec::< *mut tree_of_prox >::new(),
        proxid_of_kid: Vec::< i32 >::new(),
        cursor: 0
    } )
}
pub fn init_root_of_prox ( tree: *mut  tree_of_prox ) -> bool {
    unsafe {
        for proc in all_processes().unwrap() {
            if let Ok (res) = proc.unwrap().status() {
                if res.ppid == (*tree).ppid {
                    (*tree).proxid_of_kid.push (res.pid );
                }
            }
        }
        if (*tree).proxid_of_kid.len() > 0 { return true } false
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
