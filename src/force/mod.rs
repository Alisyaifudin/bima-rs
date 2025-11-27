mod direct;
mod gravity;
use crate::{body::Body, cm::CM, vec3::Vec3};
pub use direct::Direct;

#[derive(Clone, Debug)]
pub enum Method {
    Direct,
    Octree,
}

pub trait Force {
    fn with_bodies(&self, bodies: Vec<Body>, n_active: usize) -> Self;
    fn len(&self) -> usize;
    fn bodies(&self) -> &Vec<Body>;
    fn set_body(&mut self, id: usize, r: Vec3, v: Vec3);
    fn body(&self, id: usize) -> &Body;
    fn cm(&self) -> &CM;
    fn all(&mut self);
    fn one(&self, id: usize, r: &Vec3) -> Vec3;
}

// #[derive(Clone, Debug)]
// pub struct Force {
//     method: Method,
//     pub s: f64,
//     pub bodies: Vec<Body>,
//     pub cm: CM,
//     pub length: usize,
//     acc: Vec<Vec3>,
// }

// impl Force {
//     pub fn octree(bodies: Vec<Body>, s: f64) -> Self {
//         let n = bodies.len();
//         let cm = CM::from_bodies(&bodies);
//         let force = Self {
//             method: Method::Octree,
//             s,
//             length: bodies.len(),
//             bodies,
//             cm,
//             acc: vec![ZERO; n],
//         };
//         force
//     }
//     pub fn direct(bodies: Vec<Body>, s: f64) -> Self {
//         let n = bodies.len();
//         let cm = CM::from_bodies(&bodies);
//         let acc = vec![ZERO; n];
//         let force = Self {
//             method: Method::Octree,
//             length: bodies.len(),
//             s,
//             cm,
//             bodies,
//             acc,
//         };
//         force
//     }
//     pub fn from_force(mut force: Force, bodies: Vec<Body>) -> Self {
//         force.bodies = bodies;
//         force
//     }
//     pub fn len(&self) -> usize {
//         self.length
//     }
// }

// #[derive(Clone, Debug)]
// pub struct Tree;
