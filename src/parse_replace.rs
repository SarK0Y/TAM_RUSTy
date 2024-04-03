use crate::{tailOFF, popup_msg, read_tail};

pub trait parse_replace{
    fn validate_tag(&mut self) -> Option<String>;
    fn mk_shol(&mut self, Key: &String);
    fn to_shol(&mut self);
    fn from_shol(&mut self);
}
impl parse_replace for crate::basic{
    fn validate_tag(&mut self) -> Option<String>{
    let mut prnt = crate::read_prnt();
    let mut tag = self.get_rec_shol().0;
    let tag0 = tag.clone();
    tag = tag.replace("##", "");
    let tag = match i64::from_str_radix(&tag, 10){
        Ok(i) => i,
        _ => i64::MIN
    };
    if tag == i64::MIN{
        let msg = format!("Short link {tag0} is wrong");
        prnt = prnt.replace(&tag0, "");
        crate::set_prnt(&prnt, -48721112507);
        crate::update18::fix_screen_count(1);
        return None;
    }
    let tag = crate::get_item_from_front_list(tag, true);
    prnt = prnt.replace(&tag0, &tag);
    crate::set_prnt(&prnt, -48721112507);
    Some(tag)
}
    fn mk_shol(&mut self, Key: &String) {
    let mut prnt = crate::read_prnt();
    let mut i: usize = 0;
    let mut path = String::new();
    let mut yes_path = false;
    let mut count_ending = 0usize;
    loop {
        let char0 = match prnt.chars().nth(i){
            Some(ch) => ch,
            _ => "".chars().nth(0).unwrap()
        };
        if char0 == '/'{yes_path = true;}
        if yes_path{path.push(char0);}
        if char0 == '@' && yes_path && count_ending == 3{break;}
        if char0 == '@'{count_ending += 1}
        i += 1;
    }
   let mut sholName = read_tail(&path, "/");  
   sholName = format!("@@{}", sholName.replace("@@@", ""));
   let rec_shol = (sholName.clone(), path.clone());
   self.set_rec_shol(&rec_shol);
   prnt = prnt.replace(&path, &sholName);
   popup_msg(&path);
   crate::set_prnt(&prnt, -4954038917661);
    }
    fn to_shol(&mut self){}
    fn from_shol(&mut self){}

}