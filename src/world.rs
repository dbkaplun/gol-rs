//! Structures and functions used for constructing a Game of Live `World` which combines a `Grid` and a rule set.
//!
//! Implements the functionality required for a simple Game of Life simulation.

use std::iter::Iterator;

use grid::{ Grid };
use grid;
use rules::{ RulesFn };
use rules;

/// Provides hosting for a basic Game of Life simulation. Includes functions for modifying
/// the world and stepping the simulation both immutably and in-place.
pub struct World {
    gen: i64,
    rules: RulesFn,
    curr: Grid,
    prev: Option<Grid>,
}

impl World {

    /// Constructs a new `World` with the given `Grid`
    pub fn new(grid: Grid) -> World {
        World { gen: 0, rules: rules::standard_rules, curr: grid, prev: None  }
    }

    /// Constructs a new `World` with the given `Grid` and `RulesFn`
    pub fn new_with_rules(grid: Grid, rules: RulesFn) -> World {
        World { gen: 0, rules: rules, curr: grid, prev: None }
    }

    /// Gets the current generation for this `World`
    #[inline]
    pub fn generation(&self) -> i64 {
        self.gen
    }

    /// Gets the width of this `World`
    #[inline]
    pub fn width(&self) -> usize {
        self.curr.width()
    }

    /// Gets the height of this `World`
    #[inline]
    pub fn height(&self) -> usize {
        self.curr.height()
    }

    /// Executes a single step of this `World` in place
    pub fn step_mut(&mut self) {
        use std::mem::swap;
        // Allocate prev?
        if self.prev.is_none() {
            let (w, h) = (self.width(), self.height());
            self.prev = Some(Grid::create_dead(w, h));
        }
        let curr = &mut self.curr;
        let next = self.prev.as_mut().unwrap();
        // Generate the next world state from the current
        for (x, y, cell) in curr.iter_cells() {
            let neighbours = curr.count_neighbours(x, y);
            let new_cell = (self.rules)(&cell, neighbours);
            next.set_cell(x, y, new_cell);
        }
        // ...and swap the two values
        swap(curr, next);
        self.gen += 1;
    }

    /// Executes a single step of this `World` and returns a new, modified world
    pub fn step(&self) -> World {
        // Generate the next world state from the current
        let next =
            self.curr
                .iter_cells()
                .map(|(x, y, cell)| {
                    let neighbours = self.curr.count_neighbours(x, y);
                    (self.rules)(cell, neighbours)
                })
                .collect();

        let next = Grid::from_raw(self.width(), self.height(), next);
        World { gen: self.gen + 1, rules: self.rules,
                curr: next,
                prev: None }
    }

    /// Overwrite the cells starting at coords `(x, y)` with the data in the given `Grid`
    pub fn write_cells(&mut self, x: usize, y: usize, data: &Grid) {
        self.curr.write_cells(x, y, data);
    }

    /// Returns an iterator over rows in this world
    pub fn iter_rows(&self) -> grid::RowIter {
        self.curr.iter_rows()
    }
}

#[cfg(test)]
mod tests {

    use rules;
    use world::World;
    use grid::{ Grid };
    use grid::Cell::{ Live, Dead };
    use grid::tests as grid_test;

    fn make_square_world() -> World {
        World::new(grid_test::make_square_grid())
    }

    fn make_pipe_world() -> World {
        World::new(grid_test::make_pipe_grid())
    }

    fn make_lonely_world() -> World {
        World::new(grid_test::make_lonely_grid())
    }

    fn make_oblong_world() -> World {
        World::new(grid_test::make_oblong_grid())
    }

    fn make_glider_world() -> World {
        World::new(grid_test::make_glider_grid())
    }


    #[test]
    fn can_create_world_with_grid() {

        let grid = Grid::from_fn(10, 10, |_, _| Dead);
        let w = World::new(grid.clone());
        assert_eq!(0, w.gen);
        assert_eq!(&grid, &w.curr);
    }

    #[test]
    fn can_step_pipe_world_mutably() {
        use grid::Cell::Dead as X;
        use grid::Cell::Live as O;

        let mut w = make_pipe_world();

        w.step_mut();

        let expected = Grid::from_raw(4, 4, vec![
            X, X, X, X,
            O, X, O, O,
            X, X, X, X,
            X, X, X, X,
        ]);

        assert_eq!(&w.curr, &expected);
    }

