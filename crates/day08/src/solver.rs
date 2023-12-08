use crate::model::{Instruction, Node};

pub fn solve<F>(
    instructions: &Vec<Instruction>,
    nodes: &Vec<Node>,
    initial_node: &Node,
    is_last_node: F,
) -> usize
where
    F: Fn(&Node) -> bool,
{
    let mut result = 0;

    let mut curr_node = initial_node;
    let mut instruction_iter = instructions.iter();
    let mut next_instruction = instruction_iter.next();

    while let Some(instruction) = next_instruction {
        match instruction {
            Instruction::Left => curr_node = next_node_from_name(&curr_node.left, nodes),
            Instruction::Right => curr_node = next_node_from_name(&curr_node.right, nodes),
        }

        result += 1;

        if is_last_node(curr_node) {
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

    result
}

pub fn next_node_from_name<'a>(name: &str, nodes: &'a Vec<Node>) -> &'a Node {
    nodes.iter().find(|node| node.name == name).unwrap()
}
