use crate::record::Record;
use crate::record::line::Line;
use crate::vec3::Vec3;
use may::coroutine::{self, JoinHandle};
use may::sync::mpsc::{self, Receiver};
use std::sync::mpsc::SendError;

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
    fn new(system_t: f64, system_e: f64) -> Self {
        Data {
            time: system_t,
            energy: system_e,
        }
    }
}

pub struct MissingTime;

pub fn calc_energy(
    record: Record,
    ts: Vec<f64>,
    mut n_active: usize,
) -> Result<(Receiver<Data>, JoinHandle<Result<(), SendError<Data>>>), EmptyRecord> {
    // let total_step = ts.len();
    let number_of_objects = record.len();
    if n_active > number_of_objects {
        n_active = number_of_objects;
    }
    let mut indices = vec![0; n_active];
    // eprintln!("0");
    // let mut totals = Vec::with_capacity(ts.len());
    let (tx, rx) = mpsc::channel::<Data>();
    // eprintln!("1");
    let handle: JoinHandle<Result<(), SendError<Data>>> = unsafe {
        coroutine::spawn(move || {
            // eprintln!("2");
            for t in ts.into_iter() {
                // Calculate kinetic energy
                // eprintln!("3");
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
                // eprintln!("4");
                // all time series already confirm exist above
                let potential: f64 = indices
                    .iter()
                    .enumerate()
                    .map(|(i, index)| {
                        let object_i = &record.objects[i];
                        let ri = &object_i.path[*index].r;
                        let mi = object_i.mass;
                        let potential: f64 = (0..n_active)
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
                // eprintln!("{t}: kinetic: {kinetic}\t potential: {potential}\t total: {total}");
                let _ = tx.send(Data::new(t, e))?;
            }
            Ok(())
        })
    };
    Ok((rx, handle))
}

fn kinetic_energy(m: f64, v: &Vec3) -> f64 {
    0.5 * m * v.norm_2()
}

fn potential_energy(o1: (f64, &Vec3), o2: (f64, &Vec3)) -> f64 {
    let r = *o1.1 - *o2.1;
    return o1.0 * o2.0 / r.norm();
}
