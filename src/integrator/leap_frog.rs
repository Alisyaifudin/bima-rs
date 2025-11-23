use crate::integrator::Solution;
use crate::vec3::Vec3;
use crate::vec6::Vec6;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum State {
    FirstDrift,
    Kick,
    SecondDrift,
}

impl State {
    pub fn next(&mut self) {
        match self {
            State::FirstDrift => *self = State::Kick,
            State::Kick => *self = State::SecondDrift,
            State::SecondDrift => *self = State::FirstDrift,
        }
    }
}

pub fn solve<F: FnMut(Vec6, bool) -> Vec6>(
    w: Vec6,
    dt: f64,
    wdot_func: F,
    state: State,
) -> Solution {
    match state {
        State::FirstDrift => {
            let w_new = drift(w, dt);
            Solution::new(w_new, None)
        }
        State::Kick => {
            let (w_new, a) = kick(w, dt, wdot_func);
            Solution::new(w_new, Some(a))
        }
        State::SecondDrift => {
            let w_new = drift(w, dt);
            Solution::new(w_new, None)
        }
    }
}

fn drift(mut w: Vec6, dt: f64) -> Vec6 {
    w.r += w.v * dt / 2.;
    w
}

fn kick<F: FnMut(Vec6, bool) -> Vec6>(mut w: Vec6, dt: f64, mut wdot_func: F) -> (Vec6, Vec3) {
    let wdot = wdot_func(w, true);
    let a = wdot.v;
    w.v += a * dt;
    (w, a)
}
