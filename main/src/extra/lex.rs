use crate::custom_traits::STRN;
pub fn read_token (key: &String, txt: &String) -> Option < String > {
    if txt.len() == 0 { return None}
    let mut ret = key.clone ();
    let key_len = key.chars().count();
    let mut pos: usize = if let Some (x) = txt.find (key) { x + key_len } else {return None };
    let mut chars = txt.chars();
    let mut ch = chars.nth(0).unwrap();
    let stop_ch = ":".strn().chars().nth(0).unwrap();
    for j in pos..txt.chars().count() {
        ch = chars.nth(j).unwrap();
        if ch != stop_ch { ret.push(ch);}
    }
    None
}