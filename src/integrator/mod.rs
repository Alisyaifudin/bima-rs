mod bulirsch_stoer;
mod euler;
mod leap_frog;
mod rk4;
use crate::force::Force;
use crate::record::line::Line;
use crate::vec3::Vec3;
use crate::vec6::Vec6;
pub use bulirsch_stoer::Bs;
pub use euler::Euler;
pub use leap_frog::LeapFrog;
pub use rk4::Rk4;

pub trait Integrator<F: Force> {
    type Iter: Iterator<Item = Vec<(usize, Line)>>;
    fn iter(&self, t_stop: f64, save_acc: bool, force: F) -> Self::Iter;
}

// // pub trait Iterator {
// //     fn next(&mut self) -> Option<Vec<Line>>;
// // }

// #[derive(Clone, Debug)]
// pub struct Solution {
//     pub r: Vec3,
//     pub v: Vec3,
//     pub dt: f64,
// }

// impl Solution {
//     pub fn new(r: Vec3, v: Vec3, dt: f64) -> Self {
//         Solution { r, v, dt }
//     }
//     pub fn empty() -> Self {
//         Solution {
//             r: vec3::ZERO,
//             v: vec3::ZERO,
//             dt: 0.,
//         }
//     }
//     // pub fn unzip(self) -> (Vec3, Vec3, ) {
//     //     (self.r, self.v)
//     // }
// }

pub fn wdot_calc<F: Force>(id: usize, w: &Vec6, force: &F) -> Vec6 {
    let vdot = force.one(id, &w.r);
    let rdot = w.v;
    Vec6::new(rdot, vdot)
}

fn option_acc(a: Vec3, save_acc: bool) -> Option<Vec3> {
    if save_acc {
        return Some(a);
    } else {
        return None;
    }
}
