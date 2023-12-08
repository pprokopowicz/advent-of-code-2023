use crate::{
    model::{Instruction, Node},
    solver,
};

const FIRST_NODE_NAME: &str = "AAA";
const LAST_NODE_NAME: &str = "ZZZ";

pub fn solve(instructions: &Vec<Instruction>, nodes: &Vec<Node>) {
    let initial_node = solver::next_node_from_name(FIRST_NODE_NAME, nodes);
    let result = solver::solve(instructions, nodes, initial_node, is_last_node);

    println!("Part 1 solution: {}", result);
}

fn is_last_node(node: &Node) -> bool {
    node.name == LAST_NODE_NAME
}
