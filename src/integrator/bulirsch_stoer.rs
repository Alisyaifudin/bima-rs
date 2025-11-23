use crate::integrator::Solution;
use crate::vec6::{ONE_VEC6, Vec6, ZERO_VEC6};

const TOLERANCE2: f64 = 1e-12;

pub fn solve<F: FnMut(Vec6, bool) -> Vec6>(w: Vec6, dt: f64, mut wdot_func: F) -> Solution {
    let mut t_prevs: Vec<Vec6> = Vec::new();
    let mut t_nexts: Vec<Vec6> = Vec::new();
    let mut ns = Vec::new();
    let wdot = wdot_func(w, true);
    for r in 0..20 {
        let nr = 2 * (r + 1);
        let t_r0 = mod_midpoint(&w, &wdot, dt, nr, &mut wdot_func);
        ns.push(nr);
        t_nexts.push(t_r0);
        let diff = extrapolate(&t_prevs, &mut t_nexts, &ns, r);
        let diff_r_norm2 = diff.r.norm_2();
        t_prevs = std::mem::take(&mut t_nexts);
        if diff_r_norm2 < TOLERANCE2 {
            break;
        }
    }
    return Solution::new(
        *t_prevs.last().expect("Last approximation, must exist"),
        Some(wdot.v),
    );
}

fn mod_midpoint<F: FnMut(Vec6, bool) -> Vec6>(
    w: &Vec6,
    wdot: &Vec6,
    dt: f64,
    nr: usize,
    wdot_func: &mut F,
) -> Vec6 {
    let h = dt / nr as f64;
    let mut z = vec![*w];
    // first sub-step
    let z1 = *w + *wdot * h;
    z.push(z1);
    // midpoint rule
    for i in 1..nr {
        let wdot = wdot_func(z[i], false);
        let z_next = z[i - 1] + (2.0 * h) * wdot;
        z.push(z_next)
    }
    // smoothing
    let wdot = wdot_func(z[nr], false);
    let t_k0 = (z[nr] + z[nr - 1] + h * wdot) / 2.;
    t_k0
}

const EPSILON: f64 = 1e-24;

fn extrapolate(t_prevs: &Vec<Vec6>, t_nexts: &mut Vec<Vec6>, ns: &Vec<usize>, r: usize) -> Vec6 {
    for (c, &t_prev) in t_prevs.iter().enumerate() {
        // _0 means previous row
        let t_next_0 = *t_nexts.last().expect("Already inserted one member before.");
        // _1 means second previous row
        let t_next_1 = t_nexts
            .get(t_nexts.len().wrapping_sub(2))
            .unwrap_or(&ZERO_VEC6);
        let n_coef = ns[r] as f64 / ns[r - c - 1] as f64;
        let c1 = t_next_0 - t_prev;
        let c2 = t_next_0 - *t_next_1;
        let t_delta = c1 * c2 / (n_coef * n_coef * (c2 - c1) - c2 + EPSILON * ONE_VEC6);
        let t_target = t_next_0 + t_delta;
        t_nexts.push(t_target);
    }
    let diff =
        *t_nexts.last().expect("must exist at least one") - *t_prevs.last().unwrap_or(&ZERO_VEC6);
    return diff;
}
