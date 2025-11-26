use crate::force::Force;
use crate::integrator::{Integrator, Solution};
use crate::vec3::Vec3;
use crate::vec6::Vec6;

#[derive(Debug, Clone)]
pub struct Euler {
    dt: f64,
}
impl Euler {
    pub fn new(dt: f64) -> Self {
        Euler { dt }
    }
}

impl<F: Force + Send> Integrator<F> for Euler {
    fn dt(&self) -> f64 {
        self.dt
    }
    fn call(&self, _id: usize, a: Vec3, w: Vec6, _force: &F) -> Solution {
        let dt = self.dt;
        let r = w.r + w.v * dt;
        let v = w.v + a * dt;
        Solution::new(r, v, dt)
    }
}
