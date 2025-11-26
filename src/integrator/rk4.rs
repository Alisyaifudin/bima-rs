use crate::force::Force;
use crate::integrator::Integrator;
use crate::integrator::Solution;
use crate::integrator::wdot_calc;
use crate::vec3::Vec3;
use crate::vec6::Vec6;

#[derive(Debug, Clone)]
pub struct RK4 {
    dt: f64,
}
impl RK4 {
    pub fn new(dt: f64) -> Self {
        RK4 { dt }
    }
}

impl<F: Force + Send> Integrator<F> for RK4 {
    fn dt(&self) -> f64 {
        self.dt
    }
    fn call(&self, id: usize, a: Vec3, w0: Vec6, force: &F) -> Solution {
        let dt = self.dt;
        let k1 = Vec6::new(w0.v, a);
        let w = w0 + (dt * 0.5) * k1;
        let k2 = wdot_calc(id, &w, force);
        let w = w0 + (dt * 0.5) * k2;
        let k3 = wdot_calc(id, &w, force);
        let w = w0 + dt * k3;
        let k4 = wdot_calc(id, &w, force);
        let w = w0 + dt * (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0;
        Solution::new(w.r, w.v, dt)
    }
}
