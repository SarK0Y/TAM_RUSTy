use num_traits::bounds;

use crate::{bkp_tmp_dir, save_file, save_file_abs_adr, parse_replace, _ext_msgs, popup_msg, globs18::drop_key, getkey, 
  cached_data, checkArg, get_arg_in_cmd, stop_term_msg, free_term_msg, custom_traits::STRN};
use std::collections::{HashMap, hash_map::Entry};
#[derive(Default)]
#[derive(Clone)]
pub(crate) struct basic{
    shol_state: bool,
    swtch_shols: bool,
    mk_shol_state: u64,
    pub tmp_dir: String,
    shols: Vec<(String, String)>,
    rec_shol: (String, String),
    ext_old_modes: crate::_ext_msgs,
    pub cache: HashMap<String, HashMap<usize, Vec<String>>>,
    pub cache_state: bool,
    pub cache_window: usize,
    pub seg_size: usize,
}
impl basic{
  pub fn new() -> Self{
    let mut new_cache_window = 5000usize;
    if checkArg("-cache-window"){
      let val = String::from_iter(get_arg_in_cmd("-cache-window").s).trim_end_matches('\0').to_string();
      let val: usize = match usize::from_str_radix(&val, 10){
        Ok(i) => i,
        _ => 0
      };
      if val > 0{new_cache_window = val}
     }
     let mut seg_size_new_strn = "".to_string(); 
     let mut seg_size_new = 150usize;
     if checkArg("-cache-seg-size"){
            seg_size_new_strn = String::from_iter(get_arg_in_cmd("-cache-seg-size").s).trim_end_matches('\0').to_string();
            let ret = crate::globs18::strn_2_usize(seg_size_new_strn);
            if ret != None{seg_size_new = ret.unwrap()}
        }
    Self{
        shol_state: false,
        swtch_shols: false,
        mk_shol_state: 0,
        tmp_dir: bkp_tmp_dir(None, false),
        shols: Vec::new(),
        rec_shol: (String::new(), String::new()),
        ext_old_modes: _ext_msgs::new(),
        cache: HashMap::with_capacity(new_cache_window),
        cache_state: true,
        cache_window: new_cache_window,
        seg_size: seg_size_new,
    }
}
pub fn default() -> Self{
    basic::new()
}
pub fn build_page(&mut self, ps: &mut crate::_page_struct){basic::build_page_(self, ps)}
/*cache */
pub fn rec_from_cache(&mut self, key: &String, indx: usize) -> (String, cached_data){
  if self.cache_state == false{return ("".strn(), cached_data::no_list) }
  basic::pg_rec_from_cache(&mut self.cache, key, indx)}
pub fn rec_to_cache(&mut self, key: String, val: String){basic::pg_rec_to_cache(&mut self.cache, &key, &val)}
pub fn null_cache(&mut self, key: &String){basic::pg_0_cache(&mut self.cache, &key)}
pub fn rec_from_front_list(&mut self, indx: i64, fixed_indx: bool) -> String{basic::pg_rec_from_front_list(self, indx, fixed_indx)}
 /* shols */ pub fn set_shol_state(&mut self, new_state: bool){self.shol_state = new_state;}
  pub fn get_shol_state(&self) -> bool{self.shol_state}
  pub fn inc_mk_shol_state(&mut self){self.mk_shol_state += 1;}
  pub fn get_mk_shol_state(&self) -> u64{self.mk_shol_state}
  pub fn set_rec_shol(&mut self, rec: &(String, String)){self.rec_shol = (self.rec_shol.0.clone(), self.rec_shol.1.clone());}
  pub fn get_rec_shol(&self) -> (String, String){(self.rec_shol.0.clone(), self.rec_shol.1.clone())}
  pub fn clear_rec_shol(&mut self){self.rec_shol = (String::new(), String::new());}
  pub fn clear_shols(&mut self){self.shols.clear()}
  pub fn rec_from_shols(&self, indx: usize) -> (String, String){let len = self.shols.len(); if len < indx{return ("too high indx".to_string(), "".to_string())}
      (self.shols[indx].0.clone(), self.shols[indx].1.clone())
  }
  pub fn rm_rec_from_shols(&mut self, indx: usize){
    if self.shols_len() <= indx{return;}
    self.shols.remove(indx);
  }
  pub fn add_rec_to_shols(&mut self, rec: (String, String)){
    let rec0 = (rec.0.clone(), rec.1.clone());
    self.shols.push(rec);
    if self.shols[self.shols.len() - 1] != rec0{panic!("add_rec_to_shols failed")}
  }
  pub fn shols_len(&self) -> usize{
    self.shols.len()
  }
  pub fn prnt_shols(&self){
    let len = self.shols_len();
    for j in 0..len{
      let rec = self.rec_from_shols(j);
      let line = format!("{j}: {}; {}", rec.0, rec.1);
      println!("{line}");
    }
    getkey();
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
        crate::clear_screen();
        let mut ps: crate::_page_struct = unsafe {crate::swtch::swtch_ps(-1, None)};
        let mut data = "".to_string();
        let num_pg = crate::get_num_page(-55541555121);
        let num_pgs = crate::where_is_last_pg();
        crate::swtch::print_viewers();
        crate::swtch::print_pg_info();
        if num_pg < num_pgs {self.build_page( &mut ps);}
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
  if Key == ""{crate::set_ask_user("Hi there, Dear User. So good to C You again 💯❤️", -9147019413);}
  #[cfg(feature="in_dbg")]
  if self.read_file("ext_key_modes") == "y" || crate::breaks("ext key modes", 1, true).1 && crate::breaks("ext key modes", 1, true).0 == 1{
    println!("break ext_key_modes");
  }
  Key.push_str(&crate::getkey());
  if self.ext_old_modes.drop_dontPass_after_n_hotKeys > 0{
    if self.ext_old_modes.hotKeys_got_hits == 0{self.ext_old_modes.dontPass = false}
  }
  if crate::globs18::eq_ansi_str(&crate::kcode01::Alt_min, Key.as_str()) == 0 {
    if !self.swtch_shols{self.from_shol()}
    else {self.to_shol()}
    //self.prnt_shols();
    self.ext_old_modes.dontPass = true;
    self.swtch_shols = !self.swtch_shols;
    return drop_key(Key, &mut Some(&mut self.ext_old_modes));
  }
  let ansiKey: u8 = match Key.as_str().bytes().next(){
        Some(val) => val,
        _ => 0
    };
  if Key == "/"{crate::key_slash(); self.to_shol(); return crate::hotKeys(Key, &mut Some(&mut self.ext_old_modes))}
  if crate::kcode01::ENTER == ansiKey{
    crate::pre_Enter();
    self.from_shol_no_dead_ends();
    return crate::hotKeys(Key, &mut Some(&mut self.ext_old_modes))}
  if Key != "<" && self.mk_shol_state > 0{self.mk_shol_state = 0; self.ext_old_modes.dontPass = false}
  if Key == "<" && self.mk_shol_state == 2{self.mk_shol_state = 0; self.mk_shol(self.shols.len()); self.ext_old_modes.dontPass = false; return drop_key(Key, &mut Some(&mut self.ext_old_modes))}
  //if Key == "@" && self.mk_shol_state == 2{self.mk_shol_state += 1; }
  if Key == "<" && self.mk_shol_state == 1{self.mk_shol_state += 1; }
  if Key == "<" && self.mk_shol_state == 0{self.mk_shol_state = 1; self.ext_old_modes.dontPass = true; }
  let mut ret_tag = self.prevalidate_tag();
  if ret_tag == Some("dontPass".to_string()){stop_term_msg(); }//return crate::hotKeys(&mut "dontPass".to_string(), &mut Some(&mut self.ext_old_modes))}
  if self.shol_state && Key == " "{
    self.shol_state = false;
    use crate::parse_replacing::parse_replace;
    let mut ret_tag = self.validate_tag();
    if ret_tag == None {self.clear_rec_shol()}
    return crate::globs18::drop_key(Key, &mut Some(&mut self.ext_old_modes));
  }
  if self.shol_state{
    let shol = format!("{}/shol", self.tmp_dir);
    crate::save_file_append_abs_adr(Key.to_string(), shol);
    self.rec_shol.0.push_str(Key.as_str());
    /*self.ext_old_modes.drop_dontPass_after_n_hotKeys = 1;*/ self.ext_old_modes.dontPass = true;
    return crate::hotKeys(Key, &mut Some(&mut self.ext_old_modes));
  }
  if Key == "#" || Key == "@"{
    self.clear_rec_shol();
    self.rec_shol.0.push_str(Key.as_str());
    let shol = format!("{}/shol", self.tmp_dir);
    save_file_abs_adr(Key.to_string(), shol);
    self.shol_state = true;
    return crate::hotKeys(Key,&mut Some(&mut self.ext_old_modes));
  }
  //self.ext_old_modes.dontPass = false;
  crate::hotKeys(Key, &mut Some(&mut self.ext_old_modes))
 }
}
pub(crate) fn tst_basic() -> basic{
    basic::new()
}