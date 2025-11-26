mod bulirsch_stoer;
mod euler;
mod leap_frog;
mod rk4;
use crate::force::Force;
use crate::vec3::{self, Vec3};
use crate::vec6::Vec6;
pub use bulirsch_stoer::BS;
pub use euler::Euler;
pub use leap_frog::LeapFrog;
pub use rk4::RK4;

#[derive(Clone, Debug)]
pub enum Kind {
    Euler,
    RK4,
    BS,
    LeapFrog(leap_frog::State),
}

pub trait Integrator<F: Force> {
    fn call(&self, id: usize, a: Vec3, w: Vec6, force: &F) -> Solution;
    fn dt(&self) -> f64;
    fn pre(&mut self) -> (bool, bool) {
        // should calc force, should send data
        (true, true)
    }
    // should update time?
    fn post(&mut self) -> bool {
        true
    }
    fn set_dt(&mut self, _dt: f64) {}
}

#[derive(Clone, Debug)]
pub struct Solution {
    pub r: Vec3,
    pub v: Vec3,
    pub dt: f64,
}

impl Solution {
    pub fn new(r: Vec3, v: Vec3, dt: f64) -> Self {
        Solution { r, v, dt }
    }
    pub fn empty() -> Self {
        Solution {
            r: vec3::ZERO,
            v: vec3::ZERO,
            dt: 0.,
        }
    }
    // pub fn unzip(self) -> (Vec3, Vec3, ) {
    //     (self.r, self.v)
    // }
}

pub fn wdot_calc<F: Force + Send>(id: usize, w: &Vec6, force: &F) -> Vec6 {
    let vdot = force.one(id, &w.r);
    let rdot = w.v;
    Vec6::new(rdot, vdot)
}
