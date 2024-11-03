use num_traits::Bounded; use core::mem::size_of;
use std::char;
use std::io::Read;
#[cfg(feature="tam")] 
use crate::{globs18::strn_2_usize, usize_2_i64, core18::i64_2_usize, errMsg0};
#[cfg(feature="tam")] use crate::run_cmd_out_sync;
pub(crate) trait STRN {
    fn strn(&self) -> String;
}
pub(crate) trait STRN_usize {
    fn usize0(&self) -> usize;
}
#[cfg(feature="tam")] 
impl STRN_usize for String{
    fn usize0(&self) -> usize {
        strn_2_usize(self.strn() ).unwrap_or(0)
    }
}
pub(crate) trait turn_2_i64 {
    fn i640(&self) -> i64;
}
#[cfg(feature="tam")] 
impl turn_2_i64 for usize{
    fn i640(&self) -> i64 {
        usize_2_i64(*self )
    }
}
#[cfg(feature="tam")] 
impl turn_2_i64 for &str{
    fn i640(&self) -> i64 {
        match i64::from_str_radix(self, 10){ 
            Ok(i) => i,
            _ => {errMsg0("i640: no number was gotten."); i64::MIN }
        }
    }
}
#[cfg(feature="tam")] 
impl turn_2_i64 for String{
    fn i640(&self) -> i64 {
        match i64::from_str_radix(self, 10){ 
            Ok(i) => i,
            _ => {errMsg0("i640: no number was gotten."); i64::MIN }
        }
    }
}
#[cfg(feature="tam")] 
impl turn_2_i64 for &String{
    fn i640(&self) -> i64 {
        match i64::from_str_radix(self, 10){ 
            Ok(i) => i,
            _ => {errMsg0("i640: no number was gotten."); i64::MIN }
        }
    }
}
pub(crate) trait turn_2_usize {
    fn usize0(&self) -> usize;
}
#[cfg(feature="tam")] 
impl turn_2_usize for i64{
    fn usize0(&self) -> usize {
        i64_2_usize(*self )
    }
}
pub(crate) trait STRN_strip {
    fn del_ch(&self, ch: &str) -> String;
    fn strip_symbs(&mut self) -> String;
    fn strip_apostrophe(&mut self) -> String;
    fn strip_backslash(&mut self) -> String;
    fn strip_all_symbs(&mut self) -> String;
  //  fn _strip_all_symbs(&self) -> String;
}
impl STRN_strip for str {
    fn del_ch(&self, del_ch: &str) -> String{
        let mut ret = "".strn();
        let del_ch = del_ch.chars().nth(0);
        for ch in self.chars(){
            if let Some(ch) = del_ch{continue;}
            ret.push(ch);
        } ret
    }
fn strip_symbs(&mut self) -> String{
    let  strr: String = self.to_string();
    let strr = strr.replace(r"\-", "-");
    let strr = strr.replace(r"\ ", " ");
    let strr = strr.replace(r"\$", "$");
    let strr = strr.replace(r"\`", "`");
    let strr = strr.replace(r"\(", "(");
    let strr = strr.replace(r"\}", "}");
    let strr = strr.replace(r"\{", "{");
    let strr = strr.replace(r"\)", ")");
    let strr = strr.replace(r"\]", "]");
    let strr = strr.replace(r"\[", "[");
    let strr = strr.replace(r"\&", "&");
    //*self.replace(r"\'", "'");
    return strr
}
fn strip_apostrophe(&mut self) -> String{
    self.replace(r"\'", r"'")
}
fn strip_backslash(&mut self) -> String{
    self.replace(r"\\", "\\")
}
fn strip_all_symbs(&mut self) -> String{
    self.strip_backslash();
    self.strip_apostrophe();
    self.strip_symbs()
}
/*fn _strip_all_symbs(&self) -> String{
    self.strip_backslash();
    self.strip_apostrophe();
    self.strip_symbs()
}*/
}
impl STRN for str {
    fn strn(&self) -> String{
        String::from(self)
    }
}
impl STRN for &str {
    fn strn(&self) -> String{
        String::from(*self) 
    }
}
impl STRN for &String {
    fn strn(&self) -> String{
        self.to_string()
    }
}
impl STRN for String {
    fn strn(&self) -> String{
        self.to_string()
    }
}
impl STRN for usize {
    fn strn(&self) -> String{
        self.to_string()
    }
}
impl STRN for u64 {
    fn strn(&self) -> String{
        self.to_string()
    }
}
impl STRN for i64 {
    fn strn(&self) -> String{
        self.to_string()
    }
}
pub(crate) trait helpful_math_ops
{
    fn inc(&mut self) -> Self;
    fn dec(&mut self) -> Self;
}
pub(crate) trait arrays {
    fn low_to_high(&mut self) -> &mut Self;
}
impl arrays for [u8] {
    fn low_to_high(&mut self) -> &mut Self {
        for i in 0..self.len() / 2{
            let low = self[i]; let high = self[self.len() - i - 1];
            self[self.len() - i - 1] = low;
            self[i] = high;
        } self
    }
}
pub(crate) trait arr2number {
    fn arr2u64(&mut self) -> u64;
    fn arr2u128(&mut self) -> u128;
}
impl arr2number for [u8]{
    fn arr2u64(&mut self) -> u64 {
        let mut ret = 0u64;
        let len = if size_of::<u64>() > self.len(){self.len()}else{size_of::<u64>()};
        for i in 0..len{
            ret = ret.overflowing_add((self[i] as u64).overflowing_shl((i as u32)*8).0 as u64).0;
        }
        ret
    }
     fn arr2u128(&mut self) -> u128 {
        let mut ret = 0u128;
        let len = if size_of::<u128>() > self.len(){self.len()}else{size_of::<u128>()};
        for i in 0..len{
            let a = self[i];
            ret = ret.overflowing_add((self[i] as u128).overflowing_shl((i as u32)*8).0 as u128).0;
        }
        ret
    }
}
impl arr2number for [u16]{
    fn arr2u64(&mut self) -> u64 {
        let mut ret = 0u64;
        let len = if size_of::<u64>() > self.len(){self.len()}else{size_of::<u64>()};
        for i in 0..len{
            ret = ret.overflowing_add((self[i] as u64).overflowing_shl((i as u32)*16).0 as u64).0;
        }
        ret
    }
    fn arr2u128(&mut self) -> u128 {
        let mut ret = 0u128;
        let len = if size_of::<u128>() > self.len(){self.len()}else{size_of::<u128>()};
        for i in 0..len{
            ret = ret.overflowing_add((self[i] as u128).overflowing_shl((i as u32)*16).0 as u128).0;
        }
        ret
    }
}
impl helpful_math_ops for usize {
    fn inc(&mut self) -> Self{
       if *self < Self::MAX{*self = *self + 1; return *self;}
       *self
    }
    fn dec(&mut self) -> Self{
       if *self > Self::MIN{*self = *self - 1; return *self;}
       *self
    }
}
impl helpful_math_ops for u64 {
    fn inc(&mut self) -> Self{
       if *self < Self::MAX{*self = *self + 1; return *self;}
       *self
    }
    fn dec(&mut self) -> Self{
       if *self > Self::MIN{*self = *self - 1; return *self;}
       *self
    }
}
impl helpful_math_ops for u128 {
    fn inc(&mut self) -> Self{
       if *self < Self::MAX{*self = *self + 1; return *self;}
       *self
    }
    fn dec(&mut self) -> Self{
       if *self > Self::MIN{*self = *self - 1; return *self;}
       *self
    }
}

