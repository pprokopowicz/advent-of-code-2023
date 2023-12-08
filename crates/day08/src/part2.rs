use crate::{
    model::{Instruction, Node},
    solver,
};

pub fn solve(instructions: &Vec<Instruction>, nodes: &Vec<Node>) {
    let curr_nodes = nodes
        .iter()
        .filter(|node| node.name.chars().last() == Option::Some('A'))
        .collect::<Vec<&Node>>();

    let result = curr_nodes
        .iter()
        .map(|initial_node| solver::solve(instructions, nodes, initial_node, is_last_node))
        .reduce(|acc, next| lcm(acc, next))
        .unwrap();

    println!("Part 2 solution: {}", result);
}

fn is_last_node(node: &Node) -> bool {
    node.name.chars().last() == Option::Some('Z')
}

fn lcm(m: usize, n: usize) -> usize {
    m * n / gcd(m, n)
}

fn gcd(m: usize, n: usize) -> usize {
    let mut m = m;
    let mut n = n;

    while m != 0 {
        let old_m = m;
        m = n % m;
        n = old_m;
    }

    n
 }