use crate::force::Force;
use crate::integrator::{Integrator, option_acc};
use crate::record::line::Line;
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

impl<F: Force> Integrator<F> for Euler {
    type Iter = EulerIter<F>;
    fn iter(&self, t_stop: f64, save_acc: bool, force: F) -> EulerIter<F> {
        EulerIter::new(self.dt, t_stop, save_acc, force)
    }
}

pub struct EulerIter<F: Force> {
    t: f64,
    t_stop: f64,
    dt: f64,
    force: F,
    n: usize,
    save_acc: bool,
}

impl<F: Force> EulerIter<F> {
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
    fn integrate(&self, a: Vec3, w: Vec6) -> Vec6 {
        let dt = self.dt;
        let r = w.r + w.v * dt;
        let v = w.v + a * dt;
        Vec6::new(r, v)
    }
}

impl<F: Force> Iterator for EulerIter<F> {
    type Item = Vec<(usize, Line)>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.t >= self.t_stop {
            return None;
        }
        self.force.all();
        for i in 0..self.n {
            let body = self.force.body(i);
            let w = self.integrate(body.a, body.to_vec6());
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
