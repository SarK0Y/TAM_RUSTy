use rug::float::Round;
use rug::ops::{
    AddAssignRound, DivAssignRound,
    MulAssignRound, PowAssign as rugPowAssign,
    PowAssignRound, SubAssignRound,
    Pow as rugpow, CompleteRound,
};
use rug::{
    Assign, Integer as rugint,
    float::Constant as rugconst,
    Float as rugfloat, ops::SubFrom,
    Complete
};
use rug::float::Constant;
use std::convert::TryFrom;
use substring::Substring;
use crate::base::{glob_precision, Pi };
use crate::nth_root::__2rt;
use Mademoiselle_Entropia::custom_traits::STRN;
use Mademoiselle_Entropia::minio::InterruptMsg;
pub enum parse_cmd {
    input_rugfloat (String),
    input_rugint (String),
    out_rugfloat (rugfloat),
    out_rugint (rugint),
    out_strn (String),
    null
}
pub fn parse_strn_number (cmd: &parse_cmd, radix: i32) -> parse_cmd {
    /*match cmd {
        parse_cmd::input_rugfloat
    }*/
    todo! ()
}
pub fn exclude_wrong_symbs_from_num_strn (num: &String) -> String {
    let mut ret = String::new ();
    for ch in num.chars() {
        if check_only_num_char (ch) {
            ret.push (ch);
        }
    } return ret
}
pub fn exclude_wrong_symbs_from_float_strn (num: &String) -> String {
    let mut ret = String::new ();
    for ch in num.chars() {
        if check_only_float_char (ch) {
            ret.push (ch);
        }
    } return ret
}
pub fn ret_extended_num_format (num: &String) -> String {
    let mut ret = String::new ();
    for ch in num.chars() {
        if check_num_char (ch) {
            ret.push (ch);
        }
    } return ret
}
pub fn ret_extended_float_format (num: &String) -> String {
    let mut ret = String::new ();
    for ch in num.chars() {
        if check_float_char (ch) {
            ret.push (ch);
        }
    } return ret
}
pub fn conv_strn_2_rugint (_strn: &String, radix: i32) -> Option < rugint > {
    let _strn = &exclude_wrong_symbs_from_num_strn ( _strn );
    let num = rugint::parse_radix (_strn, radix);
    if num.is_err() { return None }
    return Some ( num.unwrap().complete () )    
}
pub fn conv_str_2_rugint (_strn: &str, radix: i32) -> Option < rugint > {
    let _strn = &exclude_wrong_symbs_from_num_strn ( &_strn.strn() );
    let num = rugint::parse_radix (_strn, radix);
    if num.is_err() { return None }
    return Some ( num.unwrap().complete () )    
}
pub fn conv_strn_2_rugfloat (_strn: &String, radix: i32) -> Option < rugfloat > {
    let mut _strn = &mut exclude_wrong_symbs_from_float_strn ( _strn );
    if _strn.as_str ().substring (0, 1 ) == "." {
        *_strn = format! ("0{_strn}");
    }
    let num = rugfloat::parse_radix (_strn, radix);
    if num.is_err() { return None }
    return Some (
        num.unwrap().complete (
        glob_precision (None)
        .try_into ()
        .unwrap()
        )
    );
}
pub fn conv_str_2_rugfloat (_strn: &str, radix: i32) -> Option < rugfloat > {
    let mut _strn = &mut exclude_wrong_symbs_from_float_strn ( &_strn.strn() );
    if _strn.as_str ().substring (0, 1 ) == "." {
        *_strn = format! ("0{_strn}");
    }
    let num = rugfloat::parse_radix (_strn, radix);
    if num.is_err() { return None }
    return Some (
        num.unwrap().complete (
        glob_precision (None)
        .try_into ()
        .unwrap()
        )
    );
}
pub fn check_num_char (symb: char) -> bool {
    match symb {
        'a' => { return true },
        'A' => { return true },
        'B' => { return true },
        'b' => { return true },
        'c' => { return true },
        'C' => { return true },
        'D' => { return true },
        'd' => { return true },
        'f' => { return true },
        'F' => { return true },
        '_' => { return true },
        ',' => { return true },
        '0' | '1' | '2' | '3'
        | '4' | '5' | '6' |
        '7' | '8' | '9' =>
        { return true }
        _ => {},
    };
    let symb_ = match u32::try_from (symb) {
        Ok (b) => {b},
        Err (_) => {return false}
    };
    if symb_ <= 9 { return true }
    return false
}
pub fn check_only_num_char (symb: char) -> bool {
    match symb {
        'a' => { return true },
        'A' => { return true },
        'B' => { return true },
        'b' => { return true },
        'c' => { return true },
        'C' => { return true },
        'D' => { return true },
        'd' => { return true },
        'f' => { return true },
        'F' => { return true },
         '0' | '1' | '2' | '3'
        | '4' | '5' | '6' |
        '7' | '8' | '9' =>
        { return true }
        _ => {},
    };
   let symb_ = match u32::try_from (symb) {
        Ok (b) => {b},
        Err (_) => {return false}
    };
    if symb_ <= 9 { return true }
    return false
}
pub fn check_only_float_char (symb: char) -> bool {
    static mut _1st: bool = true;
    match symb {
        'a' => { return true },
        'A' => { return true },
        'B' => { return true },
        'b' => { return true },
        'c' => { return true },
        'C' => { return true },
        'D' => { return true },
        'd' => { return true },
        'f' => { return true },
        'F' => { return true },
        '.' => { 
        unsafe {
            if _1st {
                _1st = false;
                return true
            } return false
        }
        },
         '0' | '1' | '2' | '3'
        | '4' | '5' | '6' |
        '7' | '8' | '9' =>
        { return true }
        _ => {},
    };
   let symb_ = match u32::try_from (symb) {
        Ok (b) => {b},
        Err (_) => {return false}
    };
    if symb_ <= 9 { return true }
    return false
}
pub fn check_float_char (symb: char) -> bool {
    static mut _1st: bool = true;
    match symb {
        'a' => { return true },
        'A' => { return true },
        'B' => { return true },
        'b' => { return true },
        'c' => { return true },
        'C' => { return true },
        'D' => { return true },
        'd' => { return true },
        'f' => { return true },
        'F' => { return true },
        '_' => { return true },
        ',' => { return true },
        '.' => { 
        unsafe {
            if _1st {
                _1st = false;
                return true
            } return false
        }
        },
         '0' | '1' | '2' | '3'
        | '4' | '5' | '6' |
        '7' | '8' | '9' =>
        { return true }
        _ => {},
    };
   let symb_ = match u32::try_from (symb) {
        Ok (b) => {b},
        Err (_) => {return false}
    };
    if symb_ <= 9 { return true }
    return false
}
pub trait Conv_Strn_2_Rugfloat {
    fn float (&self, rdx: i32) -> rugfloat;
}
impl Conv_Strn_2_Rugfloat for &str {
    fn float (&self, rdx: i32) -> rugfloat {
        return conv_str_2_rugfloat (self, rdx)
            .unwrap()
    }
}