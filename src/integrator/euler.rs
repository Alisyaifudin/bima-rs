use crate::{integrator::Solution, vec6::Vec6};

pub fn solve<F: FnMut(Vec6, bool) -> Vec6>(w: Vec6, dt: f64, mut wdot_func: F) -> Solution {
    let wdot = wdot_func(w, true);
    let w = w + wdot * dt;
    Solution::new(w, Some(wdot.v))
}
