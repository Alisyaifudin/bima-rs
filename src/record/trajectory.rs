use crate::cm::CM;
use crate::record::line::Line;
use crate::record::utils::some_acc;
use crate::body::Body;

#[derive(Clone, Debug, Default)]
pub struct Trajectory {
    pub path: Vec<Line>,
    pub mass: f64,
}

impl Trajectory {
    pub fn empty(mass: f64) -> Self {
        Trajectory { path: vec![], mass }
    }
    pub fn new(body: &Body, save_acc: bool) -> Self {
        let a = some_acc(body.a, save_acc);
        Trajectory {
            path: vec![Line::new(0.0, body.r, body.v, a)],
            mass: body.m,
        }
    }
    pub fn from_lines(lines: Vec<Line>, mass: f64) -> Self {
        Trajectory { path: lines, mass }
    }
    pub fn push(&mut self, line: Line) {
        self.path.push(line);
    }
    pub fn extend(&mut self, lines: Vec<Line>) {
        self.path.extend(lines);
    }
    pub fn to_vec(self, cm: &CM) -> Vec<Vec<f64>> {
        self.path
            .into_iter()
            .map(|line| {
                if let Some(a) = line.a {
                    vec![
                        line.t,
                        line.r.x() + cm.x(),
                        line.r.y() + cm.y(),
                        line.r.z() + cm.z(),
                        line.v.x(),
                        line.v.y(),
                        line.v.z(),
                        a.x(),
                        a.y(),
                        a.z(),
                    ]
                } else {
                    vec![
                        line.t,
                        line.r.x() + cm.x(),
                        line.r.y() + cm.y(),
                        line.r.z() + cm.z(),
                        line.v.x(),
                        line.v.y(),
                        line.v.z(),
                    ]
                }
            })
            .collect()
    }
}
