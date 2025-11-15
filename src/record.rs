use crate::vec3::Vec3;

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct Record(pub Vec<Vec<Line>>);

impl Record {
    pub fn add(&mut self, i: usize, line: Line) {
        self.0[i].push(line);
    }
}