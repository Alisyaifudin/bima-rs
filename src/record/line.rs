use crate::vec3::Vec3;

#[derive(Clone, Debug, Default)]
pub struct Line {
    pub t: f64,
    pub r: Vec3,
    pub v: Vec3,
    pub a: Option<Vec3>,
}

impl Line {
    pub fn new(t: f64, r: Vec3, v: Vec3, a: Option<Vec3>) -> Self {
        Line { t, r, v, a }
    }
}
