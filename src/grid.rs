//! The grid module provides `Grid` for working with a "torus world" grid of cells.

use std::vec::Vec;
use std::iter::Iterator;
use std::option::Option;
use std::fmt::{ Debug, Formatter, Error };

/// Represents a single Cell, alive or dead
#[derive(PartialEq, Clone, Debug)]
pub enum Cell { Live, Dead }

impl Cell {
    pub fn is_live(&self) -> bool {
        match self { &Cell::Live => true, _ => false }
    }

    pub fn is_dead(&self) -> bool {
        match self { &Cell::Dead => true, _ => false }
    }
}

/// Describes a static neighbour counting function.
///
/// This function accepts a Grid and a set of coordinates and
/// and returns the neighbour count of those coordinates.
pub type NeighboursFn = fn(grid: &Grid, coords: (usize, usize)) -> usize;

/// Implements neighbour counting for a torus world.
pub fn torus_neighbours(grid: &Grid, coords: (usize, usize)) -> usize {
    let (x, y) = coords;

    let offsets = &[-1, 0, 1];
    let (w, h) = (grid.width(), grid.height());

    offsets
        .iter()
        .flat_map(|x_off| offsets.iter().map(move |y_off| (*x_off, *y_off)))
        .filter(|&offset| offset != (0, 0))
        .map(|(x_off, y_off)| {
            let y = offset_in_dim(h, y, y_off);
            let x = offset_in_dim(w, x, x_off);
            grid.cell_at(x, y)
        })
        .filter(|cell| cell.is_live())
        .count()
}

/// An addressable grid of `Cell`s
///
/// Provides a number of functions for constructing, modifying and walking `Cell` grids.
pub struct Grid {
    width: usize,
    height: usize,
    neighbours: NeighboursFn,
    cells: Vec<Cell>
}

impl Clone for Grid {
    fn clone(&self) -> Self {
        Grid {
            width: self.width,
            height: self.height,
            neighbours: self.neighbours,
            cells: self.cells.clone()
        }
    }
}

impl PartialEq for Grid {
    fn eq(&self, other: &Grid) -> bool {
        self.width == other.width && self.height == other.height && self.cells == other.cells
    }
}

impl Grid {
    /// Constructs a Grid from raw components
    pub fn from_raw(width: usize, height: usize, state: Vec<Cell>) -> Grid {
        let count = width * height;

        if count != state.len() {
            panic!("Invalid height and width");
        }
        Grid { width: width, height: height, cells: state, neighbours: torus_neighbours  }
    }

    /// Constructs a Grid of `width` and `height` using a factory function.
    pub fn from_fn<F>(width: usize, height: usize, mut f: F) -> Grid
        where F: FnMut(usize, usize) -> Cell
    {
        let count = width * height;
        let cells = (0..count).map(|i| f(i % width, i / width)).collect();
        Grid { width: width, height: height, cells: cells, neighbours: torus_neighbours }
    }

    /// Constructs a dead grid of `width` and `height`
    pub fn create_dead(width: usize, height: usize) -> Grid {
        let count = width * height;
        Grid { width: width, height: height, cells: vec![Cell::Dead; count], neighbours: torus_neighbours }
    }

