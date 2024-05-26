use num_traits::Bounded; use core::mem::size_of;
pub(crate) trait STRN {
    fn strn(&self) -> String;
}
impl STRN for str {
    fn strn(&self) -> String{
        String::from(self)
    }
}
impl STRN for usize {
    fn strn(&self) -> String{
        self.to_string()
    }
}
impl STRN for u64 {
    fn strn(&self) -> String{
        self.to_string()
    }
}
impl STRN for i64 {
    fn strn(&self) -> String{
        self.to_string()
    }
}
pub(crate) trait helpful_math_ops
{
    fn inc(&mut self) -> Self;
    fn dec(&mut self) -> Self;
}
pub(crate) trait arr2number {
    fn arr2u64(&mut self) -> u64;
}
impl arr2number for Vec<u8>{
    fn arr2u64(&mut self) -> u64 {
        let mut ret = 0u64;
        let len = if size_of::<u64>() > self.len(){self.len()}else{size_of::<u64>()};
        for i in 0..len{
            ret +=(self[i] << i) as u64;
        }
        ret
    }
}
impl helpful_math_ops for usize {
    fn inc(&mut self) -> Self{
       if *self < Self::MAX{*self = *self + 1; return *self;}
       *self
    }
    fn dec(&mut self) -> Self{
       if *self > Self::MIN{*self = *self - 1; return *self;}
       *self
    }
}
impl helpful_math_ops for u64 {
    fn inc(&mut self) -> Self{
       if *self < Self::MAX{*self = *self + 1; return *self;}
       *self
    }
    fn dec(&mut self) -> Self{
       if *self > Self::MIN{*self = *self - 1; return *self;}
       *self
    }
}
impl helpful_math_ops for i64 {
    fn inc(&mut self) -> Self{
       if *self < Self::MAX{*self = *self + 1; return *self;}
       *self
    }
    fn dec(&mut self) -> Self{
       if *self > Self::MIN{*self = *self - 1; return *self;}
       *self
    }
}
impl helpful_math_ops for i32 {
    fn inc(&mut self) -> Self{
       if *self < Self::MAX{*self = *self + 1; return *self;}
       *self
    }
    fn dec(&mut self) -> Self{
       if *self > Self::MIN{*self = *self - 1; return *self;}
       *self
    }
}