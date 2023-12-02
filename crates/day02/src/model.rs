#[derive(Debug)]
pub struct Game {
    pub id: usize,
    pub sets: Vec<Set>,
}

#[derive(Debug)]
pub struct Set {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

impl Game {
    pub fn new(id: usize, sets: Vec<Set>) -> Game {
        Game { id, sets }
    }
}

impl Set {
    pub fn new(red: usize, green: usize, blue: usize) -> Set {
        Self { red, green, blue }
    }
}
