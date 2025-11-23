use crate::vec3::Vec3;

pub fn some_acc(a: Vec3, saved: bool) -> Option<Vec3> {
    if saved { Some(a) } else { None }
}