use chrono::round;
use num::Float;
use std::f64::consts::E; 
use malachite::num::arithmetic::traits::{Pow, PowerOf2};
use malachite::num::float::NiceFloat;
use malachite::Rational;
pub fn simple_Pi (step: f64) -> f64 {
    let num_of_step = (1.0 as f64 / step) as usize;
    let mut x: f64 = 0.0;
    let mut ret: f64  = x;
    let mut iteration = x;
    for s in 0..num_of_step {
        x += step;
        iteration = 2.0 * x - x.powi(2);
        iteration = iteration.sqrt() * step;
        ret += iteration;
    } ret * 4.0
}
pub fn simple_Pi_vs_std_Pi (step: String) -> (f64, f64) {
    dbg!(&step);
    let (_, step) = crate::split_once(&step, ":");
    let step = step.parse::<f64>().unwrap_or(0.001);
    let std_Pi = std::f64::consts::PI;
    let tst_Pi = simple_Pi (step);
    crate::krunner (Some (&tst_Pi.to_string()) );
    let msg = format! ("deviation from std Pi {}\ntst Pi {}", &(std_Pi - tst_Pi).to_string(), tst_Pi);
    crate::errMsg0( &msg);
    (tst_Pi, std_Pi - tst_Pi )
}
pub fn tst_Pi (ceil: f64) -> f64 {
    let mut x = 1.0_f64;
    let mut y = x - x;
    let ret: f64 = Gauss_Legendre_Pi(ceil); 
    ret
}
pub fn arc_val (from: f64, to: f64) -> f64 {
    let x = from;
    let From = ((x - 1.0) *(-(x - 2.0).sqrt() * x) + (x - 1.0).asin() ) / 2.0;    
    let x = to;
    let To = ((x - 1.0) *(-(x - 2.0).sqrt() * x) + (x - 1.0).asin() ) / 2.0;
    To - From
}
pub fn tst_Pi_ (error: f64) -> f64 { // failed
    let mut x = 1.0_f64;
    let mut y = x - x;
    while (x - y).abs() > error {
        x /= 2.0;
        y = (2.0 * x - x.powi(2) ).sqrt();
    }
    let n = (0.5/ x);
     dbg! ((x - y).abs() );
    y = y / 2.0;
    let ret = 8.0 * n * y; 
    dbg! (y);
    ret
}
pub fn tst_Pi_vs_std_Pi (step: String) -> (f64, f64) {
    dbg!(&step);
    let (_, step) = crate::split_once(&step, ":");
    let error = step.parse::<f64>().unwrap_or(31.0);
    println!("rounds: {error}\n");
    let std_Pi = std::f64::consts::PI;
    let control_tst_Pi = tst_Pi (error + 1.0);
    let tst_Pi = tst_Pi (error);
    crate::krunner (Some (&tst_Pi.to_string()) );
    let msg = format! ("deviation from std Pi {}\ntst Pi {}\nCos(std): {} \nCos(tst): {}\nstd_Cos(tst): {}\n
    std_Cos(800 000*tst): {}\nCos(800 000*tst): {}\ntricked_Cos(800 000*tst): {}\ntricked_Cos(811 000*tst): {}   ", 
    &(std_Pi - tst_Pi).to_string(), tst_Pi, tst_Cos (std_Pi) , tst_Cos(tst_Pi), 
    tst_Pi.cos(), (800_000.0*tst_Pi).cos(), tst_Cos(800_000.0*tst_Pi ), 
    tricked_Cos(800_000.0*tst_Pi, error ), tricked_Cos(811_000.0*tst_Pi, error ));
    let msg1 = format! ("{}\nSimple Pi(0.7^(1/13)): {}\ncontrol_tst(step + 1): {}\nstd_Pi - 2*tst_asin: {}\nstd_Pi - 2*almost_asin: {}
    \nstd_Pi - epi {}"
    , msg, simple_Pi (0.05.powf(1.0 / 11.0) ), control_tst_Pi, std_Pi - 2.0 * tst_asin(1.0, 11),
     std_Pi - 2.0 * almost_asin(1.0, 75), std_Pi - epi() );
    crate::errMsg0( &msg1);
    (tst_Pi, std_Pi - tst_Pi )
}
pub fn Gauss_Legendre_Pi (rounds: f64) -> f64 {
    let mut a = 1.0f64;
    let mut b: f64 =1.0 / 2.0.sqrt();
    let mut t: f64 = 1.0 / 4.0;
    let mut p = 1.0 as f64;
    let mut a_nxt = a;
    let mut b_nxt = a;
    let mut t_nxt = a;
    let mut p_nxt = a;
    for r in 0..rounds as usize {
        a_nxt = (a + b) / 2.0;
        b_nxt = (a * b).sqrt();
        t_nxt = t - p * (a - a_nxt).powi(2);
        p = 2.0 * p;
        if a == a_nxt {dbg! (a_nxt); dbg!(r);}
        if b == b_nxt {dbg! (b_nxt); dbg!(r);}
        if t == t_nxt {dbg! (t_nxt); dbg!(r); break;}
        a = a_nxt;
        b = b_nxt;
        t = t_nxt;
    }
    dbg!(t); dbg!(t_nxt);
(a + b).powi(2) / (4.0 * t)
}
pub fn sigma_ln (from: f64, to: f64) -> f64 {
    let x = from;
    let From = x * (x.ln() - 1.0);
    let x = to;
    let To = x * (x.ln() - 1.0);
    To - From
}
pub fn sigma_log (base: f64, from: f64, to: f64) -> f64 {
    let x = from;
    let From = x * (x.ln() - 1.0);
    let x = to;
    let To = x * (x.ln() - 1.0);
    (To - From) / base.ln()
}
pub fn tst_Cos (x: f64) -> f64 {
// cos (0) -sin(0)(1) - cos(0)(2) + sin(0)(3) + cos(0)(4) - 0(5) - 1(6) +0(7) + 1(8) 
    1.0 - (x.powi(2) / 2u64.factorial()) + (x.powi(4) / 4.factorial() ) -  (x.powi(6) / 6.factorial() ) + (x.powi(8) / 8.factorial() )
}
pub fn tricked_Cos (x: f64, rounds_to_calc_pi: f64) -> f64 {
    let pi = Gauss_Legendre_Pi(rounds_to_calc_pi);
    let rotations = x / pi;
    if rotations.floor() == rotations {
        if (rotations as usize) % 2 == 0 {return 1.0}
    }
    let x = x % pi;
// cos (0) -sin(0)(1) - cos(0)(2) + sin(0)(3) + cos(0)(4) - 0(5) - 1(6) +0(7) + 1(8) - 0(9) - 1(10) 
    let ret = 1.0 - (x.powi(2) / 2u64.factorial()) + (x.powi(4) / 4.factorial() ) -  (x.powi(6) / 6.factorial() ) + (x.powi(8) / 8.factorial() )
    - (x.powi(10) / 10.factorial() ) + (x.powi(12) / 12.factorial() );
    if rotations.floor() as usize % 2 == 1 {return -1.0 * ret ;}
    ret

}
pub fn almost_asin (x: f64, rounds: usize) -> f64 {
     let mut ret = 1f64;
    for n in 1..rounds{
        let n = n as f64;
        let numerator = (2.0 * n).fuzzy_factorial();
        let denominator:f64 = 4.0.powi(n as i32) * n.fuzzy_factorial().powi (2) * (2.0 * n as f64 + 1.0);
        ret += (numerator as f64 / denominator as f64) * (x.powi (2 * n as i32 + 1) );
    }
    dbg!(ret);
    ret
}
pub fn tst_asin (x: f64, rounds: usize) -> f64 {
     let mut ret = 1f64;
    for n in 1..rounds{
        let numerator = (2 * n).factorial_();
        let denominator:f64 = 4.0.powi(n as i32) * n.factorial_().powi (2) * (2.0 * n as f64 + 1.0);
        ret += (numerator as f64 / denominator as f64) * (x.powi (2 * n as i32 + 1) );
    }
    dbg!(ret);
    ret
}
pub trait fuzzy_Factorial {
    fn fuzzy_factorial (&self) -> f64;
}
impl fuzzy_Factorial for f64 {
    fn fuzzy_factorial (&self) -> f64 {
        let mut ret = 1.0f64;
        let mut x = *self;
        while x > 1.0{
            ret *= x;
            x -= 1.0;
        } ret
    }
}
pub trait Factorial {
    fn factorial (&mut self) -> f64;
    fn factorial_ (&self) -> f64;
}
impl Factorial for u64 {
    fn factorial (&mut self) -> f64 {
        let mut ret = 1u64;
        while *self > 1{
            ret *= *self;
            *self -= 1;
        } ret as f64
    }
    fn factorial_ (&self) -> f64 {
        let mut ret: u64 = 1;
        let mut x = *self;
        while x > 1{
            ret *= x;
            x -= 1;
        } ret as f64
    }
}
impl Factorial for usize {
    fn factorial (&mut self) -> f64 {
        let mut ret = 1usize;
        while *self > 1{
            ret *= *self;
            *self -= 1;
        } ret as f64
    }
   fn factorial_ (&self) -> f64 {
        let mut ret = 1usize;
        let mut x = *self;
        while x > 1{
            ret *= x;
            x -= 1;
        } ret as f64
    }
}
impl Factorial for i32 {
    fn factorial (&mut self) -> f64 {
        let mut ret = 1i32;
        while *self > 1{
            ret *= *self;
            *self -= 1;
        } ret as f64
    }
   fn factorial_ (&self) -> f64 {
        let mut ret = 1i32;
        let mut x = *self;
        while x > 1{
            ret *= x;
            x -= 1;
        } ret as f64
    }
}
pub fn epi () -> f64 {
   let n: f64 =587124671.0;
   let m=768614336.0;
   let ret = E.powf( (m/n).sqrt() );
   ret
}
//fn
// https://math.stackexchange.com/questions/197874/maclaurin-expansion-of-arcsin-x
// n=587124671 m=768614336.
/*
from decimal import Decimal, getcontext

def gauss_legendre_pi(precision):
    # Set the precision (number of decimal places)
    getcontext().prec = precision + 2  # Add extra digits to avoid rounding errors

    # Initial values
    a = Decimal(1)
    b = Decimal(1) / Decimal(2).sqrt()
    t = Decimal(1) / Decimal(4)
    p = Decimal(1)

    # Iterate until convergence
    for _ in range(precision):
        a_next = (a + b) / 2
        b_next = (a * b).sqrt()
        t_next = t - p * (a - a_next) ** 2
        p_next = 2 * p

        # Update values
        a, b, t, p = a_next, b_next, t_next, p_next

    # Calculate π
    pi_estimate = (a + b) ** 2 / (4 * t)
    return pi_estimate

# Example usage
precision = 100  # Number of decimal places
pi_estimate = gauss_legendre_pi(precision)
print(f"Estimated value of π to {precision} decimal places:\n{pi_estimate}")
----------------
import numpy as np
import matplotlib.pyplot as plt

# Define the interval
x = np.linspace(1, np.e, 100)

# Logarithm function
y_log = np.log(x)

# Quarter-circle approximation
h, k = 1, -1.97625  # Center of the circle
r = 2.97625         # Radius of the circle
y_circle = k + np.sqrt(r**2 - (x - h)**2)

# Plot
plt.plot(x, y_log, label="log(x)")
plt.plot(x, y_circle, label="Quarter-circle approximation")
plt.xlabel("x")
plt.ylabel("y")
plt.legend()
plt.title("Approximating log(x) as a Quarter-Circle")
plt.grid()
plt.show()
--------------
import math
def arcsin_taylor(x, terms=10):
    """Compute arcsin(x) using a Taylor series expansion."""
    result = 0
    for n in range(terms):
        numerator = math.factorial(2 * n)
        denominator = (4**n) * (math.factorial(n)**2) * (2 * n + 1)
        result += (numerator / denominator) * (x ** (2 * n + 1))
    return result

# Example usage
x = 0.5
approx = arcsin_taylor(x, terms=10)
exact = math.asin(x)
print(f"Approximation: {approx}")
print(f"Exact value:   {exact}")
---------
import math

def double_factorial(n):
    """Compute the double factorial of n."""
    if n <= 0:
        return 1
    return n * double_factorial(n - 2)

def arcsin_double_factorial(x, terms=10):
    """Compute arcsin(x) using the double factorial series."""
    result = 0
    for n in range(terms):
        numerator = double_factorial(2 * n - 1)
        denominator = double_factorial(2 * n) * (2 * n + 1)
        result += (numerator / denominator) * (x ** (2 * n + 1))
    return result

# Example usage
x = 0.5
approx = arcsin_double_factorial(x, terms=10)
exact = math.asin(x)
print(f"Approximation: {approx}")
print(f"Exact value:   {exact}")
 */