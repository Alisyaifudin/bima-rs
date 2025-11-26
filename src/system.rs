use crate::body::Body;
use crate::force::Force;
use crate::integrator::{Integrator, Solution};
use may::coroutine::{self, JoinHandle};
use may::sync::mpsc::{self, Receiver};
use std::sync::mpsc::SendError;
use std::usize;

#[derive(Clone, Debug)]
pub struct System<F: Force + Send + 'static, I: Integrator<F> + Send + 'static> {
    pub t: f64,
    pub force: F,
    pub integrator: I,
    buff: Vec<Solution>,
}

pub struct Data {
    pub bodies: Option<Vec<Body>>,
    pub percentage: f64,
    pub t: f64,
}

impl Data {
    fn new(bodies: Option<Vec<Body>>, percentage: f64, t: f64) -> Self {
        Data {
            bodies,
            percentage,
            t,
        }
    }
}

impl<F: Force + Send + 'static, I: Send + Integrator<F> + 'static> System<F, I> {
    pub fn new(force: F, integrator: I) -> Self {
        let n = force.bodies().len();
        let system = Self {
            t: 0.,
            force,
            buff: vec![Solution::empty(); n],
            integrator,
        };
        system
    }
    pub fn integrate(
        mut self,
        t_stop: f64,
        chunk: Option<usize>,
    ) -> (Receiver<Data>, JoinHandle<Result<(), SendError<Data>>>) {
        let (tx, rx) = mpsc::channel::<Data>();
        let handle: JoinHandle<Result<(), SendError<Data>>> = unsafe {
            coroutine::spawn(move || {
                let mut it = 0;
                let total_chunk = chunk.unwrap_or(usize::MAX);
                while self.t <= t_stop && it < total_chunk {
                    // eprintln!("=========================== 0");
                    // // send the last position
                    let (calc_force, send) = self.integrator.pre();
                    if calc_force {
                        self.force.all();
                    }
                    // eprintln!("=========================== 1");
                    let percentage = self.t / t_stop;
                    let bodies = if send {
                        it += 1;
                        Some(self.force.bodies().clone())
                    } else {
                        None
                    };
                    // eprintln!("=========================== 2");
                    let data = Data::new(bodies, percentage, self.t);
                    tx.send(data)?;
                    // eprintln!("=========================== 3");
                    // calculate next position
                    self.buff.iter_mut().enumerate().for_each(|(i, dummy)| {
                        let body = self.force.body(i);
                        let sol =
                            self.integrator
                                .call(body.id, body.a, body.to_vec6(), &self.force);
                        *dummy = sol;
                    });
                    // eprintln!("=========================== 4");
                    // update current position and get min dt
                    let dt_min = self
                        .force
                        .bodies_mut()
                        .iter_mut()
                        .enumerate()
                        .map(|(i, body)| {
                            body.v = self.buff[i].v;
                            body.r = self.buff[i].r;
                            self.buff[i].dt
                        })
                        .reduce(f64::min);
                    // set new dt
                    // eprintln!("=========================== 5");
                    self.integrator
                        .set_dt(dt_min.unwrap_or(self.integrator.dt()));
                    let update = self.integrator.post();
                    if update {
                        self.t += self.integrator.dt();
                    }
                    // eprintln!("=========================== 6");
                }
                Ok(())
            })
        };
        (rx, handle)
    }
}
