use once_cell::sync::Lazy;
use substring::Substring;
use std::io::{Write, self, ErrorKind};
use std::fs::metadata;
use Mademoiselle_Entropia::custom_traits::{STRN, helpful_math_ops};
use Mademoiselle_Entropia::help_funcs::{get_file_append, get_file };
use crate::strns::{split_once_or_ret_null_strns, get_attrs_for_log_vars, Char_Stream};
use crate::faav::{rExpr, type_of_vars_expr, code_ln, token_status, found_local_vars, log_name, log_attr, cleanup_dbg_attr, blocks };
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
pub fn get_lex_line_n_split (stream: &String) -> (String, String) {
    /*
    Possible outputs..
    >> block entry
    >> simple line
    >> line before block's end
    >> block ends
    */
    let fst = stream.chars().nth (0);
    if fst.is_none () { return ("".strn(), "".strn() )}
    let fst = fst.unwrap ().to_string();
    let (block_entry, other1) = sieve_n_split2 (stream, &fst, 0, "{" );
    let (simple, other) = sieve_n_split2 (stream, &fst, 0, ";" );
    if simple.is_none () && block_entry.is_none () { return ("".strn(), "".strn() )}
    let block_entry = block_entry.unwrap().txt;
    let simple = simple.unwrap().txt;
//    dbg! (&block_entry);
  //  dbg! (&fst);
  //  dbg! (&simple);
    let ln = if block_entry.chars().count() < simple.chars().count () { (block_entry.clone(), other1.clone() ) }
             else { (simple.clone(), other.clone()) };
  //  if ln.0.find ("else{").is_some () { dbg! (&ln); }
    let tst_end_of_block = sieve_n_split2 (&ln.0, &fst, 0, "}");
   // dbg! (&ln);
   // dbg!(&tst_end_of_block);
    if tst_end_of_block.0.is_none () { return ln }
    let end_of_block_txt = tst_end_of_block.0.unwrap ().txt;
    let mut left = false;
    for c in end_of_block_txt.chars () {
        left = !wrong_symb(c);
        if left { break; }
    }
    let stream_len = stream.stream_len();
    let mut end_of_block_len = end_of_block_txt.chars().count ();
    if left {
        let other = stream.substring (end_of_block_len, stream_len).strn();
        dbg! (&other);
        return (end_of_block_txt, other)        
    }
    //end_of_block_len += 1;
    let close_block = format! ("{} ", end_of_block_txt);
    let other = stream.substring (end_of_block_len, stream_len).strn();
    return (close_block, other)        
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
        let new_ln = get_lex_line_n_split (stream);
        if new_ln.0.is_empty () { break }
        *stream = new_ln.1.clone();
        let new_entry = rExpr {
        txt: new_ln.0,
        line: 0,
        column: 0,
        entry: 0,
        end: 0
        };
        ret.push (new_entry);
        if new_ln.1.is_empty () { break }
    }
    if ret.is_empty() { return ret}
    //let mut last: rExpr = ret.pop().unwrap();
    //let ch = last.txt.pop ().unwrap_or (' ');
