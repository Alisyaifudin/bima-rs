use crate::record::Record;
use crate::record::line::Line;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct EmptyRecord;

pub fn get_time_series(record: &Record) -> Result<Vec<f64>, EmptyRecord> {
    let index = record.objects.iter().min_by_key(|o| o.path.len());
    if let Some(trajectory) = index {
        let t = trajectory.path.iter().map(|p| p.t).collect();
        return Ok(t);
    }
    return Err(EmptyRecord);
}

fn get_line_idx(t: f64, path: &[Line], index: usize) -> Option<usize> {
    for i in index..path.len() {
        if path[i].t >= t {
            // If this is the first point or we're exactly at a data point
            if i == 0 || path[i].t == t {
                return Some(i);
            } else {
                // t is between path[i-1] and path[i]
                // Return the previous line segment for interpolation
                return Some(i - 1);
            }
        }
    }
    None
}

pub struct Data {
    pub time: f64,
    pub energy: f64,
}

impl Data {
    fn new(time: f64, energy: f64) -> Self {
        Data { time, energy }
    }
}

pub struct Energy {
    record: Record,
    n_active: usize,
    ts: Vec<f64>,
    pub length: usize,
}

impl Energy {
    pub fn new(record: Record, mut n_active: usize) -> Result<Self, EmptyRecord> {
        if record.len() == 0 {
            return Err(EmptyRecord);
        }
        let number_of_objects = record.len();
        if n_active > number_of_objects {
            n_active = number_of_objects;
        }
        let ts = get_time_series(&record).unwrap();
        let length = ts.len();
        Ok(Self {
            record,
            n_active,
            ts,
            length,
        })
    }
    pub fn into_iter(self) -> EnergyIter {
        let indices = vec![0; self.n_active];
        EnergyIter {
            record: self.record,
            it: 0,
            length: self.length,
            ts: self.ts,
            n_active: self.n_active,
            indices,
        }
    }
}

pub struct EnergyIter {
    record: Record,
    it: usize,
    ts: Vec<f64>,
    length: usize,
    n_active: usize,
    indices: Vec<usize>,
}

impl Iterator for EnergyIter {
    type Item = Data;
    fn next(&mut self) -> Option<Self::Item> {
        let record = &self.record;
        let indices = &mut self.indices;
        while self.it < self.length {
            let t = self.ts[self.it];
            self.it += 1;
            // Calculate kinetic energy
            let Ok(kinetic): Result<f64, MissingTime> = indices
                .iter_mut()
                .enumerate()
                .map(|(i, index)| {
                    let object = &record.objects[i];
                    let idx = get_line_idx(t, &object.path, *index)?;
                    *index = idx;
                    let v = &object.path[idx].v;
                    Some(kinetic_energy(object.mass, v))
                })
                .try_fold(0., |acc, e| {
                    let b = e.ok_or(MissingTime)?;
                    Ok(acc + b)
                })
            else {
                // some data missing t
                continue;
            };
            // all time series already confirm exist above
            let potential: f64 = indices
                .iter()
                .enumerate()
                .map(|(i, index)| {
                    let object_i = &record.objects[i];
                    let ri = &object_i.path[*index].r;
                    let mi = object_i.mass;
                    let potential: f64 = (0..self.n_active)
                        .into_iter()
                        .map(|j| {
                            if i == j {
                                return 0.;
                            }
                            let object_j = &record.objects[j];
                            let rj = &object_j.path[indices[j]].r;
                            let mj = object_j.mass;
                            potential_energy((mi, ri), (mj, rj))
                        })
                        .sum();
                    potential
                })
                .sum();
            // eprintln!("5");
            let e = kinetic - potential;
            return Some(Data::new(t, e));
        }
        None
    }
}

pub struct MissingTime;

// pub fn calc_energy(
//     record: Record,
//     ts: Vec<f64>,
//     n_active: usize,
// ) -> Result<(Receiver<Data>, JoinHandle<Result<(), SendError<Data>>>), EmptyRecord> {
//     // eprintln!("0");
//     let energy = Energy::new(record, ts, n_active);
//     let (tx, rx) = mpsc::channel::<Data>();
//     // eprintln!("1");
//     let handle: JoinHandle<Result<(), SendError<Data>>> = unsafe {
//         coroutine::spawn(move || {
//             // eprintln!("2");
//             for t in ts.into_iter() {
//                 // Calculate kinetic energy
//                 // eprintln!("3");
//                 let Ok(kinetic): Result<f64, MissingTime> = indices
//                     .iter_mut()
//                     .enumerate()
//                     .map(|(i, index)| {
//                         let object = &record.objects[i];
//                         let idx = get_line_idx(t, &object.path, *index)?;
//                         *index = idx;
//                         let v = &object.path[idx].v;
//                         Some(kinetic_energy(object.mass, v))
//                     })
//                     .try_fold(0., |acc, e| {
//                         let b = e.ok_or(MissingTime)?;
//                         Ok(acc + b)
//                     })
//                 else {
//                     // some data missing t
//                     continue;
//                 };
//                 // eprintln!("4");
//                 // all time series already confirm exist above
//                 let potential: f64 = indices
//                     .iter()
//                     .enumerate()
//                     .map(|(i, index)| {
//                         let object_i = &record.objects[i];
//                         let ri = &object_i.path[*index].r;
//                         let mi = object_i.mass;
//                         let potential: f64 = (0..n_active)
//                             .into_iter()
//                             .map(|j| {
//                                 if i == j {
//                                     return 0.;
//                                 }
//                                 let object_j = &record.objects[j];
//                                 let rj = &object_j.path[indices[j]].r;
//                                 let mj = object_j.mass;
//                                 potential_energy((mi, ri), (mj, rj))
//                             })
//                             .sum();
//                         potential
//                     })
//                     .sum();
//                 // eprintln!("5");
//                 let e = kinetic - potential;
//                 // eprintln!("{t}: kinetic: {kinetic}\t potential: {potential}\t total: {total}");
//                 let _ = tx.send(Data::new(t, e))?;
//             }
//             Ok(())
//         })
//     };
//     Ok((rx, handle))
// }

fn kinetic_energy(m: f64, v: &Vec3) -> f64 {
    0.5 * m * v.norm_2()
}

fn potential_energy(o1: (f64, &Vec3), o2: (f64, &Vec3)) -> f64 {
    let r = *o1.1 - *o2.1;
    return o1.0 * o2.0 / r.norm();
}
