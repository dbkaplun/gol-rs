extern crate rand;

pub mod plaintext;

use std::vec::Vec;
use std::iter::Iterator;
use std::option::Option;
use std::fmt::{ Debug, Formatter, Error };

use rand::{ Rng };

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

/// An addressable grid of `Cell`s
///
/// Provides a number of functions for constructing, modifying and walking `Cell` grids. 
#[derive(PartialEq, Clone)]
pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>
}

impl Grid {
    /// Constructs a Grid from raw components
    pub fn from_raw(width: usize, height: usize, state: Vec<Cell>) -> Grid {
        let count = width * height;

        if count != state.len() {
            panic!("Invalid height and width");
        }

        Grid { width: width, height: height, cells: state }
    }

    /// Constructs a Grid of `width` and `height` using a factory function.
    pub fn from_fn<F>(width: usize, height: usize, f: F) -> Grid
        where F: FnMut((usize, usize)) -> Cell
    {
        let count = width * height;
        let cells = (0..count).map(|i| (i % width, i / width)).map(f).collect();
        Grid { width: width, height: height, cells: cells }
    }

    /// Constructs a dead grid of `width` and `height`
    pub fn create_dead(width: usize, height: usize) -> Grid {
        let count = width * height;

        Grid { width: width, height: height, cells: vec![Cell::Dead; count] }
    }

    /// Constructs a random grid of `width` and `height`
    pub fn create_random<R: Rng>(rng: &mut R, width: usize, height: usize) -> Grid {
        let choices = [ Cell::Live, Cell::Dead ];
        Grid::from_fn(width, height, |_| rng.choose(&choices).unwrap().clone())
    }

    /// Gets the width of this `Grid`
    pub fn width(&self) -> usize {
        self.width
    }

    /// Gets the height of this `Grid`
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

    /// Returns an iterator over rows in this `Grid`
    pub fn iter_rows(&self) -> RowIterator {
        RowIterator { grid: self, row: 0 }
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {

        try!(write!(f, "{}x{} grid:", self.width, self.height));

        for row in (RowIterator { grid: self, row: 0 }) {
            try!(write!(f, "\n"));
            for cell in row {
                try!(write!(f, "{}", if cell.is_live() { "O" } else { "." }));
            }
        }

        Ok(())
    }
}

type Delta = isize;

/// Calculates a new index within a wrapped dimension of `dimension_size`
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
            panic!(print!("Unexpected delta: {}", delta))
        }
    }
}

/// Represents a Game of Life Grid + generation
pub struct World {
    gen: i64,
    state: Grid
}

impl World {

    /// Constructs a new `World` with the given `Grid`
    pub fn new(grid: Grid) -> World {
        World { gen: 0, state: grid }
    }

    /// Gets the current generation for this `World`
    pub fn generation(&self) -> i64 {
        self.gen
    }

    /// Gets the width of this `World`
    pub fn width(&self) -> usize {
        self.state.width
    }

    /// Gets the height of this `World`
    pub fn height(&self) -> usize {
        self.state.height
    }

    fn get_next_state(&self) -> Grid {
        // Generate the next world state from the current
        let w = self.state.width;
        let h = self.state.height;

        let new_cells = self.state.cells.iter().enumerate().map(|(index, cell)| {
            let y = index / w;
            let x = index % w;

            let neighbours = self.find_neighbours(x, y);

            match (cell, neighbours) {
                (&Cell::Live, 3) |
                (&Cell::Live, 2) |
                (&Cell::Dead, 3) => Cell::Live,
                ________________ => Cell::Dead
            }
        })
        .collect();

        Grid::from_raw(w, h, new_cells)
    }

    /// Executes a single step of this `World` in place
    pub fn step_mut(&mut self) {
        self.state = self.get_next_state();
        self.gen += 1;
    }

    /// Executes a single step of this `World`, and returns a new world
    pub fn step(&self) -> World {
        let next_state = self.get_next_state();
        World { gen: self.gen + 1, state: next_state }
    }

    fn find_neighbours(&self, x: usize, y: usize) -> usize {
    
        let offsets = &[-1, 0, 1];
        let (w, h) = (self.width(), self.height());
        
        let neighbours = 
            offsets
            .iter()
            .flat_map(|x_offset| offsets.iter().map(move |y_offset| (*x_offset, *y_offset)))
            .filter(|&offset| offset != (0, 0))
            .map(|(x_offset, y_offset)| {
                let y = offset_in_dim(h, y, y_offset);
                let x = offset_in_dim(w, x, x_offset);
                self.state.cell_at(x, y)
            })
            .filter(|cell| cell.is_live())
            .count();

        neighbours
    }

