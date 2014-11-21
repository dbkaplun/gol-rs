use std::vec::Vec;
use std::result::Result;
use std::iter::Iterator;
use std::option::Option;

#[deriving(PartialEq, Clone, Show)]
pub enum Cell { Live, Dead }

impl Cell {
    pub fn is_live(&self) -> bool {
        match *self { Cell::Live => true, _ => false }
    }

    pub fn is_dead(&self) -> bool {
        match *self { Cell::Dead => true, _ => false }
    }
}

pub struct World {
    cells: uint,
    rows: uint,
    gen: uint,
    state: Vec<Cell>
}

#[deriving(Show)]
pub enum GolError {
    InvalidState(&'static str)
}

#[deriving(Show, PartialEq)]
enum Delta {
    Less(uint),
    Zero,
    More(uint)
}

fn calculate_index(dimension_size: uint, current_index: uint, delta: Delta) -> uint {
    use Delta::{ Less, Zero, More };
    match delta {
        Less(n) => {
            if current_index >= n {
                current_index - n
            }
            else {
                dimension_size - (n - current_index)
            }
        },
        Zero => { current_index },
        More(n) => {
            let diff = dimension_size - current_index;
            if diff > n {
                current_index + n
            }
            else {
                n - diff
            }
        }
    }
}

impl World {

    pub fn generation(&self) -> uint {
        self.gen
    }

    pub fn cells(&self) -> uint {
        self.cells
    }

    pub fn rows(&self) -> uint {
        self.rows
    }

    pub fn try_create(rows: uint, cells: uint, state: Vec<Cell>) -> Result<World, GolError> {
        if cells * rows != state.len() {
            return Err(GolError::InvalidState("State does not fit rows and cells requirements"));
        }

        Ok(World { cells: cells, rows: rows, gen: 0, state: state })
    }

    fn get_next_state(&self) -> Vec<Cell> {
        // Generate the next world state from the current
        Vec::from_fn(self.cells * self.rows, |index| {
            let row = index / self.cells;
            let cell = index % self.cells;

            let curr = self.state[index];  
            let curr_neighbours = self.find_neighbours(row, cell);

            match (curr, curr_neighbours) {
                (Cell::Live, 3) |
                (Cell::Live, 2) |
                (Cell::Dead, 3) => Cell::Live,
                (Cell::Live, _) |
                (Cell::Dead, _) => Cell::Dead
            }
        })
    }

    pub fn step_mut(&mut self) {
        self.state = self.get_next_state();
        self.gen += 1;
    }

    pub fn step(&self) -> World {
        let next_state = self.get_next_state();
        World { rows: self.rows, cells: self.cells, gen: self.gen + 1, state: next_state }
    }

    fn find_neighbours(&self, row: uint, cell: uint) -> u8 {
        use Delta::{ Less, Zero, More };
    
        let mut neighbours = 0;

        for &row_offset in [Less(1), Zero, More(1)].iter() {

            for &cell_offset in [Less(1), Zero, More(1)].iter() {

                if row_offset == Zero && cell_offset == Zero {
                    continue; //Don't count "current" cell
                }

                let row = calculate_index(self.rows, row, row_offset);
                let cell = calculate_index(self.cells, cell, cell_offset);

                let neighbour_is_alive = 
                    self.state[row * self.cells + cell]
                        .is_live();

                if neighbour_is_alive {
                    neighbours += 1;
                }
            }
        }

        neighbours
    }

    pub fn write_cells(&mut self, row: uint, cell: uint, cells: uint, rows: uint, state: &[Cell]) {

        for state_row in range(0, rows) {
            for state_cell in range(0, cells) {

                let c = state[state_row * cells + state_cell];

                let row = calculate_index(self.rows, row, Delta::More(state_row));
                let cell = calculate_index(self.cells, cell, Delta::More(state_cell));

                self.state[row * self.cells + cell] = c;
            }
        }

    }
 
    pub fn iter_rows(&self) -> RowIterator {
        RowIterator { w: self, row: 0 }
    }
}

pub struct RowIterator<'a> {
    w: &'a World,
    row: uint
}

impl <'a> Iterator<&'a [Cell]> for RowIterator<'a> {
    fn next(&mut self) -> Option<&'a [Cell]> {
        let row = self.row;
        if row == self.w.rows {
            return None;
        }
        //increment iterator
        self.row += 1;
        let start = self.w.cells * row;
        let end = start + self.w.cells;
        Some(self.w.state.slice_or_fail(&start, &end))
    }
}

#[cfg(test)]
mod test {

    use super::World;
    use super::Delta;
    use super::Cell::Dead;

