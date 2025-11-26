use crate::force::Force;
use crate::integrator::{Integrator, Solution};
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

#[derive(Debug, Clone)]
pub struct LeapFrog {
    dt: f64,
    state: State,
}
impl LeapFrog {
    pub fn new(dt: f64) -> Self {
        LeapFrog {
            dt,
            state: State::FirstDrift,
        }
    }
    pub fn next(&mut self) {
        self.state.next();
    }
}

impl<F: Force + Send> Integrator<F> for LeapFrog {
    fn dt(&self) -> f64 {
        self.dt
    }
    fn call(&self, _id: usize, a: Vec3, w: Vec6, _force: &F) -> Solution {
        // TODO: update dt to make it better
        let dt = self.dt;
        match self.state {
            State::FirstDrift => {
                let w_new = drift(w, dt);
                Solution::new(w_new.r, w_new.v, dt)
            }
            State::Kick => {
                let w_new = kick(a, w, dt);
                Solution::new(w_new.r, w_new.v, dt)
            }
            State::SecondDrift => {
                let w_new = drift(w, dt);
                Solution::new(w_new.r, w_new.v, dt)
            }
        }
    }
    fn pre(&mut self) -> (bool, bool) {
        let calc_force = self.state == State::Kick;
        let send = self.state == State::FirstDrift;
        (calc_force, send)
    }
    // return should update?
    fn post(&mut self) -> bool {
        self.next();
        self.state == State::FirstDrift
    }
}

fn drift(mut w: Vec6, dt: f64) -> Vec6 {
    w.r += w.v * dt / 2.;
    w
}

fn kick(a: Vec3, mut w: Vec6, dt: f64) -> Vec6 {
    w.v += a * dt;
    w
}
