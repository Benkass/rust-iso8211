use std::io::Read;

#[derive(Debug, PartialEq)]
pub(crate) enum ParseData {
    Fixed(ParseType, usize),
    Variable(ParseType),
}

#[derive(Debug, PartialEq)]
pub(crate) enum ParseType {
    Integer,
    String,
    Float,
}

#[derive(Debug, PartialEq)]
pub enum Data {
    Integer(i64),
    String(String),
    Float(f64),
}

#[derive(Debug, PartialEq)]
pub enum E {
    UnknownDataType(char),
}

enum ParseState {
    Start,
    Type,
    Num,
    Length,
}

impl ParseData {
    pub(crate) fn new(s: &str) -> Result<(usize, ParseData), E> {
        let state = ParseState::Start;
        for c in s.chars() {}
        Ok((1, ParseData::Variable(ParseType::Integer)))
    }

    pub fn parse<R: Read>(rdr: R) -> Result<Data, E> {
        Ok(Data::Integer(5))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsedata() {
        assert_eq!(
            ParseData::new("A(3)").unwrap(),
            (1, ParseData::Fixed(ParseType::String, 3))
        );
        assert_eq!(
            ParseData::new("I(10)").unwrap(),
            (1, ParseData::Fixed(ParseType::Integer, 10))
        );
        assert_eq!(
            ParseData::new("R(5)").unwrap(),
            (1, ParseData::Fixed(ParseType::Float, 5))
        );
        assert_eq!(
            ParseData::new("5R").unwrap(),
            (5, ParseData::Variable(ParseType::Float))
        );
        assert_eq!(
            ParseData::new("10I").unwrap(),
            (10, ParseData::Variable(ParseType::Integer))
        );
        assert_eq!(
            ParseData::new("1A").unwrap(),
            (1, ParseData::Variable(ParseType::String))
        );
    }
}
