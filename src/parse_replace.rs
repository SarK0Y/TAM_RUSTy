use crate::{tailOFF, popup_msg, read_tail, repeat_char, read_prnt, getkey, escape_backslash, escape_apostrophe, escape_symbs, errMsg_dbg0, full_escape, no_esc_lst};

pub trait parse_replace{
    fn validate_tag(&mut self) -> Option<String>;
    fn mk_shol(&mut self, inc_id: usize);
    fn mk_shol_from_strn(&mut self, path: &String, tag_at: bool) -> String;
    fn to_shol(&mut self);
    fn from_shol(&mut self);
    fn from_shol_no_dead_ends(&mut self);
    fn prevalidate_tag(&mut self) -> Option<String>;
}
impl parse_replace for crate::basic{
    fn prevalidate_tag(&mut self) -> Option<String>{
        use substring::Substring;
    let mut prnt = crate::read_prnt();
    if prnt.substring(0, 10) == "term mv %a"{return Some("dontPass".to_string())}
    if prnt.substring(0, 10) == "term cp %a"{return Some("dontPass".to_string())}
    if prnt.substring(0, 12) == "term mv %enu"{return Some("dontPass".to_string())}
    if prnt.substring(0, 12) == "term cp %enu"{return Some("dontPass".to_string())}
    None
    }
    fn validate_tag(&mut self) -> Option<String>{
    let mut prnt = crate::read_prnt();
    let mut tag = self.get_rec_shol().0;
    let tag0 = tag.clone();
    let mut tag_hash =tag.clone();
    let mut tag_at = false;
    tag_hash = tag.replace("##", "");
    if tag_hash.len() != tag.len(){tag_at = true}
    tag = tag.replace("##", "");
    tag = tag.replace("@@", "");
    let tag = match i64::from_str_radix(&tag, 10){
        Ok(i) => i,
        _ => i64::MIN
    };
    if tag == i64::MIN{
        let msg = format!("Short link {tag0} is wrong");
        prnt = prnt.replace(&tag0, "");
        crate::set_prnt(&prnt, -48721112507);
       // crate::update18::fix_screen_count(1);
        return None;
    }
    let tag = crate::get_item_from_front_list(tag, true);
    let tag = self.mk_shol_from_strn(&tag, tag_at);
    prnt = prnt.replace(&tag0, &tag);
    crate::set_prnt(&prnt, -48721112507);
    Some(tag)
}
fn mk_shol_from_strn(&mut self, path: &String, tag_at: bool) -> String{
    let mut prnt = crate::read_prnt();
   let mut sholName = read_tail(&path, "/");  
   let inc_id = self.shols_len();
   if !tag_at {sholName = format!("{inc_id}@@{}", sholName.replace("< ", ""))}
   else {sholName = format!("{inc_id}##{}", sholName.replace("< ", ""))}
   crate::set_prnt(&prnt, -4654038917961);
    let mut path =full_escape(&path);
   let rec_shol = (sholName.clone(), path.clone());
   no_esc_lst(&path, true);
   self.add_rec_to_shols(rec_shol);
   if !tag_at{return sholName}
   path
    }
    fn mk_shol(&mut self, inc_id: usize) {
    let mut prnt = crate::read_prnt();
    let mut path = String::new();
    let mut yes_path = false;
    let mut count_ending = 0usize;
    for char0 in prnt.chars(){
        if char0 == '/'{yes_path = true;}
        if yes_path{path.push(char0);}
        if char0 == '<' && yes_path && count_ending == 3{break;}
        if char0 == '<'{count_ending += 1}
    }
   let mut sholName = read_tail(&path, "/");  
   sholName = format!("@{inc_id}@{}", sholName.replace("< ", ""));
   sholName = sholName.trim_end_matches("<").to_string();
   path = path.trim_end_matches("<").to_string();
   let mop_count_ending = repeat_char(count_ending + 1, "<");
   prnt = prnt.replace(&path, &sholName); prnt = prnt.replace(&mop_count_ending, "");
   crate::set_prnt(&prnt, -4954038917661);
    path = path.trim_end().to_string();
    path = full_escape(&path);
   let rec_shol = (sholName.clone(), path.clone());
   no_esc_lst(&path, true);
   self.add_rec_to_shols(rec_shol);
    }
    fn to_shol(&mut self){
        let mut prnt = crate::core18::raw_read_file("prnt");
        let prnt_len = prnt.chars().count();
        let mut shols_len = self.shols_len();
        if shols_len == 0 {return;}
        for i in 0..shols_len{
            let rec = self.rec_from_shols(i);
            let mut prnt0 = prnt.clone();
            prnt0 = prnt0.replace(&rec.1, &rec.0);
            if prnt.len() != prnt0.len(){prnt = prnt0}
        }
        crate::set_prnt(&prnt, 75094137091728);
    }
    fn from_shol_no_dead_ends(&mut self){
        let mut prnt = read_prnt();
        let prnt_len = prnt.chars().count();
        let mut rm_recs: Vec<usize> = Vec::new();
        let mut shols_len = self.shols_len();
        if shols_len == 0 {return;}
        for i in 0..shols_len{
            let rec = self.rec_from_shols(i);
            let mut prnt0 = prnt.clone();
            prnt0 = prnt0.replace(&rec.0, &rec.1);
            if prnt.len() != prnt0.len(){prnt = prnt0}
        }
        self.clear_shols();
        crate::set_prnt(&prnt, 75094137091728);
    }
fn from_shol(&mut self){
        let mut prnt = read_prnt();
        let prnt_len = prnt.chars().count();
        let mut shols_len = self.shols_len();
        if shols_len == 0 {return;}
        for i in 0..shols_len{
            let rec = self.rec_from_shols(i);
            let mut prnt0 = prnt.clone();
            prnt0 = prnt0.replace(&rec.0, &rec.1);
            if prnt.len() != prnt0.len(){prnt = prnt0}
        }
        crate::set_prnt(&prnt, 75094137091728);
    }
}