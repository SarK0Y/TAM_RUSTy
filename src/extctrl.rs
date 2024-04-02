use crate::{bkp_tmp_dir, save_file, save_file_abs_adr, parse_replace};
#[derive(Default)]
#[derive(Clone)]
pub(crate) struct basic{
    shol_state: bool,
    mk_shol_state: bool,
    tmp_dir: String,
    shols: Vec<(String, String)>,
    rec_shol: (String, String)
}
impl basic{
  pub fn new() -> Self{
    Self{
        shol_state: false,
        mk_shol_state: false,
        tmp_dir: bkp_tmp_dir(),
        shols: Vec::new(),
        rec_shol: (String::new(), String::new())
    }
}
pub fn default() -> Self{
    basic::new()
}
  pub fn set_shol_state(&mut self, new_state: bool){
    self.shol_state = new_state;
  }
  pub fn get_shol_state(&self) -> bool{
    self.shol_state
  }
  pub fn set_mk_shol_state(&mut self, new_state: bool){
    self.mk_shol_state = new_state;
  }
  pub fn get_mk_shol_state(&self) -> bool{
    self.mk_shol_state
  }
  pub fn set_rec_shol(&mut self, rec: &(String, String)){
    self.rec_shol = (self.rec_shol.0.clone(), self.rec_shol.1.clone());
  }
  pub fn get_rec_shol(&self) -> (String, String){
    (self.rec_shol.0.clone(), self.rec_shol.1.clone())
  }
  pub fn clear_rec_shol(&mut self){
    self.rec_shol = (String::new(), String::new());
  }
  pub fn clone(&self) -> basic{
    let mut base = basic::default();
    base.shol_state = self.shol_state;
    base
  }
}
pub trait Copy{
  fn Copy(&mut self) -> basic;
}
impl Copy for basic{
  fn Copy(&mut self) -> basic{
    let mut base = basic::default();
    base.shol_state = self.shol_state;
    base.shols = self.shols.clone();
    base.tmp_dir = self.tmp_dir.clone();
    base.rec_shol = (self.rec_shol.0.clone(), self.rec_shol.1.clone());
    base
  }
}
pub trait ManageLists{
  fn manage_pages(&mut self);
  fn custom_input(&mut self, Key: &mut String, ext: bool) -> String;
  fn ext_key_modes(&mut self, Key: &mut String, ext: bool) -> String;
}
impl ManageLists for basic{
  fn manage_pages(&mut self){
    let mut Key: String = "".to_string(); 
    let mut count: u64 = 0;
    let mut bal =String::new();
    loop{
        //set_prnt(&bal, -1);
        let mut ps: crate::_page_struct = unsafe {crate::swtch::swtch_ps(-1, None)};
        let mut data = "".to_string();
        let num_pg = crate::get_num_page(-55541555121);
        let num_pgs = crate::where_is_last_pg();
        crate::swtch::print_viewers();
        crate::swtch::print_pg_info();
        if num_pg < num_pgs || num_pgs ==0 {crate::build_page(&mut ps);}
        println!("{}", crate::get_prnt(-1));
        Key  = "".to_string(); 
        crate::pg18::exec_cmd(self.custom_input(&mut Key, false));
        crate::clear_screen();
    }
}
 fn custom_input(&mut self, Key: &mut String, ext: bool) -> String{
    let mut Key = Key;
    crate::form_cmd_line_default();
    return self.ext_key_modes(&mut Key, ext);
}
 fn ext_key_modes(&mut self, Key: &mut String, ext: bool) -> String{
  Key.push_str(&crate::getkey());
  if Key == " " && self.mk_shol_state{self.mk_shol_state = false;}
  if Key == "@" && self.mk_shol_state{self.mk_shol_state = false; self.mk_shol(); return crate::globs18::drop_key(Key);}
  if Key == "@"{self.mk_shol_state = true}
  if self.shol_state && Key == " "{
    self.shol_state = false;
    use crate::parse_replacing::parse_replace;
    let mut ret_tag = self.validate_tag();
    if ret_tag == None {self.clear_rec_shol()}
    else {
      let mut rec_shol = self.get_rec_shol();
      rec_shol.1.clear(); rec_shol.1.push_str(ret_tag.unwrap().as_str());
      let re_tag = format!("{}{}", self.shols.len(), rec_shol.0);
      rec_shol.0.clear(); rec_shol.0.push_str(re_tag.as_str());
      self.shols.push(rec_shol);
    }
    return crate::globs18::drop_key(Key);
  }
  if self.shol_state{
    let shol = format!("{}/shol", self.tmp_dir);
    crate::save_file_append_abs_adr(Key.to_string(), shol);
    self.rec_shol.0.push_str(Key.as_str());
    return crate::hotKeys(Key, true);
  }
  if Key == "#"{
    self.clear_rec_shol();
    self.rec_shol.0.push_str(Key.as_str());
    let shol = format!("{}/shol", self.tmp_dir);
    save_file_abs_adr(Key.to_string(), shol);
    self.shol_state = true;
    return crate::hotKeys(Key, true);
  }
  crate::hotKeys(Key, true)
 }
}
pub(crate) fn tst_basic() -> basic{
    basic::new()
}