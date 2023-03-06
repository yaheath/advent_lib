use std::cmp::max;
use std::iter::Iterator;
use std::io::Write;
use std::ops::Range;
use std::slice::{Iter, IterMut};
use std::vec::Vec;
use itertools::Itertools;

pub struct Grid<T: Copy> {
    min_x: i64,
    min_y: i64,
    x_size: usize,
    y_size: usize,
    data: Vec<T>,
}

impl<T: Copy> Grid<T> {
    pub fn new(min_x:i64, min_y:i64, max_x:i64, max_y:i64, initial_val: T) -> Self {
        let x_size = (max_x - min_x + 1) as usize;
        let y_size = (max_y - min_y + 1) as usize;
        let mut g = Self {
            min_x: min_x,
            min_y: min_y,
            x_size: x_size,
            y_size: y_size,
            data: Vec::with_capacity(x_size * y_size),
        };
        for _ in 0..x_size * y_size {
            g.data.push(initial_val);
        }
        g
    }

    pub fn from_input<F>(input: &Vec<String>, default_val: T, padding: i64, mapfunc: F) -> Self
            where F: Fn(char) -> T {
        let width = input.iter().map(|s| s.len()).fold(0, |maxw, w| max(w, maxw)) as i64;
        let height = input.len() as i64;
        let mut y = 0i64;
        let mut inst = Self::new(-padding, -padding, width-1+padding, height-1+padding, default_val);
        for line in input.iter() {
            for (ux, c) in line.chars().enumerate() {
                let x = ux as i64;
                inst.set(x, y, mapfunc(c));
            }
            y += 1;
        }
        inst
    }

    pub fn clone(&self) -> Self {
        Grid {
            min_x: self.min_x,
            min_y: self.min_y,
            x_size: self.x_size,
            y_size: self.y_size,
            data: self.data.clone(),
        }
    }

    pub fn clone_without_data(&self, initial_val: T) -> Self {
        Grid {
            min_x: self.min_x,
            min_y: self.min_y,
            x_size: self.x_size,
            y_size: self.y_size,
            data: vec![initial_val; self.data.len()],
        }
    }

    pub fn get(&self, x:i64, y:i64) -> T {
        assert!(x >= self.min_x && x < self.min_x + self.x_size as i64);
        assert!(y >= self.min_y && y < self.min_y + self.y_size as i64);
        let ux:usize = (x - self.min_x) as usize;
        let uy:usize = (y - self.min_y) as usize;
        let idx = uy * self.x_size + ux;
        self.data[idx]
    }

    pub fn get_or_default(&self, x:i64, y:i64, default: T) -> T {
        if x >= self.min_x && x < self.min_x + self.x_size as i64
          && y >= self.min_y && y < self.min_y + self.y_size as i64 {
            let ux:usize = (x - self.min_x) as usize;
            let uy:usize = (y - self.min_y) as usize;
            let idx = uy * self.x_size + ux;
            self.data[idx]
        } else {
            default
        }
    }

    pub fn set(&mut self, x:i64, y:i64, val:T) {
        assert!(x >= self.min_x && x < self.min_x + self.x_size as i64);
        assert!(y >= self.min_y && y < self.min_y + self.y_size as i64);
        let ux:usize = (x - self.min_x) as usize;
        let uy:usize = (y - self.min_y) as usize;
        let idx = uy * self.x_size + ux;
        self.data[idx] = val;
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

    pub fn iter_with_coord(&self) -> impl Iterator<Item=(T, i64, i64)> + '_ {
        self.data.iter()
            .enumerate()
            .map(|(idx, val)| {
                let x = (idx % self.x_size) as i64 + self.min_x;
                let y = (idx / self.x_size) as i64 + self.min_y;
                (*val, x, y)
            })
    }

    pub fn x_bounds(&self) -> Range<i64> {
        self.min_x .. self.min_x + self.x_size as i64
    }

    pub fn y_bounds(&self) -> Range<i64> {
        self.min_y .. self.min_y + self.y_size as i64
    }

    pub fn dump_to_file<F>(&self, file: &mut dyn Write, formatter: F)
            where F: Fn(T) -> char {
        for y in self.min_y .. self.min_y + self.y_size as i64 {
            for x in self.min_x .. self.min_x + self.x_size as i64 {
                write!(file, "{}", formatter(self.get(x, y))).unwrap();
            }
            writeln!(file, "").unwrap();
        }
    }

