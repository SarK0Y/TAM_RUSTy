pub fn poly <T: std::ops::Mul<Output = T> +
                std::ops::Sub<Output = T> +
                std::ops::Add<Output = T> +
                std::ops::AddAssign +
                std::ops::MulAssign +
//                std::ops::BitXor<Output = T> +
                PartialEq +
            //    Eq +
                std::ops::Div<Output = T> +
                std::cmp::PartialOrd +
                Copy +
                std::fmt::Debug +
                PowIt <T> >
        (coefs: &Vec <T>, x: T ) -> T {
            if coefs.len () == 0 {return x}
            let mut sum: T =  x - x;
            for i in 0..coefs.len() {
                sum += x.pow ( i ) * coefs [ i ];
            } return sum
        
}
pub trait PowIt <T: std::ops::Mul<Output = T> +
                std::ops::MulAssign +
                std::ops::AddAssign +
                std::ops::Sub<Output = T> +
                std::ops::Add<Output = T> +
                PartialEq +
        //        Eq +
                std::ops::Div<Output = T> +
  //              std::ops::BitXor<Output = T> +
                std::cmp::PartialOrd +
                Copy +
                std::fmt::Debug >{
    fn pow  (&self, exp: usize) -> T;
}
impl <T: std::ops::Mul<Output = T> +
                std::ops::MulAssign +
                std::ops::AddAssign +
                std::ops::Sub<Output = T> +
                std::ops::Add<Output = T> +
                PartialEq +
          //      Eq +
                std::ops::Div<Output = T> +
    //            std::ops::BitXor<Output = T> +
                std::cmp::PartialOrd +
                Copy +
                std::fmt::Debug > PowIt <T> for T {
    fn pow (&self, exp: usize) -> T {
        if exp == 0 {return *self / *self;}
        dbg!(&exp);
        let mut norm_exp = exp;
        let mut ret: T = *self / *self;
        let mut sq = *self;
        while norm_exp > 0 {
 //           dbg!(&norm_exp);
            if norm_exp & 1 == 1 {
                ret *= sq;
              //  dbg! (&ret);
            } sq *= sq;
           // dbg! (&sq);
            norm_exp /= 2;
        } return ret
    }
}