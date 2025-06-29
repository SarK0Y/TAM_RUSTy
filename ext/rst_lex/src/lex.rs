use once_cell::sync::Lazy;
use substring::Substring;
use std::io::Write;
use Mademoiselle_Entropia::custom_traits::{STRN, helpful_math_ops};
use Mademoiselle_Entropia::help_funcs::{get_file_append};
use crate::strns::split_once_or_ret_null_strns;
use crate::faav::{rExpr, type_of_vars_expr, token_status, found_local_vars, log_name, close_complex_var };
macro_rules! _set_usize {
    ($set0:expr, $new:expr) => {
        *$set0 = $new;
    };
}
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
pub fn stat_local_vars (expr: &Vec <rExpr>) -> found_local_vars {
/*
found_local_vars {
    pub mut_or_not: Vec <bool>,
    pub pub_or_not: Vec <bool>,
    pub static_or_not: Vec <bool>,
    pub _type: Vec <String>,
    pub line: Vec <usize>,
    pub column: Vec <usize>,
    pub name: Vec <String>,
    pub txt: Vec <String>,
}
*/
    let mut all_locals = found_local_vars::new();
    todo!();
}
pub fn slabs (strn: &String) -> Option < Vec < String > >{
    let mut slab = String::new();
    let mut ret = Vec::<String>::new();
    let len = slab.chars().count();
    let mut chars = &strn.chars();
    let mut _from = usize::MAX;
    let mut ch: char = ' ';
    let nl = char::from_u32 (0x0a).unwrap();
    for j in 0..len {
        ch = chars.clone().nth (j).unwrap();
        if ch != ' ' { _from = j; break;}
    }
    if _from > len { return None }
    for k in _from..len {
        ch = chars.clone().nth (k).unwrap();
        if ch != ' ' && ch != nl { slab.push(ch); }
        else {
            if slab.is_empty () {continue;}
            ret.push ( slab.clone() ); slab.clear();
        }
    }
    return Some ( ret )
}
pub fn collect_not_nested_let_tokens (stream: &String) -> Vec < rExpr > {
    let mut ret = Vec::< rExpr >::new ();
    let mut rexpr = rExpr::new();
    let mut run_from: usize = _1st_fn_line (stream).0;
    let stream = stream.substring(0, stream.chars().count() ).strn();
    loop {
       // println! ("run_from {run_from}");
        leave_file_mark ("/tmp/start", &format! ("got{run_from}"));
        leave_file_mark ("/tmp/func", &stream);
        if let Some ( x ) = stream_sieving1 (&stream, "let".strn(), run_from, ";".strn() ) { rexpr = x } else { break;};
        leave_file_mark ("/tmp/end", &format! ("!got{run_from}"));
        run_from = rexpr.end;
        let sub_str = stream.substring (0, run_from).strn();
     //   println! ("{sub_str}, {} {}", sub_str.chars().count(), blocks_status (None));
        ret.push (rexpr.clone() );
    }
    return ret
}
pub fn get_lines_in_fn (stream: &mut String) -> Vec < rExpr > {
    let (mut _1st_ln, header) = _1st_fn_line ( stream );
    let mut ret = Vec::<rExpr>::new();
    let header_len = header.chars().count();
    let stream_len = stream.chars().count ();
    *stream = stream.substring(header_len, stream_len).strn();
    let header = rExpr {
        txt: header,
        line: 0,
        column: 0,
        entry: 0,
        end: 0
        };
   // dbg! (&header);
    ret.push(header);
    let mut chars = stream.chars();
    let mut rexpr: Option < rExpr > = None;
    let mut collect_blocks: Option < Vec < rExpr > > = None;
    loop {
        let fst_ch = if stream.chars().count () > 0 { stream.chars().nth(0).unwrap().to_string() } else {break;};
        rexpr = stream_sieving2 (stream, &fst_ch, 0, ";");
     //   dbg! (&rexpr);
        if let Some ( ref mut y) = rexpr {
          // println! ("MM: {:?}", y);
            *stream = stream.replace (&y.txt, "").strn();
            collect_blocks = cut_blocks (&mut y.txt, 0);
          //  println! ("tst: {:?}", collect_blocks);
            dbg! ("mark0");
            dbg! (&ret);
            dbg! (&stream);
            dbg!("mark");
            if let Some ( ref cb ) = collect_blocks {
                _1st_ln = cb [ cb.len() - 1].end;
                ret.extend (cb.clone () ); continue;
            }
          //  if y_len == 0 {break; }
            ret.push (y.clone() );
        }
         
    }
    if ret.is_empty() { return ret}
    let mut last: rExpr = ret.pop().unwrap();
    let ch = last.txt.pop ().unwrap_or (' ');
//    println! ("{:?}", last);
  //  if ch == '}' {last.txt = last.txt.as_str().substring(0, last.txt.chars().count() - 1).strn();}
    last.txt.push('}');
    ret.push (last);
    println! ("***************************");
    dbg! (&ret);
    return ret
}
pub fn get_lines_in_block (stream: &mut String) -> Vec < rExpr > {
    let (mut _1st_ln, header) = _1st_fn_line ( stream );
    let mut ret = Vec::<rExpr>::new();
    let header_len = header.chars().count();
    let stream_len = stream.chars().count ();
    *stream = stream.substring(header_len, stream_len).strn();
    let header = rExpr {
        txt: header,
        line: 0,
        column: 0,
        entry: 0,
        end: 0
        };
    ret.push(header);
    let mut chars = stream.chars();
    let mut rexpr: Option < rExpr > = None;
    let mut collect_blocks: Option < Vec < rExpr > > = None;
    loop {
        let fst_ch = if stream.chars().count () > 0 { stream.chars().nth(0).unwrap().to_string() } else {break;};
        rexpr = stream_sieving2 (stream, &fst_ch, 0, ";");
      //  dbg! (&rexpr);
        if let Some ( ref mut y) = rexpr {
    //       println! ("{:?}", y);
            collect_blocks = cut_blocks (&mut y.txt, 0);
            let y_len = y.txt.chars().count();
            let stream_len = stream.chars().count ();
            *stream = stream.replace (&y.txt, "").strn();// stream.substring(y_len, stream_len).strn();
          //  println! ("tst: {:?}", collect_blocks);
            if let Some ( ref cb ) = collect_blocks {
              //  _1st_ln = cb [ cb.len() - 1].end;
                ret.extend (cb.clone () ); continue;
            }
            if y_len == 0 { break; }
            ret.push (y.clone() );
        }
    }
    if ret.is_empty() { return ret}
    let mut last: rExpr = ret.pop().unwrap();
    let ch = last.txt.pop ().unwrap_or (' ');
//    println! ("{:?}", last);
  //  if ch == '}' {last.txt = last.txt.as_str().substring(0, last.txt.chars().count() - 1).strn();}
    last.txt.push('}');
    ret.push (last);
   // println! ("***************************");
    //dbg! (&ret);
 //   dbg! (&stream);
    return ret
}
pub fn cut_blocks (expr: &mut String, prev_end: usize) -> Option < Vec < rExpr > > {
    if expr.is_empty() { return None }
    let mut edited = expr.clone();
    let mut ret = Vec::<rExpr>::new();
    let mut fst_ch = expr.chars().nth(0).unwrap().to_string();
    let mut end: usize = 0;
    let mut cut_block_off: Option < rExpr > = stream_sieving2 (&edited, &fst_ch, 0, "}");
    if let Some (mut x) = cut_block_off {
        
        if x.txt.len() == expr.len() ||
           x.txt.len() == expr.len() - 1 { return None }
           dbg! (&expr);
           dbg! (&x);
           edited = edited.replace(&x.txt, "");
           let mut lines_in_block = get_lines_in_block (&mut x.txt);
           //x.txt = format! ("mm: {}", x.txt);
         //  x.entry = prev_end;
           //x.end = prev_end + x.txt.chars().count();
           //end = x.end;
           let block_len = lines_in_block.len();
           lines_in_block[block_len - 1].end = x.txt.chars().count();
           ret.extend(lines_in_block.clone());
           if edited == "" {return Some (ret)}
    }
    loop {
         println! ("edited: {edited}");
        cut_block_off = stream_sieving2 (&edited, &fst_ch, 0, "}");
        if let Some ( mut x) = cut_block_off {
           // x.entry = end;
            //x.end = end + x.txt.chars().count();
            //end = x.end;
            println! ("xx:: {:?}", edited);
            if x.txt.len() == edited.len() ||
               x.txt.len() == edited.len() - 1 { ret.push ( x.clone() ); return Some ( ret ) }
            edited = edited.replace(&x.txt, "");
            let mut lines_in_block = get_lines_in_block (&mut x.txt);
            //_start = x.end;
            //let block_len = lines_in_block.len();
           // lines_in_block[block_len - 1].end = x.txt.chars().count();
            ret.extend(lines_in_block.clone());
        } else { return None }
        if edited.is_empty () { return Some (ret) }
        fst_ch = expr.chars().nth(0).unwrap().to_string();
    }
}
pub fn log_vars (stream: &mut String) -> String {
    let mut fn_ln_by_ln: Vec <rExpr> = get_lines_in_fn (stream);
    for j in 0..fn_ln_by_ln.len() {
        let ln = fn_ln_by_ln[j].txt.clone();
        if check_let ( &ln ) { continue }
        let categorize_var =  var_expr_or_not (&ln);
        match categorize_var {
            type_of_vars_expr::not => { continue;},
            type_of_vars_expr::simple => { fn_ln_by_ln[j].txt = make_simple_var_logged (&ln);},
            type_of_vars_expr::complex => { unimplemented!();}
        }
        
    }
    todo! ()
}
pub fn make_simple_var_logged (expr: &String ) -> String {
    todo! ()
}
pub fn log_the_var (var_name: &str, value: &str ) {
    let strn_to_log = format! ("{var_name}: {value}");
    let log_file = log_name (None ).unwrap_or ("/tmp/log_func".strn());
    let mut log_file = get_file_append (&log_file);
    log_file.expect("log_the_var failed").write_all(strn_to_log.as_bytes());
}
pub fn check_let (expr: &String) -> bool {
    let token = "let";
    if expr.trim_start().substring (0, token.chars().count () ) == token { return true }
    let token = "static";
    if expr.trim_start().substring (0, token.chars().count () ) == token { return true }
    return false
}
pub fn var_expr_or_not (expr: &String) -> type_of_vars_expr {
    let ret = stream_sieving3 (expr.trim_start(), "=", 0, ";" );
    if ret.is_some() {return type_of_vars_expr::simple }
    let ret = stream_sieving3 (expr.trim_start(), "=", 0, "{" );
    if ret.is_some() {
        blocks_status( Some (&'{' ) );
        return type_of_vars_expr::complex }
    return type_of_vars_expr::not
}
pub fn stream_sieving1 (stream: &String, token: String, run_from: usize, stop_token: String) -> Option < rExpr > {
    return stream_sieving (stream, &token, run_from, &stop_token)
}
pub fn stream_sieving2 (stream: &String, token: &str, run_from: usize, stop_token: &str) -> Option < rExpr > {
    return stream_sieving (stream, &token.to_string(), run_from, &stop_token.to_string() )
}
pub fn stream_sieving3 (stream: &str, token: &str, run_from: usize, stop_token: &str) -> Option < rExpr > {
    return stream_sieving (&stream.strn(), &token.to_string(), run_from, &stop_token.to_string() )
}
#[inline]
pub fn stream_sieving (stream: &String, token: &String, run_from: usize, stop_token: &String) -> Option < rExpr > {
    if stream.is_empty () { return None}
    let fn_name = "stream_sieving".strn();
    let mut line: usize = 0;
    let mut column = line;
    let mut entry: usize = 0;
    let mut entry1: *mut usize = &mut entry;
    let mut end = line;
    dbg! (&stop_token);
    leave_file_mark ("/tmp/line", &stop_token.to_string() );
    let nl = char::from_u32(0x0a).unwrap();
    let mut maybe = String::new();
    let stop_token_len = stop_token.chars().count();
    let token_len = token.chars().count();
    let to_stream_len: usize = stream.chars().count();
    let mut chars = stream.chars();
    let mut txt_dbg = String::new();
    for j in run_from..to_stream_len {
        let ch = chars.clone().nth (j).unwrap_or (' ');
        if ch == nl {column = 0; line.inc(); }
        maybe.push(ch);
        txt_dbg.push( ch );
       // println! ("{txt_dbg}");
        if maybe.chars().count() == token_len {
            if maybe == *token {
                leave_file_mark ("/tmp/mayb", &maybe.to_string() );
                entry = if j > token_len { j - token_len + 1 } else { j };
                leave_file_mark ("/tmp/entry0", &entry.to_string() ); 
                break;
            }
        }
        if  blocks_status ( Some (&ch) ) || (!maybe.is_empty() && token.as_str().substring (0, maybe.chars().count()) != maybe ) {maybe.clear (); }
        column.inc();
        leave_file_mark ("/tmp/ln", &line.strn() );
        leave_file_mark ("/tmp/col", &column.strn() );
    }
    leave_file_mark ("/tmp/entry", &entry.to_string() );
    leave_file_mark ("/tmp/ln1", &line.strn() );
    //println! ("{line}, {maybe}");
        leave_file_mark ("/tmp/col1", &column.strn() );
    //leave_file_mark ("/tmp/entry1", &entry1.to_string() );
    leave_file_mark ("/tmp/may", &maybe.to_string() );
    if maybe.is_empty() { return None }
    maybe.clear();
    let run_from = entry + token_len;
    let mut txt = token.clone();
    for j in run_from..to_stream_len {
       let ch = stream.chars().nth (j).unwrap ();
       if token.as_str().substring (0, maybe.chars().count()) != maybe {maybe.clear(); }
       txt.push(ch);
      if blocks_status ( Some (&ch ) ) {maybe.clear(); continue; }
      maybe.push(ch);
      if maybe.chars().count() == stop_token_len {
         if maybe == *stop_token { dbg! (&maybe); break; }
      }
    }
    end = entry + txt.chars().count ();
  //  dbg! (stream);
    //dbg! (&txt);
    //dbg! (&fn_name);
   // println! ("{}", txt);
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
    static mut cite: u64 = 0;
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
            '\"' => {cite += 1; cite %= 2; },
            _ => {}
        }
        let sum = curly + round + square + cite;
        if sum == 0 { state = false;} else { state = true; } 
    //    println! ("{state}, {ch}");
        return state
    }
}
pub fn _1st_fn_line (stream: &String) -> (usize, String) {
    let mut header = String::new();
    for j in 0..stream.chars().count() {
        let ch = stream.chars().nth ( j ).unwrap ();
        header.push(ch);
        if ch == '{' { return (j + 1, header) }
    } return (0, header)
}
pub fn token_for_loop (stream: &String, search_from: usize) -> Option < rExpr > {
    let entry: usize = _1st_fn_line ( stream ).0;
    let search_from = if search_from > entry { search_from } else { entry };
    return stream_sieving2 (stream, "for", search_from, "}")
}
pub fn leave_file_mark (nm: &str, msg: &str){
    use std::fs::File;
    use std::io::{self, Write};
    let mut file = File::create(nm).expect("Unable to create file");
    file.write_all(msg.as_bytes()).expect("Unable to write data");
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
/****************************  enums/structs ****************************/
