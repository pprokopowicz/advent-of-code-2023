mod model;
mod parser;
mod part1;
mod part2;
mod solver;

fn main() {
    let (instructions, nodes) = parser::parse();

    part1::solve(&instructions, &nodes);
    part2::solve(&instructions, &nodes);
}