    pub fn print<F>(&self, formatter: F)
            where F: Fn(T) -> char {
        for y in self.min_y .. self.min_y + self.y_size as i64 {
            for x in self.min_x .. self.min_x + self.x_size as i64 {
                print!("{}", formatter(self.get(x, y)));
            }
            println!("");
        }
    }

    pub fn find<F>(&self, predicate: F) -> Option<(i64, i64)>
            where F: Fn(T, i64, i64) -> bool {
        for y in self.min_y .. self.min_y + self.y_size as i64 {
            for x in self.min_x .. self.min_x + self.x_size as i64 {
                if predicate(self.get(x, y), x, y) {
                    return Some((x, y));
                }
            }
        }
        None
    }

    pub fn for_each<F>(&self, mut callback: F)
            where F: FnMut(T, i64, i64) {
        for y in self.min_y .. self.min_y + self.y_size as i64 {
            for x in self.min_x .. self.min_x + self.x_size as i64 {
                callback(self.get(x, y), x, y);
            }
        }
    }

    pub fn extract(&self, x: i64, y: i64, wid: i64, hei: i64) -> Self {
        assert!(wid > 0 && hei > 0);
        let mut data = Vec::with_capacity((wid * hei) as usize);
        let ux = (x - self.min_x) as usize;
        let uy = (y - self.min_y) as usize;
        (0 .. hei as usize).for_each(|row| {
            let idx = (row + uy) * self.x_size + ux;
            data.extend_from_slice(&self.data[idx .. idx + wid as usize])
        });
        Self {
            min_x: 0,
            min_y: 0,
            x_size: wid as usize,
            y_size: hei as usize,
            data,
        }
    }

