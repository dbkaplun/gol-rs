use grid::Cell::{Dead as X, Live as O};
use grid::Grid;

#[test]
fn test_cells() {
    let g: Grid = "
        .O.
        ..O
        OOO
    "
        .parse()
        .unwrap();

    #[cfg_attr(rustfmt, rustfmt_skip)]
    let expected_range = vec![
        X, O,
        X, X,
    ];
    let view = g.range((0, 0)..(2, 2));
    assert_eq!(view.cells().collect::<Vec<_>>(), expected_range);

    #[cfg_attr(rustfmt, rustfmt_skip)]
    let expected_range = vec![
        O, X,
        X, O,
    ];
    let view = g.range((1, 0)..(3, 2));
    assert_eq!(view.cells().collect::<Vec<_>>(), expected_range);

    #[cfg_attr(rustfmt, rustfmt_skip)]
    let expected_range = vec![
        X, X,
        O, O,
    ];
    let view = g.range((0, 1)..(2, 3));
    assert_eq!(view.cells().collect::<Vec<_>>(), expected_range);

    #[cfg_attr(rustfmt, rustfmt_skip)]
    let expected_range = vec![
        X, O,
        O, O,
    ];
    let view = g.range((1, 1)..(3, 3));
    assert_eq!(view.cells().collect::<Vec<_>>(), expected_range);
}
