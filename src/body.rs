use crate::{vec3::{Vec3, ZERO_VEC3}, vec6::Vec6};


#[derive(Clone, Debug, Default)]
pub struct Body {
    pub m: f64,
    pub r: Vec3,
    pub v: Vec3,
    pub a: Vec3,
    pub id: usize,
}

impl Body {
    pub fn new(id: usize, m: f64, r: Vec3, v: Vec3, a: Option<Vec3>) -> Self {
        Body {
            id,
            m,
            r,
            v,
            a: a.unwrap_or(ZERO_VEC3),
        }
    }
    pub fn empty() -> Self {
        Body {
            id: 0,
            m: 0.,
            r: ZERO_VEC3,
            v: ZERO_VEC3,
            a: ZERO_VEC3,
        }
    }
    pub fn from_vec6(w: Vec6, id: usize, m: f64) -> Self {
      Body {
        id,
        m,
        r: w.r,
        v: w.v,
        a: ZERO_VEC3
      }
    }
    pub fn to_vec6(&self) -> Vec6 {
      Vec6::new(self.r, self.v)
    }
}