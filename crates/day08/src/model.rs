#[derive(Debug)]
pub enum Instruction {
    Left,
    Right,
}

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub left: String,
    pub right: String,
}

impl Node {
    pub fn new(name: String, left: String, right: String) -> Node {
        Node { name, left, right }
    }
}
