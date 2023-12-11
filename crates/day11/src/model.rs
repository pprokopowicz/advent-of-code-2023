#[derive(Clone, Debug, PartialEq)]
pub enum Point {
    Galaxy,
    Space,
}

impl Point {
    pub fn new(c: &char) -> Point {
        match c {
            '#' => Point::Galaxy,
            '.' => Point::Space,
            _ => panic!("Unknown point!"),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Coordinate {
    pub fn new(x: usize, y: usize) -> Coordinate {
        Coordinate { x, y }
    }
}
