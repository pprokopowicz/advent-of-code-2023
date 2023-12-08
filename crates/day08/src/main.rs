mod model;
mod parser;
mod part1;

fn main() {
    let (instructions, nodes) = parser::parse();

    part1::solve(&instructions, &nodes);
}
