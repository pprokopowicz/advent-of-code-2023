#[derive(Debug, Clone)]
pub struct Point {
    pub pipe: Pipe,
    pub is_in_loop: bool,
}

impl Point {
    pub fn new(pipe: Pipe, is_in_loop: bool) -> Point {
        Point { pipe, is_in_loop }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl Pipe {
    pub fn new(value: &char) -> Pipe {
        match value {
            '|' => Pipe::Vertical,
            '-' => Pipe::Horizontal,
            'L' => Pipe::NorthEast,
            'J' => Pipe::NorthWest,
            '7' => Pipe::SouthWest,
            'F' => Pipe::SouthEast,
            '.' => Pipe::Ground,
            'S' => Pipe::Start,
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
