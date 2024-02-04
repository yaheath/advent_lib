use crate::coords::Coord2D;
use itertools::Itertools;
use std::cmp::max;
use std::io::Write;
use std::iter::Iterator;
use std::ops::Range;
use std::slice::{Iter, IterMut};
use std::vec::Vec;

#[derive(Clone)]
pub struct Grid<T: Copy> {
    min_x: i64,
    min_y: i64,
    x_size: usize,
    y_size: usize,
    data: Vec<T>,
    padding: i64,
}

impl<T: Copy> Grid<T> {
    pub fn new(min_x: i64, min_y: i64, max_x: i64, max_y: i64, initial_val: T) -> Self {
        let x_size = (max_x - min_x + 1) as usize;
        let y_size = (max_y - min_y + 1) as usize;
        Self {
            min_x,
            min_y,
            x_size,
            y_size,
            data: vec![initial_val; x_size * y_size],
            padding: 0,
        }
    }

    pub fn from_input(input: &[String], default_val: T, padding: i64) -> Self
    where
        T: From<char>,
    {
        Self::from_input_map(input, default_val, padding, |c: char| c.into())
    }

    pub fn from_input_map<F>(input: &[String], default_val: T, padding: i64, mapfunc: F) -> Self
    where
        F: Fn(char) -> T,
    {
        let width = input
            .iter()
            .map(|s| s.len())
            .fold(0, |maxw, w| max(w, maxw)) as i64;
        let height = input.len() as i64;
        let mut inst = Self::new(
            -padding,
            -padding,
            width - 1 + padding,
            height - 1 + padding,
            default_val,
        );
        inst.padding = padding;
        for (uy, line) in input.iter().enumerate() {
            for (ux, c) in line.chars().enumerate() {
                inst.set(ux as i64, uy as i64, mapfunc(c));
            }
        }
        inst
    }

    pub fn try_from_input(
        input: &[String],
        default_val: T,
        padding: i64,
    ) -> Result<Self, <T as TryFrom<char>>::Error>
    where
        T: TryFrom<char>,
    {
        let width = input
            .iter()
            .map(|s| s.len())
            .fold(0, |maxw, w| max(w, maxw)) as i64;
        let height = input.len() as i64;
        let mut inst = Self::new(
            -padding,
            -padding,
            width - 1 + padding,
            height - 1 + padding,
            default_val,
        );
        inst.padding = padding;
        for (uy, line) in input.iter().enumerate() {
            for (ux, c) in line.chars().enumerate() {
                inst.set(ux as i64, uy as i64, c.try_into()?);
            }
        }
        Ok(inst)
    }

    pub fn clone_without_data(&self, initial_val: T) -> Self {
        Grid {
            min_x: self.min_x,
            min_y: self.min_y,
            x_size: self.x_size,
            y_size: self.y_size,
            data: vec![initial_val; self.data.len()],
            padding: self.padding,
        }
    }

    pub fn get(&self, x: i64, y: i64) -> T {
        assert!(x >= self.min_x && x < self.min_x + self.x_size as i64);
        assert!(y >= self.min_y && y < self.min_y + self.y_size as i64);
        let ux: usize = (x - self.min_x) as usize;
        let uy: usize = (y - self.min_y) as usize;
        let idx = uy * self.x_size + ux;
        self.data[idx]
    }

    pub fn get_c<C>(&self, coord: C) -> T
    where
        C: Into<Coord2D>,
    {
        let c: Coord2D = coord.into();
        self.get(c.x, c.y)
    }

    pub fn get_xform(&self, x: i64, y: i64, xform: GridTransform) -> T {
        let (x, y) = self.apply_transform(x, y, xform);
        self.get(x, y)
    }

    pub fn get_or_default(&self, x: i64, y: i64, default: T) -> T {
        if x >= self.min_x
            && x < self.min_x + self.x_size as i64
            && y >= self.min_y
            && y < self.min_y + self.y_size as i64
        {
            let ux: usize = (x - self.min_x) as usize;
            let uy: usize = (y - self.min_y) as usize;
            let idx = uy * self.x_size + ux;
            self.data[idx]
        } else {
            default
        }
    }

