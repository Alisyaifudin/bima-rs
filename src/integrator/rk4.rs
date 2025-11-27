use crate::force::Force;
use crate::integrator::Integrator;
use crate::integrator::option_acc;
use crate::integrator::wdot_calc;
use crate::record::line::Line;
use crate::vec3::Vec3;
use crate::vec6::Vec6;

#[derive(Debug, Clone)]
pub struct Rk4 {
    dt: f64,
}
impl Rk4 {
    pub fn new(dt: f64) -> Self {
        Rk4 { dt }
    }
}

impl<F: Force> Integrator<F> for Rk4 {
    type Iter = Rk4Iter<F>;
    fn iter(&self, t_stop: f64, save_acc: bool, force: F) -> Rk4Iter<F> {
        Rk4Iter::new(self.dt, t_stop, save_acc, force)
    }
}

pub struct Rk4Iter<F: Force> {
    t: f64,
    t_stop: f64,
    dt: f64,
    force: F,
    n: usize,
    save_acc: bool,
}

impl<F: Force> Rk4Iter<F> {
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
    fn integrate(&self, id: usize, a: Vec3, w0: Vec6) -> Vec6 {
        let dt = self.dt;
        let k1 = Vec6::new(w0.v, a);
        let w = w0 + (dt * 0.5) * k1;
        let k2 = wdot_calc(id, &w, &self.force);
        let w = w0 + (dt * 0.5) * k2;
        let k3 = wdot_calc(id, &w, &self.force);
        let w = w0 + dt * k3;
        let k4 = wdot_calc(id, &w, &self.force);
        let w = w0 + dt * (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0;
        w
    }
}

impl<F: Force> Iterator for Rk4Iter<F> {
    type Item = Vec<(usize, Line)>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.t >= self.t_stop {
            return None;
        }
        self.force.all();
        for i in 0..self.n {
            let body = self.force.body(i);
            let w = self.integrate(body.id, body.a, body.to_vec6());
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
