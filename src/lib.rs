#![allow(dead_code, unused_imports, unused_variables)]

use std::vec::Vec;
use std::option::Option;
use std::result::Result;
use std::fmt::{ Show, Formatter, FormatError };

#[deriving(PartialEq, Clone)]
pub enum Cell { Live, Dead }

impl Cell {
    fn is_live(&self) -> bool {
        match *self { Live => true, _ => false }
    }

    fn is_dead(&self) -> bool {
        match *self { Dead => true, _ => false }
    }
}

impl Show for Cell {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        match *self {
            Live => write!(f, "O"),
            Dead => write!(f, "D")
        } 
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

    pub fn try_create(width: uint, height: uint, state: Vec<Cell>) -> Result<World, GolError> {
        if width * height != state.len() {
            return Err(InvalidState("State does not match height and length dimensions"));
        }

        Ok(World { width: width, height: height, state: state })
    }

    fn step(&mut self) {
        // Generate the next world state from the previous
        let next_state = Vec::from_fn(self.width * self.height, |index| {
            let row = index / self.height;
            let cell = index % self.height;

            let curr = self.state[index];  
            let curr_neighbours = self.find_neighbours(row, cell);

            match (curr, curr_neighbours) {
                (Live, 3) |
                (Live, 2) |
                (Dead, 3) => Live,
                (Live, _) |
                (Dead, _) => Dead
            }
        });
        self.state = next_state;
    }

    fn find_neighbours(&self, row: uint, cell: uint) -> u8 {
        
        let mut neighbours = 0;

        for &row_offset in [-1, 0, 1].iter() {

            let row_actual = get_actual_index(self.height, row, row_offset); 

            for &cell_offset in [-1, 0, 1].iter() {

                if row_offset == 0 && cell_offset == 0 {
                    continue; //Don't count "current" cell
                }

                let cell_actual = get_actual_index(self.width, cell, cell_offset);

                let neighbour_is_alive = 
                    self.state[row_actual * self.height + cell_actual]
                        .is_live();

                if neighbour_is_alive {
                    neighbours += 1;
                }
            }
        }

        neighbours
    }
}



mod test {

    use super::{ World, Cell, Live, Dead };

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

    fn make_square_board() -> World {
        let state = vec![
            Live, Live, Live,
            Live, Dead,  Live,
            Live, Live, Live,
        ];
        World::try_create(3, 3, state).unwrap()
    }

    fn make_pipe_board() -> World {
        let state = vec![
            Dead, Dead, Dead, Live,
            Dead, Dead, Dead, Live,
            Dead, Dead, Dead, Live,
            Dead, Dead, Dead, Dead,
        ];
        World::try_create(4, 4, state).unwrap()
    }

    fn make_lonely_board() -> World {
        let state = vec![
            Dead, Dead, Dead,
            Dead, Live, Dead,
            Dead, Dead, Dead,
        ];
        World::try_create(3, 3, state).unwrap()
    }

    #[test]
    fn can_count_neighbours_on_square_board() {

        let w = make_square_board();

        let neighbours = w.find_neighbours(1, 1);

        assert_eq!(neighbours, 8);
    }

    #[test]
    fn can_count_neighbours_on_pipe_board() {

        let w = make_pipe_board();

        let neighbours = w.find_neighbours(1, 2);

        assert_eq!(neighbours, 3);
    }

    #[test]
    fn can_count_neighbours_on_edge_of_pipe_board() {

        let w = make_pipe_board();

        let neighbours = w.find_neighbours(1, 0);

        assert_eq!(neighbours, 3);
    }

    #[test]
    fn can_count_neighbours_on_lonely_board() {

        let w = make_lonely_board();

        let neighbours = w.find_neighbours(1, 1);

        assert_eq!(neighbours, 0);
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
    fn can_step_pipe_world() {
        let mut w = make_pipe_board();

        w.step();

        let expected = [
            Dead, Dead, Dead, Dead,
            Live, Dead, Live, Live,
            Dead, Dead, Dead, Dead,
            Dead, Dead, Dead, Dead,
        ];

        assert_eq!(expected.as_slice(), w.state.as_slice());
    }
}