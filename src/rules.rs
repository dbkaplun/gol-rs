use super::{ Cell };

/// Standard Game of Life rules
/// Calculates the next state of the given cell
pub fn standard_rules(cell: &Cell, neighbours: usize) -> Cell {
    match (cell, neighbours) {
        (&Cell::Live, 3) |
        (&Cell::Live, 2) |
        (&Cell::Dead, 3) => Cell::Live,
        ________________ => Cell::Dead
    }
}