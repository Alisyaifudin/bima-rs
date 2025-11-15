use crate::close_encounter::CloseEncounter;
use crate::system::{Body,  System};
use crate::vec3::{Vec3, ZERO_VEC};


#[derive(Clone, Debug)]
pub enum ForceMethod {
    Direct,
    Octree(Tree),
}


#[derive(Clone, Debug)]
pub struct Tree;

impl ForceMethod {
    pub fn new_octree() -> ForceMethod {
        ForceMethod::Octree(Tree)
    }
}
// force the first body felt
pub fn gravity(b1: &Body, b2: &Body, close_encounter: &CloseEncounter) -> Vec3 {
    let r = b2.r - b1.r;
    let rhat = r.hat();
    let r2 = r.norm_2();
    let divisor = match close_encounter {
        CloseEncounter::Regularized => r2,
        CloseEncounter::Soften(s) => r2 + s * s,
        CloseEncounter::Truncated(s) => r2.max(s * s),
    };
    let value = b1.m * b2.m / divisor;
    value * rhat
}

pub fn direct(system: &mut System) {
    let n = system.bodies.len();
    for body in system.bodies.iter_mut() {
        body.a = ZERO_VEC;
    }
    for i in 0..n {
        for j in (i + 1)..n {
            let b1 = &system.bodies[i];
            let b2 = &system.bodies[j];
            let m1 = b1.m;
            let m2 = b2.m;
            let f = gravity(b1, b2, &system.close_encounter);
            system.bodies[i].a += f / m1;
            system.bodies[j].a -= f / m2;
        }
    }
}
