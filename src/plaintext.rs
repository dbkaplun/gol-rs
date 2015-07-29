use grid::{ Cell, Grid };
use grid::Cell::*;

use std::vec::Vec;
use std::result;
use std::io;
use std::fmt;
use std::convert;
use std::str::FromStr;
use std::iter;

/// Struct for the contents of a [Plaintext](http://conwaylife.com/wiki/Plaintext) format Game of Life file.
///
/// # Optional padding syntax
///
/// The plaintext parser also provides a `Padding` extension, e.g.:
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

/// Describes padding in the order `top`, `right`, `bottom`, `left`
#[derive(PartialEq, Debug)]
pub struct Padding {
    pub top: usize,
    pub right: usize,
    pub bottom: usize,
    pub left: usize,
}
    
macro_rules! unwrap_or {
    ( $v:expr, $otherwise:expr ) => {
        match $v {
            Some(Err(..)) => return Err(()),
            Some(Ok(v)) => v,
            None => $otherwise,
        }
    }
}

impl Padding {
    /// Constructs a new instance of the Padding struct
    pub fn new(top: usize, right: usize, bottom: usize, left: usize) -> Padding {
        Padding { top: top, right: right, bottom: bottom, left: left }
    }
}

impl FromStr for Padding {
    type Err = ();
    
    /// Parses a css-style `top[,right[,bottom[,left]]]` expression
    /// into a Padding struct
    fn from_str(s: &str) -> Result<Padding, ()> {
        let mut parts = s.split(',').map(|p| usize::from_str(p.trim()));
        let top    = unwrap_or!(parts.next(), return Err(()));
        let right  = unwrap_or!(parts.next(), top);
        let bottom = unwrap_or!(parts.next(), top);
        let left   = unwrap_or!(parts.next(), right);
        //Assert no more parts
        if parts.next().is_some() {
            return Err(());
        }
        Ok(Padding::new(top, right, bottom, left))
    }
}

#[cfg(test)]
mod padding_tests {
    use super::Padding;
    use std::str::FromStr;

    #[test]
    fn can_parse_single_value() {
        let expected = Ok(Padding::new(10, 10, 10, 10));
        let actual = Padding::from_str("10");
        assert_eq!(expected, actual)
    }

    #[test]
    fn can_parse_two_values() {
        let expected = Ok(Padding::new(10, 20, 10, 20));
        let actual = Padding::from_str("10,20");
        assert_eq!(expected, actual)
    }

    #[test]
    fn can_parse_three_values() {
        let expected = Ok(Padding::new(10, 20, 30, 20));
        let actual = Padding::from_str("10,20,30");
        assert_eq!(expected, actual)
    }

    #[test]
    fn can_parse_four_values() {
        let expected = Ok(Padding::new(10, 20, 30, 40));
        let actual = Padding::from_str("10,20,30,40");
        assert_eq!(expected, actual)
    }

    #[test]
    fn can_ignore_whitespace() {
        let expected = Ok(Padding::new(10, 20, 30, 40));
        let actual = Padding::from_str(" 10 , 20 , 30 , 40 ");
        assert_eq!(expected, actual)
    }

    #[test]
    fn fails_with_more_than_five_values() {
        let expected = Err(());
        let actual = Padding::from_str("10,20,30,40,60");
        assert_eq!(expected, actual)
    }

    #[test]
    fn fails_with_no_values() {
        let expected = Err(());
        let actual = Padding::from_str("");
        assert_eq!(expected, actual)
    }

    #[test]
    fn fails_with_invalid_value() {
        let expected = Err(());
        let actual = Padding::from_str("1,this isn't an int");
        assert_eq!(expected, actual)
    }

}

/// Represents any errors which occur during the Plaintext parsing process
#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    NameLineMissing,
    Invalid
}

/// Represents the result of a Plaintext parse operation
pub type ParseResult = result::Result<PlainText, Error>;

impl convert::From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;
        match *self {
            Io(ref e) => write!(fmt, "I/O Error: {}", e),
            NameLineMissing => write!(fmt, "Name line missing"),
            Invalid => write!(fmt, "Body contained invalid data"),
        }
    }
}

fn sub_string_from(source: &str, from: usize) -> Option<&str> {
    source.char_indices().nth(from).map(|(char_idx, _)| &source[char_idx..])
}

/// Parses the [Plaintext](http://conwaylife.com/wiki/Plaintext) format from a buffered stream
pub fn parse_plaintext<R>(reader: R) -> Result<PlainText, Error>
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
                return Err(Error::NameLineMissing);
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
                if let Ok(p) = Padding::from_str(line) {
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
            let mut row = Vec::new();
            for c in line.trim().chars() {
                match c {
                    'O' => row.push(Live),
                    '.' => row.push(Dead),
                     _  => return Err(Error::Invalid),
                }
            }
            if rows.len() == 0 {
                width = row.len();
            }
            else if width != row.len() {
                return Err(Error::Invalid);
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
    
    cells.extend(iter::repeat(Dead).take(p.top * width));
    for row in &rows {
        cells.extend(iter::repeat(Dead).take(p.left));
        cells.extend(row.iter().map(|c| c.clone()));
        cells.extend(iter::repeat(Dead).take(p.right));
    }
    cells.extend(iter::repeat(Dead).take(p.bottom * width));
    
    Grid::from_raw(width, height, cells)
}

#[cfg(test)]
mod plaintext_tests {

    use std::io;
    use grid::Cell::{ Live, Dead };

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
