use crate::record::{Line, Record};
use crate::solve::{self, SolveMethod};
use crate::system::System;

#[derive(Clone, Debug)]
pub enum TimestepMethod {
    Constant(f64),
    // Adaptive
}

pub fn constant_step(system: &mut System, delta_t: f64, record: &mut Record) {
    let t = system.t;
    let n = system.bodies.len();
    for i in 0..n {
        let body = &mut system.bodies[i];
        match system.solve_method {
            SolveMethod::Euler => solve::euler(body, delta_t),
            SolveMethod::RK4 => todo!(),
        }
        let saved_a = if system.save_acc { Some(body.a) } else { None };
        record.add(i, Line::new(t, body.r, body.v, saved_a));
    }
}
