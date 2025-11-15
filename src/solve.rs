use crate::system::Body;

#[derive(Clone, Debug)]
pub enum SolveMethod {
    Euler,
    RK4,
}

pub fn euler(body: &mut Body, delta_t: f64) {
    body.r += body.v * delta_t;
    body.v += body.a * delta_t;
}