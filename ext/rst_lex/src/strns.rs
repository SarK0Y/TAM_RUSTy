use Mademoiselle_Entropia::custom_traits::{STRN, STRN_usize, helpful_math_ops}; 
use crate::faav::log_attr;
use substring::Substring;
pub fn split_once_or_ret_null_strns(in_string: &str, delim: &str) -> (String, String) {
    if delim.chars().count() > 1{return split_once_alt_o_null_strns(&in_string.to_string(), &delim.to_string());}
let mut splitter = in_string.splitn(2, delim);
let first = match splitter.next(){
    Some(val) => val,
    _ => return ("".to_string(), "".to_string())
};
let second = match splitter.next(){
    Some(val) => val,
    _ => return (first.to_string(), "".to_string())
};
return  (first.to_string(), second.to_string());
}
pub(crate) fn split_once_alt_o_null_strns(strn: &String, delim: &String) -> (String, String) {
    let mut maybe = String::new();
    let mut found = false;
    let delim_len = delim.chars().count();
    let strn_len = strn.chars().count();
    let mut count_delim_chars = 0usize;
    let mut ret = (String::new(), String::new());
    for i in strn.chars() {
        if count_delim_chars < delim_len
            && Some(i) == delim.chars().nth(count_delim_chars)
            && !found
        {
            maybe.push(i);
            count_delim_chars += 1;
            if maybe == *delim {
                found = true;
            }
        } else {
            if found {
                ret.1.push(i);
                continue;
            }
            // if maybe == *delim {ret.1.push(i); found = true; continue;}
            count_delim_chars = 0;
            ret.0.push_str(maybe.as_str());
            ret.0.push(i);
            maybe = String::new();
        }
    }
    if !found {
        return ("".strn(), "".strn());
    }
    ret
}
pub fn get_attrs_for_log_vars (attrs: &String) -> log_attr {
    let mut ret = log_attr::new();
    let (attr0, attr1) = split_once_alt_o_null_strns (attrs, &",".strn() );
    get_attr_for_log_vars (&attr0, &mut ret);
    get_attr_for_log_vars (&attr1, &mut ret);
    return ret
}
pub fn get_attr_for_log_vars (attr: &String, ret: &mut log_attr ) {
    if attr.is_empty () {panic! ("Please, set attributes for log file.. Ex: #[log_vars(log_size=10K,log_path=/tst/log)]\nRemark: attr is empty")}
    let (attr0_0, attr0_1) = split_once_alt_o_null_strns (&attr, &"=".strn() );
    let attr0_0 = attr0_0.trim();//.strn();
    match attr0_0 {
        "log_size" => {ret.size = get_size_from_log_conf(&attr0_1);},
        "log_path" => {ret.path = attr0_1.trim().strn();},
        _ => {panic! ("Please, set attributes for log file.. Ex: #[log_vars(log_size=10K,log_path=/tst/log)]")}
    }
}
pub fn get_size_from_log_conf (attr: &String) -> usize {
    let mark = attr.chars().nth (attr.chars().count() - 1).unwrap();
    let coef = size_mark (mark);
    let ret = if coef == 1 { strn_2_usize (&attr).unwrap_or (0) } else {
        let attr = attr.substring (0, attr.chars().count() -1 ).strn ();
        dbg! (&attr);
        strn_2_usize (&attr).unwrap_or(0) * coef
    };
    if ret == 0 {panic! ("Please, set attributes for log file.. Ex: #[log_vars(log_size=10K,log_path=/tst/log)]\nRemark: can't get file size for log.");}
    dbg! (&ret);
    return ret
}
pub fn size_mark (m: char) -> usize {
    match m {
        'K'|'k' => { return 1024 },
        'M'|'m' => { return 1048576 },
        'G'|'g' => { return 1073741824 },
        'T'|'t' => { return 1099511627776},
        _ => { return 1}
    }
}
pub fn strn_2_usize(strn: &String) -> Option<usize> {
    match usize::from_str_radix(&strn, 10) {
        Ok(num) => Some(num),
        _ => None,
    }
}
