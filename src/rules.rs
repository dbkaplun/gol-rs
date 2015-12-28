//! Module containing functions implementing Game of Life rulesets.

use grid::{ Grid, Cell };

/// Describes a static ruleset function.
///
/// This function accepts a current cell state and the count of neighbours
/// that cell has and returns a new cell state.
pub type RulesFn = fn(cell: &Cell, neighbours: usize) -> Cell;

/// Implements the [standard rules](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life#Rules)
/// of the Game of Life.
pub fn standard_rules(cell: &Cell, neighbours: usize) -> Cell {
    match (cell, neighbours) {
        (&Cell::Live, 3) |
        (&Cell::Live, 2) |
        (&Cell::Dead, 3) => Cell::Live,
        ________________ => Cell::Dead
    }
}


/// Describes a static neighbour counting function.
///
/// This function accepts a Grid and a set of coordinates and
/// and returns the neighbour count of those coordinates.
pub type NeighboursFn = fn(grid: &Grid, x: usize, y: usize) -> usize;

/// Implements neighbour counting for a torus world.
pub fn torus_neighbours(grid: &Grid, x: usize, y: usize) -> usize {
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
mod tests {

    #[test]
    fn can_calculate_index() {
        use super::{ Delta };

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

    /*


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
    }*/
}