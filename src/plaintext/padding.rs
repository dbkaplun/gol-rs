//! Module for parsing a Padding expression

use std::error;
use std::fmt;
use std::num;
use std::str::FromStr;

/// Describes padding in the order `top`, `right`, `bottom`, `left`
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Padding {
    pub top: usize,
    pub right: usize,
    pub bottom: usize,
    pub left: usize,
}

impl Padding {
    /// Constructs a new instance of the Padding struct
    pub fn new(top: usize, right: usize, bottom: usize, left: usize) -> Padding {
        Padding {
            top,
            right,
            bottom,
            left,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    TooManyParts,
    InvalidPart(num::ParseIntError),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ParseError::*;
        match *self {
            TooManyParts => write!(f, "Too few parts"),
            InvalidPart(_) => write!(f, "Part was not a valid padding"),
        }
    }
}

impl error::Error for ParseError {
    fn description(&self) -> &str {
        "Unable to parse Padding expression"
    }
    fn cause(&self) -> Option<&error::Error> {
        use self::ParseError::*;
        match *self {
            InvalidPart(ref err) => Some(err),
            _ => None,
        }
    }
}

pub type ParseResult = Result<Padding, ParseError>;

macro_rules! unwrap_or {
    ($v:expr, $otherwise:expr) => {
        match $v {
            Some(Err(e)) => return Err(ParseError::InvalidPart(e)),
            Some(Ok(v)) => v,
            None => $otherwise,
        }
    };
}

impl FromStr for Padding {
    type Err = ParseError;

    /// Parses a css-style `top[,right[,bottom[,left]]]` expression
    /// into a Padding struct
    fn from_str(s: &str) -> ParseResult {
        let mut parts = s.split(',').map(|p| p.trim().parse());
        let top = unwrap_or!(parts.next(), unreachable!());
        let right = unwrap_or!(parts.next(), top);
        let bottom = unwrap_or!(parts.next(), top);
        let left = unwrap_or!(parts.next(), right);
        //Assert no more parts
        if parts.next().is_some() {
            return Err(ParseError::TooManyParts);
        }
        Ok(Padding::new(top, right, bottom, left))
    }
}

#[cfg(test)]
mod tests {
    use super::{Padding, ParseError, ParseResult};
    use std::error::Error;

    #[test]
    fn can_parse_single_value() {
        let expected = Ok(Padding::new(10, 10, 10, 10));
        let actual = "10".parse();
        assert_eq!(expected, actual);
    }

    #[test]
    fn can_parse_two_values() {
        let expected = Ok(Padding::new(10, 20, 10, 20));
        let actual = "10,20".parse();
        assert_eq!(expected, actual);
    }

    #[test]
    fn can_parse_three_values() {
        let expected = Ok(Padding::new(10, 20, 30, 20));
        let actual = "10,20,30".parse();
        assert_eq!(expected, actual);
    }

    #[test]
    fn can_parse_four_values() {
        let expected = Ok(Padding::new(10, 20, 30, 40));
        let actual = "10,20,30,40".parse();
        assert_eq!(expected, actual);
    }

    #[test]
    fn can_ignore_whitespace() {
        let expected = Ok(Padding::new(10, 20, 30, 40));
        let actual = " 10 , 20 , 30 , 40 ".parse();
        assert_eq!(expected, actual);
    }

    #[test]
    fn fails_with_more_than_five_values() {
        let expected: ParseResult = Err(ParseError::TooManyParts);
        let actual = "10,20,30,40,60".parse();
        assert_eq!(expected, actual);
    }

    #[test]
    fn fails_with_empty_string() {
        let actual: ParseResult = "".parse();
        assert!(actual.is_err());
        assert_eq!(
            actual
                .unwrap_err()
                .cause()
                .expect("Expected cause")
                .description(),
            "cannot parse integer from empty string"
        );
    }

    #[test]
    fn fails_with_invalid_value() {
        let actual: ParseResult = "1,this isn't an int".parse();
        assert!(actual.is_err());
        assert_eq!(
            actual
                .unwrap_err()
                .cause()
                .expect("Expected cause")
                .description(),
            "invalid digit found in string"
        );
    }
}
