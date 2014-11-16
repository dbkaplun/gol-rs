use std::vec::Vec;
use std::result::Result;
use std::iter::Iterator;
use std::option::Option;

#[deriving(PartialEq, Clone, Show)]
pub enum Cell { Live, Dead }

impl Cell {
    pub fn is_live(&self) -> bool {
        match *self { Live => true, _ => false }
    }

    pub fn is_dead(&self) -> bool {
        match *self { Dead => true, _ => false }
    }
}

pub struct World {
    width: uint,
    height: uint,
    state: Vec<Cell>
}

#[deriving(Show)]
pub enum GolError {
    InvalidState(&'static str)
}

fn get_actual_index(dimension_len: uint, current_index: uint, offset: int) -> uint {
    match offset {
        -1 => if current_index == 0 { dimension_len - 1 } else { current_index - 1 },
         0 => current_index,
         1 => if current_index >= (dimension_len - 1) { 0 } else { current_index + 1 },
         _ => panic!("invalid offset")
    }
}

impl World {

    pub fn width(&self) -> uint {
        self.width
    }

    pub fn height(&self) -> uint {
        self.height
    }

    pub fn try_create(width: uint, height: uint, state: Vec<Cell>) -> Result<World, GolError> {
        if width * height != state.len() {
            return Err(InvalidState("State does not fit height and width requirements"));
        }

        Ok(World { width: width, height: height, state: state })
    }

    fn get_next_state(&self) -> Vec<Cell> {
        // Generate the next world state from the current
        Vec::from_fn(self.width * self.height, |index| {
            let row = index / self.width;
            let cell = index % self.width;

            let curr = self.state[index];  
            let curr_neighbours = self.find_neighbours(row, cell);

            match (curr, curr_neighbours) {
                (Live, 3) |
                (Live, 2) |
                (Dead, 3) => Live,
                (Live, _) |
                (Dead, _) => Dead
            }
        })
    }

    pub fn step_mut(&mut self) {
        self.state = self.get_next_state();
    }

    pub fn step(&self) -> World {
        let next_state = self.get_next_state();
        World { height: self.height, width: self.width, state: next_state }
    }

    fn find_neighbours(&self, row: uint, cell: uint) -> u8 {
    
        let mut neighbours = 0;

        for &row_offset in [-1, 0, 1].iter() {

            for &cell_offset in [-1, 0, 1].iter() {

                if row_offset == 0 && cell_offset == 0 {
                    continue; //Don't count "current" cell
                }

                let row = get_actual_index(self.height, row, row_offset);
                let cell = get_actual_index(self.width, cell, cell_offset);

                let neighbour_is_alive = 
                    self.state[row * self.width + cell]
                        .is_live();

                if neighbour_is_alive {
                    neighbours += 1;
                }
            }
        }

        neighbours
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
        if row >= (self.w.height - 1) {
            return None;
        }
        self.row += 1;
        let start = self.w.height * row;
        let end = start + self.w.width;
        Some(self.w.state.slice_or_fail(&start, &end))
    }
}

#[cfg(test)]
mod test {

    use super::{ World, Live, Dead };

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
        let state = vec![
            Live, Live, Live,
            Live, Dead,  Live,
            Live, Live, Live,
        ];
        World::try_create(3, 3, state).unwrap()
    }

    fn make_pipe_world() -> World {
        let state = vec![
            Dead, Dead, Dead, Live,
            Dead, Dead, Dead, Live,
            Dead, Dead, Dead, Live,
            Dead, Dead, Dead, Dead,
        ];
        World::try_create(4, 4, state).unwrap()
    }

    fn make_lonely_world() -> World {
        let state = vec![
            Dead, Dead, Dead,
            Dead, Live, Dead,
            Dead, Dead, Dead,
        ];
        World::try_create(3, 3, state).unwrap()
    }

    fn make_oblong_world() -> World {
        let state = vec![
        /*      0     1     2     3     4  */
        /* 0 */ Dead, Dead, Live, Dead, Dead,
        /* 1 */ Dead, Live, Dead, Live, Dead,
        /* 2 */ Dead, Dead, Live, Dead, Dead,
        ];
        World::try_create(5, 3, state).unwrap()
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
    fn can_get_actual_index() {

        //Verify that get_actual_index correctly wraps the world

        //Middle of dimension
        assert_eq!(super::get_actual_index(10, 5,  1), 6);
        assert_eq!(super::get_actual_index(10, 5,  0), 5);
        assert_eq!(super::get_actual_index(10, 5, -1), 4);

        //End of dimension
        assert_eq!(super::get_actual_index(10, 9,  1), 0);
        assert_eq!(super::get_actual_index(10, 9,  0), 9);
        assert_eq!(super::get_actual_index(10, 9, -1), 8);
        
        //Start of dimension
        assert_eq!(super::get_actual_index(10, 0,  1), 1);
        assert_eq!(super::get_actual_index(10, 0,  0), 0);
        assert_eq!(super::get_actual_index(10, 0, -1), 9);
    }

    #[test]
    #[should_fail]
    fn can_panic_with_invalid_offset() {
        super::get_actual_index(10, 0,  2);
    }

    #[test]
    fn can_step_pipe_world_mutably() {
        let mut w = make_pipe_world();

        w.step_mut();

        let expected = [
            Dead, Dead, Dead, Dead,
            Live, Dead, Live, Live,
            Dead, Dead, Dead, Dead,
            Dead, Dead, Dead, Dead,
        ];

        assert_eq!(expected.as_slice(), w.state.as_slice());
    }

    #[test]
    fn can_step_pipe_world_immutably() {
        let w = make_pipe_world();

        let w2 = w.step();

        let expected = [
            Dead, Dead, Dead, Dead,
            Live, Dead, Live, Live,
            Dead, Dead, Dead, Dead,
            Dead, Dead, Dead, Dead,
        ];

        assert_eq!(expected.as_slice(), w2.state.as_slice());
    }
}