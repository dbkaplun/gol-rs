//! Module for parsing the [`PlainText`](http://conwaylife.com/wiki/PlainText) Game of Life
//! file format into a `Grid`.

use grid::Grid;
use std::error::Error;
use std::io::{self, BufRead};
use std::str::FromStr;

/// Struct for the contents of a `PlainText` format Game of Life file.
///
/// ```
/// use gol::plaintext::PlainText;
/// use gol::grid::Cell::{Dead as X, Live as O};
/// let data: PlainText = "
///     !Name: Example
///     .O.
///     ..O
///     OOO
/// ".parse().unwrap();
///
/// assert_eq!(data.header, "Name: Example\n");
/// assert_eq!(data.grid.cells(), &[
///     X, O, X,
///     X, X, O,
///     O, O, O,
/// ]);
/// ```
#[derive(Debug)]
pub struct PlainText {
    pub header: String,
    pub grid: Grid,
}

impl FromStr for PlainText {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut header = String::new();
        let mut grid_str = String::new();

        let mut cursor = io::Cursor::new(s);
        loop {
            let mut line = String::new();
            cursor.read_line(&mut line)?;
            if line.is_empty() {
                break;
            }

            let line = line.trim_left();
            if line.starts_with('!') {
                if !grid_str.is_empty() {
                    return Err("Header content must come before grid content".into());
                }
                header.push_str(&line[1..]);
            } else {
                grid_str.push_str(line);
            }
        }

        Ok(Self {
            header,
            grid: grid_str.parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::PlainText;
    use grid::Cell::{Dead as X, Live as O};

    #[test]
    fn test_parse_plaintext() {
        let tests = vec![
            (
                "!Name: Tumbler
!
! This is a header
.O
O.
",
                Ok((
                    "Name: Tumbler\n\n This is a header\n",
                    2,
                    2,
                    #[cfg_attr(rustfmt, rustfmt_skip)]
                    vec![
                        X, O,
                        O, X,
                    ],
                )),
            ),
            (
                "!Name: Tumbler
!
! This is a header
!
.O
O.
",
                Ok((
                    "Name: Tumbler\n\n This is a header\n\n",
                    2,
                    2,
                    #[cfg_attr(rustfmt, rustfmt_skip)]
                    vec![
                        X, O,
                        O, X,
                    ],
                )),
            ),
            (
                "
                    !Name: Glider
                    !
                    ! This is a header
                    .O.
                    ..O
                    OOO
                ",
                Ok((
                    "Name: Glider\n\n This is a header\n",
                    3,
                    3,
                    #[cfg_attr(rustfmt, rustfmt_skip)]
                    vec![
                        X, O, X,
                        X, X, O,
                        O, O, O,
                    ],
                )),
            ),
            ("!Name: Tumbler\n.", Ok(("Name: Tumbler\n", 1, 1, vec![X]))),
            (
                "
                    ...
                    OOO
                    ...
                ",
                Ok((
                    "",
                    3,
                    3,
                    #[cfg_attr(rustfmt, rustfmt_skip)]
                    vec![
                        X, X, X,
                        O, O, O,
                        X, X, X,
                    ],
                )),
            ),
            (
                "
                    ...
                    O.O
                    !Header in the middle of grid should error
                    ...
                ",
                Err("Header content must come before grid content"),
            ),
            (
                "
                    ...
                    OzO
                    ...
                ",
                Err("found character z, expected \'O\' or \'.\'"),
            ),
        ];
        for (input, expected) in tests {
            let actual: Result<PlainText, _> = input.parse();
            match expected {
                Ok((expected_header, expected_width, expected_height, expected_cells)) => {
                    let actual = actual.unwrap();
                    assert_eq!(actual.header, expected_header);
                    assert_eq!(actual.grid.width(), expected_width);
                    assert_eq!(actual.grid.height(), expected_height);
                    assert_eq!(actual.grid.cells(), &expected_cells[..]);
                }
                _ => assert_eq!(actual.unwrap_err().description(), expected.unwrap_err()),
            }
        }
    }
}