    #[test]
    fn can_create_world() {
        
        let state = Vec::from_fn(100, |_| Dead);

        let w = World::try_create(10, 10, state.clone());
        assert!(w.is_ok());

        let w = w.unwrap();
        assert_eq!(state.as_slice(), w.state.as_slice());
    }

    #[test]
    fn can_fail_to_create_world() {
        
        let state = Vec::from_fn(99, |_| Dead);

        let w = World::try_create(10, 10, state);

        assert!(w.is_err());
    }

    fn make_square_world() -> World {
        use super::Cell::Dead as X;
        use super::Cell::Live as O;

        let state = vec![
            O, O, O,
            O, X, O,
            O, O, O,
        ];
        World::try_create(3, 3, state).unwrap()
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
        World::try_create(4, 4, state).unwrap()
    }

    fn make_lonely_world() -> World {
        use super::Cell::Dead as X;
        use super::Cell::Live as O;

        let state = vec![
            X, X, X,
            X, O, X,
            X, X, X,
        ];
        World::try_create(3, 3, state).unwrap()
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
        World::try_create(3, 5, state).unwrap()
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
        World::try_create(5, 6, state).unwrap()
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

        //Verify that calculate_index correctly wraps the world

        //Middle of dimension
        assert_eq!(super::calculate_index(10, 5, Delta::More(6)), 1);
        assert_eq!(super::calculate_index(10, 5, Delta::More(4)), 9);
        assert_eq!(super::calculate_index(10, 5, Delta::More(1)), 6);
        assert_eq!(super::calculate_index(10, 5, Delta::Zero),    5);
        assert_eq!(super::calculate_index(10, 5, Delta::Less(1)), 4);
        assert_eq!(super::calculate_index(10, 5, Delta::Less(4)), 1);
        assert_eq!(super::calculate_index(10, 5, Delta::Less(6)), 9);

        //End of dimension
        assert_eq!(super::calculate_index(10, 9, Delta::More(2)), 1);
        assert_eq!(super::calculate_index(10, 9, Delta::More(1)), 0);
        assert_eq!(super::calculate_index(10, 9, Delta::Zero),    9);
        assert_eq!(super::calculate_index(10, 9, Delta::Less(1)), 8);
        assert_eq!(super::calculate_index(10, 9, Delta::Less(2)), 7);
        
        //Start of dimension
        assert_eq!(super::calculate_index(10, 0, Delta::More(2)), 2);
        assert_eq!(super::calculate_index(10, 0, Delta::More(1)), 1);
        assert_eq!(super::calculate_index(10, 0, Delta::Zero),    0);
        assert_eq!(super::calculate_index(10, 0, Delta::Less(1)), 9);
        assert_eq!(super::calculate_index(10, 0, Delta::Less(2)), 8);
    }

    #[test]
    fn can_step_pipe_world_mutably() {
        use super::Cell::Dead as X;
        use super::Cell::Live as O;

        let mut w = make_pipe_world();

        w.step_mut();

        let expected = [
            X, X, X, X,
            O, X, O, O,
            X, X, X, X,
            X, X, X, X,
        ];

        assert_eq!(expected.as_slice(), w.state.as_slice());
    }

    #[test]
    fn can_step_pipe_world_immutably() {
        use super::Cell::Dead as X;
        use super::Cell::Live as O;

        let w = make_pipe_world();

        let w2 = w.step();

        let expected = [
            X, X, X, X,
            O, X, O, O,
            X, X, X, X,
            X, X, X, X,
        ];

        assert_eq!(expected.as_slice(), w2.state.as_slice());
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

        let expected = [
            X, X, X, X, X, X,
            X, X, X, O, X, X,
            X, X, X, X, O, X,
            X, X, O, O, O, X,
            X, X, X, X, X, X,
        ];

        assert_eq!(w.state.as_slice(), expected.as_slice());
    }

    #[test]
    fn can_step_lonely_world() {
        use super::Cell::Dead as X;

        //Step the lonely twice, and assert that every 
        //cell died and that none came back to life
        let mut w = make_lonely_world();

        w.step_mut();
        w.step_mut();

        let expected = [
            X, X, X,
            X, X, X,
            X, X, X,
        ];

        assert_eq!(w.state.as_slice(), expected.as_slice());
    }

    #[test]
    fn can_iterate_rows_in_oblong_world_correctly() {
        use super::Cell::Dead as X;
        use super::Cell::Live as O;

        let w = make_oblong_world();

        let mut iter = w.iter_rows();

        assert_eq!(iter.next().unwrap(), [X, X, O, X, X].as_slice());
        assert_eq!(iter.next().unwrap(), [X, O, X, O, X].as_slice());
        assert_eq!(iter.next().unwrap(), [X, X, O, X, X].as_slice());
        assert!(iter.next().is_none());
    }
}