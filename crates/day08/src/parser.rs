use reader::{read_file, File};

use crate::model::{Instruction, Node};

pub fn parse<'a>() -> (Vec<Instruction>, Vec<Node>) {
    let contents = read_file(File::Day08);
    let mut lines = contents.lines();

    let instructions_str = lines.next().unwrap();
    let instructions = instructions_str
        .chars()
        .map(|c| match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!("Unknown instruction"),
        })
        .collect::<Vec<Instruction>>();

    _ = lines.next();

    let nodes = lines
        .map(|line| {
            let mut split = line.split(" = ");
            let current = split.next().unwrap().to_string();

            let mut comma_split = split.next().unwrap().split(", ");

            let left = comma_split.next().unwrap();
            let left = left[1..left.len()].to_string();

            let right = comma_split.next().unwrap();
            let right = right[0..right.len() - 1].to_string();

            Node::new(current, left, right)
        })
        .collect();

    (instructions, nodes)
}
