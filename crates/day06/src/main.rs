mod model;
mod parser;
mod part1;
mod part2;
mod solver;

fn main() {
    let (times, distances) = parser::parse();

    part1::solve(&times, &distances);
    part2::solve(&times, &distances);
}
