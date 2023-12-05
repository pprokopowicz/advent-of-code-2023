mod model;
mod parser;
mod part1;
mod part2;
mod solver;

fn main() {
    let input = parser::parse();

    part1::solve(&input.0, &input.1);
    part2::solve(&input.0, &input.1);
}
