mod model;
mod parser;
mod part1;
mod part2;

fn main() {
    let mut input = parser::parse();

    part1::solve(&mut input);
    part2::solve(&input);
}
