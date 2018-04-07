use grid::{Cell::{Dead as X, Live as O}, Grid};

#[test]
fn test_shrink_nop() {
    let expected = Grid::from_raw(2, 1, vec![O, X]);
    let mut input = expected.clone();
    input.shrink((expected.width(), expected.height()), (0, 0));
    assert_eq!(input, expected);
}

#[test]
fn test_shrink_no_offset() {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let mut input = Grid::from_raw(3, 3, vec![
        O, X, X,
        X, X, X,
        X, X, O,
    ]);
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let expected = Grid::from_raw(2, 2, vec![
        O, X,
        X, X,
    ]);

    input.shrink((expected.width(), expected.height()), (0, 0));
    assert_eq!(input, expected);
}

#[test]
fn test_shrink_offset() {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let mut input = Grid::from_raw(3, 3, vec![
        O, X, X,
        O, X, X,
        X, X, O,
    ]);
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let expected = Grid::from_raw(2, 2, vec![
        O, X,
        X, X,
    ]);

    input.shrink((expected.width(), expected.height()), (0, 1));
    assert_eq!(input, expected);
}

#[test]
#[should_panic]
fn test_shrink_larger_width_no_offset() {
    let (w, h) = (2, 3);
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let mut input = Grid::from_raw(w, h, vec![
        O, X,
        X, X,
        O, X,
    ]);
    input.shrink((w + 1, h), (0, 0));
}

#[test]
#[should_panic]
fn test_shrink_larger_width_offset() {
    let (w, h) = (2, 3);
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let mut input = Grid::from_raw(w, h, vec![
        O, X,
        X, X,
        O, X,
    ]);
    input.shrink((w, h), (1, 0));
}

#[test]
#[should_panic]
fn test_shrink_larger_height_no_offset() {
    let (w, h) = (4, 3);
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let mut input = Grid::from_raw(w, h, vec![
        O, O, X, O,
        O, X, X, X,
        X, X, O, O,
    ]);
    input.shrink((w, h + 1), (0, 0));
}

#[test]
#[should_panic]
fn test_shrink_larger_height_offset() {
    let (w, h) = (4, 3);
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let mut input = Grid::from_raw(w, h, vec![
        O, O, X, O,
        O, X, X, X,
        X, X, O, O,
    ]);
    input.shrink((w, h), (0, 1));
}