    pub fn get_or_default_xform(&self, x: i64, y: i64, default: T, xform: GridTransform) -> T {
        let (x, y) = self.apply_transform(x, y, xform);
        self.get_or_default(x, y, default)
    }

    pub fn contains_coord<C>(&self, coord: C) -> bool
    where
        C: Into<Coord2D>,
    {
        let c: Coord2D = coord.into();
        self.x_bounds().contains(&c.x) && self.y_bounds().contains(&c.y)
    }

    pub fn set(&mut self, x: i64, y: i64, val: T) {
        assert!(x >= self.min_x && x < self.min_x + self.x_size as i64);
        assert!(y >= self.min_y && y < self.min_y + self.y_size as i64);
        let ux: usize = (x - self.min_x) as usize;
        let uy: usize = (y - self.min_y) as usize;
        let idx = uy * self.x_size + ux;
        self.data[idx] = val;
    }

    pub fn set_c<C>(&mut self, coord: C, val: T)
    where
        C: Into<Coord2D>,
    {
        let c: Coord2D = coord.into();
        self.set(c.x, c.y, val);
    }

    pub fn set_xform(&mut self, x: i64, y: i64, val: T, xform: GridTransform) {
        let (x, y) = self.apply_transform(x, y, xform);
        self.set(x, y, val);
    }

