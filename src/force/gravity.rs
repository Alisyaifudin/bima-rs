use crate::vec3::Vec3;

// force the first body felt
pub fn call(b0: (f64, &Vec3), b1: (f64, &Vec3), s: f64) -> Vec3 {
    let r = *b1.1 - *b0.1;
    let rhat = r.hat();
    let r2 = r.norm_2();
    let value = b0.0 * b1.0 / (r2 + s * s);
    value * rhat
}
