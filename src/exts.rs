#![feature(trace_macros)]
macro_rules! use_all {
    () => {
//trace_macros!(true);
use std::env;
use colored::Colorize;
use substring::Substring;
use std::str::{self, from_utf8};
use std::string;
use chrono::{DateTime, Local};
use std::io::{self, Write};
use std::any::{self, type_name};
use std::fmt;
use std::fs;
use std::fs::File;
use cli_table::format;
use once_cell::sync::OnceCell;
use std::cell::Cell;
use std::str::Split;
use std::thread;
use threadpool::{ThreadPool, Builder};
use std::sync::mpsc;
use std::ffi::CString;
use std::process::{Command, Stdio};
use std::path::Path;
use num_traits::cast::ToPrimitive;
use std::io::{BufRead, BufReader};
use std::os::unix::io::{AsRawFd, RawFd};
#[path = "mae.rs"]
#[cfg(feature="mae")] mod mae;
#[cfg(feature="mae")] use mae::*;
#[path = "keycodes.rs"]
mod kcode01;
#[path = "switch.rs"]
mod swtch;
#[path = "init.rs"]
mod init;
#[path = "nvim.ext.rs"]
mod nvim;
#[path = "vim.ext.rs"]
mod vim;
#[path = "func_id.rs"]
mod func_id18;
use func_id18::*;
#[path = "info.rs"]
mod info;
use info::*;
#[path = "custom.traits.rs"]
mod custom_traits;
use custom_traits::*;
#[path = "cmd_keys.rs"]
mod cmd_keys;
use cmd_keys::*;
#[path = "no.flood.rs"]
mod smart_lags;
#[path = "key_handlers.rs"]
mod key_handlers;
use key_handlers::*;
#[path = "types.rs"]
mod types;
use types::*;
#[path = "lst.rs"]
mod lst;
use lst::*;
#[cfg(feature="in_dbg")] #[path = "in_dbg0.rs"]
mod in_dbg0;
#[cfg(feature="in_dbg")]
use in_dbg0::*;
#[path = "enums.rs"]
mod enums;
use enums::*;
#[path = "basic.pg.rs"]
mod basic_pg;
use basic_pg::*;
#[path = "cache.rs"]
mod cache;
use cache::*;
#[path = "parse_replace.rs"]
mod parse_replacing;
use parse_replacing::*;
#[path = "ext_msgs.rs"]
mod __ext_msgs;
use __ext_msgs::*;
#[path = "env.rs"]
mod tam_env;
use tam_env::*;
#[path = "globs.rs"]
mod globs18;
#[path = "term_app.rs"]
mod term_app;
use term_app::*;
#[path = "extctrl.rs"]
mod extctrl;
use extctrl::*;
#[path = "page_struct_.rs"]
mod ps18;
use ps18::*;
#[path = "pg.rs"]
mod pg18;
use pg18::*;
#[path = "update.rs"]
mod update18;
#[macro_use]
#[path = "mcrs.rs"]
mod mcrs18;
use mcrs18::*;
#[path = "subs.rs"]
mod subs;
use subs::*;
use dirty;
use getStop_code__;
use close_termios__;
use set_prnt_;
use C;
use C_;
#[path = "core.rs"]
mod core18;
use core18::*;
    }
}
macro_rules! pg_uses {
    () => {
const id: u64 = 1407;
use std::env;
use colored::Colorize;
use substring::Substring;
use std::str::{self, from_utf8};
use std::string;
use chrono::{DateTime, Local};
use std::io::{self, Write};
use std::any::{self, type_name};
use std::fmt;
use std::fs;
use std::fs::File;
use std::os::unix::io::FromRawFd;
use std::io::Read;
use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};
use cli_table::format;
use cli_table::{format::Justify, Cell, Style, Table};
use cli_table::print_stdout;
use std::str::Split;
use std::thread;
use threadpool::{ThreadPool, Builder};
use std::sync::mpsc;
use std::ffi::CString;
use std::process::{Command, Stdio};
use std::path::Path;
use num_traits::cast::ToPrimitive;
use std::io::{BufRead, BufReader};
use std::os::fd::AsFd;
use ansi_term::ANSIString;
//#[path = "func_id.rs"]
//mod func_id0;
//#[path = "globs.rs"]
//mod globs0;
//use crate::globs::*;  
//use crate::ps::*;
#[macro_use]
#[path = "mcrs.rs"]
mod mcrs0;
use getStop_code__;
use dirty;
use close_termios__;
#[path = "keycodes.rs"]
mod kcode;
use cli_table::CellStruct;
use once_cell::sync::OnceCell;   
    };
}
macro_rules! page_struct_uses {
    () => {
use std::env;
use std::cell::UnsafeCell;
use colored::Colorize;
use substring::Substring;
use std::str::{self, from_utf8};
use std::string;
use chrono::{DateTime, Local};
use crate::{pg18::repeat_char, swtch::share_usize};
use std::io::{self, Write};
use std::any::{self, type_name};
use std::fmt;
use std::fs;
use std::fs::File;
use cli_table::format;
use once_cell::sync::OnceCell;
use std::cell::Cell;
use std::str::Split;
use std::thread;
use threadpool::{ThreadPool, Builder};
use std::sync::mpsc;
use std::ffi::CString;
use std::process::{Command, Stdio};
use std::path::Path;
use num_traits::cast::ToPrimitive;
use std::io::{BufRead, BufReader};
use std::sync::RwLock;
#[macro_use]
#[path = "mcrs.rs"]
mod mcrs909;
use set_prnt_;
    };
}
macro_rules! globs_uses {
    () => {
    //use rustix::path::arg::Arg;
    use std::path::Path;
    use std::ffi::OsString;
    use substring::Substring;
    use std::sync::{RwLock, Arc};
    use once_cell::unsync::Lazy;
    use ansi_term::ANSIString;
    use once_cell::sync::OnceCell;
    use libc::{fcntl, F_GETFL, F_SETFL, O_NONBLOCK};
    use std::io::{self, ErrorKind, Read};
    use std::os::unix::io::{AsRawFd, RawFd};
    #[macro_use]
    #[path = "mcrs.rs"]
    mod mcrs13;
   use C;
   //use C_;
    }
}
macro_rules! mcrs_uses {
    () => {
//#[path = "page_struct_.rs"]


    };
}
macro_rules! update_uses {
    () => {
use std::time::Duration;
use std::env;
use colored::Colorize;
use substring::Substring;
use std::str::{self, from_utf8};
use std::string;
use std::ffi::OsString;
use std::ffi::OsStr;
use chrono::{DateTime, Local};
use std::io::{self, Write};
use std::any::{self, type_name};
use std::fmt;
use std::fs;
use std::fs::File;
use cli_table::format;
use once_cell::unsync::OnceCell;
use std::cell::Cell;
use std::str::Split;
use std::thread;
use threadpool::{ThreadPool, Builder};
use std::sync::mpsc;
use std::ffi::CString;
use std::process::{Command, Stdio};
use std::path::Path;
use num_traits::cast::ToPrimitive;
use std::io::{BufRead, BufReader};
use std::thread::spawn;
use std::{i64, usize};
#[path = "func_id.rs"]
mod func_id17;
#[path = "globs.rs"]
mod globs17;
#[path = "page_struct_.rs"]
mod ps0;  
#[path = "pg.rs"]
mod pg17;
use crate::get_arg_in_cmd;
#[macro_use]
 #[path = "mcrs.rs"]
mod mcrs1311;
use C;
use C_;
}; }
macro_rules! core_use {
    () => {
use std::env;
use std::time::Duration;
use std::mem::size_of;
use std::fs::OpenOptions;
use std::io;
use std::{i64, usize};
use std::io::{Read, Write, Seek, SeekFrom};
//use gag::Redirect;
use colored::Colorize;
use substring::Substring;
use std::str::{self, from_utf8};
use std::string;
use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};
use std::io::{BufRead, BufReader};
use std::os::unix::io::{AsRawFd, RawFd};
use chrono::{DateTime, Local};
//use std::io::{self, Write};
use std::any::{self, type_name};
use std::fmt;
use std::fs;
use std::fs::File;
use cli_table::format;
use once_cell::unsync::OnceCell;
use std::cell::Cell;
use std::str::Split;
use std::thread;
use threadpool::{ThreadPool, Builder};
use std::sync::mpsc;
use std::ffi::CString;
use std::process::{Command, Stdio};
use std::path::Path;
use num_traits::cast::ToPrimitive;
use std::os::fd::FromRawFd;
use std::thread::spawn;
#[path = "page_struct_.rs"]
mod ps21;
#[path = "func_id.rs"]
mod func_id21;
use func_id21::*;
#[path = "globs.rs"]
mod globs21;
use globs21::*;
#[path = "pg.rs"]
mod pg21;
use pg21::*;
};
}
pub(crate) use core_use;
pub(crate) use update_uses;
pub(crate) use mcrs_uses;
pub(crate) use use_all;
pub(crate) use page_struct_uses;
pub(crate) use pg_uses;
pub(crate) use  globs_uses;
