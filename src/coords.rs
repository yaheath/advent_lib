use std::ops::{
    Add, AddAssign,
    Sub, SubAssign,
    Mul, MulAssign,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CDir {
    N, S, E, W,
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
    pub fn to_coord(&self) -> Coord2D {
        match self {
            CDir::N => Coord2D::new(0, 1),
            CDir::E => Coord2D::new(1, 0),
            CDir::S => Coord2D::new(0, -1),
            CDir::W => Coord2D::new(-1, 0),
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
    pub fn mdist_to(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
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
impl AddAssign for Coord2D {
    fn add_assign(&mut self, other: Self) {
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
impl SubAssign for Coord2D {
    fn sub_assign(&mut self, other: Self) {
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
