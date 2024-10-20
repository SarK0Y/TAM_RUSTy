/* example
#[cfg(feature="in_dbg")]
    if crate::breaks("break patch", 1, true).0 == 1 && crate::breaks("break patch", 1, true).1{
        println!("break patch 1")
    }
 */
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use once_cell::sync::Lazy; use substring::Substring;

pub(crate) fn breaks(name: &str, id: u64, justRet: bool) -> (u64, bool){
    static mut br: Lazy<HashMap<String, u64>> = Lazy::new(||{HashMap::new()});
    if name.substring(0, 2) == "d:"{
        let name = name.replace("d:", "").trim_end().trim_start().to_string();
       crate::C!(br.remove(&name)); return (0, false);
    }

    match crate::C!(br.entry(name.to_string())){
        Entry::Occupied(entry) => {return (*entry.get(), true)},
        Entry::Vacant(entry) => {if !justRet {entry.insert(id); println!("inserted {}", unsafe{br.get(&name.to_string()).unwrap_or(&u64::MAX)}); crate::getkey();} return (id, false)}
    }
}
pub(crate) fn manage_breaks(cmd: &String){
    let cmd = cmd.replace("br:", "").trim_start_matches(' ').to_string();
    let (name, id) = crate::split_once(cmd.as_str(), "::");
    let name = name.trim_end_matches(' ').trim_start_matches(' ').to_string(); let id = id.trim_end().trim_start().to_string();
    println!("id: {id}");
    let id = match crate::globs18::strn_2_u64(id){Some(id) => id, _ => return crate::errMsg0("Example: br:my mark::num")};
    breaks(&name, id, false);
}
pub(crate) fn just_break(){
    println!("Just break")
}
pub(crate) fn report(msg: &String, mark: &str){
    static mut msgs: Lazy<Vec<String>> = Lazy::new( ||{ Vec::new() } );
    if unsafe{mark.is_empty()} {
        let len = {crate::C!(msgs.len())}; 
        for v in 0..len{
            if unsafe{msgs[v].substring(0, mark.len())} == mark{
                println!("{}", crate::C!(msgs[v].clone()))
            }
        } crate::dont_scrn_fix(true); crate::getkey(); return
    } unsafe{msgs.push(format!("{mark}: {msg}"))};
}
// fn