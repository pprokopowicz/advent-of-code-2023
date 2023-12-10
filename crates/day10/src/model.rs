#[derive(Debug, PartialEq)]
pub enum Point {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl Point {
    pub fn new(value: &char) -> Point {
        match value {
            '|' => Point::Vertical,
            '-' => Point::Horizontal,
            'L' => Point::NorthEast,
            'J' => Point::NorthWest,
            '7' => Point::SouthWest,
            'F' => Point::SouthEast,
            '.' => Point::Ground,
            'S' => Point::Start,
            _ => panic!("Unknown point!"),
        }
    }
}

#[derive(Debug)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Coordinate {
    pub fn new(x: usize, y: usize) -> Coordinate {
        Coordinate { x, y }
    }
}
