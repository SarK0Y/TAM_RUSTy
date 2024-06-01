use num_traits::Bounded; use core::mem::size_of;
pub(crate) trait STRN {
    fn strn(&self) -> String;
}
pub(crate) trait STRN_strip {
    fn strip(&self, ch: &str) -> String;
}
impl STRN_strip for str {
    fn strip(&self, ch_strip: &str) -> String{
        let mut ret = "".strn();
        let ch_strip = ch_strip.chars().nth(0);
        for ch in self.chars(){
            if let Some(ch) = ch_strip{continue;}
            ret.push(ch);
        } ret
    }
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
pub(crate) trait arrays {
    fn low_to_high(&mut self) -> &mut Self;
}
impl arrays for [u8] {
    fn low_to_high(&mut self) -> &mut Self {
        for i in 0..self.len() / 2{
            let low = self[i]; let high = self[self.len() - i - 1];
            self[self.len() - i - 1] = low;
            self[i] = high;
        } self
    }
}
pub(crate) trait arr2number {
    fn arr2u64(&mut self) -> u64;
    fn arr2u128(&mut self) -> u128;
}
impl arr2number for [u8]{
    fn arr2u64(&mut self) -> u64 {
        let mut ret = 0u64;
        let len = if size_of::<u64>() > self.len(){self.len()}else{size_of::<u64>()};
        for i in 0..len{
            ret = ret.overflowing_add((self[i] as u64).overflowing_shl((i as u32)*8).0 as u64).0;
        }
        ret
    }
     fn arr2u128(&mut self) -> u128 {
        let mut ret = 0u128;
        let len = if size_of::<u128>() > self.len(){self.len()}else{size_of::<u128>()};
        for i in 0..len{
            let a = self[i];
            ret = ret.overflowing_add((self[i] as u128).overflowing_shl((i as u32)*8).0 as u128).0;
        }
        ret
    }
}
impl arr2number for [u16]{
    fn arr2u64(&mut self) -> u64 {
        let mut ret = 0u64;
        let len = if size_of::<u64>() > self.len(){self.len()}else{size_of::<u64>()};
        for i in 0..len{
            ret = ret.overflowing_add((self[i] as u64).overflowing_shl((i as u32)*16).0 as u64).0;
        }
        ret
    }
    fn arr2u128(&mut self) -> u128 {
        let mut ret = 0u128;
        let len = if size_of::<u128>() > self.len(){self.len()}else{size_of::<u128>()};
        for i in 0..len{
            ret = ret.overflowing_add((self[i] as u128).overflowing_shl((i as u32)*16).0 as u128).0;
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