use grid::Grid;
use tests::trim_lines;

#[test]
fn test_cells() {
    let input = trim_lines(
        "
        .O.
        ..O
        OOO
    ",
    );
    let g: Grid = input.parse().unwrap();

    assert_eq!(
        format!("{}", g.range((0, 0)..(g.width(), g.height()))),
        input
    );
    assert_eq!(
        format!("{}", g.range((1, 1)..(g.width(), g.height()))),
        trim_lines(
            "
                .O
                OO
            ",
        )
    );
}
