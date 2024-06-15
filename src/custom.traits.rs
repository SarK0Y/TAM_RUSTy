use num_traits::Bounded; use core::mem::size_of;
use std::io::Read;
pub(crate) trait STRN {
    fn strn(&self) -> String;
}
pub(crate) trait STRN_strip {
    fn del_ch(&self, ch: &str) -> String;
    fn strip_symbs(&mut self) -> String;
    fn strip_apostrophe(&mut self) -> String;
    fn strip_backslash(&mut self) -> String;
    fn strip_all_symbs(&mut self) -> String;
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
}
impl STRN for str {
    fn strn(&self) -> String{
        String::from(self)
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