    #[test]
    fn can_step_pipe_world_immutably() {
        use grid::Cell::Dead as X;
        use grid::Cell::Live as O;

        let w = make_pipe_world();

        let w2 = w.step();

        let expected = Grid::from_raw(4, 4, vec![
            X, X, X, X,
            O, X, O, O,
            X, X, X, X,
            X, X, X, X,
        ]);

        assert_eq!(&w2.curr, &expected);
    }


    #[test]
    fn can_increment_generation() {
        //initial generation
        let w = make_square_world();
        assert_eq!(w.generation(), 0);

        //immutable step
        let w_two = w.step();
        assert_eq!(w_two.generation(), 1);

        //mutable step
        let mut w_mut = w;
        w_mut.step_mut();
        assert_eq!(w_mut.generation(), 1);
    }

    #[test]
    fn can_step_glider_world() {
        use grid::Cell::Dead as X;
        use grid::Cell::Live as O;

        //Step the glider twice, and assert we got the correct output
        let mut w = make_glider_world();

        w.step_mut();
        w.step_mut();

        let expected = Grid::from_raw(6, 5, vec![
            X, X, X, X, X, X,
            X, X, X, O, X, X,
            X, X, X, X, O, X,
            X, X, O, O, O, X,
            X, X, X, X, X, X,
        ]);

        assert_eq!(&w.curr, &expected);
    }

    #[test]
    fn can_step_lonely_world() {
        use grid::Cell::Dead as X;

        //Step the lonely twice, and assert that every
        //cell died and that none came back to life
        let mut w = make_lonely_world();

        w.step_mut();
        w.step_mut();

        let expected = Grid::from_raw(3, 3, vec![
            X, X, X,
            X, X, X,
            X, X, X,
        ]);

        assert_eq!(&w.curr, &expected);
    }

    #[test]
    fn can_iterate_rows_in_oblong_world_correctly() {
        use grid::Cell::Dead as X;
        use grid::Cell::Live as O;

        let w = make_oblong_world();

        let mut iter = w.iter_rows();

        assert_eq!(iter.next().unwrap(), &[X, X, O, X, X]);
        assert_eq!(iter.next().unwrap(), &[X, O, X, O, X]);
        assert_eq!(iter.next().unwrap(), &[X, X, O, X, X]);
        assert!(iter.next().is_none());
    }

    #[test]
    fn can_get_and_set_cell_in_grid() {

        let mut grid = Grid::create_dead(10, 10);

        assert_eq!(&Dead, grid.cell_at(2, 2));

        grid.set_cell(2, 2, Live);

        assert_eq!(&Live, grid.cell_at(2, 2));
    }

    #[test]
    #[should_panic(expected="out of range")]
    fn get_cell_out_of_range_panics() {

        let grid = Grid::create_dead(10, 10);

        grid.cell_at(10, 10);
    }

    #[test]
    #[should_panic(expected="out of range")]
    fn set_cell_out_of_range_panics() {

        let mut grid = Grid::create_dead(10, 10);

        grid.set_cell(10, 10, Live);
    }

    #[test]
    fn can_set_region_in_world() {

        use grid::Cell::Dead as X;
        use grid::Cell::Live as O;

        let new_data = Grid::from_raw(3, 3, vec![
            O, O, O,
            O, X, O,
            O, O, O,
        ]);

        //write from top left
        let mut w = make_lonely_world();
        w.write_cells(0, 0, &new_data);

        //NOTE: Overwrites entire world
        assert_eq!(&w.curr, &new_data);

        //write from bottom right
        let mut w = make_lonely_world();
        w.write_cells(2, 2, &new_data);

        //NOTE: Overwrites bottom corner
        let expected = &Grid::from_raw(3, 3, vec![
            X, X, X,
            X, O, X,
            X, X, O,
        ]);
        assert_eq!(&w.curr, expected);

    }

    // Benchmarks

    use test::Bencher;

    fn make_even_grid(w: usize, h: usize) -> Grid {
        Grid::from_fn(w, h, |x, y| if (x + y) % 2 == 0 { Live } else { Dead })
    }

    #[bench]
    fn bench_standard_rules(b: &mut Bencher) {

        let grid = make_even_grid(500, 500);
        let mut world = World::new_with_rules(grid, rules::standard_rules);

        b.iter(|| world.step_mut());
    }
}
