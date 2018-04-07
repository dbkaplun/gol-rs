//! The grid module provides `Grid` for working with a "torus world" grid of cells.

use std::error::Error;
use std::fmt;
use std::ops::{Index, IndexMut, Range, RangeFull};
use std::str::FromStr;

use gridview::GridView;

/// Represents a single Cell, alive or dead
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Cell {
    Live,
    Dead,
}

impl Cell {
    pub fn is_live(&self) -> bool {
        match *self {
            Cell::Live => true,
            _ => false,
        }
    }

    pub fn is_dead(&self) -> bool {
        match *self {
            Cell::Dead => true,
            _ => false,
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", if self.is_live() { "O" } else { "." })
    }
}

pub type Coord = (usize, usize);

#[derive(PartialEq, Eq, Hash, Clone)]
/// An addressable grid of `Cell`s
///
/// Provides a number of functions for constructing, modifying and walking `Cell` grids.
pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Grid {
    /// Constructs a Grid from raw components
    pub fn from_raw(width: usize, height: usize, state: Vec<Cell>) -> Grid {
        let count = width * height;

        if count != state.len() {
            panic!("Invalid height and width");
        }
        Grid {
            width,
            height,
            cells: state,
        }
    }

    /// Constructs a Grid of `width` and `height` using a factory function.
    pub fn from_fn<F>(width: usize, height: usize, mut f: F) -> Grid
    where
        F: FnMut(usize, usize) -> Cell,
    {
        let count = width * height;
        let cells = (0..count).map(|i| f(i % width, i / width)).collect();
        Grid {
            width,
            height,
            cells,
        }
    }

    /// Constructs a dead grid of `width` and `height`
    pub fn create_dead(width: usize, height: usize) -> Grid {
        let count = width * height;
        Grid {
            width,
            height,
            cells: vec![Cell::Dead; count],
        }
    }

    /// Gets the size of this `Grid`
    #[inline]
    pub fn size(&self) -> usize {
        self.width() * self.height()
    }

    /// Gets the width of this `Grid`
    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    /// Gets the height of this `Grid`
    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns a contiguous slice containing the cells of this `Grid`
    #[inline]
    pub fn cells(&self) -> &[Cell] {
        &self.cells[0..self.size()]
    }

    /// Returns a contiguous slice containing the cells of this `Grid`
    #[inline]
    pub fn cells_mut(&mut self) -> &mut [Cell] {
        let size = self.size();
        &mut self.cells[0..size]
    }

    /// Returns a reference to the `Cell` at the given coordinates
    #[inline]
    pub fn cell_at(&self, x: usize, y: usize) -> Cell {
        match self.cells.get(y * self.width + x) {
            Some(&c) => c,
            None => panic!("Coordinates ({}, {}) out of range", x, y),
        }
    }

    /// Overwrites the `Cell` at the given coordinates with the given value
    #[inline]
    pub fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        if let Some(c) = self.cells.get_mut(y * self.width + x) {
            *c = cell;
        } else {
            panic!("Coordinates ({}, {}) out of range", x, y)
        }
    }

    /// Shrinks grid to given size. Panics if size is larger than current grid.
    pub fn shrink(&mut self, size: Coord, offset: Coord) {
        let (old_w, old_h) = (self.width(), self.height());
        let (new_w, new_h) = size;
        let (off_x, off_y) = offset;

        if old_w < new_w + off_x || old_h < new_h + off_y {
            panic!("new size plus offset must be at most current size");
        }

        let mut i = 0;
        self.cells.retain(|_| {
            let old_col = i % old_w;
            let old_row = i / old_w;

            let is_col_ok = off_x <= old_col && old_col < off_x + new_w;
            let is_row_ok = off_y <= old_row && old_row < off_y + new_h;

            i += 1;
            is_row_ok && is_col_ok
        });
        self.width = new_w;
        self.height = new_h;
    }

    /// Overwrite the cells starting at `offset` with the data in the given `Grid`.
    /// Panics if any coordinates are outside the grid.
    pub fn write_cells(&mut self, data: &Grid, offset: Coord) {
        let (sw, _sh) = (self.width(), self.height());
        let (dw, dh) = (data.width(), data.height());
        let (ox, oy) = offset;
        for dy in 0..dh {
            let sy = oy + dy;
            let sro = sy * sw;
            let dro = dy * dw;
            self.cells[sro + ox..sro + ox + dw].copy_from_slice(&data.cells[dro..dro + dw]);
        }
    }

    /// Returns a view over a range of cells in this `Grid`
    pub fn range(&self, range: Range<Coord>) -> GridView {
        GridView { grid: self, range }
    }

    /// Returns an iterator over rows in this `Grid`
    pub fn iter_rows(&self) -> RowIter {
        RowIter { grid: self, row: 0 }
    }

    /// Returns an iterator over `Cell`s in this `Grid`
    pub fn iter_cells(&self) -> CellIter {
        CellIter {
            grid: self,
            index: 0,
        }
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "!{}x{} grid:\n{}", self.width, self.height, self)
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for (i, row) in self.iter_rows().enumerate() {
            if i != 0 {
                try!(write!(f, "\n"));
            }
            for cell in row {
                try!(write!(f, "{}", cell));
            }
        }
        Ok(())
    }
}

impl FromStr for Grid {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cells = vec![];
        let mut width = 0;
        let mut height = 0;

        let mut row = vec![];
        for line in s.lines() {
            let is_first_row = cells.is_empty();

            for c in line.chars() {
                match c {
                    'O' => {
                        row.push(Cell::Live);
                    }
                    '.' => {
                        row.push(Cell::Dead);
                    }
                    _ if c.is_whitespace() => {}
                    _ => {
                        return Err(format!("found character {}, expected 'O' or '.'", c).into());
                    }
                }
            }

            if row.is_empty() {
                continue;
            }

            if is_first_row {
                width = row.len();
            } else if width != row.len() {
                return Err(format!("expected width {}, found {}", width, row.len()).into());
            }

            cells.append(&mut row); // clears `row`
            height += 1;
        }

        Ok(Self {
            width,
            height,
            cells,
        })
    }
}

impl Index<RangeFull> for Grid {
    type Output = [Cell];

    fn index(&self, _i: RangeFull) -> &Self::Output {
        self.cells()
    }
}

impl IndexMut<RangeFull> for Grid {
    fn index_mut(&mut self, _i: RangeFull) -> &mut Self::Output {
        self.cells_mut()
    }
}

/// Iterator for the rows in a `Grid`
pub struct RowIter<'a> {
    grid: &'a Grid,
    row: usize,
}

impl<'a> Iterator for RowIter<'a> {
    type Item = &'a [Cell];
    fn next(&mut self) -> Option<&'a [Cell]> {
        let row = self.row;
        if row == self.grid.height {
            return None;
        }
        //increment iterator
        self.row += 1;
        let start = self.grid.width * row;
        let end = start + self.grid.width;
        Some(&self.grid.cells[start..end])
    }
}

/// Iterator for the cells in a `Grid`
pub struct CellIter<'a> {
    grid: &'a Grid,
    index: usize,
}

impl<'a> Iterator for CellIter<'a> {
    type Item = (usize, usize, &'a Cell);

    fn next(&mut self) -> Option<(usize, usize, &'a Cell)> {
        let len = self.grid.cells.len();
        let index = self.index;
        if index < len {
            self.index += 1;
            let x = index % self.grid.width;
            let y = index / self.grid.width;
            Some((x, y, &self.grid.cells[index]))
        } else {
            None
        }
    }
}
