extern crate rand;

use std::vec::Vec;
use std::iter::Iterator;
use std::option::Option;
use std::fmt::{ Debug, Formatter, Error };

use rand::{ Rng };

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

pub struct World {
    gen: usize,
    state: Grid
}

#[derive(PartialEq, Clone)]
pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>
}

impl Grid {
    fn cell_at(&self, x: usize, y: usize) -> &Cell {
        &self.cells[y * self.width + x]
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
fn offset_in_dim(dimension_size: usize, current_index: usize, delta: &Delta) -> usize {

    match *delta {
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

impl Grid {
    pub fn from_raw(width: usize, height: usize, state: Vec<Cell>) -> Grid {
        let count = width * height;

        if count != state.len() {
            panic!("Invalid height and width");
        }

        Grid { width: width, height: height, cells: state }
    }

    fn from_fn<F>(width: usize, height: usize, f: F) -> Grid where F: FnMut(usize) -> Cell {
        let count = width * height;
        let cells = (0..count).map(f).collect();
        Grid { width: width, height: height, cells: cells }
    }

    pub fn create_dead(height: usize, width: usize) -> Grid {
        let count = width * height;

        Grid { width: width, height: height, cells: vec![Cell::Dead; count] }
    }

    pub fn create_random<R: Rng>(rng: &mut R, width: usize, height: usize) -> Grid {
        let choices = [ Cell::Live, Cell::Dead ];
        Grid::from_fn(width, height, |_| rng.choose(&choices).unwrap().clone())
    }
}

impl World {

    pub fn new(grid: Grid) -> World {
        World { gen: 0, state: grid }
    }

    pub fn generation(&self) -> usize {
        self.gen
    }

    pub fn width(&self) -> usize {
        self.state.width
    }

    pub fn height(&self) -> usize {
        self.state.height
    }

    fn get_next_state(&self) -> Grid {
        // Generate the next world state from the current
        let w = self.state.width;
        let h = self.state.height;

        let new_cells = self.state.cells.iter().enumerate().map(|(index, cell)| {
            let row_index = index / w;
            let cell_index = index % w;

            let neighbours = self.find_neighbours(row_index, cell_index);

            match (cell, neighbours) {
                (&Cell::Live, 3) |
                (&Cell::Live, 2) |
                (&Cell::Dead, 3) => Cell::Live,
                (&Cell::Live, _) |
                (&Cell::Dead, _) => Cell::Dead
            }
        })
        .collect();

        Grid::from_raw(w, h, new_cells)
    }

    pub fn step_mut(&mut self) {
        self.state = self.get_next_state();
        self.gen += 1;
    }

    pub fn step(&self) -> World {
        let next_state = self.get_next_state();
        World { gen: self.gen + 1, state: next_state }
    }

    pub fn find_neighbours(&self, row_index: usize, cell_index: usize) -> usize {
    
        let offsets = [-1, 0, 1];
        let w = self.width();
        let h = self.height();

        let mut neighbours = 0;

        for row_offset in &offsets {
            for cell_offset in &offsets {

                if *row_offset == 0 && *cell_offset == 0 {
                    continue; //Don't count "current" cell_index
                }

                let row_index = offset_in_dim(h, row_index, row_offset);
                let cell_index = offset_in_dim(w, cell_index, cell_offset);

                let neighbour_is_alive = 
                    self.state.cells[row_index * w + cell_index]
                        .is_live();

                if neighbour_is_alive {
                    neighbours += 1;
                }
            }
        }

        neighbours
    }

    pub fn find_neighbours_2(&self, row_index: usize, cell_index: usize) -> usize {
    
        let offsets = [-1, 0, 1];
        let w = self.width();
        let h = self.height();
        
        let neighbours =
            offsets.iter()
                    .flat_map(|x| offsets.iter().map(move |y| (x, y)))
                    .filter(|&(x, y)| !(*x == 0 && *y == 0))
                    .map(|(x, y)| {
                        let row = offset_in_dim(h, row_index, y);
                        let cell = offset_in_dim(w, cell_index, x);
                        self.state.cell_at(cell, row)
                    })
                    .filter(|cell| cell.is_live())
                    .count();

        neighbours
    }

    /*
    pub fn write_cells(&mut self, row: usize, cell: usize, width: usize, height: usize, state: &[Cell]) {

        for state_row in (0..height) {
            for state_cell in (0..width) {

                let c = state[state_row * width + state_cell].clone();

                let row = offset_in_dim(self.height(), row, &Delta::More(state_row));
                let cell = offset_in_dim(self.width(), cell, &Delta::More(state_cell));

                self.state.cells[row * self.width() + cell] = c;
            }
        }

    }
    */
 
    pub fn iter_rows(&self) -> RowIterator {
        RowIterator { grid: &self.state, row: 0 }
    }
}

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
mod tests {

    use rand::{ SeedableRng, StdRng };

    use super::{ World, Grid, Delta };
    use super::Cell::{ Live, Dead };

    fn make_rng() -> StdRng {
        let seed: &[_] = &[1, 2, 3, 4];
        SeedableRng::from_seed(seed)
    }

    #[test]
    fn can_create_grid_from_fn() {
        
        let grid = Grid::from_fn(10, 10, |_| Live);

        assert_eq!(grid.width, 10);
        assert_eq!(grid.height, 10);
        assert_eq!(grid.cells.len(), 100);

        for cell in &grid.cells {
            assert_eq!(&Live, cell)
        }
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

        let neighbours = w.find_neighbours(1, 2);

        assert_eq!(neighbours, 3);
    }

    #[test]
    fn can_count_neighbours_on_edge_of_pipe_world() {

        let w = make_pipe_world();

        let neighbours = w.find_neighbours(1, 0);

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

        let neighbours = w.find_neighbours(1, 2);

        assert_eq!(neighbours, 4);
    }

    #[test]
    fn can_count_neighbours_at_bottom_right_of_oblong_world() {
        let w = make_oblong_world();

        let neighbours = w.find_neighbours(2, 4);

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
        assert_eq!(super::offset_in_dim(10, 5, &(6 as Delta)), 1);
        assert_eq!(super::offset_in_dim(10, 5, &(4 as Delta)), 9);
        assert_eq!(super::offset_in_dim(10, 5, &(1 as Delta)), 6);
        assert_eq!(super::offset_in_dim(10, 5, &(0 as Delta)), 5);
        assert_eq!(super::offset_in_dim(10, 5, &(-1 as Delta)), 4);
        assert_eq!(super::offset_in_dim(10, 5, &(-4 as Delta)), 1);
        assert_eq!(super::offset_in_dim(10, 5, &(-6 as Delta)), 9);

        //End of dimension
        assert_eq!(super::offset_in_dim(10, 9, &(2 as Delta)), 1);
        assert_eq!(super::offset_in_dim(10, 9, &(1 as Delta)), 0);
        assert_eq!(super::offset_in_dim(10, 9, &(0 as Delta)), 9);
        assert_eq!(super::offset_in_dim(10, 9, &(-1 as Delta)), 8);
        assert_eq!(super::offset_in_dim(10, 9, &(-2 as Delta)), 7);
        
        //Start of dimension
        assert_eq!(super::offset_in_dim(10, 0, &(2 as Delta)), 2);
        assert_eq!(super::offset_in_dim(10, 0, &(1 as Delta)), 1);
        assert_eq!(super::offset_in_dim(10, 0, &(0 as Delta)),    0);
        assert_eq!(super::offset_in_dim(10, 0, &(-1 as Delta)), 9);
        assert_eq!(super::offset_in_dim(10, 0, &(-2 as Delta)), 8);
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
    fn can_iterate_height_in_oblong_world_correctly() {
        use super::Cell::Dead as X;
        use super::Cell::Live as O;

        let w = make_oblong_world();

        let mut iter = w.iter_rows();

        assert_eq!(iter.next().unwrap(), &[X, X, O, X, X]);
        assert_eq!(iter.next().unwrap(), &[X, O, X, O, X]);
        assert_eq!(iter.next().unwrap(), &[X, X, O, X, X]);
        assert!(iter.next().is_none());
    }
}