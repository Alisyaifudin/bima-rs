use crate::{integrator::Solution, vec6::Vec6};

pub fn solve<F: FnMut(Vec6, bool) -> Vec6>(w: Vec6, dt: f64, mut wdot_func: F) -> Solution {
    let k1 = wdot_func(w, true);
    let tmp = w + (dt * 0.5) * k1;
    let k2 = wdot_func(tmp, false);
    let tmp = w + (dt * 0.5) * k2;
    let k3 = wdot_func(tmp, false);
    let tmp = w + dt * k3;
    let k4 = wdot_func(tmp, false);
    let w_new = w + dt * (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0;
    Solution::new(w_new, Some(k1.v))
}
