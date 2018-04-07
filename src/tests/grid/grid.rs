use grid::Cell::{Dead as X, Live as O};
use grid::Grid;
use std::fmt::Write;

use super::*;

#[test]
fn can_create_grid_from_fn() {
    let grid = Grid::from_fn(2, 2, |x, y| match (x, y) {
        (0, 0) => O,
        (1, 0) => X,
        (0, 1) => X,
        (1, 1) => O,
        ____ => unreachable!(),
    });

    assert_eq!(grid.width(), 2);
    assert_eq!(grid.height(), 2);
    assert_eq!(grid.size(), 4);
    assert_eq!(grid.cells(), &[O, X, X, O]);
}

#[test]
fn can_create_dead_grid() {
    use grid::Cell::Dead;

    let grid = Grid::create_dead(10, 10);

    assert_eq!(grid.width(), 10);
    assert_eq!(grid.height(), 10);
    assert_eq!(grid.size(), 100);

    for cell in grid.cells() {
        assert_eq!(&Dead, cell)
    }
}

#[test]
fn can_debug_grid() {
    let mut output = String::new();
    write!(&mut output, "{:?}", make_oblong_grid()).unwrap();
    assert_eq!(output, "!5x3 grid:\n..O..\n.O.O.\n..O..");
}

#[test]
fn can_display_grid() {
    let mut output = String::new();
    write!(&mut output, "{}", make_oblong_grid()).unwrap();
    assert_eq!(output, "..O..\n.O.O.\n..O..");
}

#[test]
#[should_panic(expected = "Invalid height and width")]
fn creating_grid_with_invalid_raw_state_panics() {
    let state = vec![X; 99];

    Grid::from_raw(10, 10, state);
}

#[test]
fn can_grid_index_rangefull() {
    let grid = Grid::create_dead(2, 3);
    assert_eq!(grid[..], [X, X, X, X, X, X]);
}

#[test]
fn can_grid_indexmut_rangefull() {
    let mut grid = Grid::create_dead(3, 3);
    grid[..].copy_from_slice(&[X, O, X, O, X, O, X, O, X]);
    assert_eq!(grid[..][1..=3], [O, X, O]);
}

#[test]
#[should_panic]
fn can_grid_indexmut_rangefull_small_size_panic() {
    let mut grid = Grid::create_dead(1, 2);
    grid[..].copy_from_slice(&[X]);
}

#[test]
#[should_panic]
fn can_grid_indexmut_rangefull_large_size_panic() {
    let mut grid = Grid::create_dead(1, 2);
    grid[..].copy_from_slice(&[O, X, O]);
}

#[test]
fn test_parse_plaintext() {
    let tests = vec![
        (
            "
.O
O.
",
            Ok(Grid::from_raw(
                2,
                2,
                #[cfg_attr(rustfmt, rustfmt_skip)]
                vec![
                    X, O,
                    O, X,
                ],
            )),
        ),
        (
            "
.O
O.
",
            Ok(Grid::from_raw(
                2,
                2,
                #[cfg_attr(rustfmt, rustfmt_skip)]
                vec![
                    X, O,
                    O, X,
                ],
            )),
        ),
        (
            "
                .O.
                ..O
                OOO
            ",
            Ok(Grid::from_raw(
                3,
                3,
                #[cfg_attr(rustfmt, rustfmt_skip)]
                vec![
                    X, O, X,
                    X, X, O,
                    O, O, O,
                ],
            )),
        ),
        (".", Ok(Grid::from_raw(1, 1, vec![X]))),
        (
            "
                ...
                OOO
                ...
            ",
            Ok(Grid::from_raw(
                3,
                3,
                #[cfg_attr(rustfmt, rustfmt_skip)]
                vec![
                    X, X, X,
                    O, O, O,
                    X, X, X,
                ],
            )),
        ),
        (
            "
                ...
                OzO
                ...
            ",
            Err("found character z, expected \'O\' or \'.\'"),
        ),
    ];
    for (input, expected) in tests {
        let actual: Result<Grid, _> = input.parse();
        match expected {
            Ok(expected_grid) => assert_eq!(actual.unwrap(), expected_grid),
            _ => assert_eq!(actual.unwrap_err().description(), expected.unwrap_err()),
        }
    }
}