//    println! ("{:?}", last);
  //  if ch == '}' {last.txt = last.txt.as_str().substring(0, last.txt.chars().count() - 1).strn();}
   // last.txt.push('}');
   // ret.push (last);
  //  println! ("***************************");
   // dbg! (&ret);
   leave_file_mark ("/tmp/dbg_fn_ln_by_ln0", &format! ("{:?}", ret));
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
    let mut rexpr: (Option < rExpr >, String ) = (None, "".strn() );
    let mut collect_blocks: Option < Vec < rExpr > > = None;
    loop {
        let fst_ch = if stream.chars().count () > 0 { stream.chars().nth(0).unwrap().to_string() } else {break;};
        rexpr = sieve_n_split2 (stream, &fst_ch, 0, ";");
      //  dbg! (&rexpr);
        if let Some ( ref mut y) = rexpr.0 {
    //       println! ("{:?}", y);
            collect_blocks = cut_blocks (&mut y.txt, 0);
            let y_len = y.txt.chars().count();
            let stream_len = stream.chars().count ();
            *stream = rexpr.1;//stream.replace (&y.txt, "").strn();// stream.substring(y_len, stream_len).strn();
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
    let mut cut_block_off: (Option < rExpr >, String ) = sieve_n_split2 (&edited, &fst_ch, 0, "}");
    if let Some (mut x) = cut_block_off.0 {
        
        if x.txt.len() == expr.len() ||
           x.txt.len() == expr.len() - 1 { return None }
         //  dbg! (&expr);
           //dbg! (&x);
           edited = cut_block_off.1;//edited.replace(&x.txt, "");
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
        cut_block_off = sieve_n_split2 (&edited, &fst_ch, 0, "}");
        if let Some ( mut x) = cut_block_off.0 {
           // x.entry = end;
            //x.end = end + x.txt.chars().count();
            //end = x.end;
            println! ("xx:: {:?}", edited);
            if x.txt.len() == edited.len() /*||
               x.txt.len() == edited.len() - 1*/ { ret.push ( x.clone() ); return Some ( ret ) }
            edited = cut_block_off.1; //edited.replace(&x.txt, "");
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
pub fn control_nested_blocks (state: &code_ln, depth: &mut usize ) {
    match state {
        code_ln::enter_block => { depth.inc (); },
        code_ln::exit_block  => { depth.dec (); },
        _                  => {},
    }
}
pub fn _log_vars (stream: &mut String, attrs: &String) -> String {
    let extract_log_attrs = get_attrs_for_log_vars (attrs);
    dbg! (&extract_log_attrs);
    let mut fn_ln_by_ln: Vec <rExpr> = get_lines_in_fn (stream);
    let dbg_fn_ln_by_ln = format! ("{:?}", fn_ln_by_ln);
    leave_file_mark ("/tmp/dbg_fn_ln_by_ln", &dbg_fn_ln_by_ln);
    let nl = char::from_u32(0x0a).unwrap().to_string();
    let deps = fn_ln_by_ln[0].txt.clone();
    let attrs_str = format! ("let attrs = log_attr {{path: \"{}\".to_string(), size: {} }};{nl}", extract_log_attrs.path, extract_log_attrs.size );
    let deps = format! ("{deps}{nl}use rst_lex::lex::log_the_var;{nl}use rst_lex::faav::log_attr;{nl}{attrs_str}");
    fn_ln_by_ln[0].txt = deps;
    leave_file_mark ("/tmp/steps", "");
    let mut complex_var_ending = Vec::<String>::new();
    let fn_ln_by_ln_len = fn_ln_by_ln.len();
    dbg! (&fn_ln_by_ln_len);
    let mut nested_depth: usize = 0;
    for j in 1..fn_ln_by_ln_len {
        let ln = fn_ln_by_ln[j].txt.clone();
        let add_to_log = format! ("{j}: {ln}\nend line {j}\n");
     //   dbg!("check here");
     //   dbg! (&ln);
        if check_proc_macro ( &ln ) { } /* MUST BE MORE SOPHISTICATED HANDLING */
        let categorize_var =  var_expr_or_not (&ln);
        dbg! (&categorize_var);
        dbg! (&complex_var_ending);
        add_file_mark_to ("/tmp/steps", &add_to_log);
        match categorize_var {
            type_of_vars_expr::not (other) => {control_nested_blocks ( &other, &mut nested_depth );},
            type_of_vars_expr::simple => { fn_ln_by_ln[j].txt = make_simple_var_logged (j, &ln);},
            type_of_vars_expr::simple_let => { fn_ln_by_ln[j].txt = make_simple_var_logged (j, &ln);},
            type_of_vars_expr::complex => {
                dbg! ("complex"); nested_depth.inc();
                let item = make_complex_var_logged (&ln);
                if item.is_empty () {continue}
                complex_var_ending.push (item);
                dbg! (&complex_var_ending);
            }
        }
        close_complex_var (j, &mut fn_ln_by_ln, &mut complex_var_ending, &mut nested_depth );
      //  dbg! (&fn_ln_by_ln[j].txt);
    }
    let mut ret = String::new ();
    for iter in fn_ln_by_ln {
        ret.push_str( iter.txt.trim());
    }
  //  ret = ret.replace("\n", "");
    leave_file_mark ("/tmp/log_func", &ret);
    return ret
}
pub fn make_simple_var_logged (ln_num: usize, expr: &String) -> String {
    let expr = expr.trim().strn();
    let nl = char::from_u32(0x0a).unwrap();
   // dbg! (&expr);
   dbg! ("msvl");
    let var_name = extract_var_name (&expr, 417);
    dbg! ("msvl");
    dbg! (&var_name);
    let value = format! ("let __88value__359 = format! (\"{{:?}}\", {} );", var_name);
    let ln_num = ln_num + 1;
    let log_ins = format! ("{nl}{value};{nl}log_the_var({ln_num}, \"{var_name}\", &__88value__359, &attrs);");
    let logged_ln = format! ("{expr}{log_ins}");
    return logged_ln
}
pub fn make_complex_var_logged( expr: &String) -> String {
    let expr = expr.trim().strn();
    let nl = char::from_u32(0x0a).unwrap();
    //dbg! (expr);
    dbg! ("mcvl");
    let var_name = extract_var_name (&expr, 351);
    dbg! ("mcvl");
    dbg! (&var_name);
    let value = format! ("let __88value__359 = format! (\"{{:?}}\", {} );", var_name);
    let ln_num = "__ln_num__";
    let log_ins = format! ("{nl}{value};{nl}log_the_var({ln_num}, \"{var_name}\", &__88value__359, &attrs);");
    let logged_ln = format! ("{log_ins}");
    return logged_ln
}
pub fn close_complex_var (ln_num: usize, lines: &mut Vec <rExpr>, endings: &mut Vec <String>, depth: &mut usize ) {
    let end = _7block_ending ( &lines [ln_num].txt );
    dbg! (&end);
    if !end { dbg! (&lines [ln_num].txt); return }
    if endings.len() <= *depth { depth.dec (); return }
    let expr = endings.last();
    dbg! (&expr);
    let expr = if let Some (x) = expr { x.clone() } else { return };
    let ln = lines [ln_num].txt.clone();
    let ln = ln.trim();
    /*let last_indx_in_ln = ln.chars().count () - 1; 
    let token = ln.chars().nth ( last_indx_in_ln );
    if token != Some (';') { return };
    let token = ln.chars().nth ( last_indx_in_ln - 1);
    if token != Some ('}') { return };*/
    let ln_num_str = ln_num.to_string();
    let expr = expr.replace ("__ln_num__", &ln_num_str).strn();
    dbg! (&ln);
    let expr = format! ("{ln};\n{expr}");
    depth.dec();
    dbg! (&expr);
    lines [ ln_num ].txt = expr.clone(); let _ = endings.pop ();
}
pub fn log_the_var (ln_num: usize, var_name: &str, value: &str, attrs: &log_attr ) {
    static mut depth: u8 = 0;
    let strn_to_log = format! ("__{ln_num}. {var_name}: {value} ");
    let _depth = unsafe { depth };
    let log_file = attrs.path.clone();
     if !std::path::Path::new(&log_file).exists(){dbg! (&log_file); let mut filo = std::fs::File::create_new (&log_file).unwrap(); dbg! (&filo); let _ = filo.write_all (" ".as_bytes());}
    let mut log_file = get_file_append (&log_file);
    if log_file.is_err() {
        unsafe {depth += 1} return log_the_var (ln_num, var_name, value, attrs)
    }
    let err_msg = "failed to check log size".strn();
    let err_set_len = "failed to set log size in 0".strn();
    let cur_file_len = metadata(&attrs.path).expect(&err_msg).len() as usize;
    if cur_file_len > attrs.size {set_file_size (&mut log_file, 0);}
    //dbg! (&cur_file_len);
    //dbg!(&attrs.path);
    let _ = log_file.expect("log_the_var failed").write_all(strn_to_log.as_bytes());
}
pub fn check_let (expr: &String) -> bool {
    let token = "let";
    if expr.trim_start().substring (0, token.chars().count () ) == token { return true }
    let token = "static";
    if expr.trim_start().substring (0, token.chars().count () ) == token { return true }
    return false
}
pub fn check_proc_macro (expr: &String) -> bool {
    let token = "#";
    if expr.trim_start().substring (0, token.chars().count () ) == token { return true }
    return false
}
pub fn extract_var_name (expr: &String, func_id: usize) -> String {
    if expr.is_empty() { return "".strn()}
    let err_msg = format! ("Expression {expr} has no var name, func_id: {func_id}");
    let (mut var_name, _) = split_once_or_ret_null_strns (expr, "=");
    if var_name == "" {dbg! (&expr); panic! ("{err_msg}");}
    let (var_name_, _) = split_once_or_ret_null_strns (&var_name, ":");
    if var_name_.len() > 0 { var_name = var_name_; }
    var_name = tail_trim_var (&var_name);
    var_name = var_name
                .trim ()
                .trim_start_matches (";")
                .trim ()
                .trim_start_matches ("static mut")
                .trim_start_matches ("static")
                .trim ()
                .trim_start_matches ("let mut")
                .trim_start_matches ("let")
                .trim ()
                .trim_start_matches ("mut")
                .trim ()
             //   .trim_start_matches ("} ") // dirty fix
               // .trim_start_matches (") ")
                .strn(); 
    var_name = trim_var (&var_name);
    dbg! (&var_name);
    if wrong_name_of_var (&var_name) {dbg! (&var_name); panic! ("Wrong var name {var_name} expr: {expr}.");}
    return var_name
}
pub fn var_expr_or_not (expr: &String) -> type_of_vars_expr {
    //let expr = expr.replace ("\n", "").trim().strn();
    if  *expr == ";}" { return type_of_vars_expr::not (code_ln::exit_block )  }
    if  *expr == ";\n}" { return type_of_vars_expr::not (code_ln::exit_block) } // MUST BE MORE DETAILED
    if  check_proc_macro (expr) { return type_of_vars_expr::not (code_ln::proc_macro) }
    if eqeq (expr) {dbg! ("category eqeq"); return type_of_vars_expr::not (code_ln::enter_block) }
    let mut ret_curly = stream_sieving3 (&expr, "=", 0, "{" );
    dbg! (&ret_curly);
    //if ret_curly.is_none () { dbg! ("category curly"); return type_of_vars_expr::not }
    let mut ret = stream_sieving3 (&expr, "=", 0, ";" );
    if ret.is_none() && ret_curly.is_none () { return type_of_vars_expr::not (code_ln::perhaps_error) }
    let mut block_ = blocks::new();
    //  compile_error!("gggggggggggg");
    let ret_len = if let Some (_ret) = ret.as_ref() {_ret.txt.len()} else {0 };
    let ret_curly_len = if let Some (_ret) = ret_curly.as_ref() {_ret.txt.len()} else {usize::MAX };
    if  ret_len >= ret_curly_len { ret_curly = None}
    dbg! (&ret_len);
    dbg! (&ret_curly_len);
    dbg! (&ret_curly);
    let tst_var = extract_var_name (&expr, 203);
    dbg! (&tst_var);
    let nolog = "nolog_";
    if tst_var.substring (0, 6) == nolog { return type_of_vars_expr::not (code_ln::simple) }
    if  wrong_symb_in_var (&tst_var ){dbg! ("category wrong symb"); dbg! (&tst_var); return type_of_vars_expr::not (code_ln::perhaps_error) }
    if ret_curly.is_some() {
        blocks_status( Some (&'{' ), Some (&mut block_) );
        return type_of_vars_expr::complex }
    let ret_is_some = ret.is_some();
    if ret_is_some && check_let (&expr) {return type_of_vars_expr::simple_let }
    if ret_is_some {return type_of_vars_expr::simple }
    dbg! ("end var_expr_or_not");
    return type_of_vars_expr::not (code_ln::perhaps_error)
}
pub fn set_file_size (handle: &mut Result <std::fs::File, ErrorKind >, size: usize) {
    let err_set_len = "failed to set log size in 0".strn();
    let mut _handle: *mut Result <std::fs::File, ErrorKind > = handle;
    unsafe {
        let mut deref = &*_handle;
        if let Ok (file) = deref {
            file.set_len(size as u64); return
        } panic! ("{err_set_len}");
    }
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
    let mut block_ = blocks::new ();
    let mut end = line;
 //   dbg! (&stop_token);
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
        if  blocks_status ( Some (&ch), Some (&mut block_) ) || (!maybe.is_empty() && token.as_str().substring (0, maybe.chars().count()) != maybe ) {maybe.clear (); }
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
       if stop_token.as_str().substring (0, maybe.chars().count()) != maybe {maybe.clear(); }
       txt.push(ch);
      if blocks_status ( Some (&ch ), Some (&mut block_) ) {maybe.clear(); }
      maybe.push(ch);
      if maybe.chars().count() == stop_token_len {
         if maybe == *stop_token { 
         break; }
      }
    }
    if maybe == "{" {dbg! (stream); dbg! (&maybe); }
    if maybe != *stop_token { dbg! ("failed stop_token"); dbg! (&stop_token); dbg! (&maybe); dbg!(stream); return None }
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
pub fn stream_cleanup1 (stream: &String, token: String, run_from: usize, stop_token: String) -> Option <String> {
    return stream_cleanup (stream, &token, run_from, &stop_token)
}
pub fn stream_cleanup2 (stream: &String, token: &str, run_from: usize, stop_token: &str) -> Option <String> {
    return stream_cleanup (stream, &token.to_string(), run_from, &stop_token.to_string() )
}
pub fn stream_cleanup3 (stream: &str, token: &str, run_from: usize, stop_token: String) -> Option <String>  {
    return stream_cleanup (&stream.strn(), &token.to_string(), run_from, &stop_token.to_string() )
}
#[inline]
pub fn stream_cleanup (stream: &String, token: &String, run_from: usize, stop_token: &String) -> Option <String>  {
    if stream.is_empty () { return None}
    let fn_name = "stream_cleanup".strn();
    let mut block_ = blocks::new ();
    let mut ret = String::new ();
    let mut entry: usize = 0;
 //   dbg! (&stop_token);
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
        maybe.push(ch);
        txt_dbg.push( ch );
       // println! ("{txt_dbg}");
        if maybe.chars().count() == token_len {
            if maybe == *token {
                leave_file_mark ("/tmp/mayb", &maybe.to_string() );
                entry = if j > token_len { j - token_len + 1 } else { j };
                break;
            }
        }
        if  not_curly_blocks_status ( Some (&ch), Some (&mut block_) ) || (!maybe.is_empty() && token.as_str().substring (0, maybe.chars().count()) != maybe ) 
                                                                                                                {ret.push_str(maybe.as_str() ); maybe.clear (); }
    }
    //println! ("{line}, {maybe}");
    //leave_file_mark ("/tmp/entry1", &entry1.to_string() );
    leave_file_mark ("/tmp/may", &maybe.to_string() );
    if maybe.is_empty() { return None }
    maybe.clear();
    let mut write_char_or_not = false;
    let run_from = entry + token_len;
    let mut txt = token.clone();
    for j in run_from..to_stream_len {
       let ch = stream.chars().nth (j).unwrap ();
      if !write_char_or_not {
        if stop_token.as_str().substring (0, maybe.chars().count()) != maybe {maybe.clear(); }
        if not_curly_blocks_status ( Some (&ch ), Some (&mut block_) ) {maybe.clear(); }
        maybe.push(ch);
        if maybe.chars().count() == stop_token_len {
            if maybe == *stop_token { 
            write_char_or_not = true; }
        } continue;
      } ret.push (ch);
    }
    if maybe == "{" {dbg! (stream); dbg! (&maybe); }
    if maybe != *stop_token { dbg! ("failed stop_token"); dbg! (&stop_token); dbg! (&maybe); dbg!(stream); return None }
  //  dbg! (stream);
    //dbg! (&txt);
    //dbg! (&fn_name);
   // println! ("{}", txt);
    return Some (ret );
}
pub fn sieve_n_split1 (stream: &String, token: String, run_from: usize, stop_token: String) -> (Option < rExpr >, String) {
    return sieve_n_split (stream, &token, run_from, &stop_token)
}
pub fn sieve_n_split2 (stream: &String, token: &str, run_from: usize, stop_token: &str) -> (Option < rExpr >, String) {
    return sieve_n_split (stream, &token.to_string(), run_from, &stop_token.to_string() )
}
pub fn sieve_n_split3 (stream: &str, token: &str, run_from: usize, stop_token: &str) -> (Option < rExpr >, String) {
    return sieve_n_split (&stream.strn(), &token.to_string(), run_from, &stop_token.to_string() )
}
#[inline]
pub fn sieve_n_split (stream: &String, token: &String, run_from: usize, stop_token: &String) -> (Option < rExpr >, String) {
    if stream.is_empty () { return (None, "".strn() )}
    let fn_name = "stream_sieving".strn();
    let mut line: usize = 0;
    let mut column = line;
    let mut entry: usize = 0;
    let mut entry1: *mut usize = &mut entry;
    let mut end = line;
    let mut block_ = blocks::new();
  //  dbg! (&stop_token);
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
       if  not_curly_blocks_status ( Some (&ch), Some (&mut block_) ) || (!maybe.is_empty() && token.as_str().substring (0, maybe.chars().count()) != maybe ) {maybe.clear (); }
        //dbg! ("blocks_status_not_curly");
        //dbg! (blocks_status_not_curly (None, Some (&mut block_) ));
        if maybe.chars().count() == token_len {
            if maybe == *token {
                leave_file_mark ("/tmp/mayb", &maybe.to_string() );
                entry = if j > token_len { j - token_len + 1 } else { j };
                leave_file_mark ("/tmp/entry0", &entry.to_string() ); 
                break;
            }
        }
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
    if maybe.is_empty() { return (None, stream.clone() ) }
    maybe.clear();
    let mut run_from = entry + token_len;
    let mut txt = token.clone();
    for j in run_from..to_stream_len {
       let ch = stream.chars().nth (j).unwrap ();
       run_from = j;
       maybe.push(ch);
       txt.push(ch);
       //dbg! (&txt);
       //dbg! (&maybe);
      if not_curly_blocks_status ( Some (&ch ), Some (&mut block_) ) || stop_token.as_str().substring (0, maybe.chars().count()) != maybe {maybe.clear(); }//continue; };
     // dbg! (&maybe);
      if maybe.chars().count() == stop_token_len {
       // dbg! (&maybe);
         if maybe == *stop_token { dbg! (&maybe);  break; }
      }
    }
    run_from.inc();
    end = entry + txt.chars().count (); let mut out = String::new();
    for j in run_from..to_stream_len {
        out.push (stream.chars().nth(j).unwrap() );
    }
  //  dbg! (stream);
    //dbg! (&txt);
    //dbg! (&fn_name);
   // println! ("{}", txt);
   leave_file_mark ("/tmp/stop_token", &stop_token);
   leave_file_mark ("/tmp/1st", &txt);
  // panic! ("sieve_n_split");
    let ret = (Some (
        rExpr {
            txt,
            line,
            column,
            entry,
            end
        }
    ), out );
  //  dbg! (&ret);
    return ret
}

pub fn blocks_status (ch: Option < &char >, ext: Option < &mut blocks > ) -> bool {
    if let Some ( x ) = ext {
        if ch.is_none () { return x.state }
        let ch = ch.unwrap();
        match *ch {
            '{' => {x.curly.inc(); }, 
            '}' => {x.curly.dec(); },
            '(' => {x.round.inc(); },
            ')' => {x.round.dec(); }, 
            '[' => {x.square.inc(); }, 
            ']' => {x.square.dec(); },
            '\"' => {x.cite += 1; x.cite %= 2; },
            _ => {}
        }
        let sum = x.curly + x.round + x.square + x.cite;
        if sum == 0 { x.state = false;} else { x.state = true; } 
        return x.state
    }
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
pub fn not_curly_blocks_status (ch: Option < &char >, ext: Option < &mut blocks > ) -> bool {
    if let Some ( x ) = ext {
        if ch.is_none () { return x.state }
        let ch = ch.unwrap();
        match *ch {
            '(' => {x.round.inc(); },
            ')' => {x.round.dec(); }, 
            '[' => {x.square.inc(); }, 
            ']' => {x.square.dec(); },
            '\"' => {x.cite += 1; x.cite %= 2; },
            _ => {}
        }
        let sum = x.round + x.square + x.cite;
      //  dbg! (&sum);
        if sum == 0 { x.state = false;} else { x.state = true; } 
        dbg! (&x);
        return x.state
    }
    static mut round: u64 = 0;
    static mut square: u64 = 0;
    static mut cite: u64 = 0;
    static mut state: bool = false;
    unsafe {
        if ch.is_none () { return state }
        let ch = ch.unwrap();
        match *ch {
            '(' => {round.inc(); },
            ')' => {round.dec(); }, 
            '[' => {square.inc(); }, 
            ']' => {square.dec(); },
            '\"' => {cite += 1; cite %= 2; },
            _ => {}
        }
        let sum = round + square + cite;
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
pub fn add_file_mark_to (nm: &str, msg: &str){
    let mut file =get_file_append (&nm.strn() ).expect("Unable to create file [add_file_mark_to]");
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
pub fn wrong_name_of_var (tst: &String) -> bool {
    //let tst = extract_var_name (tst);
    if tst.is_empty () { return true }
    dbg! (&tst);
    match tst.trim() {
        "let"|"for"|"while"|"if"|"mut" => return true,
        _ => return false,
        }
}
pub fn wrong_symb_in_var (tst: &String) -> bool {
    //let tst = extract_var_name (tst);
    if tst.is_empty () { return true }
    dbg! (&tst);
    for b in tst.chars() {
        if !stop_wrong_symb (b) {return false }
    } return true
}
pub fn stop_wrong_symb (tst: char) -> bool {
    //let tst = extract_var_name (tst);
    dbg! (&tst);
    match tst {
        ':'|'\''|'\"'|','|'['|'(' => return true,
        _ => return false,
    }
}
pub fn wrong_symb (tst: char) -> bool {
    //let tst = extract_var_name (tst);
    dbg! (&tst);
    match tst {
        '\n'|'+'|'-'|'*'|'/'|' '|'}'|'{' => return true,
        _ => return false,
    }
}
pub fn eqeq (expr: &String ) -> bool {
    if expr.find ("==").is_some() { return true }
    if expr.find ("<=").is_some() { return true }
    if expr.find (">=").is_some() { return true }
    if expr.find ("!=").is_some() { return true }
    if expr.find ("=>").is_some() { return true }
    if expr.find ("if let").is_some() { return true }
    if expr.find ("while let").is_some() { return true }
    return false
}
pub fn trim_var (var: &String) -> String {
    let mut ret = String::new();
    for c in var.chars () {
        if wrong_symb (c ) { continue }
        if stop_wrong_symb (c) { panic! ("Dear Dev, Can't trim variable - it contains very wrong symb {c} in {var}")}
        ret.push (c);
    } return ret
}
pub fn tail_trim_var (var: &String) -> String {
    let mut writeIt = false;
    let mut ret = String::new();
    for c in var.chars () {
        if !wrong_symb (c ) && !stop_wrong_symb (c) { writeIt = true }
        if writeIt { ret.push (c); }
    } return ret
}
pub fn _7block_ending (expr: &String) -> bool {
    let mut block_ = blocks::new ();
    for c in expr.chars() {
        if not_curly_blocks_status ( Some (&c), Some (&mut block_ ) ) { continue }
        if c == '}' { return true }
    } return false
}
//fn 
//clear;cargo build --no-default-features --features in_dbg --features=mae --features=tst_macro --features=tam
