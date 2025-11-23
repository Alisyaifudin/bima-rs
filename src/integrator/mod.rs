use crate::{ vec3::Vec3, vec6::Vec6};
mod bulirsch_stoer;
mod euler;
pub mod leap_frog;
mod rk4;

#[derive(Clone, Debug)]
pub enum Integrator {
    Euler,
    RK4,
    BS,
    LeapFrog(leap_frog::State),
}

impl Integrator {
    pub fn new_leap_frog() -> Self {
        Integrator::LeapFrog(leap_frog::State::FirstDrift)
    }
}

pub struct Solution {
    pub w: Vec6,
    pub a: Option<Vec3>,
}

impl Solution {
    pub fn new(w: Vec6, a: Option<Vec3>) -> Self {
        Solution { w, a }
    }
    pub fn unzip(self) -> (Vec6, Option<Vec3>) {
        (self.w, self.a)
    }
}

pub fn euler<F: FnMut(Vec6, bool) -> Vec6>(w: Vec6, dt: f64, wdot_func: F) -> Solution {
    euler::solve(w, dt, wdot_func)
}

pub fn rk4<F: FnMut(Vec6, bool) -> Vec6>(w: Vec6, dt: f64, wdot_func: F) -> Solution {
    rk4::solve(w, dt, wdot_func)
}

pub fn bs<F: FnMut(Vec6, bool) -> Vec6>(w: Vec6, dt: f64, wdot_func: F) -> Solution {
    bulirsch_stoer::solve(w, dt, wdot_func)
}

pub fn lf<F: FnMut(Vec6, bool) -> Vec6>(
    w: Vec6,
    dt: f64,
    wdot_func: F,
    state: leap_frog::State,
) -> Solution {
    leap_frog::solve(w, dt, wdot_func, state)
}
