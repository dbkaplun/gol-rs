use grid::{Cell::{Dead as X, Live as O}, Grid};

#[test]
fn test_write_cells() {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let expected = Grid::from_raw(3, 3, vec![
        O, X, X,
        X, X, X,
        X, X, O,
    ]);

    let mut g = Grid::create_dead(3, 3);
    g.write_cells(&expected, (0, 0));
    assert_eq!(g, expected);
}

#[test]
fn test_write_cells_offset() {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let new_data = Grid::from_raw(3, 3, vec![
        O, O, O,
        O, X, O,
        O, O, O,
    ]);

    let mut g = Grid::create_dead(5, 5);
    g.write_cells(&new_data, (2, 2));

    #[cfg_attr(rustfmt, rustfmt_skip)]
    let expected = Grid::from_raw(5, 5, vec![
        X, X, X, X, X,
        X, X, X, X, X,
        X, X, O, O, O,
        X, X, O, X, O,
        X, X, O, O, O,
    ]);
    assert_eq!(&g, &expected);
}

#[test]
#[should_panic]
fn test_write_cells_out_of_range() {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let expected = Grid::from_raw(3, 3, vec![
        O, X, X,
        X, X, X,
        X, X, O,
    ]);

    let mut g = Grid::create_dead(0, 0);
    g.write_cells(&expected, (0, 0));
}
