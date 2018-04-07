use grid::Grid;

#[test]
fn test_eq() {
    let g: Grid = "
        .O.
        O.O
        OOO
    "
        .parse()
        .unwrap();
    let view1 = g.range((0, 1)..(0, 2));
    let view2 = g.range((2, 1)..(2, 2));
    assert!(view1.eq(&view2));
}
