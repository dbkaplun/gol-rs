use plaintext::PlainText;
use tests::trim_lines;

#[test]
fn test_cells() {
    let input = trim_lines(
        "
        !GridView: (0, 0)..(3, 3)
        .O.
        ..O
        OOO
    ",
    );
    let p: PlainText = input.parse().unwrap();
    let g = p.grid;

    assert_eq!(
        format!("{:?}", g.range((0, 0)..(g.width(), g.height()))),
        input
    );
    assert_eq!(
        format!("{:?}", g.range((1, 0)..(g.width(), g.height()))),
        trim_lines(
            "
                !GridView: (1, 0)..(3, 3)
                O.
                .O
                OO
            ",
        )
    );
}
