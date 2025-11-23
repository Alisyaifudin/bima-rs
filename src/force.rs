use crate::body::Body;
use crate::close_encounter::CloseEncounter;
use crate::vec3::{Vec3, ZERO_VEC3};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum ForceMethod {
    Direct,
    Octree,
}

impl ForceMethod {
    pub fn new_octree() -> ForceMethod {
        ForceMethod::Octree
    }
}

// #[derive(Clone, Debug)]
// pub struct Tree;

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

pub fn direct(
    body: &Body,
    bodies: &Vec<Body>,
    close_encounter: &CloseEncounter,
    mut cache: Option<&mut HashMap<(usize, usize), Vec3>>,
) -> Vec3 {
    let mut total_force = ZERO_VEC3;

    for other in bodies.iter() {
        if body.id == other.id {
            continue;
        }

        // Check cache if it exists
        if let Some(cache_ref) = cache.as_ref() {
            if let Some(force) = cache_ref.get(&(body.id, other.id)) {
                total_force += *force;
                continue;
            }
        }

        let force = gravity(body, other, close_encounter);
        total_force += force;

        // Store in cache if it exists
        if let Some(cache_ref) = cache.as_mut() {
            cache_ref.insert((other.id, body.id), -1. * force);
            cache_ref.insert((body.id, other.id), force);
        }
    }

    total_force / body.m
}

pub fn calc(
    body: &Body,
    bodies: &Vec<Body>,
    force_method: &ForceMethod,
    close_encounter: &CloseEncounter,
    cache: Option<&mut HashMap<(usize, usize), Vec3>>,
) -> Vec3 {
    match force_method {
        ForceMethod::Direct => direct(body, bodies, close_encounter, cache),
        ForceMethod::Octree => todo!(),
    }
}
