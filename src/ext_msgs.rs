#[derive(Default)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct _ext_msgs{
    pub new_mode: bool,
    pub dontPass: bool,
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
            dontPass: false
        }
    }
   /* pub fn default(&mut self) ->Option<&mut Self>{
        None
    }*/
}
/*impl AsRef<_ext_msgs> for _ext_msgs{
    fn as_ref(&self) -> &Self{
        self
    } 
}*/