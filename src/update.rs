use crate::effect::Effect;
use crate::record::Record;
use crate::system::System;
use crate::timestep::{self, TimestepMethod};

pub enum UpdateErr<EffErr, SigErr> {
    SignalErr(SigErr),
    EffectErr(EffErr),
}

pub fn update_loop<EffErr, SigErr, E: Effect<EffErr>>(
    signal: impl Fn() -> Result<(), SigErr>,
    effect: &mut E,
    system: &mut System,
    t_stop: f64,
    record: &mut Record,
) -> Result<(), UpdateErr<EffErr, SigErr>> {
    match system.timestep_method {
        TimestepMethod::Constant(delta_t) => {
            if delta_t <= 0.0 {
                return Ok(());
            }
            while system.t < t_stop {
                signal().map_err(|e| UpdateErr::SignalErr(e))?;
                system.calc_forces();
                timestep::constant_step(system, delta_t, record);
                system.t += delta_t;
                effect
                    .update(system.t, record)
                    .map_err(|e| UpdateErr::EffectErr(e))?;
            }
        }
    };
    Ok(())
}
