use crate::force::Force;
use crate::integrator::{Integrator, option_acc, wdot_calc};
use crate::record::line::Line;
use crate::vec3::Vec3;
use crate::vec6::{self, Vec6};

const SEQUENCE: [u8; 20] = [
    2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40,
];

#[derive(Debug, Clone)]
pub struct Bs {
    dt: f64,
    tol: f64,
    n_try: usize,
}
impl Bs {
    pub fn new(dt: f64, tol: f64, n_try: usize) -> Self {
        Bs { dt, tol, n_try }
    }
}

impl<F: Force> Integrator<F> for Bs {
    type Iter = BsIter<F>;
    fn iter(&self, t_stop: f64, save_acc: bool, force: F) -> BsIter<F> {
        BsIter::new(self.dt, t_stop, save_acc, self.tol, self.n_try, force)
    }
}

pub struct BsIter<F: Force> {
    t: f64,
    n: usize,
    dt: f64,
    t_stop: f64,
    save_acc: bool,
    tol: f64,
    n_try: usize,
    force: F,
    t_prevs: Vec<Vec6>,
}

impl<F: Force> BsIter<F> {
    fn new(dt: f64, t_stop: f64, save_acc: bool, tol: f64, n_try: usize, force: F) -> Self {
        let n = force.len();
        Self {
            t: 0.,
            n,
            dt,
            t_stop,
            save_acc,
            tol,
            n_try: if n_try > 20 { 20 } else { n_try },
            force,
            t_prevs: Vec::new(),
        }
    }
    fn integrate(&mut self, id: usize, a: Vec3, w: Vec6) -> Option<Vec6> {
        let dt = self.dt;
        let wdot = Vec6::new(w.v, a);
        self.t_prevs.clear();
        for r in 0..self.n_try {
            let nr = SEQUENCE[r];
            let t_r0 = mod_midpoint(id, &w, &wdot, dt, nr, &self.force);
            let t_prev = *self.t_prevs.last().unwrap_or(&vec6::ZERO);
            extrapolate(&mut self.t_prevs, t_r0, r);
            let t_next = *self.t_prevs.last().unwrap_or(&t_r0);
            let diff = t_next - t_prev;
            let err = frac_err(&diff, &t_prev);
            let converged = err < self.tol;
            if converged {
                // TODO: update dt to make it better
                let w = t_next;
                return Some(Vec6::new(w.r, w.v));
            }
        }
        None
    }
}

fn frac_err(nom: &Vec6, denom: &Vec6) -> f64 {
    if *denom == vec6::ZERO {
        return 1.;
    }
    let components = [
        (nom.x(), denom.x()),
        (nom.y(), denom.y()),
        (nom.z(), denom.z()),
        (nom.vx(), denom.vx()),
        (nom.vy(), denom.vy()),
        (nom.vz(), denom.vz()),
    ];

    components
        .into_iter()
        .map(|(n, d)| (n / d).abs())
        .reduce(f64::max)
        .unwrap()
}

impl<F: Force> Iterator for BsIter<F> {
    type Item = Vec<(usize, Line)>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.t >= self.t_stop {
            return None;
        }
        self.force.all();
        for i in 0..self.n {
            let body = self.force.body(i);
            if let Some(w) = self.integrate(body.id, body.a, body.to_vec6()) {
                self.force.set_body(i, w.r, w.v);
            } else {
                eprintln!("\nFailed to get good result.");
                return None;
            }
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

fn mod_midpoint<F: Force>(id: usize, w: &Vec6, wdot: &Vec6, dt: f64, nr: u8, force: &F) -> Vec6 {
    let h = dt / nr as f64;
    let mut z_prev = *w;
    // first sub-step
    let mut z_next = *w + *wdot * h;
    // midpoint rule
    for _ in 1..nr {
        let wdot = wdot_calc(id, &z_next, force);
        let z_tmp = z_prev + wdot * (2.0 * h);
        z_prev = z_next;
        z_next = z_tmp;
    }
    // smoothing
    let wdot = wdot_calc(id, &z_next, force);
    let t_k0 = (z_next + z_prev + h * wdot) / 2.;
    t_k0
}

// POLYNOMIAL EXTRAPOLATION
fn extrapolate(t_prevs: &mut Vec<Vec6>, mut t_next: Vec6, r: usize) {
    for c in 0..t_prevs.len() {
        let n_coef = SEQUENCE[r] as f64 / SEQUENCE[r - c - 1] as f64;
        let t_prev = t_prevs[c];
        let t_delta = (t_next - t_prev) / (n_coef * n_coef - 1.);
        let t_target = t_next + t_delta;
        t_prevs[c] = t_next;
        t_next = t_target;
    }
    t_prevs.push(t_next);
}

// POLYNOMIAL EXTRAPOLATION
// fn extrapolate(t_prevs: &Vec<Vec6>, t_nexts: &mut Vec<Vec6>, r: usize) {
//     for (c, &t_prev) in t_prevs.iter().enumerate() {
//         let t_next = *t_nexts.last().expect("Already inserted one member before.");
//         let n_coef = SEQUENCE[r] as f64 / SEQUENCE[r - c - 1] as f64;
//         let t_delta = (t_next - t_prev) / (n_coef * n_coef - 1.);
//         let t_target = t_next + t_delta;
//         t_nexts.push(t_target);
//     }
// }

// THIS IS FOR RATIONAL FUNCTION EXTRAPOLATION
// MANY SOURCES SAYS POLYNOMIAL EXTRAPOLATION IS GOOD ENOUGH
// fn extrapolate(t_prevs: &Vec<Vec6>, t_nexts: &mut Vec<Vec6>, ns: &Vec<usize>, r: usize) {
//     for (c, &t_prev) in t_prevs.iter().enumerate() {
//         // _0 means previous row
//         let t_next_0 = *t_nexts.last().expect("Already inserted one member before.");
//         // _1 means second previous row
//         let t_next_1 = t_nexts
//             .get(t_nexts.len().wrapping_sub(2))
//             .unwrap_or(&vec6::ZERO);
//         let n_coef = ns[r] as f64 / ns[r - c - 1] as f64;
//         let c1 = t_next_0 - t_prev;
//         let c2 = t_next_0 - *t_next_1;
//         let t_delta = c1 * c2 / (n_coef * n_coef * (c2 - c1) - c2 + EPSILON * vec6::ONE);
//         let t_target = t_next_0 + t_delta;
//         t_nexts.push(t_target);
//     }
// }
