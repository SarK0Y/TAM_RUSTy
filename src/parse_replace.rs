use crate::{tailOFF, popup_msg, read_tail, repeat_char};

pub trait parse_replace{
    fn validate_tag(&mut self) -> Option<String>;
    fn mk_shol(&mut self, inc_id: usize);
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
    fn mk_shol(&mut self, inc_id: usize) {
    let mut prnt = crate::read_prnt();
    let mut i: usize = 0;
    let mut path = String::new();
    let mut yes_path = false;
    let mut count_ending = 0usize;
    for char0 in prnt.chars(){
        /*let char0 = match prnt.chars().nth(i){
            Some(ch) => ch,
            _ => '@'
        };*/
        if char0 == '/'{yes_path = true;}
        if yes_path{path.push(char0);}
        if char0 == '@' && yes_path && count_ending == 3{break;}
        if char0 == '@'{count_ending += 1}
        i += 1;
    }
   let mut sholName = read_tail(&path, "/");  
   sholName = format!("@{inc_id}@{}", sholName.replace("@ ", ""));
   let rec_shol = (sholName.clone(), path.clone());
   self.set_rec_shol(&rec_shol);
   let mop_count_ending = repeat_char(count_ending + 1, "@");
   prnt = prnt.replace(&path, &sholName); prnt = prnt.replace(&mop_count_ending, "");
   popup_msg(&prnt);
   popup_msg(&path);
   crate::set_prnt(&prnt, -4954038917661);
   popup_msg(&sholName);
    }
    fn to_shol(&mut self){}
    fn from_shol(&mut self){}

}