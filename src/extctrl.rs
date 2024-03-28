pub(crate) struct basic{
    shol_state: bool
}
impl basic{
  pub fn new() -> Self{
    Self{
        shol_state: false
    }
}
  pub fn set_shol_state(&mut self, new_state: bool){
    self.shol_state = new_state;
  }
  pub fn get_shol_state(&mut self, new_state: bool){
    self.shol_state = new_state;
  }
}
pub(crate) fn tst_basic() -> basic{
    basic::new()
}