use Mademoiselle_Entropia::custom_traits::STRN;
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
pub fn stream_sieving (stream: &String, token: &String, run_from: usize, stop_token: &String) -> rExpr {
    let mut line: usize = 0;
    let mut column = line;
    let mut maybe = String::new();
    let stream_len: usize = stream.len();
    for j in run_from..stream_len {
        
    }
    todo!();
}
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
pub struct rExpr {
    pub txt: String,
    pub line: usize,
    pub column: usize,
}
