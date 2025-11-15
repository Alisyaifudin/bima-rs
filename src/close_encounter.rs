
#[derive(Clone, Debug)]
pub enum CloseEncounter {
    Truncated(f64),
    Soften(f64),
    Regularized,
}