    pub fn tile_split(&self, x_size: usize, y_size: usize)
            -> impl Iterator<Item=Grid<T>> + '_ {
        let x_tiles = self.x_size / x_size;
        let y_tiles = self.y_size / y_size;
        (0..y_tiles).cartesian_product(0..x_tiles)
            .map(move |(ty, tx)| self.extract(
                    self.min_x + (tx * x_size) as i64,
                    self.min_y + (ty * y_size) as i64,
                    x_size as i64,
                    y_size as i64,
            ))
    }

    pub fn from_tiles(tiles: &Vec<Self>, n_tiles_width: usize, n_tiles_height: usize) -> Self {
        let tilewidth = tiles[0].x_size;
        let tileheight = tiles[0].y_size;
        assert!(tiles.iter().all(|t| t.x_size == tilewidth && t.y_size == tileheight));
        assert!(tiles.len() == n_tiles_width * n_tiles_height);

        let width = tilewidth * n_tiles_width;
        let height = tileheight * n_tiles_height;
        let mut data = Vec::with_capacity(n_tiles_width * n_tiles_height);
        tiles
            .chunks(n_tiles_width)
            .for_each(|chunk| {
                chunk.iter()
                    .cycle()
                    .enumerate()
                    .take(n_tiles_width * tileheight)
                    .for_each(|(idx, tile)| {
                        let row = idx / n_tiles_width;
                        let start = row * tilewidth;
                        data.extend_from_slice(&tile.data[start..start+tilewidth]);
                    });
            });
        Self {
            min_x: 0,
            min_y: 0,
            x_size: width,
            y_size: height,
            data,
        }
    }

    pub fn h_flip_inplace(&mut self) {
        for row in 0 .. self.y_size {
            let rowidx = row * self.x_size;
            for col in 0 .. self.x_size / 2 {
                self.data.swap(rowidx + col, rowidx + (self.x_size - 1 - col))
            }
        }
    }

    pub fn h_flip(&self) -> Self {
        let mut data = Vec::with_capacity(self.x_size * self.y_size);
        for row in 0 .. self.y_size {
            let rowidx = row * self.x_size;
            for col in 0 .. self.x_size {
                data.push(self.data[rowidx + (self.x_size - 1 - col)]);
            }
        }
        Self {
            min_x: self.min_x,
            min_y: self.min_y,
            x_size: self.x_size,
            y_size: self.y_size,
            data,
        }
    }

    pub fn v_flip_inplace(&mut self) {
        for row in 0 .. self.y_size / 2 {
            let rowidx = row * self.x_size;
            let row2idx = (self.y_size - 1 - row) * self.x_size;
            for col in 0 .. self.x_size {
                self.data.swap(rowidx + col, row2idx + col);
            }
        }
    }

    pub fn v_flip(&self) -> Self {
        let mut data = Vec::with_capacity(self.x_size * self.y_size);
        for row in 0 .. self.y_size {
            let rowidx = (self.y_size - 1 - row) * self.x_size;
            for col in 0 .. self.x_size {
                data.push(self.data[rowidx + col]);
            }
        }
        Self {
            min_x: self.min_x,
            min_y: self.min_y,
            x_size: self.x_size,
            y_size: self.y_size,
            data,
        }
    }

    pub fn rot180_inplace(&mut self) {
        for row in 0 .. self.y_size / 2 {
            let rowidx = row * self.x_size;
            let row2idx = (self.y_size - 1 - row) * self.x_size;
            for col in 0 .. self.x_size {
                self.data.swap(rowidx + col, row2idx + (self.x_size - 1 - col));
            }
        }
    }

    pub fn rot180(&self) -> Self {
        let mut data = Vec::with_capacity(self.x_size * self.y_size);
        for row in 0 .. self.y_size {
            let rowidx = (self.y_size - 1 - row) * self.x_size;
            for col in 0 .. self.x_size {
                data.push(self.data[rowidx + (self.x_size - 1 - col)]);
            }
        }
        Self {
            min_x: self.min_x,
            min_y: self.min_y,
            x_size: self.x_size,
            y_size: self.y_size,
            data,
        }
    }

    pub fn rot90(&self) -> Self {
        let mut data = Vec::with_capacity(self.x_size * self.y_size);
        for col in 0 .. self.x_size {
            for row in 0 .. self.y_size {
                data.push(self.data[(self.y_size - 1 - row) * self.x_size + col]);
            }
        }

        Self {
            min_x: self.min_y,
            min_y: self.min_x,
            x_size: self.y_size,
            y_size: self.x_size,
            data,
        }
    }
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
        let mut grid: Grid<u32> = Grid::new(-2, -2, 7, 7, 0);
        fill(&mut grid);
        let grid2 = grid.h_flip();
        grid.h_flip_inplace();
        assert_eq!(grid.get(-2, -2), 9);
        assert_eq!(grid.get(-2, 0), 29);
        assert_eq!(grid.get(7, 7), 90);
        assert_eq!(grid.get(5, 7), 92);
        assert_eq!(grid.data, grid2.data);
    }

    #[test]
    fn test_v_flip() {
        let mut grid: Grid<u32> = Grid::new(-2, -2, 7, 7, 0);
        fill(&mut grid);
        let grid2 = grid.v_flip();
        grid.v_flip_inplace();
        assert_eq!(grid.get(-2, -2), 90);
        assert_eq!(grid.get(-2, 0), 70);
        assert_eq!(grid.get(7, 7), 9);
        assert_eq!(grid.get(5, 7), 7);
        assert_eq!(grid.data, grid2.data);
    }

    #[test]
    fn test_rot180() {
        let mut grid: Grid<u32> = Grid::new(-2, -2, 7, 7, 0);
        fill(&mut grid);
        let grid2 = grid.rot180();
        grid.rot180_inplace();
        assert_eq!(grid.get(-2, -2), 99);
        assert_eq!(grid.get(-2, 0), 79);
        assert_eq!(grid.get(7, 7), 0);
        assert_eq!(grid.get(5, 7), 2);
        assert_eq!(grid.data, grid2.data);
    }

    #[test]
    fn test_rot90() {
        let mut grid: Grid<u32> = Grid::new(-2, -2, 7, 7, 0);
        fill(&mut grid);
        let grid = grid.rot90();
        assert_eq!(grid.get(-2, -2), 90);
        assert_eq!(grid.get(-2, 0), 92);
        assert_eq!(grid.get(7, 7), 9);
        assert_eq!(grid.get(5, 7), 29);
        assert_eq!(grid.get(6, -1), 11);
    }

}
