//! The grid module provides `Grid` for working with a "torus world" grid of cells.

use std::fmt::{Debug, Error, Formatter};
use std::iter::Iterator;
use std::option::Option;
use std::vec::Vec;

/// Represents a single Cell, alive or dead
#[derive(PartialEq, Clone, Debug)]
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

#[derive(PartialEq, Clone)]
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

    /// Returns a reference to the `Cell` at the given coordinates
    #[inline]
    pub fn cell_at(&self, x: usize, y: usize) -> &Cell {
        match self.cells.get(y * self.width + x) {
            Some(c) => c,
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

    /// Overwrite the cells starting at coords `(x, y)` with the data in the given `Grid`
    /// If any coordinates are outside the grid no action is taken.
    pub fn write_cells(&mut self, x: usize, y: usize, data: &Grid) {
        for data_y in 0..data.height() {
            for data_x in 0..data.width() {
                let grid_y = y + data_y;
                let grid_x = x + data_x;

                if grid_x >= self.width || grid_y >= self.height {
                    continue;
                }

                let cell = data.cell_at(data_x, data_y);
                self.set_cell(grid_x, grid_y, cell.clone());
            }
        }
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

impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        try!(write!(f, "{}x{} grid:", self.width, self.height));

        for row in self.iter_rows() {
            try!(write!(f, "\n"));
            for cell in row {
                try!(write!(f, "{}", if cell.is_live() { "#" } else { "." }));
            }
        }

        Ok(())
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

#[cfg(test)]
pub mod tests {
    use super::Cell::{Dead, Live};
    use super::Grid;

    pub fn make_square_grid() -> Grid {
        use super::Cell::Dead as X;
        use super::Cell::Live as O;

        #[cfg_attr(rustfmt, rustfmt_skip)]
        let state = vec![
            O, O, O,
            O, X, O,
            O, O, O,
        ];

        Grid::from_raw(3, 3, state)
    }

    pub fn make_pipe_grid() -> Grid {
        use super::Cell::Dead as X;
        use super::Cell::Live as O;

        #[cfg_attr(rustfmt, rustfmt_skip)]
        let state = vec![
            X, X, X, O,
            X, X, X, O,
            X, X, X, O,
            X, X, X, X,
        ];

        Grid::from_raw(4, 4, state)
    }

    pub fn make_lonely_grid() -> Grid {
        use super::Cell::Dead as X;
        use super::Cell::Live as O;

        #[cfg_attr(rustfmt, rustfmt_skip)]
        let state = vec![
            X, X, X,
            X, O, X,
            X, X, X,
        ];

        Grid::from_raw(3, 3, state)
    }

    pub fn make_oblong_grid() -> Grid {
        use super::Cell::Dead as X;
        use super::Cell::Live as O;

        #[cfg_attr(rustfmt, rustfmt_skip)]
        let state = vec![
        /*      0  1  2  3  4 */
        /* 0 */ X, X, O, X, X,
        /* 1 */ X, O, X, O, X,
        /* 2 */ X, X, O, X, X,
        ];

        Grid::from_raw(5, 3, state)
    }

    pub fn make_glider_grid() -> Grid {
        use super::Cell::Dead as X;
        use super::Cell::Live as O;

        #[cfg_attr(rustfmt, rustfmt_skip)]
        let state = vec![
            X, X, X, X, X, X,
            X, X, X, O, X, X,
            X, O, X, O, X, X,
            X, X, O, O, X, X,
            X, X, X, X, X, X,
        ];

        Grid::from_raw(6, 5, state)
    }

    #[test]
    fn can_create_grid_from_fn() {
        let grid = Grid::from_fn(2, 2, |x, y| match (x, y) {
            (0, 0) => Live,
            (1, 0) => Dead,
            (0, 1) => Dead,
            (1, 1) => Live,
            ____ => unreachable!(),
        });

        assert_eq!(grid.width, 2);
        assert_eq!(grid.height, 2);
        assert_eq!(grid.cells.len(), 4);
        assert_eq!(grid.cells[0], Live);
        assert_eq!(grid.cells[1], Dead);
        assert_eq!(grid.cells[2], Dead);
        assert_eq!(grid.cells[3], Live);
    }

    #[test]
    fn can_create_dead_grid() {
        let grid = Grid::create_dead(10, 10);

        assert_eq!(grid.width, 10);
        assert_eq!(grid.height, 10);
        assert_eq!(grid.cells.len(), 100);

        for cell in &grid.cells {
            assert_eq!(&Dead, cell)
        }
    }

    #[test]
    #[should_panic(expected = "Invalid height and width")]
    fn creating_grid_with_invalid_raw_state_panics() {
        let state = vec![Dead; 99];

        Grid::from_raw(10, 10, state);
    }
}
