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
            } 
      //      println!("{} {}", 2.0.log_norma(1.023), 2.0_f64.pow (11));
            return sum
        
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
 //       dbg!(&exp);
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
pub trait Log_Norma {
    type S;
    fn log_norma  (&self, ceil: Self::S) -> Self::S;
}
impl Log_Norma for f32 {
    type S = f32;
    fn log_norma  (&self, ceil: Self::S) -> Self::S {
        if ceil == ceil -ceil {return *self;}
 //       dbg!(&exp);
        let mut ret: Self::S = self.log (ceil).abs();
        while ret > ceil {
          //  dbg!(&ret);
            ret = self.log (ret).abs();
        } return ret
    }
}
//fn