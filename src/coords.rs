use std::fmt;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum CDir {
    N,
    E,
    S,
    W,
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
    L,
    R,
}

impl Add<Turn> for CDir {
    type Output = Self;
    fn add(self, other: Turn) -> Self {
        match other {
            Turn::L => self.left(),
            Turn::R => self.right(),
        }
    }
}
impl AddAssign<Turn> for CDir {
    fn add_assign(&mut self, other: Turn) {
        *self = match other {
            Turn::L => self.left(),
            Turn::R => self.right(),
        };
    }
}

impl Neg for CDir {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            CDir::N => CDir::S,
            CDir::E => CDir::W,
            CDir::W => CDir::E,
            CDir::S => CDir::N,
        }
    }
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
    pub fn x() -> Self {
        Coord2D { x: 1, y: 0 }
    }
    pub fn y() -> Self {
        Coord2D { x: 0, y: 1 }
    }
    #[rustfmt::skip]
    pub fn neighbors4(&self) -> Vec<Self> {
        [
                      Coord2D::new(0, -1),
            Coord2D::new(-1, 0), Coord2D::new(1, 0),
                      Coord2D::new(0, 1),
        ]
        .iter()
        .map(|o| *self + *o)
        .collect()
    }
    #[rustfmt::skip]
    pub fn neighbors8(&self) -> Vec<Self> {
        [
            Coord2D::new(-1, -1), Coord2D::new(0, -1), Coord2D::new(1, -1),
            Coord2D::new(-1, 0),                       Coord2D::new(1, 0),
            Coord2D::new(-1, 1),  Coord2D::new(0, 1),  Coord2D::new(1, 1),
        ]
        .iter()
        .map(|o| *self + *o)
        .collect()
    }
    pub fn mdist_to(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl fmt::Display for Coord2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
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

impl From<(i64, i64)> for Coord2D {
    fn from(value: (i64, i64)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<Coord2D> for (i64, i64) {
    fn from(value: Coord2D) -> Self {
        (value.x, value.y)
    }
}

impl<T> Add<T> for Coord2D
where
    T: Into<Coord2D>,
{
    type Output = Self;
    fn add(self, other: T) -> Self {
        let other: Coord2D = other.into();
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> AddAssign<T> for Coord2D
where
    T: Into<Coord2D>,
{
    fn add_assign(&mut self, other: T) {
        let other: Coord2D = other.into();
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl<T> Sub<T> for Coord2D
where
    T: Into<Coord2D>,
{
    type Output = Self;
    fn sub(self, other: T) -> Self {
        let other: Coord2D = other.into();
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> SubAssign<T> for Coord2D
where
    T: Into<Coord2D>,
{
    fn sub_assign(&mut self, other: T) {
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
            x: itr
                .next()
                .ok_or(())
                .map(|s| s.trim().parse::<i64>().map_err(|_| ()))??,
            y: itr
                .next()
                .ok_or(())
                .map(|s| s.trim().parse::<i64>().map_err(|_| ()))??,
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
    pub fn x() -> Self {
        Coord3D { x: 1, y: 0, z: 0 }
    }
    pub fn y() -> Self {
        Coord3D { x: 0, y: 1, z: 0 }
    }
    pub fn z() -> Self {
        Coord3D { x: 0, y: 0, z: 1 }
    }
    pub fn mdist_to(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
    pub fn neighbors6(&self) -> Vec<Self> {
        [
            Coord3D::new(-1, 0, 0),
            Coord3D::new(1, 0, 0),
            Coord3D::new(0, -1, 0),
            Coord3D::new(0, 1, 0),
            Coord3D::new(0, 0, -1),
            Coord3D::new(0, 0, 1),
        ]
        .iter()
        .map(|o| *self + *o)
        .collect()
    }
}

impl fmt::Display for Coord3D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl From<(i64, i64, i64)> for Coord3D {
    fn from(v: (i64, i64, i64)) -> Self {
        Coord3D {
            x: v.0,
            y: v.1,
            z: v.2,
        }
    }
}

impl From<Coord3D> for (i64, i64, i64) {
    fn from(value: Coord3D) -> Self {
        (value.x, value.y, value.z)
    }
}

impl<T> Add<T> for Coord3D
where
    T: Into<Coord3D>,
{
    type Output = Self;
    fn add(self, other: T) -> Self {
        let other: Coord3D = other.into();
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl<T> AddAssign<T> for Coord3D
where
    T: Into<Coord3D>,
{
    fn add_assign(&mut self, other: T) {
        let other: Coord3D = other.into();
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl<T> Sub<T> for Coord3D
where
    T: Into<Coord3D>,
{
    type Output = Self;
    fn sub(self, other: T) -> Self {
        let other: Coord3D = other.into();
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl<T> SubAssign<T> for Coord3D
where
    T: Into<Coord3D>,
{
    fn sub_assign(&mut self, other: T) {
        let other: Coord3D = other.into();
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
            x: itr
                .next()
                .ok_or(())
                .map(|s| s.trim().parse::<i64>().map_err(|_| ()))??,
            y: itr
                .next()
                .ok_or(())
                .map(|s| s.trim().parse::<i64>().map_err(|_| ()))??,
            z: itr
                .next()
                .ok_or(())
                .map(|s| s.trim().parse::<i64>().map_err(|_| ()))??,
        })
    }
}
