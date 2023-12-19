use std::ops::{
    Add, AddAssign,
    Sub, SubAssign,
    Mul, MulAssign,
};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum CDir {
    N, E, S, W,
}
impl CDir {
    pub fn left(&self) -> Self {
        match self {
            CDir::N => CDir::W,
            CDir::E => CDir::N,
            CDir::S => CDir::E,
            CDir::W => CDir::S,
        }
    }
    pub fn right(&self) -> Self {
        match self {
            CDir::N => CDir::E,
            CDir::E => CDir::S,
            CDir::S => CDir::W,
            CDir::W => CDir::N,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Turn {
    L, R,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct Coord2D {
    pub x: i64,
    pub y: i64,
}
impl Coord2D {
    pub fn new(x: i64, y: i64) -> Self {
        Coord2D { x, y }
    }
    pub fn neighbors4(&self) -> Vec<Self> {
        [ Coord2D::new(-1, 0), Coord2D::new(1, 0), Coord2D::new(0, -1), Coord2D::new(0, 1) ]
            .iter()
            .map(|o| *self + *o)
            .collect()
    }
    pub fn neighbors8(&self) -> Vec<Self> {
        [ Coord2D::new(-1, -1), Coord2D::new(0, 1), Coord2D::new(1, -1),
          Coord2D::new(-1, 0),                       Coord2D::new(1, 0),
          Coord2D::new(-1, 1), Coord2D::new(0, -1), Coord2D::new(1, 1) ]
            .iter()
            .map(|o| *self + *o)
            .collect()
    }
    pub fn mdist_to(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl From<CDir> for Coord2D {
    fn from(value: CDir) -> Self {
        match value {
            CDir::N => Coord2D::new(0, -1),
            CDir::E => Coord2D::new(1, 0),
            CDir::S => Coord2D::new(0, 1),
            CDir::W => Coord2D::new(-1, 0),
        }
    }
}

impl Add for Coord2D {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Add<CDir> for Coord2D {
    type Output = Self;
    fn add(self, other: CDir) -> Self {
        let other: Coord2D = other.into();
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl AddAssign for Coord2D {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}
impl AddAssign<CDir> for Coord2D {
    fn add_assign(&mut self, other: CDir) {
        let other: Coord2D = other.into();
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Sub for Coord2D {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl Sub<CDir> for Coord2D {
    type Output = Self;
    fn sub(self, other: CDir) -> Self {
        let other: Coord2D = other.into();
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl SubAssign for Coord2D {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}
impl SubAssign<CDir> for Coord2D {
    fn sub_assign(&mut self, other: CDir) {
        let other: Coord2D = other.into();
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl Mul<i64> for Coord2D {
    type Output = Self;
    fn mul(self, other: i64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}
impl MulAssign<i64> for Coord2D {
    fn mul_assign(&mut self, other: i64) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
        };
    }
}

impl FromStr for Coord2D {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut itr = s.split(',');
        Ok(Self {
            x: itr.next().ok_or(()).map(|s| s.trim().parse::<i64>().map_err(|_| ()))??,
            y: itr.next().ok_or(()).map(|s| s.trim().parse::<i64>().map_err(|_| ()))??,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Coord3D {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}
impl Coord3D {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Coord3D { x, y, z }
    }
}

impl Add for Coord3D {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl AddAssign for Coord3D {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl Sub for Coord3D {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl SubAssign for Coord3D {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

impl Mul<i64> for Coord3D {
    type Output = Self;
    fn mul(self, other: i64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}
impl MulAssign<i64> for Coord3D {
    fn mul_assign(&mut self, other: i64) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        };
    }
}

impl FromStr for Coord3D {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut itr = s.split(',');
        Ok(Self {
            x: itr.next().ok_or(()).map(|s| s.trim().parse::<i64>().map_err(|_| ()))??,
            y: itr.next().ok_or(()).map(|s| s.trim().parse::<i64>().map_err(|_| ()))??,
            z: itr.next().ok_or(()).map(|s| s.trim().parse::<i64>().map_err(|_| ()))??,
        })
    }
}
