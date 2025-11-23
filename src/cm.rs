use crate::body::Body;
use crate::vec3::{Vec3, ZERO_VEC3};

pub struct CM(Vec3);

pub struct ZeroMass;

impl CM {
    pub fn r(&self) -> Vec3 {
        self.0
    }
    pub fn from_bodies(bodies: &[Body]) -> Result<Self, ZeroMass> {
        let m_total = bodies.iter().fold(0., |acc, e| acc + e.m);
        if m_total == 0.0 {
            return Err(ZeroMass);
        }
        let r = bodies.iter().fold(ZERO_VEC3, |acc, e| e.m * e.r + acc) / m_total;
        Ok(CM(r))
    }
    pub fn x(&self) -> f64 {
        self.0.x()
    }
    pub fn y(&self) -> f64 {
        self.0.y()
    }
    pub fn z(&self) -> f64 {
        self.0.z()
    }
}
