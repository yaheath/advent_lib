use crate::coords::Coord2D;
use std::cmp::{max, min};
use std::collections::hash_map::{Iter, IterMut};
use std::collections::HashMap;
use std::io::Write;
use std::ops::Range;

#[derive(Clone)]
pub struct InfiniteGrid<T: Copy> {
    default: T,
    data: HashMap<(i64, i64), T>,
    x_range: Range<i64>,
    y_range: Range<i64>,
    flip_y: bool,
}

impl<T: Copy> InfiniteGrid<T> {
    pub fn new(default_val: T) -> Self {
        Self {
            default: default_val,
            data: HashMap::new(),
            x_range: Range { start: 0, end: 0 },
            y_range: Range { start: 0, end: 0 },
            flip_y: false,
        }
    }

    pub fn from_input<F>(input: &[String], default_val: T, mapfunc: F) -> Self
    where
        F: Fn(char, i64, i64) -> Option<T>,
    {
        let mut inst = Self::new(default_val);
        for (uy, line) in input.iter().enumerate() {
            for (ux, c) in line.chars().enumerate() {
                let x = ux as i64;
                let y = uy as i64;
                if let Some(val) = mapfunc(c, x, y) {
                    inst.set(x, y, val);
                };
            }
        }
        inst
    }

    pub fn from_other<U: Copy, F>(other: &InfiniteGrid<U>, default_val: T, mapfunc: F) -> Self
    where
        F: Fn(U) -> Option<T>,
    {
        let mut inst = Self::new(default_val);
        for y in other.y_bounds() {
            for x in other.x_bounds() {
                let other_val = other.get(x, y);
                if let Some(val) = mapfunc(other_val) {
                    inst.set(x, y, val);
                }
            }
        }
        inst
    }

    pub fn iter(&self) -> Iter<(i64, i64), T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<(i64, i64), T> {
        self.data.iter_mut()
    }

    pub fn get(&self, x: i64, y: i64) -> T {
        if let Some(cell) = self.data.get(&(x, y)) {
            *cell
        } else {
            self.default
        }
    }
    pub fn get_c(&self, c: Coord2D) -> T {
        self.get(c.x, c.y)
    }

    pub fn set(&mut self, x: i64, y: i64, val: T) {
        self.data.insert((x, y), val);
        if self.x_range.is_empty() {
            self.x_range.start = x;
            self.x_range.end = x + 1;
        } else if x < self.x_range.start {
            self.x_range.start = x;
        } else if x >= self.x_range.end {
            self.x_range.end = x + 1;
        }
        if self.y_range.is_empty() {
            self.y_range.start = y;
            self.y_range.end = y + 1;
        } else if y < self.y_range.start {
            self.y_range.start = y;
        } else if y >= self.y_range.end {
            self.y_range.end = y + 1;
        }
    }

    pub fn set_c(&mut self, c: Coord2D, val: T) {
        self.set(c.x, c.y, val);
    }

    // Untested
    pub fn purge(
        &mut self,
        min_x: Option<i64>,
        min_y: Option<i64>,
        max_x: Option<i64>,
        max_y: Option<i64>,
    ) {
        let mut r: Vec<(i64, i64)> = Vec::new();
        let mut new_x_start: i64 = 0;
        let mut new_y_start: i64 = 0;
        let mut new_x_end: i64 = 0;
        let mut new_y_end: i64 = 0;
        for ((x, y), _) in self.data.iter() {
            if let Some(val) = min_x {
                if x < &val {
                    r.push((*x, *y));
                    continue;
                }
            }
            if let Some(val) = min_y {
                if y < &val {
                    r.push((*x, *y));
                    continue;
                }
            }
            if let Some(val) = max_x {
                if x > &val {
                    r.push((*x, *y));
                    continue;
                }
            }
            if let Some(val) = max_y {
                if y > &val {
                    r.push((*x, *y));
                    continue;
                }
            }
            if new_x_start == new_x_end {
                new_x_start = *x;
                new_y_start = *y;
                new_x_end = *x + 1;
                new_y_end = *y + 1;
            } else {
                new_x_start = min(*x, new_x_start);
                new_y_start = min(*y, new_y_start);
                new_x_end = max(*x + 1, new_x_end);
                new_y_end = max(*y + 1, new_y_end);
            }
        }
        for c in r {
            self.data.remove(&c);
        }
        self.x_range.start = new_x_start;
        self.y_range.start = new_y_start;
        self.x_range.end = new_x_end;
        self.y_range.end = new_y_end;
    }

    pub fn x_bounds(&self) -> Range<i64> {
        self.x_range.clone()
    }

    pub fn y_bounds(&self) -> Range<i64> {
        self.y_range.clone()
    }

    pub fn flip_y(&mut self, val: bool) {
        self.flip_y = val;
    }

    pub fn dump_to_file<F>(&self, file: &mut dyn Write, formatter: F)
    where
        F: Fn(T) -> char,
    {
        for y in self.y_range.clone() {
            for x in self.x_range.clone() {
                write!(file, "{}", formatter(self.get(x, y))).unwrap();
            }
            writeln!(file).unwrap();
        }
    }

    pub fn print<F>(&self, formatter: F)
    where
        F: Fn(T) -> char,
    {
        if self.flip_y {
            for y in self.y_range.clone().rev() {
                for x in self.x_range.clone() {
                    print!("{}", formatter(self.get(x, y)));
                }
                println!();
            }
        } else {
            for y in self.y_range.clone() {
                for x in self.x_range.clone() {
                    print!("{}", formatter(self.get(x, y)));
                }
                println!();
            }
        }
    }
}