    // Sets the neighbour counting function for this grid
    #[inline]
    pub fn set_neighbours_fn(&mut self, neighbours: NeighboursFn) {
        self.neighbours = neighbours;
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
            None    => panic!("Coordinates ({}, {}) out of range", x, y),
        }
    }

    /// Overwrites the `Cell` at the given coordinates with the given value
    #[inline]
    pub fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        if let Some(c) = self.cells.get_mut(y * self.width + x) {
            *c = cell;
        }
        else {
            panic!("Coordinates ({}, {}) out of range", x, y)
        }
    }

    /// Overwrite the cells starting at coords `(x, y)` with the data in the given `Grid`
    /// If any coordinates are outside the grid no action is taken.
    pub fn write_cells(&mut self, x: usize, y: usize, data: &Grid) {
        for data_y in 0 .. data.height() {
            for data_x in 0 .. data.width() {

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

    /// Count the number of neighbours the `Cell` at the given
    /// coordinates has.
    pub fn count_neighbours(&self, x: usize, y: usize) -> usize {
        (self.neighbours)(self, (x, y))
    }

    /// Returns an iterator over rows in this `Grid`
    pub fn iter_rows(&self) -> RowIter {
        RowIter { grid: self, row: 0 }
    }

    /// Returns an iterator over `Cell`s in this `Grid`
    pub fn iter_cells(&self) -> CellIter {
        CellIter { grid: self, index: 0 }
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {

        try!(write!(f, "{}x{} grid:", self.width, self.height));

        for row in (RowIter { grid: self, row: 0 }) {
            try!(write!(f, "\n"));
            for cell in row {
                try!(write!(f, "{}", if cell.is_live() { "O" } else { "." }));
            }
        }

        Ok(())
    }
}

/// Iterator for the rows in a `Grid`
pub struct RowIter<'a> {
    grid: &'a Grid,
    row: usize
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
    index: usize
}

impl <'a> Iterator for CellIter<'a> {
    type Item = (usize, usize, &'a Cell);

    fn next(&mut self) -> Option<(usize, usize, &'a Cell)> {
        let len = self.grid.cells.len();
        let index = self.index;
        if index < len {
            self.index += 1;
            let x = index % self.grid.width;
            let y = index / self.grid.width;
            Some((x, y, &self.grid.cells[index]))
        }
        else {
            None
        }
    }
}

type Delta = isize;

/// Utility function to calculate a new index within a torus dimension of `dimension_size`
/// based on a `current_index` and a `delta`.
fn offset_in_dim(dimension_size: usize, current_index: usize, delta: Delta) -> usize {

    match delta {
        n if n < 0 => {
            //convert to unsigned representing a subtraction
            let to_subtract = n.abs() as usize;

            if current_index >= to_subtract {
                current_index - to_subtract
            }
            else {
                //wrap to end of dimension
                dimension_size - (to_subtract - current_index)
            }
        },
        0 => {
            current_index
        },
        n if n > 0 => {
            //convert to unsigned representing an addition
            let to_add = n.abs() as usize;

            let delta_to_end = dimension_size - current_index;
            if delta_to_end > to_add {
                current_index + to_add
            }
            else {
                //wrap to beginning of dimension
                to_add - delta_to_end
            }
        },
        _ => {
            panic!(format!("Unexpected delta: {}", delta))
        }
    }
}

#[cfg(test)]
pub mod tests {

    use super::{ Grid, Delta };
    use super::Cell::{ Live, Dead };

    pub fn make_square_grid() -> Grid {
        use super::Cell::Dead as X;
        use super::Cell::Live as O;

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

        let state = vec![
        /*      0     1     2     3     4  */
        /* 0 */ X, X, O, X, X,
        /* 1 */ X, O, X, O, X,
        /* 2 */ X, X, O, X, X,
        ];

        Grid::from_raw(5, 3, state)
    }

    pub fn make_glider_grid() -> Grid {
        use super::Cell::Dead as X;
        use super::Cell::Live as O;

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

        let grid = Grid::from_fn(2, 2, |x, y| {
             match (x, y) {
                (0, 0) => Live,
                (1, 0) => Dead,
                (0, 1) => Dead,
                (1, 1) => Live,
                 ____  => unreachable!()
            }
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


    #[test]
    fn can_count_neighbours_on_square_grid() {

        let g = make_square_grid();

        let neighbours = g.count_neighbours(1, 1);

        assert_eq!(neighbours, 8);
    }

    #[test]
    fn can_count_neighbours_on_pipe_grid() {

        let g = make_pipe_grid();

        let neighbours = g.count_neighbours(2, 1);

        assert_eq!(neighbours, 3);
    }

    #[test]
    fn can_count_neighbours_on_edge_of_pipe_grid() {

        let g = make_pipe_grid();

        let neighbours = g.count_neighbours(0, 1);

        assert_eq!(neighbours, 3);
    }

    #[test]
    fn can_count_neighbours_on_lonely_grid() {

        let g = make_lonely_grid();

        let neighbours = g.count_neighbours(1, 1);

        assert_eq!(neighbours, 0);
    }

    #[test]
    fn can_count_neighbours_on_oblong_grid() {
        let g = make_oblong_grid();

        let neighbours = g.count_neighbours(2, 1);

        assert_eq!(neighbours, 4);
    }

    #[test]
    fn can_count_neighbours_at_bottom_right_of_oblong_grid() {
        let g = make_oblong_grid();

        let neighbours = g.count_neighbours(4, 2);

        assert_eq!(neighbours, 1);
    }

    #[test]
    fn can_count_neighbours_at_top_left_of_oblong_grid() {
        let g = make_oblong_grid();

        let neighbours = g.count_neighbours(0, 0);

        assert_eq!(neighbours, 1);
    }

    #[test]
    fn can_calculate_index() {

        //Verify that offset_in_dim correctly wraps the world

        //Middle of dimension
        assert_eq!(super::offset_in_dim(10, 5, (6 as Delta)), 1);
        assert_eq!(super::offset_in_dim(10, 5, (4 as Delta)), 9);
        assert_eq!(super::offset_in_dim(10, 5, (1 as Delta)), 6);
        assert_eq!(super::offset_in_dim(10, 5, (0 as Delta)), 5);
        assert_eq!(super::offset_in_dim(10, 5, (-1 as Delta)), 4);
        assert_eq!(super::offset_in_dim(10, 5, (-4 as Delta)), 1);
        assert_eq!(super::offset_in_dim(10, 5, (-6 as Delta)), 9);

        //End of dimension
        assert_eq!(super::offset_in_dim(10, 9, (2 as Delta)), 1);
        assert_eq!(super::offset_in_dim(10, 9, (1 as Delta)), 0);
        assert_eq!(super::offset_in_dim(10, 9, (0 as Delta)), 9);
        assert_eq!(super::offset_in_dim(10, 9, (-1 as Delta)), 8);
        assert_eq!(super::offset_in_dim(10, 9, (-2 as Delta)), 7);

        //Start of dimension
        assert_eq!(super::offset_in_dim(10, 0, (2 as Delta)), 2);
        assert_eq!(super::offset_in_dim(10, 0, (1 as Delta)), 1);
        assert_eq!(super::offset_in_dim(10, 0, (0 as Delta)),    0);
        assert_eq!(super::offset_in_dim(10, 0, (-1 as Delta)), 9);
        assert_eq!(super::offset_in_dim(10, 0, (-2 as Delta)), 8);
    }

}