    /// Overwrite the cells starting at coords `(x, y)` with the data in the given `Grid`
    pub fn write_cells(&mut self, x: usize, y: usize, data: &Grid) {

        for data_y in (0..data.height) {
            for data_x in (0..data.width) {

                let state_y = offset_in_dim(self.state.height, y, data_y as isize);
                let state_x = offset_in_dim(self.state.width, x, data_x as isize);

                let cell = data.cell_at(data_x, data_y);
                self.state.set_cell(state_x, state_y, cell.clone());
            }
        }

    }
 
    /// Returns an iterator over rows in this world
    pub fn iter_rows(&self) -> RowIterator {
        RowIterator { grid: &self.state, row: 0 }
    }
}

/// Iterator for the rows in a `Grid`
pub struct RowIterator<'a> {
    grid: &'a Grid,
    row: usize
}

impl <'a> Iterator for RowIterator<'a> {
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

#[cfg(test)]
mod gol_tests {

    use rand::{ SeedableRng, StdRng };

    use super::{ World, Grid, Delta };
    use super::Cell::{ Live, Dead };

    fn make_rng() -> StdRng {
        let seed: &[_] = &[1, 2, 3, 4];
        SeedableRng::from_seed(seed)
    }

    #[test]
    fn can_create_grid_from_fn() {

        let grid = Grid::from_fn(2, 2, |coords| match coords {
            (0, 0) => Live,
            (1, 0) => Dead,
            (0, 1) => Dead,
            (1, 1) => Live,
             ____  => unreachable!()
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
    fn can_create_random_grid() {
        
        let mut rng = make_rng();

        let grid = Grid::create_random(&mut rng, 10, 10);

        assert_eq!(grid.width, 10);
        assert_eq!(grid.height, 10);
        assert_eq!(grid.cells.len(), 100);
    }

    #[test]
    fn can_create_world_with_grid() {
        
        let grid = Grid::from_fn(10, 10, |_| Dead);
        let w = World::new(grid.clone());
        assert_eq!(0, w.gen);
        assert_eq!(&grid.cells, &w.state.cells);
    }

    #[test]
    #[should_panic(expected = "Invalid height and width")]
    fn creating_grid_with_invalid_raw_state_panics() {
        
        let state = vec![Dead; 99];

        Grid::from_raw(10, 10, state);
    }

    fn make_square_world() -> World {
        use super::Cell::Dead as X;
        use super::Cell::Live as O;

        let state = vec![
            O, O, O,
            O, X, O,
            O, O, O,
        ];

        World::new(Grid::from_raw(3, 3, state))
    }

    fn make_pipe_world() -> World {
        use super::Cell::Dead as X;
        use super::Cell::Live as O;

        let state = vec![
            X, X, X, O,
            X, X, X, O,
            X, X, X, O,
            X, X, X, X,
        ];

        World::new(Grid::from_raw(4, 4, state))
    }

    fn make_lonely_world() -> World {
        use super::Cell::Dead as X;
        use super::Cell::Live as O;

        let state = vec![
            X, X, X,
            X, O, X,
            X, X, X,
        ];

        World::new(Grid::from_raw(3, 3, state))
    }

    fn make_oblong_world() -> World {
        use super::Cell::Dead as X;
        use super::Cell::Live as O;

        let state = vec![
        /*      0     1     2     3     4  */
        /* 0 */ X, X, O, X, X,
        /* 1 */ X, O, X, O, X,
        /* 2 */ X, X, O, X, X,
        ];

        World::new(Grid::from_raw(5, 3, state))
    }

    fn make_glider_world() -> World {
        use super::Cell::Dead as X;
        use super::Cell::Live as O;

        let state = vec![
            X, X, X, X, X, X,
            X, X, X, O, X, X,
            X, O, X, O, X, X,
            X, X, O, O, X, X,
            X, X, X, X, X, X,
        ];

        World::new(Grid::from_raw(6, 5, state))
    }

    #[test]
    fn can_count_neighbours_on_square_world() {

        let w = make_square_world();

        let neighbours = w.find_neighbours(1, 1);

        assert_eq!(neighbours, 8);
    }

    #[test]
    fn can_count_neighbours_on_pipe_world() {

        let w = make_pipe_world();

        let neighbours = w.find_neighbours(2, 1);

        assert_eq!(neighbours, 3);
    }

    #[test]
    fn can_count_neighbours_on_edge_of_pipe_world() {

        let w = make_pipe_world();

        let neighbours = w.find_neighbours(0, 1);

        assert_eq!(neighbours, 3);
    }

    #[test]
    fn can_count_neighbours_on_lonely_world() {

        let w = make_lonely_world();

        let neighbours = w.find_neighbours(1, 1);

        assert_eq!(neighbours, 0);
    }

    #[test]
    fn can_count_neighbours_on_oblong_world() {
        let w = make_oblong_world();

        let neighbours = w.find_neighbours(2, 1);

        assert_eq!(neighbours, 4);
    }

    #[test]
    fn can_count_neighbours_at_bottom_right_of_oblong_world() {
        let w = make_oblong_world();

        let neighbours = w.find_neighbours(4, 2);

        assert_eq!(neighbours, 1);
    }

    #[test]
    fn can_count_neighbours_at_top_left_of_oblong_world() {
        let w = make_oblong_world();

        let neighbours = w.find_neighbours(0, 0);

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

    #[test]
    fn can_step_pipe_world_mutably() {
        use super::Cell::Dead as X;
        use super::Cell::Live as O;

        let mut w = make_pipe_world();

        w.step_mut();

        let expected = Grid::from_raw(4, 4, vec![
            X, X, X, X,
            O, X, O, O,
            X, X, X, X,
            X, X, X, X,
        ]);

        assert_eq!(&w.state, &expected);
    }

    #[test]
    fn can_step_pipe_world_immutably() {
        use super::Cell::Dead as X;
        use super::Cell::Live as O;

        let w = make_pipe_world();

        let w2 = w.step();

        let expected = Grid::from_raw(4, 4, vec![
            X, X, X, X,
            O, X, O, O,
            X, X, X, X,
            X, X, X, X,
        ]);

        assert_eq!(&w2.state, &expected);
    }


    #[test]
    fn can_increment_generation() {
        //initial generation
        let w = make_square_world();
        assert_eq!(w.generation(), 0);
        
        //immutable step
        let w_two = w.step();
        assert_eq!(w_two.generation(), 1);

        //mutable step
        let mut w_mut = w;
        w_mut.step_mut();
        assert_eq!(w_mut.generation(), 1);
    }

    #[test]
    fn can_step_glider_world() {
        use super::Cell::Dead as X;
        use super::Cell::Live as O;

        //Step the glider twice, and assert we got the correct output
        let mut w = make_glider_world();

        w.step_mut();
        w.step_mut();

        let expected = Grid::from_raw(6, 5, vec![
            X, X, X, X, X, X,
            X, X, X, O, X, X,
            X, X, X, X, O, X,
            X, X, O, O, O, X,
            X, X, X, X, X, X,
        ]);

        assert_eq!(&w.state, &expected);
    }

    #[test]
    fn can_step_lonely_world() {
        use super::Cell::Dead as X;

        //Step the lonely twice, and assert that every 
        //cell died and that none came back to life
        let mut w = make_lonely_world();

        w.step_mut();
        w.step_mut();

        let expected = Grid::from_raw(3, 3, vec![
            X, X, X,
            X, X, X,
            X, X, X,
        ]);

        assert_eq!(&w.state, &expected);
    }

    #[test]
    fn can_iterate_rows_in_oblong_world_correctly() {
        use super::Cell::Dead as X;
        use super::Cell::Live as O;

        let w = make_oblong_world();

        let mut iter = w.iter_rows();

        assert_eq!(iter.next().unwrap(), &[X, X, O, X, X]);
        assert_eq!(iter.next().unwrap(), &[X, O, X, O, X]);
        assert_eq!(iter.next().unwrap(), &[X, X, O, X, X]);
        assert!(iter.next().is_none());
    }

    #[test]
    fn can_get_and_set_cell_in_grid() {

        let mut grid = Grid::create_dead(10, 10);

        assert_eq!(&Dead, grid.cell_at(2, 2));

        grid.set_cell(2, 2, Live);

        assert_eq!(&Live, grid.cell_at(2, 2));
    }

    #[test]
    #[should_panic(expected="out of range")]
    fn get_cell_out_of_range_panics() {

        let grid = Grid::create_dead(10, 10);

        grid.cell_at(10, 10);
    }

    #[test]
    #[should_panic(expected="out of range")]
    fn set_cell_out_of_range_panics() {

        let mut grid = Grid::create_dead(10, 10);

        grid.set_cell(10, 10, Live);
    }

    #[test]
    fn can_set_region_in_world() {

        use super::Cell::Dead as X;
        use super::Cell::Live as O;

        let new_data = Grid::from_raw(3, 3, vec![
            O, O, O,
            O, X, O,
            O, O, O,
        ]);

        //write from top left
        let mut w = make_lonely_world();
        w.write_cells(0, 0, &new_data);

        assert_eq!(&w.state, &new_data);

        //write from bottom right
        let mut w = make_lonely_world();
        w.write_cells(2, 2, &new_data);

        let expected = &Grid::from_raw(3, 3, vec![
            X, O, O,
            O, O, O,
            O, O, O,
        ]);
        assert_eq!(&w.state, expected);

    }
}
