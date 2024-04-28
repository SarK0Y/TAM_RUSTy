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
    let id = crate::globs18::strn_2_u64(id).unwrap();
    breaks(&name, id, false);
}
// fn