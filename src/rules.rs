use grid::{ Cell };

/// Implements the standard rules of the Game of Life
pub fn standard_rules(cell: &Cell, neighbours: usize) -> Cell {
    match (cell, neighbours) {
        (&Cell::Live, 3) |
        (&Cell::Live, 2) |
        (&Cell::Dead, 3) => Cell::Live,
        ________________ => Cell::Dead
    }
}