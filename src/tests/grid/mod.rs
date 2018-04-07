pub mod grid;
pub mod shrink;
pub mod write_cells;

use grid::{Cell::{Dead as X, Live as O}, Grid};

pub fn make_square_grid() -> Grid {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let state = vec![
        O, O, O,
        O, X, O,
        O, O, O,
    ];

    Grid::from_raw(3, 3, state)
}

pub fn make_pipe_grid() -> Grid {
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
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let state = vec![
        X, X, X,
        X, O, X,
        X, X, X,
    ];

    Grid::from_raw(3, 3, state)
}

pub fn make_oblong_grid() -> Grid {
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
