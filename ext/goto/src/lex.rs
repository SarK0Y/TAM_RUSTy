use once_cell::sync::Lazy;
use substring::Substring;
use Mademoiselle_Entropia::custom_traits::{STRN, helpful_math_ops};
pub fn read_token (key: &String, txt: &String) -> Option < Vec <String> > {
    if txt.len() == 0 { return None}
    let mut ret0 = key.clone ();
    let key_len = key.chars().count();
    //let mut pos: usize = if let Some (x) = txt.find (key) { x + key_len } else {return None };
    let mut pos: Vec <usize> = txt.match_indices (key).map (|(j, _)| j + key_len ).collect();
    let mut chars = txt.chars();
    let mut ch = chars.nth(0).unwrap();
    let stop_ch = ":".strn().chars().nth(0).unwrap();
    let mut out = Vec:: <String>:: new ();
    while let Some (nxt_pos) = pos.pop() {
        let mut ret = ret0.clone();
        for j in nxt_pos..txt.chars().count() {
            ch = chars.nth(j).unwrap();
            if ch != stop_ch { ret.push(ch);}
        }
        out.push(ret);
    }
    return Some (out.clone() )
}
pub fn stat_local_vars (fn_str: String) -> found_local_vars {
    let mut all_locals = found_local_vars::new();
    todo!();
}
pub fn collect_not_nested_let_tokens (stream: &String) -> Vec < rExpr > {
    let mut ret = Vec::< rExpr >::new ();
    let mut rexpr = rExpr::new();
    let mut run_from: usize = 0;
    loop {
        leave_file_mark ("/tmp/start", &format! ("got{run_from}"));
        leave_file_mark ("/tmp/func", stream);
        //if let Some ( x ) = stream_sieving1 (stream, "let".strn(), run_from, ";".strn() ) { rexpr = x } else { break;};
        leave_file_mark ("/tmp/end", &format! ("got{run_from}"));
        run_from = rexpr.end;
        ret.push (rexpr.clone() );
    }
    return ret
}
pub fn stream_sieving1 (stream: &String, token: String, run_from: usize, stop_token: String) -> Option < rExpr > {
    return stream_sieving (stream, &token, run_from, &stop_token)
}
pub fn stream_sieving (stream: &String, token: &String, run_from: usize, stop_token: &String) -> Option < rExpr > {
    let mut line: usize = 0;
    let mut column = line;
    let mut entry = line;
    let mut end = line;
    let nl = char::from_u32(0x0a);//.unwrap().to_string();
    let mut maybe = String::new();
    let stop_token_len = stop_token.chars().count();
    let token_len = token.chars().count();
    let to_stream_len: usize = stream.chars().count();
    let mut chars = stream.chars();
    for j in run_from..to_stream_len {
        column.inc();
        if chars.nth (j) == nl {column = 0; line.inc(); }
        let ch = chars.nth (j).unwrap ();
        maybe.push(ch);
        if maybe.chars().count() == token_len {
            if maybe == *token { entry = j - token_len + 1; break; }
        }
        if token.as_str().substring (0, maybe.chars().count()) != maybe {maybe.clear (); }
    }
    
    if maybe.is_empty() { return None }
    
    maybe.clear();
    let run_from = run_from + token_len - 1;
    let mut txt = token.clone();
    for j in run_from..to_stream_len {
        let ch = stream.chars().nth (j).unwrap ();
        if token.as_str().substring (0, maybe.chars().count()) != maybe {maybe.clear(); }
        txt.push(ch);
        if blocks_status ( Some (&ch ) ) {continue; }
        maybe.push(ch);
        if maybe.chars().count() == stop_token_len {
            if maybe == *stop_token { break; }
        }
    }
    txt.push_str ( stop_token.clone().as_str () );
    end = entry + txt.chars().count ();
    return Some (
        rExpr {
            txt,
            line,
            column,
            entry,
            end
        }
    )
}
pub fn blocks_status (ch: Option < &char > ) -> bool {
    static mut curly: u64 = 0;
    static mut round: u64 = 0;
    static mut square: u64 = 0;
    static mut state: bool = false;
    unsafe {
        if ch.is_none () { return state }
        let ch = ch.unwrap();
        match *ch {
            '{' => {curly.inc(); }, 
            '}' => {curly.dec(); },
            '(' => {round.inc(); },
            ')' => {round.dec(); }, 
            '[' => {square.inc(); }, 
            ']' => {square.dec(); },
            _ => {}
        }
        let sum = curly + round + square;
        if sum == 0 { state = false;} else { state = true; } return state
    }
}
pub fn leave_file_mark (nm: &str, msg: &str){
    use std::fs::File;
    use std::io::{self, Write};
    let mut file = File::create(nm).expect("Unable to create file");
    file.write_all(msg.as_bytes()).expect("Unable to write data");
}
#[derive(Clone, Debug)]
pub struct found_local_vars {
    pub mut_or_not: Vec <bool>,
    pub pub_or_not: Vec <bool>,
    pub static_or_not: Vec <bool>,
    pub _type: Vec <String>,
    pub line: Vec <usize>,
    pub column: Vec <usize>,
    pub name: Vec <String>,
    pub txt: Vec <String>,
}
impl found_local_vars {
    fn new () -> Self {
        return Self {
            mut_or_not: Vec::<bool>::new(),
            pub_or_not: Vec::<bool>::new(),
            static_or_not: Vec::<bool>::new(),
            _type: Vec::<String>::new(),
            name: Vec::<String>::new(),
            txt: Vec::<String>::new(),
            line: Vec::<usize>::new(),
            column: Vec::<usize>::new(),
        }
    }
}
#[derive(Clone, Debug)]
pub struct rExpr {
    pub txt: String,
    pub line: usize,
    pub column: usize,
    pub entry: usize,
    pub end: usize
}
impl rExpr {
    fn new () -> Self {
        return Self {
            txt: String::new(),
            line: 0,
            column: 0,
            entry: 0,
            end: 0
        }
    }
}
pub fn get_token (indx: usize) -> token_status {
    return set_of_tokens (None, indx)
}
pub fn set_of_tokens (add_nxt: Option <String>, get: usize) -> token_status {
    static mut tokens: Lazy <Vec <String> > = Lazy::new (|| {Vec::<String>::new()});
    unsafe {
        if let Some (x) = add_nxt { tokens.push (x); return token_status::new_added }
        let len = tokens.len();
        if len == 0 { return token_status::empty }
        if get < len { return token_status::ret ( tokens [get].clone() ) } return token_status::too_large_indx
    }
}
pub enum token_status {
    too_large_indx,
    ret (String),
    empty,
    new_added
}
pub enum prime_token {
    semicolon,
    colon,
    dot,
    comma,
    paren (char),
    any_symb (char)
}
