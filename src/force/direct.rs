use crate::body::Body;
use crate::cm::CM;
use crate::force::Force;
use crate::force::gravity;
use crate::vec3::{self, Vec3};

#[derive(Debug, Clone)]
pub struct Direct {
    pub s: f64,
    pub bodies: Vec<Body>,
    pub cm: CM,
    pub n_active: usize,
    acc: Vec<Vec3>,
}

impl Direct {
    pub fn empty(s: f64) -> Self {
        Direct {
            s,
            bodies: Vec::new(),
            cm: CM::zero(),
            n_active: 0,
            acc: Vec::new(),
        }
    }
    pub fn new(s: f64, bodies: Vec<Body>, mut n_active: usize) -> Self {
        let n_total = bodies.len();
        if n_active > n_total {
            n_active = n_total;
        }
        let cm = CM::from_bodies(&bodies, n_active);
        let acc = vec![vec3::ZERO; n_total];
        Direct {
            s,
            bodies,
            cm,
            acc,
            n_active,
        }
    }
    fn force(&self, id: usize, r: &Vec3) -> Vec3 {
        let mut total_force = vec3::ZERO;
        let m = self.bodies[id].m;
        for other in self.bodies.iter() {
            if id == other.id {
                continue;
            }
            let force = gravity::call((m, r), (other.m, &other.r), self.s);
            total_force += force;
        }
        total_force
    }
}

impl Force for Direct {
    fn with_bodies(&self, bodies: Vec<Body>, n_active: usize) -> Direct {
        Direct::new(self.s, bodies, n_active)
    }
    fn len(&self) -> usize {
        self.bodies.len()
    }
    fn set_body(&mut self, id: usize, r: Vec3, v: Vec3) {
        self.bodies[id].r = r;
        self.bodies[id].v = v;
    }
    fn bodies(&self) -> &Vec<Body> {
        &self.bodies
    }
    fn body(&self, id: usize) -> &Body {
        &self.bodies[id]
    }
    fn cm(&self) -> &CM {
        &self.cm
    }

    // parallelization is only the best when not doing triangluar calculation (double counting tho)
    fn all(&mut self) {
        let n_total = self.bodies.len();
        self.acc.iter_mut().for_each(|b| *b = vec3::ZERO);
        for i in 0..n_total {
            let ri = &self.bodies[i].r;
            let mi = self.bodies[i].m;
            // let mut a = vec3::ZERO;
            for j in 0..self.n_active {
                if i == j {
                    continue;
                }
                let rj = &self.bodies[j].r;
                let mj = self.bodies[j].m;
                let force = gravity::call((mi, ri), (mj, rj), self.s);
                self.acc[i] += force / mi;
            }
        }
        self.bodies
            .iter_mut()
            .zip(self.acc.iter())
            .for_each(|(b, a)| b.a = *a);

        // self.acc.par_iter_mut().enumerate().for_each(|(i, ai)| {
        //     let ri = &self.bodies[i].r;
        //     let mi = self.bodies[i].m;
        //     for j in 0..self.n_active {
        //         if i == j {
        //             continue;
        //         }
        //     }
        //     *ai = a;
        // });
        // self.bodies
        //     .par_iter_mut()
        //     .enumerate()
        //     .for_each(|(i, body)| {
        //         body.a = self.acc[i];
        //     });
    }
    fn one(&self, id: usize, r: &Vec3) -> Vec3 {
        let m = self.bodies[id].m;
        self.force(id, r) / m
    }
}
