use num_traits::Bounded;
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
pub(crate) trait helpful_math_ops <T> where T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::cmp::PartialOrd + std::cmp::Ord
                                                {
    fn inc(&self) -> Option<T>;
    fn dec(&self) -> Option<T>;
}
pub(crate) trait MinMax{
    const MAX: Self;
    const MIN: Self;
}
impl dyn helpful_math_ops<usize> {
    fn inc(&self) -> Option<usize>{
        //if self < T::MAX(){}
        Some(self)
    }
}