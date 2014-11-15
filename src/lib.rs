#![allow(dead_code, unused_imports, unused_variables)]

use std::vec::Vec;
use std::option::Option;

#[deriving(PartialEq)]
enum Cell { Live, Dead }

impl Cell {
    fn is_live(&self) -> bool {
        match *self { Live => true, _ => false }
    }

    fn is_dead(&self) -> bool {
        match *self { Dead => true, _ => false }
    }
}

struct World {
    width: uint,
    height: uint,
    state: Vec<Cell>
}

#[deriving(PartialEq)]
enum CellOffset {
    MinusOne,
    NoOffset,
    PlusOne
}

fn get_actual_index(max: uint, current_index: uint, offset: &CellOffset) -> uint {
    match *offset {
        MinusOne => if current_index == 0 { max - 1 } else { current_index - 1 },
        NoOffset => current_index,
        PlusOne  => if current_index >= (max - 1) { 0 } else { current_index + 1 }
    }
}

impl World {

    fn try_create(width: uint, height: uint, state: Vec<Cell>) -> Option<World> {
        if width * height != state.len() {
            None
        }
        else {
            Some(World { width: width, height: height, state: state })
        }
    }

    fn find_neighbours(&self, row: uint, cell: uint) -> u8 {
        
        let mut neighbours = 0;

        for row_offset in vec![MinusOne, NoOffset, PlusOne].iter() {

            let row_actual = get_actual_index(self.height, row, row_offset); 

            for cell_offset in vec![MinusOne, NoOffset, PlusOne].iter() {

                if row_offset == &NoOffset && cell_offset == &NoOffset {
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
    fn math_checks_out() {
        assert_eq!(25i, 5i * 5i);
    }

    #[test]
    fn can_create_world() {
        
        let state = Vec::from_fn(100, |_| Dead);

        let w = World::try_create(10, 10, state);

        assert!(w.is_some());
        //TODO assert state value was passed in
    }

    #[test]
    fn can_fail_to_create_world() {
        
        let state = Vec::from_fn(99, |_| Dead);

        let w = World::try_create(10, 10, state);

        assert!(w.is_none());
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
            Dead, Dead, Live,
            Dead, Dead, Live,
            Dead, Dead, Live,
        ];
        World::try_create(3, 3, state).unwrap()
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

        let neighbours = w.find_neighbours(1, 1);

        assert_eq!(neighbours, 3);
    }

    #[test]
    fn can_count_neighbours_on_lonely_board() {

        let w = make_lonely_board();

        let neighbours = w.find_neighbours(1, 1);

        assert_eq!(neighbours, 0);
    }
}