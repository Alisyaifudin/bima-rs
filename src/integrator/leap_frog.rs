use crate::force::Force;
use crate::integrator::{Integrator, option_acc};
use crate::record::line::Line;
use crate::vec3::Vec3;
use crate::vec6::Vec6;

#[derive(Debug, Clone)]
pub struct LeapFrog {
    dt: f64,
}
impl LeapFrog {
    pub fn new(dt: f64) -> Self {
        LeapFrog { dt }
    }
}

impl<F: Force> Integrator<F> for LeapFrog {
    type Iter = LeapFrogIter<F>;
    fn iter(&self, t_stop: f64, save_acc: bool, force: F) -> LeapFrogIter<F> {
        LeapFrogIter::new(self.dt, t_stop, save_acc, force)
    }
}

pub struct LeapFrogIter<F: Force> {
    t: f64,
    t_stop: f64,
    dt: f64,
    force: F,
    n: usize,
    save_acc: bool,
}

impl<F: Force> LeapFrogIter<F> {
    fn new(dt: f64, t_stop: f64, save_acc: bool, force: F) -> Self {
        let n = force.len();
        Self {
            t: 0.,
            dt,
            force,
            n,
            t_stop,
            save_acc,
        }
    }
    fn drift(&self, mut w: Vec6, dt: f64) -> Vec6 {
        w.r += w.v * dt / 2.;
        w
    }

    fn kick(&self, a: Vec3, mut w: Vec6, dt: f64) -> Vec6 {
        w.v += a * dt;
        w
    }
}

impl<F: Force> Iterator for LeapFrogIter<F> {
    type Item = Vec<(usize, Line)>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.t >= self.t_stop {
            return None;
        }
        // first drift
        for i in 0..self.n {
            let body = self.force.body(i);
            let w = self.drift(body.to_vec6(), self.dt);
            self.force.set_body(i, w.r, w.v);
        }
        // kick
        self.force.all();
        for i in 0..self.n {
            let body = self.force.body(i);
            let w = self.kick(body.a, body.to_vec6(), self.dt);
            self.force.set_body(i, w.r, w.v);
        }
        // second drift
        for i in 0..self.n {
            let body = self.force.body(i);
            let w = self.drift(body.to_vec6(), self.dt);
            self.force.set_body(i, w.r, w.v);
        }
        self.t += self.dt;
        let lines = self
            .force
            .bodies()
            .iter()
            .map(|b| {
                (
                    b.id,
                    Line::new(self.t, b.r, b.v, option_acc(b.a, self.save_acc)),
                )
            })
            .collect();
        return Some(lines);
    }
}
