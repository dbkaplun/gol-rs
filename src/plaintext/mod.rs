//! Module for parsing the [`PlainText`](http://conwaylife.com/wiki/PlainText) Game of Life
//! file format into a `Grid`.

use grid::{Cell, Grid};

use std::convert;
use std::fmt;
use std::io;
use std::result;
use std::vec::Vec;

/// Struct for the contents of a `PlainText` format Game of Life file.
///
/// ```text
/// !Name: Example
/// !
/// .O.
/// O.O
/// .O.
/// ```
///
pub struct PlainText {
    pub comment: String,
    pub grid: Grid,
}

/// Represents any errors which occur during the `PlainText` parsing process
#[derive(Debug)]
pub enum ParseError {
    Io(io::Error),
    Invalid,
}

impl fmt::Display for ParseError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::ParseError::*;
        match *self {
            Io(ref e) => write!(fmt, "I/O Error: {}", e),
            Invalid => write!(fmt, "Body contained invalid data"),
        }
    }
}

impl convert::From<io::Error> for ParseError {
    fn from(err: io::Error) -> ParseError {
        ParseError::Io(err)
    }
}

/// Represents the result of a `PlainText` parse operation
pub type ParseResult = result::Result<PlainText, ParseError>;

fn sub_string_from(source: &str, from: usize) -> Option<&str> {
    let len = source.len();
    if len == 0 || len <= from {
        return None;
    }
    Some(&source[from..])
}

/// Parses the [`PlainText`](http://conwaylife.com/w/index.php?title=Plaintext) format from a buffered stream
pub fn parse_plaintext<R>(reader: R) -> ParseResult
where
    R: io::BufRead,
{
    let mut comment = String::new();
    let mut rows = Vec::new();
    let mut width = 0;

    for line in reader.lines() {
        let line = try!(line);
        let line = line.trim_left();
        if line.is_empty() {
            continue;
        }

        if line.starts_with('!') {
            if !comment.is_empty() {
                comment.push_str("\n");
            }
            let line = sub_string_from(line, 1).unwrap_or("").trim();
            comment.push_str(line);
        } else {
            let mut row = Vec::with_capacity(width);
            for c in line.trim().chars() {
                match c {
                    'O' => row.push(Cell::Live),
                    '.' => row.push(Cell::Dead),
                    _ => return Err(ParseError::Invalid),
                }
            }
            if rows.is_empty() {
                width = row.len();
            } else if width != row.len() {
                return Err(ParseError::Invalid);
            }
            rows.push(row);
        }
    }

    let height = rows.len();
    let cells = rows.into_iter().flat_map(|row| row).collect();
    let grid = Grid::from_raw(width, height, cells);

    Ok(PlainText { comment, grid })
}

#[cfg(test)]
mod tests {
    use grid::Cell::{Dead as X, Live as O};
    use std::io;

    #[test]
    fn sub_string_from_tests() {
        use super::sub_string_from;

        assert_eq!(Some("a"), sub_string_from("a", 0));

        assert_eq!(None, sub_string_from("", 0));
        assert_eq!(None, sub_string_from("", 1));

        assert_eq!(Some("abc"), sub_string_from("abc", 0));
        assert_eq!(Some("bc"), sub_string_from("abc", 1));
        assert_eq!(None, sub_string_from("abc", 3));
        assert_eq!(None, sub_string_from("abc", 4));
    }

    #[test]
    fn can_parse_simple_plaintext() {
        const PLAINTEXT: &'static str = "!Name: Tumbler
!
! This is a comment
.O
O.
";

        let bytes = PLAINTEXT.to_string().into_bytes();
        let cursor = io::Cursor::new(bytes);
        let read = io::BufReader::new(cursor);

        let result = super::parse_plaintext(read);

        assert!(result.is_ok(), "Result is not Ok");

        let value = result.unwrap();

        assert_eq!(value.comment, "Name: Tumbler\n\nThis is a comment");
        assert_eq!(value.grid.width(), 2);
        assert_eq!(value.grid.height(), 2);

        #[cfg_attr(rustfmt, rustfmt_skip)]
        let expected = vec![
            X, O,
            O, X,
        ];

        for (left, right) in value.grid.iter_cells().zip(expected) {
            assert_eq!(left.2, &right)
        }
    }

    #[test]
    fn can_parse_simple_plaintext2() {
        const PLAINTEXT: &'static str = "!Name: Tumbler
!
! This is a comment
!
.O
O.
";

        let bytes = PLAINTEXT.to_string().into_bytes();
        let cursor = io::Cursor::new(bytes);
        let read = io::BufReader::new(cursor);

        let result = super::parse_plaintext(read);

        assert!(result.is_ok(), "Result is not Ok");

        let value = result.unwrap();

        assert_eq!(value.comment, "Name: Tumbler\n\nThis is a comment\n");
        assert_eq!(value.grid.width(), 2);
        assert_eq!(value.grid.height(), 2);

        #[cfg_attr(rustfmt, rustfmt_skip)]
        let expected = vec![
            X, O,
            O, X,
        ];

        for (left, right) in value.grid.iter_cells().zip(expected) {
            assert_eq!(left.2, &right)
        }
    }

    #[test]
    fn can_parse_simple_plaintext_with_leading_whitespace() {
        const PLAINTEXT: &'static str = "
            !Name: Glider
            !
            ! This is a comment
            .O.
            ..O
            OOO
        ";

        let bytes = PLAINTEXT.to_string().into_bytes();
        let cursor = io::Cursor::new(bytes);
        let read = io::BufReader::new(cursor);

        let result = super::parse_plaintext(read);

        assert!(result.is_ok(), "Result is not Ok");

        let value = result.unwrap();

        assert_eq!(value.comment, "Name: Glider\n\nThis is a comment");
        assert_eq!(value.grid.width(), 3);
        assert_eq!(value.grid.height(), 3);

        #[cfg_attr(rustfmt, rustfmt_skip)]
        let expected = vec![
            X, O, X,
            X, X, O,
            O, O, O,
        ];

        for (left, right) in value.grid.iter_cells().zip(expected) {
            assert_eq!(left.2, &right)
        }
    }

    #[test]
    fn can_parse_single_cell() {
        const PLAINTEXT: &'static str = "!Name: Tumbler\n.";

        let bytes = PLAINTEXT.to_string().into_bytes();
        let cursor = io::Cursor::new(bytes);
        let read = io::BufReader::new(cursor);

        let result = super::parse_plaintext(read);

        assert!(result.is_ok(), "Result is not Ok");

        let value = result.unwrap();

        assert_eq!(value.comment, "Name: Tumbler");
        assert_eq!(value.grid.width(), 1);
        assert_eq!(value.grid.height(), 1);
        assert_eq!(
            value
                .grid
                .iter_cells()
                .map(|(_x, _y, &cell)| cell)
                .collect::<Vec<_>>(),
            vec![X]
        );
    }

    #[test]
    fn parse_fails_when_invalid_chars_in_body() {
        const PLAINTEXT: &'static str = "...\nOzO\n...";

        let bytes = PLAINTEXT.to_string().into_bytes();
        let cursor = io::Cursor::new(bytes);
        let read = io::BufReader::new(cursor);

        let result = super::parse_plaintext(read);

        assert!(!result.is_ok(), "Result is Ok");
    }
}
