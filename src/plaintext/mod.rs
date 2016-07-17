//! Module for parsing the [PlainText](http://conwaylife.com/wiki/PlainText) Game of Life
//! file format into a `Grid`.

mod padding;

use grid::{ Cell, Grid };
use grid::Cell::*;

use self::padding::Padding;

use std::vec::Vec;
use std::result;
use std::io;
use std::fmt;
use std::convert;
use std::iter;

/// Struct for the contents of a PlainText format Game of Life file.
///
/// # Optional padding syntax
///
/// The PlainText parser also provides a `Padding` extension, e.g.:
///
/// ```text
/// !Name: Example
/// !Padding: 5,10,5
/// !
/// .O.
/// O.O
/// .O.
/// ```
///
/// Resulting in the following padding being applied to the result:
///
/// | Top | Right | Bottom | Left |
/// |-----|-------|--------|------|
/// | 5   | 10    | 5      | 10   |
///
pub struct PlainText {
    pub name: String,
    pub comment: String,
    pub data: Grid
}

/// Represents any errors which occur during the PlainText parsing process
#[derive(Debug)]
pub enum ParseError {
    Io(io::Error),
    NameLineMissing,
    Invalid
}

impl fmt::Display for ParseError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::ParseError::*;
        match *self {
            Io(ref e) => write!(fmt, "I/O Error: {}", e),
            NameLineMissing => write!(fmt, "Name line missing"),
            Invalid => write!(fmt, "Body contained invalid data"),
        }
    }
}

impl convert::From<io::Error> for ParseError {
    fn from(err: io::Error) -> ParseError {
        ParseError::Io(err)
    }
}

/// Represents the result of a PlainText parse operation
pub type ParseResult = result::Result<PlainText, ParseError>;

fn sub_string_from(source: &str, from: usize) -> Option<&str> {
    let len = source.len();
    if len == 0 || len <= from {
        return None;
    }
    Some(&source[from..])
}

/// Parses the [PlainText](http://conwaylife.com/wiki/PlainText) format from a buffered stream
pub fn parse_plaintext<R>(reader: R) -> ParseResult
    where R: io::BufRead
{
    #[derive(PartialEq)]
    enum S { Name, Comment, Body }

    let mut state = S::Name;

    let mut name = String::new();
    let mut comment = String::new();
    let mut rows = Vec::new();
    let mut width = 0;
    let mut padding = Padding::new(0, 0, 0, 0);

    for line in reader.lines() {
        let line = try!(line);
        if state == S::Name {
            if !line.starts_with("!Name:") {
                return Err(ParseError::NameLineMissing);
            }
            let line = sub_string_from(&line, 6).unwrap_or("").trim();
            name.push_str(line);
            state = S::Comment;
            continue;
        }
        if state == S::Comment {
            if !line.starts_with("!") {
                state = S::Body;
            }
            else if line.starts_with("!Padding:") {
                //special padding extension
                let line = sub_string_from(&line, 9).unwrap_or("").trim();
                if let Ok(p) = line.parse() {
                    padding = p;
                }
            }
            else {
                if comment.len() != 0 {
                    comment.push_str("\n");
                }
                let line = sub_string_from(&line, 1).unwrap_or("").trim();
                comment.push_str(line);
            }
        }
        if state == S::Body {
            let mut row = Vec::with_capacity(width);
            for c in line.trim().chars() {
                match c {
                    'O' => row.push(Live),
                    '.' => row.push(Dead),
                     _  => return Err(ParseError::Invalid),
                }
            }
            if rows.len() == 0 {
                width = row.len();
            }
            else if width != row.len() {
                return Err(ParseError::Invalid);
            }
            rows.push(row);
        }
    }

    let grid = pad_and_create_grid(rows, width, padding);

    Ok(PlainText {
        name: name,
        comment: comment,
        data: grid
    })
}

fn pad_and_create_grid(rows: Vec<Vec<Cell>>, width: usize, p: Padding) -> Grid {

    let width = width + p.left + p.right;
    let height = rows.len() + p.top + p.bottom;

    let mut cells = Vec::with_capacity(width * height);
    let dead_cells = |c| iter::repeat(Dead).take(c);

    cells.extend(dead_cells(p.top * width));
    for row in rows {
        cells.extend(dead_cells(p.left));
        cells.extend(row);
        cells.extend(dead_cells(p.right));
    }
    cells.extend(dead_cells(p.bottom * width));

    Grid::from_raw(width, height, cells)
}

#[cfg(test)]
mod tests {

    use std::io;
    use grid::Cell::{ Live, Dead };

    #[test]
    fn sub_string_from_tests() {

        use super::sub_string_from;

        assert_eq!(Some("a"), sub_string_from("a", 0));

        assert_eq!(None, sub_string_from("", 0));
        assert_eq!(None, sub_string_from("", 1));

        assert_eq!(Some("abc"), sub_string_from("abc", 0));
        assert_eq!(Some("bc"),  sub_string_from("abc", 1));
        assert_eq!(None,        sub_string_from("abc", 3));
        assert_eq!(None,        sub_string_from("abc", 4));
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

        assert_eq!(value.name, "Tumbler");
        assert_eq!(value.comment, "This is a comment");
        assert_eq!(value.data.width(), 2);
        assert_eq!(value.data.height(), 2);

        let expected = vec![
            Dead, Live,
            Live, Dead
        ];

        for (left, right) in value.data.iter_cells().zip(expected) {
            assert_eq!(left.2, &right)
        }
    }

    #[test]
    fn can_parse_simple_plaintext_with_padding() {

        const PLAINTEXT: &'static str = "!Name: Tumbler
!Padding: 1,2
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

        assert_eq!(value.name, "Tumbler");
        assert_eq!(value.comment, "This is a comment\n");
        assert_eq!(value.data.width(), 4 + 2);
        assert_eq!(value.data.height(), 2 + 2);

        let expected = vec![
            Dead, Dead, Dead, Dead, Dead, Dead,
            Dead, Dead, Dead, Live, Dead, Dead,
            Dead, Dead, Live, Dead, Dead, Dead,
            Dead, Dead, Dead, Dead, Dead, Dead,
        ];

        for (left, right) in value.data.iter_cells().zip(expected) {
            assert_eq!(left.2, &right)
        }
    }

    #[test]
    fn can_exclude_comment() {

        const PLAINTEXT: &'static str = "!Name: Tumbler\n.";

        let bytes = PLAINTEXT.to_string().into_bytes();
        let cursor = io::Cursor::new(bytes);
        let read = io::BufReader::new(cursor);

        let result = super::parse_plaintext(read);

        assert!(result.is_ok(), "Result is not Ok");

        let value = result.unwrap();

        assert_eq!(value.name, "Tumbler");
        assert_eq!(value.comment, "");
    }

    #[test]
    fn parse_fails_when_name_missing() {

        const PLAINTEXT: &'static str = ".";

        let bytes = PLAINTEXT.to_string().into_bytes();
        let cursor = io::Cursor::new(bytes);
        let read = io::BufReader::new(cursor);

        let result = super::parse_plaintext(read);

        assert!(!result.is_ok(), "Result is Ok");
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
