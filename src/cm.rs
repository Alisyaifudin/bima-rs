use crate::system::Body;
use crate::vec3::{Vec3, ZERO_VEC};

pub struct CM {
    pub r: Vec3,
    pub v: Vec3,
}

pub struct ZeroMass;

impl CM {
    pub fn from_bodies(bodies: &Vec<Body>) -> Result<Self, ZeroMass> {
        let m_total = bodies.iter().fold(0., |acc, e| acc + e.m);
        if m_total == 0.0 {
            return Err(ZeroMass);
        }
        let r = bodies.iter().fold(ZERO_VEC, |acc, e| e.m * e.r + acc) / m_total;
        let v = bodies.iter().fold(ZERO_VEC, |acc, e| e.m * e.v + acc) / m_total;
        Ok(CM { r, v })
    }
}
