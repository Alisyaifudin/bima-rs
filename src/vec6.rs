use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};
use crate::vec3::{ONE_VEC3, Vec3, ZERO_VEC3};

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Vec6 {
    pub r: Vec3,
    pub v: Vec3,
}

pub const ZERO_VEC6: Vec6 = Vec6 {
    r: ZERO_VEC3,
    v: ZERO_VEC3,
};

pub const ONE_VEC6: Vec6 = Vec6 {
    r: ONE_VEC3,
    v: ONE_VEC3,
};

impl Default for Vec6 {
    fn default() -> Self {
        ZERO_VEC6
    }
}

impl Vec6 {
    pub fn new(r: Vec3, v: Vec3) -> Self {
        Vec6 { r, v }
    }
    pub fn zero() -> Self {
        Vec6::default()
    }
    pub fn zeros(n: usize) -> Vec<Self> {
        vec![Vec6::zero(); n]
    }
    pub fn x(&self) -> f64 {
        self.r.x()
    }
    pub fn y(&self) -> f64 {
        self.r.y()
    }
    pub fn z(&self) -> f64 {
        self.r.z()
    }
    pub fn vx(&self) -> f64 {
        self.v.x()
    }
    pub fn vy(&self) -> f64 {
        self.v.y()
    }
    pub fn vz(&self) -> f64 {
        self.v.z()
    }
    pub fn to_str(&self) -> String {
        format!(
            "({:.9}, {:.9}, {:.9}), {:.9}), {:.9}), {:.9})",
            self.x(),
            self.y(),
            self.z(),
            self.vx(),
            self.vy(),
            self.vz()
        )
    }
    pub fn rel_err(&self, other: &Self) -> f64 {
        let diff = *self - *other;
        let mut max = (diff.x() / other.x()).abs() ;
        let y_err = (diff.y() / other.y()).abs();
        if max < y_err {
            max = y_err;
        } 
        let z_err = (diff.z() / other.z()).abs();
        if max < z_err {
            max = z_err;
        } 
        let vx_err = (diff.vx() / other.vx()).abs();
        if max < vx_err {
            max = vx_err
        } 
        let vy_err = (diff.vy() / other.vy()).abs();
        if max < vy_err {
            max = vy_err;
        } 
        let vz_err = (diff.vz() / other.vz()).abs();
        if max < vz_err {
            max = vz_err;
        } 
        max
    }
}

impl Add for Vec6 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec6 {
            r: self.r + rhs.r,
            v: self.v + rhs.v,
        }
    }
}

impl AddAssign for Vec6 {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.v += rhs.v;
    }
}
impl AddAssign<&Vec6> for Vec6 {
    fn add_assign(&mut self, rhs: &Self) {
        self.r += rhs.r;
        self.v += rhs.v;
    }
}

impl SubAssign for Vec6 {
    fn sub_assign(&mut self, rhs: Self) {
        self.r -= rhs.r;
        self.v -= rhs.v;
    }
}

impl Sub for Vec6 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec6 {
            r: self.r - rhs.r,
            v: self.v - rhs.v,
        }
    }
}
// impl<'a> Sub<&'a Vec6> for &'a Vec6 {
//     type Output = Self;
//     fn sub(self, rhs: Self) -> Self::Output {
//         &Vec6 {
//             r: self.r - rhs.r,
//             v: self.v - rhs.v,
//         }
//     }
// }

impl Mul<f64> for Vec6 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec6 {
            r: self.r * rhs,
            v: self.v * rhs,
        }
    }
}

impl Mul<Vec6> for f64 {
    type Output = Vec6;
    fn mul(self, rhs: Vec6) -> Self::Output {
        Vec6 {
            r: rhs.r * self,
            v: rhs.v * self,
        }
    }
}

impl Mul<Vec6> for Vec6 {
    type Output = Vec6;
    fn mul(self, rhs: Vec6) -> Self::Output {
        Vec6 {
            r: rhs.r * self.r,
            v: rhs.v * self.v,
        }
    }
}

impl Div<f64> for Vec6 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Vec6 {
            r: self.r / rhs,
            v: self.v / rhs,
        }
    }
}

impl Div<Vec6> for Vec6 {
    type Output = Self;
    fn div(self, rhs: Vec6) -> Self::Output {
        Vec6 {
            r: self.r / rhs.r,
            v: self.v / rhs.v,
        }
    }
}