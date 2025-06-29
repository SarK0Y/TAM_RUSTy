use Mademoiselle_Entropia::custom_traits::{STRN, helpful_math_ops}; 
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
