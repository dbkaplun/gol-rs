//! Module containing functions implementing Game of Life rulesets.

use grid::{ Cell };

/// Describes a static ruleset function.
///
/// This function accepts a current cell state and the count of neighbours
/// that cell has and returns a new cell state.
pub type RulesFn = fn(cell: &Cell, neighbours: usize) -> Cell;

/// Implements the [standard rules](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life#Rules)
/// of the Game of Life.
pub fn standard_rules(cell: &Cell, neighbours: usize) -> Cell {
    match (cell, neighbours) {
        (&Cell::Live, 3) |
        (&Cell::Live, 2) |
        (&Cell::Dead, 3) => Cell::Live,
        ________________ => Cell::Dead
    }
}