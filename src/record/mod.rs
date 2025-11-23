pub mod line;
pub mod trajectory;
pub mod utils;
use crate::body::Body;
use crate::cm::CM;
use line::Line;
use trajectory::Trajectory;

#[derive(Clone, Debug)]
pub struct Record {
    pub objects: Vec<Trajectory>,
    pub save_acc: bool,
}

impl Record {
    pub fn empty(masses: &Vec<f64>, save_acc: bool) -> Self {
        Record {
            objects: masses.iter().map(|m| Trajectory::empty(*m)).collect(),
            save_acc,
        }
    }
    pub fn new(bodies: Vec<Body>, save_acc: bool) -> Self {
        let n = bodies.len();
        let mut trajectories = Vec::with_capacity(n);
        for b in bodies {
            trajectories.push(Trajectory::new(&b, save_acc));
        }
        let record = Record {
            objects: trajectories,
            save_acc,
        };
        record
    }
    pub fn from_trajectories(trajectories: Vec<Trajectory>, save_acc: bool) -> Self {
        Record {
            objects: trajectories,
            save_acc,
        }
    }
    pub fn len(&self) -> usize {
        self.objects.len()
    }
    pub fn take(&mut self, i: usize) -> Trajectory {
        std::mem::take(&mut self.objects[i])
    }
    pub fn add(&mut self, i: usize, line: Line) {
        self.objects[i].push(line);
    }
    pub fn add_many(&mut self, i: usize, lines: Vec<Line>) {
        self.objects[i].extend(lines);
    }
    pub fn to_vec(self, cm: &CM) -> Vec<Vec<Vec<f64>>> {
        self.objects
            .into_iter()
            .map(|trajectory| trajectory.to_vec(cm))
            .collect()
    }
}
