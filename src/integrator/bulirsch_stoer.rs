use crate::force::Force;
use crate::integrator::{Integrator, Solution, wdot_calc};
use crate::vec3::Vec3;
use crate::vec6::{self, Vec6};

const EPSILON: f64 = 1e-24;

#[derive(Debug, Clone)]
pub struct BS {
    dt: f64,
    tol: f64,
    n_try: usize,
}
impl BS {
    pub fn new(dt: f64, tol: f64, n_try: usize) -> Self {
        BS { dt, tol, n_try }
    }
}


impl<F: Force + Send> Integrator<F> for BS {
    fn dt(&self) -> f64 {
        self.dt
    }
    fn call(&self, id: usize, a: Vec3, w: Vec6, force: &F) -> Solution {
        let dt = self.dt;
        let mut t_prevs: Vec<Vec6> = Vec::new();
        let mut t_nexts: Vec<Vec6> = Vec::new();
        let mut ns = Vec::new();
        let wdot = Vec6::new(w.v, a);
        for r in 0..self.n_try {
            let nr = 2 * (r + 1);
            let t_r0 = mod_midpoint(id, &w, &wdot, dt, nr, force);
            ns.push(nr);
            t_nexts.push(t_r0);
            extrapolate(&t_prevs, &mut t_nexts, &ns, r);
            let t_prev = *t_prevs.last().unwrap_or(&vec6::ZERO);
            let diff = *t_nexts.last().expect("must exist at least one")
                - t_prev;
            let diff_r_norm2 = diff.r.norm_2() / t_prev.r.norm_2();
            t_prevs = std::mem::take(&mut t_nexts);
            if diff_r_norm2 < self.tol {
                break;
            }
        }
        let w = *t_prevs.last().expect("Last approximation, must exist");
        // TODO: update dt to make it better
        return Solution::new(w.r, w.v, dt);
    }
}

fn mod_midpoint<F: Force + Send>(
    id: usize,
    w: &Vec6,
    wdot: &Vec6,
    dt: f64,
    nr: usize,
    force: &F,
) -> Vec6 {
    let h = dt / nr as f64;
    let mut z = vec![*w];
    // first sub-step
    let z1 = *w + *wdot * h;
    z.push(z1);
    // midpoint rule
    for i in 1..nr {
        let wdot = wdot_calc(id, &z[i], force);
        let z_next = z[i - 1] + (2.0 * h) * wdot;
        z.push(z_next)
    }
    // smoothing
    let wdot = wdot_calc(id, &z[nr], force);
    let t_k0 = (z[nr] + z[nr - 1] + h * wdot) / 2.;
    t_k0
}

fn extrapolate(t_prevs: &Vec<Vec6>, t_nexts: &mut Vec<Vec6>, ns: &Vec<usize>, r: usize) {
    for (c, &t_prev) in t_prevs.iter().enumerate() {
        // _0 means previous row
        let t_next_0 = *t_nexts.last().expect("Already inserted one member before.");
        // _1 means second previous row
        let t_next_1 = t_nexts
            .get(t_nexts.len().wrapping_sub(2))
            .unwrap_or(&vec6::ZERO);
        let n_coef = ns[r] as f64 / ns[r - c - 1] as f64;
        let c1 = t_next_0 - t_prev;
        let c2 = t_next_0 - *t_next_1;
        let t_delta = c1 * c2 / (n_coef * n_coef * (c2 - c1) - c2 + EPSILON * vec6::ONE);
        let t_target = t_next_0 + t_delta;
        t_nexts.push(t_target);
    }
}
