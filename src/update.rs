use crate::effect::Effect;
use crate::record::Record;
use crate::system::System;
use crate::timestep::{self, TimestepMethod};

pub struct SignalErr<E>(pub E);

pub fn update_loop<Err, E: Effect>(
    signal: impl Fn() -> Result<(), SignalErr<Err>>,
    effect: &mut E,
    system: &mut System,
    t_stop: f64,
    record: &mut Record,
) -> Result<(), SignalErr<Err>> {
    match system.timestep_method {
        TimestepMethod::Constant(delta_t) => {
            if delta_t <= 0.0 {
                return Ok(());
            }
            while system.t < t_stop {
                signal()?;
                system.calc_forces();
                timestep::constant_step(system, delta_t, record);
                system.t += delta_t;
                effect.update(system.t);
            }
        }
    };
    Ok(())
}
