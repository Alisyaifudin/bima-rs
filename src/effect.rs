use crate::record::Record;

pub trait Effect<E> {
    fn update(&mut self, t: f64, record: &mut Record) -> Result<(), E>;
}
