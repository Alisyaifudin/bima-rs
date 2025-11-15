pub trait Effect {
    fn update(&mut self, t: f64);
}