impl helpful_math_ops for u32 {
    fn inc(&mut self) -> Self{
       if *self < Self::MAX{*self = *self + 1; return *self;}
       *self
    }
    fn dec(&mut self) -> Self{
       if *self > Self::MIN{*self = *self - 1; return *self;}
       *self
    }
}
impl helpful_math_ops for i64 {
    fn inc(&mut self) -> Self{
       if *self < Self::MAX{*self = *self + 1; return *self;}
       *self
    }
    fn dec(&mut self) -> Self{
       if *self > Self::MIN{*self = *self - 1; return *self;}
       *self
    }
}
impl helpful_math_ops for i32 {
    fn inc(&mut self) -> Self{
       if *self < Self::MAX{*self = *self + 1; return *self;}
       *self
    }
    fn dec(&mut self) -> Self{
       if *self > Self::MIN{*self = *self - 1; return *self;}
       *self
    }
}
pub trait content_stat {
    fn count_chars(&mut self, ch: &str);
}
impl content_stat for std::fs::File{
    fn count_chars(&mut self, ch: &str) {
        let func_name = "count_chars".strn();
        let ch = match ch.chars().nth(0){Some(ch) => ch, _ => return println!("{func_name} got wrong character.")};
        let mut count = 0u64;
        let buf_size = 20*1024;
        let mut buf = vec![0u8; buf_size];
        loop{
            let len_of_read = match self.read(&mut buf){Ok(f) => f, Err(e) => return println!("func {func_name} got {:?}", e)};
            if len_of_read == 0{break;}
            let str0 = String::from_utf8_lossy(&buf);
            for j in str0.chars(){
                if j == ch{count.inc();}
            }
        }
        println!("File contains {count} of {ch}");        
    }
}
pub trait fs_tools {
    fn unreel_link_to_file__(&mut self) -> Self;
#[cfg(feature="tam")]
    fn unreel_link_to_file(&mut self) -> Self;
#[cfg(feature="tam")]
    fn unreel_link_to_file0(&mut self) -> Self;
#[cfg(feature="tam")]
    fn __unreel_link_to_file(&mut self) -> Self;
#[cfg(feature="tam")]
    fn unreel_link_to_depth(&mut self, count_down: u64) -> Self;
}
impl fs_tools for String{
    fn unreel_link_to_file__(&mut self) -> Self{
        let mut run = true;
        while run{
            match std::fs::read_link(& self){
                Ok(ln) => {*self = ln.to_str().unwrap().strn();},
                _ => {run = false;}
            }
        }
      //std::process::Command::new("notify-send").arg(self.clone() ).output().expect("");
        self.strn()
    }
#[cfg(feature="tam")]
    fn unreel_link_to_file(&mut self) -> Self{
        let mut prime_path = self.clone();
        if *self == ""{return "".strn() }
        let mut run = true;
        while run{
            match std::fs::read_link(& self){
                Ok(ln) => {*self = ln.to_str().unwrap().strn();},
                _ => {run = false;}
            }
        }
        self.strn()
    }
#[cfg(feature="tam")]
    fn __unreel_link_to_file(&mut self) -> Self{
        let mut prime_path = self.clone();
        if *self == ""{return "".strn() }
        let mut run = true;
        while run{
            match std::fs::read_link(& self){
                Ok(ln) => {*self = ln.to_str().unwrap().strn();},
                _ => {run = false;}
            }
        }
     /* if !std::path::Path::new(self).exists() && *self != ""{
        crate::core18::tailOFF(&mut prime_path, "/");
        prime_path = crate::globs18::take_list_adr_env(&prime_path);
        return prime_path.unreel_link_to_file__();}*/
        self.strn()
    }
#[cfg(feature="tam")]
    fn unreel_link_to_file0(&mut self) -> Self {
        if *self == ""{return "".strn() }
        let mut prime_path = self.clone();
        loop{
            let cmd = format!("readlink {}", prime_path.clone() );
            prime_path = run_cmd_out_sync(cmd).trim_end().strn();
            if prime_path == *self || prime_path == "" {break;}
            *self = prime_path.clone();
        }
        if prime_path == ""{
            prime_path = self.strn();
            crate::core18::tailOFF(&mut prime_path, "/");
            prime_path = crate::globs18::take_list_adr_env(&prime_path);
            if std::path::Path::new(&prime_path).exists(){return prime_path.unreel_link_to_file__();}
        }
        self.clone()
    }
#[cfg(feature="tam")]
    fn unreel_link_to_depth(&mut self, count_down: u64) -> Self {
        if *self == ""{return "".strn() }
        let mut count_down = count_down;
        let mut run = true;
        while run && count_down > 0{
            match std::fs::read_link(& self){
                Ok(ln) => {*self = ln.to_str().unwrap().strn();},
                _ => {run = false;}
            }
            count_down.dec();
        }
        self.strn()
    }
}
#[cfg(feature="tam")]
pub(crate) trait find_substrn {
    fn enum_entry_points_of_substrn(&self, delim: &String) -> Vec<usize>;
}
#[cfg(feature="tam")]
impl find_substrn for String{
    fn enum_entry_points_of_substrn(&self, delim: &String) -> Vec<usize> {
    let mut EPs: Vec<usize> = Vec::new();
    let mut count: usize = 0;
    let mut maybe = String::new();
    let delim_len = delim.chars().count();
    let strn_len = self.chars().count();
    let mut count_delim_chars = 0usize;
    for i in self.chars(){
        if count_delim_chars < delim_len && Some(i) == delim.chars().nth(count_delim_chars){
            maybe.push(i);
            count_delim_chars += 1;
            //println!("{}", maybe);
        } else {
            if maybe == *delim {EPs.push(count - delim_len );}
            count_delim_chars = 0;
            maybe = String::new();
            if count_delim_chars < delim_len && Some(i) == delim.chars().nth(count_delim_chars){
            maybe.push(i);
            count_delim_chars += 1;
            }
        }
        count.inc();
    }
    if maybe == *delim {EPs.push(strn_len - delim_len );}
    EPs
}
}
pub(crate) trait vec_tools <T>{
    fn up2 (&self, bar: T) -> Vec<T>;
    fn down2 (&self, bar: T) -> Vec<T>;
}
impl <T> vec_tools <T> for Vec<T> where T: Clone + ?Sized + Eq + std::cmp::PartialOrd + helpful_math_ops + std::ops::Sub<Output = T>  {
    fn up2 (&self, bar: T ) -> Vec<T> {
        let mut vecc: Vec<T> = Vec::new();
        if self.len() == 0{return vecc}
        let zero = self[0].clone() - self[0].clone();
        for i in 0..self.len(){
            if self[i] >= bar {break;}
            vecc.push(self[i].clone());
        }
        vecc
    }
    fn down2 (&self, bar: T) -> Vec<T> {
        let mut vecc: Vec<T> = Vec::new();
        if self.len() == 0{return vecc}
        let zero = self[0].clone() - self[0].clone();
        for i in 0..self.len(){
            if self[i] == zero{ continue;}
            if self[i] < bar {continue;}
            vecc.push(self[i].clone().inc());
        }
        vecc
    }
}
#[cfg(feature = "tam")]
pub trait escaped_chars {
    fn replace_unesc_ch (&self, ch: &str, new0: &str ) -> String;
}
#[cfg( feature = "tam")]
impl <T> escaped_chars for T where T: ToString + STRN + AsRef < str > {
    fn replace_unesc_ch (&self, ch0: &str, new0: &str) -> String {
        let mut ret = "".strn();
        let mut prev_ch: Option <char> = None;
        let mut zelf = self.strn().replace (";;", "c0mm@").strn ();
        for ch in zelf.chars () {
            if ch0.chars().nth(0).unwrap() == ch && prev_ch != Some( '\\' ) {ret.push_str(new0 )}
            else { ret.push ( ch ) }
            prev_ch = Some(ch );
        }
        ret.replace("c0mm@", ";").strn()
    }
}
#[cfg(feature = "tam")]
pub trait escaped_chars_mut {
    fn replace_unesc_ch (&mut self, ch: &str, new0: &str ) -> String;
}
#[cfg( feature = "tam")]
impl escaped_chars_mut for String {
    fn replace_unesc_ch (&mut self, ch0: &str, new0: &str) -> String {
        let mut ret = "".strn();
        let mut prev_ch: Option <char> = None;
        for ch in self.chars () {
            if ch0.chars().nth(0).unwrap() == ch && prev_ch != Some( '\\' ) {ret.push_str(new0 )}
            else { ret.push ( ch ) }
            prev_ch = Some(ch );
        }
        *self = ret;
        self.clone()
    }
}

//fn
