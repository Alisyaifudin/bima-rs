use crate::close_encounter::CloseEncounter;
use crate::force::{self, ForceMethod};
use crate::solve::SolveMethod;
use crate::timestep::TimestepMethod;
use crate::vec3::{Vec3, ZERO_VEC};

#[derive(Clone, Debug)]
pub struct Body {
    pub m: f64,
    pub r: Vec3,
    pub v: Vec3,
    pub a: Vec3,
}

impl Body {
    pub fn new(m: f64, r: Vec3, v: Vec3) -> Self {
        Body {
            m,
            r,
            v,
            a: ZERO_VEC,
        }
    }
}

#[derive(Clone, Debug)]
pub struct System {
    pub t: f64,
    pub bodies: Vec<Body>,
    pub force_method: ForceMethod,
    pub solve_method: SolveMethod,
    pub timestep_method: TimestepMethod,
    pub close_encounter: CloseEncounter,
    pub save_acc: bool,
}

impl System {
    pub fn calc_forces(&mut self) {
        match self.force_method {
            ForceMethod::Direct => force::direct(self),
            ForceMethod::Octree(_) => todo!(),
        }
    }
}
