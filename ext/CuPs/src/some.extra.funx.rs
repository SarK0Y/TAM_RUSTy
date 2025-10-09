use Mademoiselle_Entropia::custom_traits::helpful_math_ops;
pub(crate) fn delta<T>(fst: T, nd: T) -> T
where
	T: PartialEq + Eq + std::ops::Sub<Output = T> + std::cmp::PartialOrd,
{
    if fst > nd {
        return fst - nd;
    }
    return nd - fst;
}
pub fn new_obj_id () -> u64 {
    static mut id: u64 = 0;
    unsafe {
        if id == u64::MAX { id = 0; return 0; }
        id.inc(); return id - 1;
    }
}
 
