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