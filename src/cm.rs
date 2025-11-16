use crate::system::Body;
use crate::vec3::{Vec3, ZERO_VEC};

pub struct CM(pub Vec3);

pub struct ZeroMass;

impl CM {
    pub fn from_bodies(bodies: &[Body]) -> Result<Self, ZeroMass> {
        let m_total = bodies.iter().fold(0., |acc, e| acc + e.m);
        if m_total == 0.0 {
            return Err(ZeroMass);
        }
        let r = bodies.iter().fold(ZERO_VEC, |acc, e| e.m * e.r + acc) / m_total;
        Ok(CM(r))
    }
}
