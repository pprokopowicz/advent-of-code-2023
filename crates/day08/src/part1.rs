use crate::model::{Instruction, Node};

const FIRST_NODE_NAME: &str = "AAA";
const LAST_NODE_NAME: &str = "ZZZ";

pub fn solve(instructions: &Vec<Instruction>, nodes: &Vec<Node>) {
    let mut result: usize = 0;

    let mut curr_node = next_node_from_name(FIRST_NODE_NAME, nodes);

    let mut instruction_iter = instructions.iter();
    let mut next_instruction = instruction_iter.next();

    while let Some(instruction) = next_instruction {
        match instruction {
            Instruction::Left => curr_node = next_node_from_name(&curr_node.left, nodes),
            Instruction::Right => curr_node = next_node_from_name(&curr_node.right, nodes),
        }

        result += 1;

        if curr_node.name == LAST_NODE_NAME {
            break;
        }

        next_instruction = instruction_iter.next();
        match next_instruction {
            Option::Some(_) => (),
            Option::None => {
                instruction_iter = instructions.iter();
                next_instruction = instruction_iter.next();
            }
        }
    }

    println!("Part 1 solution: {}", result);
}

fn next_node_from_name<'a>(name: &str, nodes: &'a Vec<Node>) -> &'a Node {
    nodes.iter().find(|node| node.name == name).unwrap()
}
