use grid::{Cell, Coord, Grid};
use std::fmt;
use std::ops::Range;

pub struct GridView<'g> {
    pub(crate) grid: &'g Grid,
    pub(crate) range: Range<Coord>,
}

impl<'a> GridView<'a> {
    pub fn cells(&self) -> impl Iterator<Item = Cell> + 'a {
        let (ox, oy) = self.range.start;
        let (w, h) = (self.range.end.0 - ox, self.range.end.1 - oy);
        let grid_width = self.grid.width();
        self.grid
            .cells()
            .iter()
            .skip(grid_width * oy)
            .take(grid_width * h)
            .enumerate()
            .filter_map(move |(view_i, &c)| {
                let x = view_i % grid_width;
                if ox <= x && x < w + ox {
                    // `oy <= y && y < h + oy` not necessary due to .skip and .take
                    Some(c)
                } else {
                    None
                }
            })
    }
}

impl<'a> fmt::Debug for GridView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "!GridView: {:?}\n{}", self.range, self)
    }
}

impl<'a> fmt::Display for GridView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let w = self.range.end.0 - self.range.start.0;
        for (i, cell) in self.cells().enumerate() {
            if i % w == 0 && i != 0 {
                writeln!(f)?;
            }
            write!(f, "{}", cell)?;
        }
        Ok(())
    }
}

impl<'a> PartialEq<GridView<'a>> for GridView<'a> {
    fn eq(&self, other: &GridView) -> bool {
        self.cells().eq(other.cells())
    }
}
