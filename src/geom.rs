use std::fmt;
use std::ops::{
    Add, AddAssign,
    Sub, SubAssign,
    Mul, MulAssign,
};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    pub fn x() -> Self {
        Self { x:1_f64, y:0_f64 }
    }
    pub fn y() -> Self {
        Self { x:0_f64, y:1_f64 }
    }
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<(f64,f64)> for Point2D {
    fn from(v: (f64, f64)) -> Self {
        Self { x: v.0, y: v.1 }
    }
}

impl Add for Point2D {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl AddAssign for Point2D {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Sub for Point2D {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl SubAssign for Point2D {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl Mul<f64> for Point2D {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}
impl MulAssign<f64> for Point2D {
    fn mul_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
        };
    }
}

impl FromStr for Point2D {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut itr = s.split(',');
        Ok(Self {
            x: itr.next().ok_or(()).map(|s| s.trim().parse::<f64>().map_err(|_| ()))??,
            y: itr.next().ok_or(()).map(|s| s.trim().parse::<f64>().map_err(|_| ()))??,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Ray2D {
    pub origin: Point2D,
    pub dir: Point2D,
}
impl Ray2D {
    pub fn new(origin: Point2D, dir: Point2D) -> Self {
        Self { origin, dir }
    }
    pub fn intersect_with(&self, other: &Ray2D) -> Option<Point2D> {
        let det = other.dir.x * self.dir.y - other.dir.y * self.dir.x;
        if det == 0.0 { return None; }

        let d = other.origin - self.origin;
        let u = (d.y * other.dir.x - d.x * other.dir.y) / det;
        let v = (d.y * self.dir.x - d.x * self.dir.y) / det;
        if u > 0.0 && v > 0.0 {
            Some(self.origin + self.dir * u)
        }
        else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn x() -> Self {
        Self { x:1_f64, y:0_f64, z:0_f64 }
    }
    pub fn y() -> Self {
        Self { x:0_f64, y:1_f64, z:0_f64 }
    }
    pub fn z() -> Self {
        Self { x:0_f64, y:0_f64, z:1_f64 }
    }

    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl fmt::Display for Point3D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Add for Point3D {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl AddAssign for Point3D {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl Sub for Point3D {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl SubAssign for Point3D {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

impl Mul<f64> for Point3D {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}
impl MulAssign<f64> for Point3D {
    fn mul_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        };
    }
}

impl FromStr for Point3D {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut itr = s.split(',');
        Ok(Self {
            x: itr.next().ok_or(()).map(|s| s.trim().parse::<f64>().map_err(|_| ()))??,
            y: itr.next().ok_or(()).map(|s| s.trim().parse::<f64>().map_err(|_| ()))??,
            z: itr.next().ok_or(()).map(|s| s.trim().parse::<f64>().map_err(|_| ()))??,
        })
    }
}
