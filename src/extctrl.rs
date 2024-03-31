#[derive(Default)]
#[derive(Clone)]
pub(crate) struct basic{
    shol_state: bool
}
impl basic{
  pub fn new() -> Self{
    Self{
        shol_state: false
    }
}
pub fn default() -> Self{
    basic::new()
}
  pub fn set_shol_state(&mut self, new_state: bool){
    self.shol_state = new_state;
  }
  pub fn get_shol_state(&mut self, new_state: bool){
    self.shol_state = new_state;
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
  crate::hotKeys(Key, true)
 }
}
pub(crate) fn tst_basic() -> basic{
    basic::new()
}