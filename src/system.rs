use crate::body::Body;
use crate::close_encounter::CloseEncounter;
use crate::force::{self, ForceMethod};
use crate::integrator::Integrator;
use crate::timestep::{self, TimestepMethod};
use crate::vec3::Vec3;
use may::coroutine::{self, JoinHandle};
use may::sync::mpsc::{self, Receiver};
use std::collections::HashMap;
use std::sync::mpsc::SendError;

#[derive(Clone, Debug)]
pub struct System {
    pub t: f64,
    pub bodies: Vec<Body>,
    pub force_method: ForceMethod,
    pub integrator: Integrator,
    pub timestep_method: TimestepMethod,
    pub close_encounter: CloseEncounter,
    pub cache: HashMap<(usize, usize), Vec3>,
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

impl System {
    pub fn new(
        t: f64,
        bodies: Vec<Body>,
        force_method: ForceMethod,
        solve_method: Integrator,
        timestep_method: TimestepMethod,
        close_encounter: CloseEncounter,
    ) -> Self {
        let mut system = System {
            t,
            bodies,
            force_method,
            integrator: solve_method,
            timestep_method,
            close_encounter,
            cache: HashMap::new(),
        };
        for i in 0..system.bodies.len() {
            system.bodies[i].a = force::calc(
                &system.bodies[i],
                &system.bodies,
                &system.force_method,
                &system.close_encounter,
                Some(&mut system.cache),
            );
        }
        system
    }
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
    pub fn integrate(
        mut self,
        t_stop: f64,
    ) -> (Receiver<Data>, JoinHandle<Result<(), SendError<Data>>>) {
        let (tx, rx) = mpsc::channel::<Data>();
        let handle: JoinHandle<Result<(), SendError<Data>>> = unsafe {
            coroutine::spawn(move || {
                match self.timestep_method {
                    TimestepMethod::Constant(dt) => {
                        if dt <= 0.0 {
                            return Ok(());
                        }
                        let mut tmp = Vec::new();
                        let mut store = true;
                        while self.t < t_stop {
                            let percentage = self.t / t_stop;
                            let bodies = if store {
                                Some(self.bodies.clone())
                            } else {
                                None
                            };
                            let data = Data::new(bodies, percentage, self.t);
                            tx.send(data)?;
                            let proceed = timestep::constant_step(&mut self, dt, &mut tmp);
                            if proceed {
                                store = true;
                                self.t += dt;
                            } else {
                                store = false;
                            }
                        }
                    }
                };
                Ok(())
            })
        };
        (rx, handle)
    }
}
