#[derive(Default, Clone, PartialEq)]
pub struct _ext_msgs{
    pub new_mode: bool,
    pub dontPass: bool,
    pub drop_dontPass_after_n_hotKeys: u8,
    pub hotKeys_got_hits: u8
}
pub trait  Default {
   fn default() ->Option<&'static mut Self>;
}
impl Default for &mut _ext_msgs {
    fn default() -> Option< &'static mut Self>{None}
}
impl _ext_msgs {
    pub fn new() -> _ext_msgs{
        Self{
            new_mode: false,
            dontPass: false,
            drop_dontPass_after_n_hotKeys: 0,
            hotKeys_got_hits: 0
        }
    }

   pub fn dec_hotKeys_got_hits(&mut self){
        if self.hotKeys_got_hits < self.drop_dontPass_after_n_hotKeys{self.hotKeys_got_hits += 1; return;}
        self.drop_dontPass_after_n_hotKeys = 0; self.hotKeys_got_hits = 0;
    }
}
impl AsRef<_ext_msgs> for _ext_msgs{
    fn as_ref(&self) -> &Self{
        self
    } 
}
impl AsMut<_ext_msgs> for _ext_msgs{
    fn as_mut(&mut self) -> &mut Self{
        self
    } 
}