    pub fn iter(&self) -> Iter<T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.data.iter_mut()
    }

    pub fn data(&self) -> &Vec<T> {
        &self.data
    }

    pub fn iter_with_coord(&self) -> impl Iterator<Item = (T, i64, i64)> + '_ {
        self.data.iter().enumerate().map(|(idx, val)| {
            let x = (idx % self.x_size) as i64 + self.min_x;
            let y = (idx / self.x_size) as i64 + self.min_y;
            (*val, x, y)
        })
    }

    pub fn x_bounds(&self) -> Range<i64> {
        self.min_x..self.min_x + self.x_size as i64
    }

    pub fn y_bounds(&self) -> Range<i64> {
        self.min_y..self.min_y + self.y_size as i64
    }

    // bounds not including padding (i.e., original size from input)
    pub fn x_bounds_orig(&self) -> Range<i64> {
        self.min_x + self.padding..self.min_x + self.x_size as i64 - self.padding
    }
    pub fn y_bounds_orig(&self) -> Range<i64> {
        self.min_y + self.padding..self.min_y + self.y_size as i64 - self.padding
    }

    pub fn x_bounds_xform(&self, xform: GridTransform) -> Range<i64> {
        match xform {
            GridTransform::Rot90
            | GridTransform::Rot270
            | GridTransform::Rot90HFlip
            | GridTransform::Rot270HFlip => self.y_bounds(),
            GridTransform::Identity
            | GridTransform::Rot180
            | GridTransform::HFlip
            | GridTransform::VFlip => self.x_bounds(),
        }
    }
    pub fn y_bounds_xform(&self, xform: GridTransform) -> Range<i64> {
        match xform {
            GridTransform::Rot90
            | GridTransform::Rot270
            | GridTransform::Rot90HFlip
            | GridTransform::Rot270HFlip => self.x_bounds(),
            GridTransform::Identity
            | GridTransform::Rot180
            | GridTransform::HFlip
            | GridTransform::VFlip => self.y_bounds(),
        }
    }

    fn apply_transform(&self, x: i64, y: i64, xform: GridTransform) -> (i64, i64) {
        let xsize = self.x_bounds().end - self.x_bounds().start;
        let ysize = self.y_bounds().end - self.y_bounds().start;
        let x = x - self.min_x;
        let y = y - self.min_y;
        let (x, y) = match xform {
            GridTransform::Identity => (x, y),
            GridTransform::Rot90 => (y, ysize - 1 - x),
            GridTransform::Rot180 => (xsize - 1 - x, ysize - 1 - y),
            GridTransform::Rot270 => (xsize - 1 - y, x),
            GridTransform::HFlip => (xsize - 1 - x, y),
            GridTransform::Rot90HFlip => (y, x),
            GridTransform::VFlip => (x, ysize - 1 - y),
            GridTransform::Rot270HFlip => (xsize - 1 - y, ysize - 1 - x),
        };
        (x + self.min_x, y + self.min_y)
    }

    pub fn dump_to_file<F>(&self, file: &mut dyn Write, formatter: F)
    where
        F: Fn(T) -> char,
    {
        for y in self.min_y..self.min_y + self.y_size as i64 {
            for x in self.min_x..self.min_x + self.x_size as i64 {
                write!(file, "{}", formatter(self.get(x, y))).unwrap();
            }
            writeln!(file).unwrap();
        }
    }

    pub fn print(&self)
    where
        T: Into<char>,
    {
        for y in self.min_y..self.min_y + self.y_size as i64 {
            for x in self.min_x..self.min_x + self.x_size as i64 {
                print!("{}", Into::<char>::into(self.get(x, y)));
            }
            println!();
        }
    }

    pub fn print_str<F>(&self, formatter: F)
    where
        F: Fn(T) -> String,
    {
        for y in self.min_y..self.min_y + self.y_size as i64 {
            for x in self.min_x..self.min_x + self.x_size as i64 {
                print!("{}", formatter(self.get(x, y)));
            }
            println!();
        }
    }

    pub fn format(&self) -> String
    where
        T: Into<char>,
    {
        let mut s = String::with_capacity(self.y_size * (self.x_size + 1));
        for y in self.min_y..self.min_y + self.y_size as i64 {
            for x in self.min_x..self.min_x + self.x_size as i64 {
                s.push(self.get(x, y).into());
            }
            s.push('\n');
        }
        s
    }

    pub fn format_str<F>(&self, formatter: F) -> String
    where
        F: Fn(T) -> String,
    {
        let mut s = String::with_capacity(self.y_size * (self.x_size + 1));
        for y in self.min_y..self.min_y + self.y_size as i64 {
            for x in self.min_x..self.min_x + self.x_size as i64 {
                s.push_str(&formatter(self.get(x, y)));
            }
            s.push('\n');
        }
        s
    }

    pub fn find<F>(&self, predicate: F) -> Option<(i64, i64)>
    where
        F: Fn(T, i64, i64) -> bool,
    {
        for y in self.min_y..self.min_y + self.y_size as i64 {
            for x in self.min_x..self.min_x + self.x_size as i64 {
                if predicate(self.get(x, y), x, y) {
                    return Some((x, y));
                }
            }
        }
        None
    }

    pub fn for_each<F>(&self, mut callback: F)
    where
        F: FnMut(T, i64, i64),
    {
        for y in self.min_y..self.min_y + self.y_size as i64 {
            for x in self.min_x..self.min_x + self.x_size as i64 {
                callback(self.get(x, y), x, y);
            }
        }
    }

    pub fn rows(&self) -> impl Iterator<Item = Vec<T>> + '_ {
        (0..self.y_size).map(|y| Vec::from(&self.data[y * self.x_size..(y + 1) * self.x_size]))
    }

    pub fn cols(&self) -> impl Iterator<Item = Vec<T>> + '_ {
        (0..self.x_size)
            .map(|x| Vec::from_iter((0..self.y_size).map(|y| self.data[x + y * self.x_size])))
    }

    pub fn extract(&self, x: i64, y: i64, wid: i64, hei: i64) -> Self {
        assert!(wid > 0 && hei > 0);
        let mut data = Vec::with_capacity((wid * hei) as usize);
        let ux = (x - self.min_x) as usize;
        let uy = (y - self.min_y) as usize;
        (0..hei as usize).for_each(|row| {
            let idx = (row + uy) * self.x_size + ux;
            data.extend_from_slice(&self.data[idx..idx + wid as usize])
        });
        Self {
            min_x: 0,
            min_y: 0,
            x_size: wid as usize,
            y_size: hei as usize,
            data,
            padding: 0,
        }
    }

    pub fn tile_split(&self, x_size: usize, y_size: usize) -> impl Iterator<Item = Grid<T>> + '_ {
        let x_tiles = self.x_size / x_size;
        let y_tiles = self.y_size / y_size;
        (0..y_tiles)
            .cartesian_product(0..x_tiles)
            .map(move |(ty, tx)| {
                self.extract(
                    self.min_x + (tx * x_size) as i64,
                    self.min_y + (ty * y_size) as i64,
                    x_size as i64,
                    y_size as i64,
                )
            })
    }

    pub fn from_tiles(tiles: &Vec<Self>, n_tiles_width: usize, n_tiles_height: usize) -> Self {
        let tilewidth = tiles[0].x_size;
        let tileheight = tiles[0].y_size;
        assert!(tiles
            .iter()
            .all(|t| t.x_size == tilewidth && t.y_size == tileheight));
        assert!(tiles.len() == n_tiles_width * n_tiles_height);

        let width = tilewidth * n_tiles_width;
        let height = tileheight * n_tiles_height;
        let mut data = Vec::with_capacity(n_tiles_width * n_tiles_height);
        tiles.chunks(n_tiles_width).for_each(|chunk| {
            chunk
                .iter()
                .cycle()
                .enumerate()
                .take(n_tiles_width * tileheight)
                .for_each(|(idx, tile)| {
                    let row = idx / n_tiles_width;
                    let start = row * tilewidth;
                    data.extend_from_slice(&tile.data[start..start + tilewidth]);
                });
        });
        Self {
            min_x: 0,
            min_y: 0,
            x_size: width,
            y_size: height,
            data,
            padding: 0,
        }
    }

    pub fn h_flip_inplace(&mut self) {
        for row in 0..self.y_size {
            let rowidx = row * self.x_size;
            for col in 0..self.x_size / 2 {
                self.data
                    .swap(rowidx + col, rowidx + (self.x_size - 1 - col))
            }
        }
    }

    pub fn h_flip(&self) -> Self {
        let mut data = Vec::with_capacity(self.x_size * self.y_size);
        for row in 0..self.y_size {
            let rowidx = row * self.x_size;
            for col in 0..self.x_size {
                data.push(self.data[rowidx + (self.x_size - 1 - col)]);
            }
        }
        Self {
            min_x: self.min_x,
            min_y: self.min_y,
            x_size: self.x_size,
            y_size: self.y_size,
            data,
            padding: self.padding,
        }
    }

    pub fn v_flip_inplace(&mut self) {
        for row in 0..self.y_size / 2 {
            let rowidx = row * self.x_size;
            let row2idx = (self.y_size - 1 - row) * self.x_size;
            for col in 0..self.x_size {
                self.data.swap(rowidx + col, row2idx + col);
            }
        }
    }

    pub fn v_flip(&self) -> Self {
        let mut data = Vec::with_capacity(self.x_size * self.y_size);
        for row in 0..self.y_size {
            let rowidx = (self.y_size - 1 - row) * self.x_size;
            for col in 0..self.x_size {
                data.push(self.data[rowidx + col]);
            }
        }
        Self {
            min_x: self.min_x,
            min_y: self.min_y,
            x_size: self.x_size,
            y_size: self.y_size,
            data,
            padding: self.padding,
        }
    }

    pub fn rot180_inplace(&mut self) {
        for row in 0..(self.y_size + 1) / 2 {
            let rowidx = row * self.x_size;
            let row2idx = (self.y_size - 1 - row) * self.x_size;
            let w = if rowidx == row2idx {
                self.x_size / 2
            } else {
                self.x_size
            };
            for col in 0..w {
                let a = rowidx + col;
                let b = row2idx + (self.x_size - 1 - col);
                if a != b {
                    self.data.swap(a, b);
                }
            }
        }
    }

    pub fn rot180(&self) -> Self {
        let mut data = Vec::with_capacity(self.x_size * self.y_size);
        for row in 0..self.y_size {
            let rowidx = (self.y_size - 1 - row) * self.x_size;
            for col in 0..self.x_size {
                data.push(self.data[rowidx + (self.x_size - 1 - col)]);
            }
        }
        Self {
            min_x: self.min_x,
            min_y: self.min_y,
            x_size: self.x_size,
            y_size: self.y_size,
            data,
            padding: self.padding,
        }
    }

    pub fn rot90(&self) -> Self {
        let mut data = Vec::with_capacity(self.x_size * self.y_size);
        for col in 0..self.x_size {
            for row in 0..self.y_size {
                data.push(self.data[(self.y_size - 1 - row) * self.x_size + col]);
            }
        }

        Self {
            min_x: self.min_y,
            min_y: self.min_x,
            x_size: self.y_size,
            y_size: self.x_size,
            data,
            padding: self.padding,
        }
    }

    pub fn roll_row(&mut self, y: i64, n: i64) {
        let go_left = n < 0;
        let n = n.abs() % self.x_size as i64;
        if n == 0 {
            return;
        }
        let uy: usize = (y - self.min_y) as usize;
        let idx = uy * self.x_size;
        let row_slice = &mut self.data[idx..idx + self.x_size];
        if go_left {
            row_slice.rotate_left(n as usize);
        } else {
            row_slice.rotate_right(n as usize);
        }
    }

    pub fn roll_col(&mut self, x: i64, n: i64) {
        let go_up = n < 0;
        let n = n.abs() % self.y_size as i64;
        if n == 0 {
            return;
        }
        let ux: usize = (x - self.min_x) as usize;
        let mut col = Vec::from_iter((0..self.y_size).map(|row| self.data[row * self.x_size + ux]));
        if go_up {
            col.rotate_left(n as usize);
        } else {
            col.rotate_right(n as usize);
        }
        (0..self.y_size).for_each(|row| self.data[row * self.x_size + ux] = col[row]);
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum GridTransform {
    Identity,
    Rot90,
    Rot180,
    Rot270,
    HFlip,
    VFlip,
    Rot90HFlip,
    Rot270HFlip,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fill(grid: &mut Grid<u32>) {
        let mut c = 0u32;
        for y in grid.y_bounds() {
            for x in grid.x_bounds() {
                grid.set(x, y, c);
                c += 1;
            }
        }
    }

    #[allow(dead_code)]
    fn print(grid: &Grid<u32>) {
        grid.print_str(|v| format!("{:3}", v));
    }

    #[test]
    fn test_extract() {
        let mut grid: Grid<u32> = Grid::new(-2, -2, 7, 7, 0);
        fill(&mut grid);
        let grid2 = grid.extract(-1, -1, 3, 3);
        assert_eq!(grid2.data, vec![11, 12, 13, 21, 22, 23, 31, 32, 33]);
    }

    #[test]
    fn test_tiling() {
        let mut grid: Grid<u32> = Grid::new(-2, -2, 1, 1, 0);
        fill(&mut grid);
        let tiles: Vec<Grid<u32>> = grid.tile_split(2, 2).collect();
        assert_eq!(tiles[0].data, vec![0, 1, 4, 5]);
        assert_eq!(tiles[1].data, vec![2, 3, 6, 7]);
        assert_eq!(tiles[2].data, vec![8, 9, 12, 13]);
        assert_eq!(tiles[3].data, vec![10, 11, 14, 15]);
        let grid2 = Grid::from_tiles(&tiles, 2, 2);
        assert_eq!(grid.data, grid2.data);
    }

    #[test]
    fn test_h_flip() {
        let mut grid: Grid<u32> = Grid::new(-2, -2, 7, 6, 0);
        fill(&mut grid);
        let grid2 = grid.h_flip();
        grid.h_flip_inplace();
        assert_eq!(grid.get(-2, -2), 9);
        assert_eq!(grid.get(-2, 0), 29);
        assert_eq!(grid.get(7, 6), 80);
        assert_eq!(grid.get(5, 6), 82);
        assert_eq!(grid.data, grid2.data);
    }

    #[test]
    fn test_v_flip() {
        let mut grid: Grid<u32> = Grid::new(-2, -2, 7, 6, 0);
        fill(&mut grid);
        let grid2 = grid.v_flip();
        grid.v_flip_inplace();
        assert_eq!(grid.get(-2, -2), 80);
        assert_eq!(grid.get(-2, 0), 60);
        assert_eq!(grid.get(7, 6), 9);
        assert_eq!(grid.get(5, 6), 7);
        assert_eq!(grid.data, grid2.data);
    }

    #[test]
    fn test_rot180() {
        let mut grid: Grid<u32> = Grid::new(-2, -2, 7, 6, 0);
        fill(&mut grid);
        let grid2 = grid.rot180();
        grid.rot180_inplace();
        assert_eq!(grid.get(-2, -2), 89);
        assert_eq!(grid.get(-2, 0), 69);
        assert_eq!(grid.get(7, 6), 0);
        assert_eq!(grid.get(5, 6), 2);
        assert_eq!(grid.data, grid2.data);

        let mut grid: Grid<u32> = Grid::new(-2, -2, 6, 7, 0);
        fill(&mut grid);
        let grid2 = grid.rot180();
        grid.rot180_inplace();
        assert_eq!(grid.get(-2, -2), 89);
        assert_eq!(grid.get(-2, 0), 71);
        assert_eq!(grid.get(6, 7), 0);
        assert_eq!(grid.get(4, 7), 2);
        assert_eq!(grid.data, grid2.data);
    }

    #[test]
    fn test_rot90() {
        let mut grid: Grid<u32> = Grid::new(-2, -2, 7, 6, 0);
        fill(&mut grid);
        let grid = grid.rot90();
        assert_eq!(grid.get(-2, -2), 80);
        assert_eq!(grid.get(-2, 0), 82);
        assert_eq!(grid.get(6, 7), 9);
        assert_eq!(grid.get(5, 7), 19);
        assert_eq!(grid.get(5, -1), 11);
    }

    #[test]
    fn test_roll_row() {
        let mut grid: Grid<u32> = Grid::new(-2, -2, 2, 2, 0);
        fill(&mut grid);
        grid.roll_row(1, 3);

        #[rustfmt::skip]
        assert_eq!(grid.data, vec![ 0,  1,  2,  3,  4,
                                    5,  6,  7,  8,  9,
                                   10, 11, 12, 13, 14,
                                   17, 18, 19, 15, 16,
                                   20, 21, 22, 23, 24]);

        let mut grid: Grid<u32> = Grid::new(-2, -2, 2, 2, 0);
        fill(&mut grid);
        grid.roll_row(1, -1);

        #[rustfmt::skip]
        assert_eq!(grid.data, vec![ 0,  1,  2,  3,  4,
                                    5,  6,  7,  8,  9,
                                   10, 11, 12, 13, 14,
                                   16, 17, 18, 19, 15,
                                   20, 21, 22, 23, 24]);
    }

    #[test]
    fn test_roll_col() {
        let mut grid: Grid<u32> = Grid::new(-2, -2, 2, 2, 0);
        fill(&mut grid);
        grid.roll_col(1, 3);

        #[rustfmt::skip]
        assert_eq!(grid.data, vec![ 0,  1,  2, 13,  4,
                                    5,  6,  7, 18,  9,
                                   10, 11, 12, 23, 14,
                                   15, 16, 17,  3, 19,
                                   20, 21, 22,  8, 24]);

        let mut grid: Grid<u32> = Grid::new(-2, -2, 2, 2, 0);
        fill(&mut grid);
        grid.roll_col(1, -1);

        #[rustfmt::skip]
        assert_eq!(grid.data, vec![ 0,  1,  2,  8,  4,
                                    5,  6,  7, 13,  9,
                                   10, 11, 12, 18, 14,
                                   15, 16, 17, 23, 19,
                                   20, 21, 22,  3, 24]);
    }

    #[test]
    fn test_xform() {
        /*
        let inp: Vec<&str> = vec![
            "####..",
            "#...#.",
            "#...#.",
            "####..",
            "#.#...",
            "#..#..",
            "#...#.",
        ];
        let mut grid: Grid<char> = Grid::from_input(&inp, '.', 0, |c| c);
        */
        let mut grid: Grid<u32> = Grid::new(-2, -2, 2, 3, 0);
        fill(&mut grid);

        let grid2 = grid.rot90();
        for y in grid2.y_bounds() {
            for x in grid2.x_bounds() {
                assert_eq!(grid.get_xform(x, y, GridTransform::Rot90), grid2.get(x, y));
            }
        }
        let grid2 = grid.rot180();
        for y in grid2.y_bounds() {
            for x in grid2.x_bounds() {
                assert_eq!(grid.get_xform(x, y, GridTransform::Rot180), grid2.get(x, y));
            }
        }
        let grid2 = grid.h_flip();
        for y in grid2.y_bounds() {
            for x in grid2.x_bounds() {
                assert_eq!(grid.get_xform(x, y, GridTransform::HFlip), grid2.get(x, y));
            }
        }
        let grid2 = grid.v_flip();
        for y in grid2.y_bounds() {
            for x in grid2.x_bounds() {
                assert_eq!(grid.get_xform(x, y, GridTransform::VFlip), grid2.get(x, y));
            }
        }
    }
}
