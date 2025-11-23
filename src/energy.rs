use crate::effect::{Effect, PayloadRef};
use crate::record::Record;
use crate::record::line::Line;
use std::fmt::Display;

pub struct EmptyRecord;

fn get_t(record: &Record) -> Result<Vec<f64>, EmptyRecord> {
    let index = record.objects.iter().min_by_key(|o| o.path.len());
    if let Some(trajectory) = index {
        let t = trajectory.path.iter().map(|p| p.t).collect();
        return Ok(t);
    }
    return Err(EmptyRecord);
}

fn get_line<'p>(t: f64, path: &'p Vec<Line>, index: &mut usize) -> Option<&'p Line> {
    for i in *index..path.len() {
        if path[i].t >= t {
            // If this is the first point or we're exactly at a data point
            if i == 0 || path[i].t == t {
                *index = i;
                return Some(&path[i]);
            } else {
                // t is between path[i-1] and path[i]
                // Return the previous line segment for interpolation
                *index = i - 1;
                return Some(&path[i - 1]);
            }
        }
    }
    None
}

// store (time, energy)
pub fn calc_energy<E: Display, Eff: Effect<E, usize, usize>>(
    record: &Record,
    effect: &mut Eff,
) -> Result<Vec<(f64, f64)>, EmptyRecord> {
    let ts = get_t(record)?;
    let total_step = ts.len();
    let number_of_objects = record.len();
    let mut indices = vec![0; number_of_objects];
    let mut totals = Vec::with_capacity(ts.len());

    't_level: for (it, t) in ts.into_iter().enumerate() {
        let mut kinetics = 0.0;
        let mut potentials = 0.0;

        // Calculate kinetic energy
        for i in 0..number_of_objects {
            let object = &record.objects[i];
            let Some(line) = get_line(t, &object.path, &mut indices[i]) else {
                continue 't_level;
            };
            let kinetic = kinetic_energy(object.mass, line);
            kinetics += kinetic;
        }
        // calculate potential energy
        for i in 0..number_of_objects {
            for j in (i + 1)..number_of_objects {
                let object_i = &record.objects[i];
                let object_j = &record.objects[j];
                let Some(line_i) = get_line(t, &object_i.path, &mut indices[i]) else {
                    continue 't_level;
                };
                let Some(line_j) = get_line(t, &object_j.path, &mut indices[j]) else {
                    continue 't_level;
                };
                potentials += potential_energy((object_i.mass, line_i), (object_j.mass, line_j));
            }
        }

        let total = kinetics - potentials;
        totals.push((t, total));

        if let Err(e) = effect.update(it, PayloadRef::Ref(&(total_step - 1))) {
            eprintln!("Some error on update: {}", e);
        }
    }

    Ok(totals)
}

fn kinetic_energy(m: f64, line: &Line) -> f64 {
    0.5 * m * line.v.norm_2()
}

fn potential_energy(o1: (f64, &Line), o2: (f64, &Line)) -> f64 {
    let r = o1.1.r - o2.1.r;
    return o1.0 * o2.0 / r.norm();
}
