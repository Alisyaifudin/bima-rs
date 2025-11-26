use crate::body::Body;
use crate::vec3::{Vec3, ZERO};

#[derive(Debug, Clone)]
pub struct CM {
    pub r: Vec3,
    pub m: f64,
}

impl CM {
    pub fn zero() -> Self {
        CM { r: ZERO, m: 0. }
    }
    pub fn from_bodies(bodies: &[Body], n_active: usize) -> Self {
        if bodies.len() == 0 || n_active > bodies.len() {
            return CM::zero();
        }
        let m_total =
            bodies.iter().enumerate().fold(
                0.,
                |acc, (i, e)| {
                    if i >= n_active { acc } else { acc + e.m }
                },
            );
        let r = bodies.iter().fold(ZERO, |acc, e| e.m * e.r + acc) / m_total;
        CM { r, m: m_total }
    }
    pub fn extend_one(&mut self, body: &Body) {
        let m_total = body.m + self.m;
        let r_new = self.r * self.m + body.r * body.m;
        *self = CM {
            r: r_new / m_total,
            m: m_total,
        };
    }
    pub fn extend(&mut self, bodies: &[Body], n_active: usize) {
        let cm = CM::from_bodies(bodies, n_active);
        let m_total = cm.m + self.m;
        let r_new = self.r * self.m + cm.r * cm.m;
        *self = CM {
            r: r_new / m_total,
            m: m_total,
        };
    }
    pub fn x(&self) -> f64 {
        self.r.x()
    }
    pub fn y(&self) -> f64 {
        self.r.y()
    }
    pub fn z(&self) -> f64 {
        self.r.z()
    }
}
