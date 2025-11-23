use crate::body::Body;
use crate::close_encounter::CloseEncounter;
use crate::force::{self, ForceMethod};
use crate::integrator::{self, Integrator, leap_frog};
use crate::system::System;
use crate::vec3::Vec3;
use crate::vec6::Vec6;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum TimestepMethod {
    Constant(f64),
    // Adaptive
}

pub fn constant_step(system: &mut System, dt: f64, tmp: &mut Vec<Body>) -> bool {
    let n = system.bodies.len();
    let bodies = &system.bodies;
    let solve = &mut system.integrator;
    let force_method = &system.force_method;
    let close_encounter = &system.close_encounter;
    let cache = &mut system.cache;
    let mut proceed = true;
    for id in 0..n {
        let body = &bodies[id];
        let m = body.m;
        let a = body.a;
        let w = body.to_vec6();
        let wdot_func = |w: Vec6, use_cache: bool| {
            let dummy = Body::new(id, m, w.r, w.v, None);
            calc_wdot(
                &dummy,
                bodies,
                force_method,
                close_encounter,
                cache,
                use_cache,
            )
        };
        let sol = match solve {
            Integrator::Euler => integrator::euler(w, dt, wdot_func),
            Integrator::RK4 => integrator::rk4(w, dt, wdot_func),
            Integrator::BS => integrator::bs(w, dt, wdot_func),
            Integrator::LeapFrog(state) => {
                let s = integrator::lf(w, dt, wdot_func, *state);
                if id == n - 1 {
                    state.next();
                }
                if *state == leap_frog::State::SecondDrift {
                    proceed = true;
                } else {
                    proceed = false;
                }
                s
            }
        };
        let (w_new, a_new) = sol.unzip();
        let new_body = Body::new(id, m, w_new.r, w_new.v, a_new.or(Some(a)));
        tmp.push(new_body);
    }
    system.clear_cache();
    system.bodies = std::mem::take(tmp);
    proceed
}

pub fn calc_wdot(
    body: &Body,
    bodies: &Vec<Body>,
    force_method: &ForceMethod,
    close_encounter: &CloseEncounter,
    cache: &mut HashMap<(usize, usize), Vec3>,
    use_cache: bool,
) -> Vec6 {
    let cache = if use_cache { Some(cache) } else { None };
    let vdot = force::calc(body, bodies, force_method, close_encounter, cache);
    let rdot = body.v;
    Vec6::new(rdot, vdot)
}
