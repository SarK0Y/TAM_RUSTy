pub trait parse_replace{
    fn validate_tag(&mut self) -> Option<String>;
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

}